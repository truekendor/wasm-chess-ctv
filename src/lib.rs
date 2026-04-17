use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use shakmaty::{Chess, Color, Move, Position, Square, fen::Fen, san::San, zobrist::Zobrist64};

use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::{parsing::MovesAndError, pgn_loader::pgn_reader::parse_pgn};

mod get_legal_moves;
mod parsing;
mod pgn_loader;
mod tests;

#[wasm_bindgen]
struct WasmChess {
    chess: Chess,
    history: Vec<History>,
    hash: Zobrist64,
    position_count: HashMap<Zobrist64, i32>,
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
    #[wasm_bindgen(constructor)]
    pub fn new(fen: Option<String>) -> Result<WasmChess, String> {
        let starting_fen: String = fen.unwrap_or(
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

    #[wasm_bindgen(js_name = "move")]
    pub fn make_move(&mut self, move_str: &str) -> Result<(), String> {
        let internal_move = parsing::str_to_move(move_str, &self.chess).map_err(|err| {
            return err.to_string();
        })?;

        if !self.chess.is_legal(internal_move) {
            return Err(format!("Illegal move: {}\nFEN: {}", move_str, self.fen()));
        }

        self.push_history_entry(internal_move);

        self.chess.play_unchecked(internal_move);

        self.hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);
        *self.position_count.entry(self.hash).or_insert(0) += 1;

        return Ok(());
    }

    fn make_move_from_obj(&mut self, move_obj: JsValue) -> Result<(), String> {
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

    /// resets to default starting position
    ///
    /// TODO: need to double-check what is does in chess.js
    pub fn reset(&mut self) {
        self.chess = Chess::default();
        self.hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);

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

        self.hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);
        self.position_count = HashMap::from([(self.hash, 1)]);

        Ok(())
    }

    pub fn fen(&self) -> String {
        let fen = Fen::from_position(&self.chess, shakmaty::EnPassantMode::Always);

        fen.to_string()
    }

    // as for now the api of this is strange since
    // without any moves played it will return `None`
    #[wasm_bindgen(js_name = "fenAt")]
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

    #[wasm_bindgen(js_name = "legalMovesUCI")]
    pub fn legal_moves_uci(&self) -> Vec<String> {
        get_legal_moves::uci(&self.chess)
    }

    #[wasm_bindgen(js_name = "legalMovesSAN")]
    pub fn legal_moves_san(&self) -> Vec<String> {
        get_legal_moves::san(&self.chess)
    }

    // fn legal_moves_strict(&self) ->
    // Vec<StrictMove>

    //  {
    //     todo!()
    // }

    pub fn perft(&self, depth: usize) -> u64 {
        shakmaty::perft(&self.chess, depth as u32)
    }

    pub fn fullmoves(&self) -> u32 {
        let move_number = &self.chess.fullmoves();

        move_number.get()
    }

    pub fn halfmoves(&self) -> u32 {
        self.chess.halfmoves()
    }

    #[wasm_bindgen(js_name = "isGameOver")]
    pub fn is_game_over(&self) -> bool {
        self.chess.is_game_over() || self.is_draw_by_fifty_moves() || self.is_threefold_repetition()
    }

    #[wasm_bindgen(js_name = "isCheck")]
    pub fn is_check(&self) -> bool {
        self.chess.is_check()
    }

    #[wasm_bindgen(js_name = "isCheckmate")]
    pub fn is_checkmate(&self) -> bool {
        self.chess.is_checkmate()
    }

    #[wasm_bindgen(js_name = "isDrawByFiftyMoves")]
    pub fn is_draw_by_fifty_moves(&self) -> bool {
        self.chess.halfmoves() >= 100
    }

    #[wasm_bindgen(js_name = "isInsufficientMaterial")]
    pub fn is_insufficient_material(&self) -> bool {
        self.chess.is_insufficient_material()
    }

    #[wasm_bindgen(js_name = "isThreefoldRepetition")]
    pub fn is_threefold_repetition(&self) -> bool {
        match self.position_count.get(&self.hash) {
            Some(val) => {
                return *val >= 3;
            }
            None => false,
        }
    }

    #[wasm_bindgen(js_name = "isDraw")]
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

    pub fn board(&self) -> Vec<String> {
        let result: Vec<String> = Square::ALL
            .iter()
            .map(|sq| {
                let piece = self.chess.board().piece_at(*sq);

                match piece {
                    Some(p) => p.char().to_string(),
                    None => " ".to_string(),
                }
            })
            .collect();

        result
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

    fn put(&mut self, piece: String, square: String) -> Result<(), String> {
        todo!()
    }

    #[wasm_bindgen(js_name = "historySAN")]
    pub fn history_san(&self) -> Result<Vec<String>, String> {
        Ok(self
            .history
            .iter()
            .map(|history| {
                let san_move = San::from_move(&history.position, history.internal_move);

                san_move.to_string()
            })
            .collect())
    }

    #[wasm_bindgen(js_name = "historyUCI")]
    pub fn history_uci(&self) -> Result<Vec<String>, String> {
        Ok(self
            .history
            .iter()
            .map(|h| {
                let uci_move = h.internal_move.to_uci(shakmaty::CastlingMode::Chess960);

                uci_move.to_string()
            })
            .collect())
    }

    // TODO upgrade to return structs later???
    // TODO -> Result<VEc<MoveObj>, String> or something like that
    fn history_verbose(&self) -> Result<Vec<String>, String> {
        Ok(self
            .history
            .iter()
            .map(|history| {
                format!(
                    "move: {}, fen: {}, turn: {:?}",
                    history.internal_move,
                    history.fen.to_string(),
                    history.turn
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

    /// converts Vec of UCI moves `Vec<["e2e4", "e7e5", ...]>`, into Vec of SAN moves
    #[wasm_bindgen(js_name = "uciToSan")]
    pub fn uci_to_san(
        &self,
        uci_moves: Vec<String>,
        starting_fen: Option<String>,
    ) -> MovesAndError {
        parsing::uci_to_san(uci_moves, starting_fen)
    }

    /// converts Vec of SAN moves `Vec<["e4", "e5", "Nf3", ...]>`, into Vec of UCI moves
    #[wasm_bindgen(js_name = "sanToUci")]
    pub fn san_to_uci(
        &self,
        san_moves: Vec<String>,
        starting_fen: Option<String>,
    ) -> MovesAndError {
        parsing::san_to_uci(san_moves, starting_fen)
    }

    fn set_fen(&mut self, fen: Fen) -> Result<(), String> {
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

        self.position_count = position_count;
        self.hash = zobrist_hash;
        self.chess = chess;

        Ok(())
    }

    fn load_pgn(&mut self, pgn: String) -> Result<(), String> {
        let pgn_headers = parse_pgn(pgn);

        if let Err(pgn_error) = pgn_headers {
            return Err(format!("something something {}", pgn_error));
        }

        let pgn_headers = pgn_headers.unwrap();

        let starting_fen = pgn_headers.starting_fen;
        let moves_list = pgn_headers.move_list.iter();

        self.set_fen(starting_fen)?;

        for san_move in moves_list {
            self.make_move(san_move)?;
        }

        return Ok(());
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
