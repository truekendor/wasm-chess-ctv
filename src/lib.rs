use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use shakmaty::{Chess, Color, Move, Position, fen::Fen, zobrist::Zobrist64};

use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::parsing::MovesAndError;

mod native_tests;
mod parsing;

#[wasm_bindgen]
struct WasmChess {
    chess: Chess,
    history: Vec<History>,
    hash: Zobrist64,
    position_count: HashMap<Zobrist64, i32>,
}

#[wasm_bindgen]
#[derive(Serialize, Clone)]
pub struct MoveVerbose {
    from: String,
    to: String,
    san: String,
    promotion: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct JSMoveObj {
    from: String,
    to: String,
    promotion: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
struct JSPreserveHeaders {
    skip_validation: Option<bool>,
    preserve_headers: Option<bool>,
}

struct History {
    internal_move: Move,
    // maybe drop fen from history and just
    // calculate it on the fly when needed
    fen: Fen,
    move_number: u32,
    half_moves: u32,
    turn: Color,
    position: Chess,
}

#[wasm_bindgen]
impl WasmChess {
    pub fn new(fen: Option<String>) -> Result<WasmChess, String> {
        let starting_fen = fen.unwrap_or(
            Fen::from_position(&Chess::default(), shakmaty::EnPassantMode::Legal).to_string(),
        );

        let fen: Fen = match starting_fen.parse() {
            Ok(val) => val,
            Err(err) => {
                return Err(format!(
                    "Error parsing fen string\nError message: {}\n«{}» is not a valid fen",
                    err, starting_fen
                ));
            }
        };

        let chess: Chess = match fen.clone().into_position(shakmaty::CastlingMode::Chess960) {
            Ok(val) => val,
            Err(err) => {
                return Err(format!(
                    "Error converting FEN into chess position\nError message: {}\nFEN: {}",
                    err,
                    fen.to_string()
                ));
            }
        };

        let zobrist_hash: Zobrist64 = chess.zobrist_hash(shakmaty::EnPassantMode::Legal);

        let position_count: HashMap<Zobrist64, i32> = HashMap::from([(zobrist_hash, 1)]);

        Ok(WasmChess {
            chess: chess,
            hash: zobrist_hash,
            position_count,
            history: vec![],
        })
    }

    pub fn make_move(&mut self, move_str: &str) -> Result<(), String> {
        let internal_move = parsing::str_to_move(move_str, &self.chess).map_err(|err| {
            return err.to_string();
        })?;

        if self.chess.is_legal(internal_move) {
            self.push_history_entry(internal_move);
        } else {
            return Err(format!("Illegal move: {}\nFEN: {}", move_str, self.fen()));
        }

        match self.chess.clone().play(internal_move) {
            Ok(val) => {
                self.chess = val;

                self.hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);
                *self.position_count.entry(self.hash).or_insert(0) += 1;

                return Ok(());
            }
            Err(err) => {
                return Err(format!(
                    "Error: {}\nMove attempted: {}\nFEN: {}",
                    err.to_string(),
                    move_str,
                    self.fen()
                ));
            }
        }
    }

    pub fn make_move_from_obj(&mut self, move_obj: JsValue) -> Result<(), String> {
        if !move_obj.is_object() {
            return Err("Input is not an object".to_string());
        }

        let move_obj: JSMoveObj =
            serde_wasm_bindgen::from_value(move_obj).map_err(|err| err.to_string())?;

        let mut uci = format!("{}{}", move_obj.from, move_obj.to);

        if let Some(promo) = move_obj.promotion {
            uci.push_str(&promo.to_lowercase());
        }

        self.make_move(&uci)
    }

    pub fn reset(&mut self) {
        let chess = Chess::default();
        self.hash = chess.zobrist_hash(shakmaty::EnPassantMode::Legal);

        self.chess = chess;
        self.history.clear();
        self.position_count = HashMap::from([(self.hash, 1)]);
    }

    // Idk what it is doing in chess.js but it is different from reset, so I am keeping it for now
    pub fn clear(&mut self) {
        let fen: Fen = "8/8/8/8/8/8/8/8 w - - 0 1"
            .parse()
            .expect("valid empty FEN");

        let chess: Chess = fen
            .clone()
            .into_position(shakmaty::CastlingMode::Chess960)
            .expect("valid position");

        self.hash = chess.zobrist_hash(shakmaty::EnPassantMode::Legal);
        self.chess = chess;
        self.history.clear();
        self.position_count = HashMap::from([(self.hash, 1)]);
    }

    pub fn load(
        &mut self,
        starting_fen: String,
        // TODO: I don't even know if we can just skip fen validation
        // options: JsValue
    ) -> Result<(), String> {
        self.history.clear();
        self.hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);
        self.position_count = HashMap::from([(self.hash, 1)]);

        let fen: Fen = starting_fen.parse::<Fen>().map_err(|err| {
            return format!(
                "Error parsing fen string\nError message: {}\n«{}» is not a valid fen",
                err, starting_fen
            );
        })?;

        self.chess = fen
            .clone()
            .into_position(shakmaty::CastlingMode::Chess960)
            .map_err(|err| {
                return format!(
                    "Error converting FEN into chess position\nError message: {}\nFEN: {}",
                    err, fen
                );
            })?;

        Ok(())
    }

