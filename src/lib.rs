use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use shakmaty::{
    Chess, Color, Move, Position,
    fen::{self, Fen},
    zobrist::Zobrist64,
};

use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use web_sys::console;

use crate::parsing::ErrorWithValue;

mod native_tests;
mod parsing;

#[wasm_bindgen]
struct WasmChess {
    chess: Chess,
    // TODO remove
    fen_history: Vec<Fen>,
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
            fen_history: vec![fen],
            chess: chess,
            hash: zobrist_hash,
            position_count,
            history: vec![],
        })
    }

    pub fn make_move(&mut self, move_str: &str) -> Result<(), String> {
        let internal_move = parsing::str_to_move(move_str, &self.chess);

        let internal_move: Move = match internal_move {
            Ok(val) => val,
            Err(err) => return Err(err),
        };

        self.push_history_entry(internal_move);

        if self.chess.is_legal(internal_move) {
            let zobrist_hash_update: Zobrist64 = match self.chess.update_zobrist_hash(
                self.hash,
                internal_move,
                shakmaty::EnPassantMode::Legal,
            ) {
                Some(val) => val,
                None => {
                    let zobrist_hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);

                    zobrist_hash
                }
            };

            self.hash = zobrist_hash_update;

            match self.position_count.get_mut(&zobrist_hash_update) {
                Some(val) => {
                    *val += 1;
                }
                None => {
                    self.position_count.insert(zobrist_hash_update, 1);
                }
            }
        }

        match self.chess.clone().play(internal_move) {
            Ok(val) => {
                self.chess = val;

                let fen = Fen::from_position(&self.chess, shakmaty::EnPassantMode::Legal);
                self.fen_history.push(fen);

                return Ok(());
            }
            Err(err) => {
                return Err(format!(
                    "Error: {}\nMove attempted: {}\nWith FEN: {}",
                    err.to_string(),
                    move_str,
                    self.fen()
                ));
            }
        }
    }

    fn make_move_from_obj(&mut self, move_obj: JsValue) -> Result<(), String> {
        if !move_obj.is_object() {
            return Err(format!("Input is not an object"));
        }

        let move_obj: JSMoveObj = match serde_wasm_bindgen::from_value::<JSMoveObj>(move_obj) {
            Ok(val) => val,
            Err(err) => {
                return Err(format!("{}", err.to_string()));
            }
        };

        // need to handle captures, castles, en passant

        todo!()
    }

    fn reset(&self) {
        todo!()
    }

    fn clear() {
        todo!()
    }

    pub fn load(
        &mut self,
        starting_fen: String,
        // TODO: I don't even know if we can just skip fen validation
        // options: JsValue
    ) -> Result<(), String> {
        self.history.clear();

        // let options = match serde_wasm_bindgen::from_value::<JSPreserveHeaders>(options) {
        //     Ok(val) => val,
        //     Err(err) => {
        //         // idk if we should return an error if option parsing went wrong

        //         JSPreserveHeaders::default()
        //     }
        // };

        let fen: Fen = match starting_fen.parse() {
            Ok(val) => val,
            Err(err) => {
                return Err(format!(
                    "Error parsing fen string\nError message: {}\n«{}» is not a valid fen",
                    err, starting_fen
                ));
            }
        };

        self.chess = match fen.clone().into_position(shakmaty::CastlingMode::Chess960) {
            Ok(val) => val,
            Err(err) => {
                return Err(format!(
                    "Error converting FEN into chess position\nError message: {}\nFEN: {}",
                    err,
                    fen.to_string()
                ));
            }
        };

        self.fen_history.push(fen);

        Ok(())
    }

    pub fn fen(&self) -> String {
        let fen = Fen::from_position(&self.chess, shakmaty::EnPassantMode::Legal);

        fen.to_string()
    }

    pub fn fen_at(&self, index: usize) -> String {
        if index >= self.history.len() {
            return Fen::from_position(&self.chess, shakmaty::EnPassantMode::Legal).to_string();
        }

        let fen = &self.history[index].fen;

        fen.to_string()
    }

    fn undo() {
        todo!()
    }

    fn load_pgn() {
        todo!()
    }

    fn get_headers() {
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

    pub fn history(&self) {
        todo!()
    }

    fn get_comments() {
        todo!()
    }

    fn set_comment() {
        todo!()
    }

    /// converts Vec of uci moves `Vec<["e2e4", "e7e5", ...]>`, into Vec of SAN moves
    pub fn uci_to_san(
        &self,
        uci_moves: Vec<String>,
        starting_fen: Option<String>,
    ) -> Result<Vec<String>, JsValue> {
        let san_moves_vec: Vec<String> =
            parsing::uci_to_san(uci_moves, starting_fen).map_err(|err| {
                return self.convert_error_to_js_value(err);
            })?;

        Ok(san_moves_vec)
    }

    /// converts Vec<string> of a  PV's into Vec of SAN moves
    /// PV is a string of UCI moves separated by a whitespace char, like "e2e4 e7e6 b1c3"
    pub fn uci_pv_to_san(
        &self,
        uci_moves: Vec<String>,
        starting_fen: Option<String>,
    ) -> Result<Vec<String>, JsValue> {
        let san_moves_vec: Vec<String> =
            parsing::uci_pv_to_san(uci_moves, starting_fen).map_err(|err| {
                return self.convert_error_to_js_value(err);
            })?;

        Ok(san_moves_vec)
    }

    fn convert_error_to_js_value(&self, err: ErrorWithValue) -> JsValue {
        let error_with_value = match serde_wasm_bindgen::to_value(&err) {
            Ok(val) => val,
            Err(err) => {
                console::log_1(
                    &format!(
                        "Failed to convert error with value to JsValue: {}",
                        err.to_string()
                    )
                    .into(),
                );

                return JsValue::null();
            }
        };

        return error_with_value;
    }
}