    pub fn fen(&self) -> String {
        let fen = Fen::from_position(&self.chess, shakmaty::EnPassantMode::Legal);

        fen.to_string()
    }

    // as for now the api of this is strange since
    // without any moves played it will return `None`
    pub fn fen_at(&self, index: usize) -> Option<String> {
        if index >= self.history.len() {
            return None;
        }

        let fen = &self.history[index].fen;

        Some(fen.to_string())
    }

    pub fn undo(&mut self) -> Result<String, String> {
        let last = match self.history.pop() {
            Some(h) => h,
            None => return Err("No moves to undo".to_string()),
        };

        if let Some(count) = self.position_count.get_mut(&self.hash) {
            *count -= 1;
            if *count <= 0 {
                self.position_count.remove(&self.hash);
            }
        }
        self.chess = last.position;

        self.hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);

        self.position_count.entry(self.hash).or_insert(1);

        Ok(last.internal_move.to_string())
    }

    // TODO!!!!
    pub fn legal_moves(&self) -> Vec<String> {
        let aa = self.chess.legal_moves();
        todo!()
    }

    pub fn fullmoves(&self) -> u32 {
        let move_number = &self.chess.fullmoves();

        move_number.get()
    }

    pub fn halfmoves(&self) -> u32 {
        self.chess.halfmoves()
    }

    pub fn is_game_over(&self) -> bool {
        self.chess.is_game_over() || self.is_draw_by_fifty_moves() || self.is_threefold_repetition()
    }

    pub fn is_check(&self) -> bool {
        self.chess.is_check()
    }

    pub fn is_checkmate(&self) -> bool {
        self.chess.is_checkmate()
    }

    pub fn is_draw_by_fifty_moves(&self) -> bool {
        self.chess.halfmoves() >= 100
    }

    pub fn is_insufficient_material(&self) -> bool {
        self.chess.is_insufficient_material()
    }

    pub fn is_threefold_repetition(&self) -> bool {
        match self.position_count.get(&self.hash) {
            Some(val) => {
                return *val >= 3;
            }
            None => false,
        }
    }

    pub fn is_draw(&self) -> bool {
        self.chess.is_stalemate()
            || self.chess.is_insufficient_material()
            || self.is_draw_by_fifty_moves()
            || self.is_threefold_repetition()
    }

    pub fn turn(&self) -> String {
        match self.chess.turn() {
            Color::White => "w".to_string(),
            Color::Black => "b".to_string(),
        }
    }

    // TODO implement board state as array of 64 squares not vector ?
    // TODO make working piece to string parser
    pub fn board(&self) -> Vec<String> {
        // Square::ALL
        //     .iter()
        //     .map(|sq| self.chess.board().piece_at(*sq).map(|p| p.to_string()))
        //     .collect()

        todo!()
    }

    pub fn get(&self, square: String) -> Option<String> {
        let sq: shakmaty::Square = square.parse().ok()?;
        let piece = self.chess.board().piece_at(sq);
        let char = match piece {
            // TODO handle panic
            Some(p) => Some(p.char()).unwrap(),
            None => {
                return None;
            }
        };

        Some(char.to_string())
    }

    pub fn put(&mut self, piece: String, square: String) -> Result<(), String> {
        todo!()
    }

    pub fn history(&self) -> Result<Vec<String>, String> {
        Ok(self
            .history
            .iter()
            .map(|h| h.internal_move.to_string())
            .collect())
    }

    // TODO upgrade to return structs later???
    // TODO -> Result<VEc<MoveObj>, String> or something like that
    pub fn history_verbose(&self) -> Result<Vec<String>, String> {
        Ok(self
            .history
            .iter()
            .map(|h| {
                format!(
                    "move: {}, fen: {}, turn: {:?}",
                    h.internal_move,
                    h.fen.to_string(),
                    h.turn
                )
            })
            .collect())
    }

    fn push_history_entry(&mut self, internal_move: Move) {
        self.history.push(History {
            internal_move,
            fen: Fen::from_position(&self.chess, shakmaty::EnPassantMode::Legal),

            move_number: self.fullmoves(),
            half_moves: self.halfmoves(),
            turn: self.chess.turn(),
            position: self.chess.clone(),
        });
    }

    /// converts Vec of uci moves `Vec<["e2e4", "e7e5", ...]>`, into Vec of SAN moves
    pub fn uci_to_san(
        &self,
        uci_moves: Vec<String>,
        starting_fen: Option<String>,
    ) -> MovesAndError {
        parsing::uci_to_san(uci_moves, starting_fen)
    }

    fn load_pgn() {
        todo!()
    }

    fn get_headers() {
        todo!()
    }

    fn get_comments() {
        todo!()
    }

    fn set_comment() {
        todo!()
    }
}
