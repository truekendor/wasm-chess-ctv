use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use shakmaty::{Chess, Move, Position, fen::Fen, san::San, uci::UciMove, zobrist::Zobrist64};

use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

mod parsing;

#[wasm_bindgen]
struct WasmChess {
    chess: Chess,
    history: Vec<Fen>,
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
            history: vec![fen],
            chess: chess,
            hash: zobrist_hash,
            position_count,
        })
    }

    pub fn make_move(&mut self, move_str: &str) -> Result<(), String> {
        let internal_move = self.str_to_move(move_str);

        let internal_move: Move = match internal_move {
            Ok(val) => val,
            Err(err) => return Err(err),
        };

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
                self.history.push(fen);

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

    fn str_to_move(&self, move_str: &str) -> Result<Move, String> {
        // if move is in a UCI format we immediately
        // try to return it
        // otherwise we know that there is a parsing error
        // because move is either in SAN format or illegal
        match move_str.parse::<UciMove>() {
            Ok(val) => match val.to_move(&self.chess) {
                Ok(valid_move) => return Ok(valid_move),
                Err(err) => {
                    return Err(format!(
                        "Error: {}\nMove: {}\nFEN: {}",
                        err.to_string(),
                        move_str,
                        self.fen()
                    ));
                }
            },
            Err(_) => {
                // parsing error that we handle later
            }
        };

        // if we're here it means that the move is either
        // illegal or in a SAN format
        match move_str.parse::<San>() {
            Ok(val) => match val.to_move(&self.chess) {
                Ok(valid_move) => return Ok(valid_move),
                Err(err) => {
                    return Err(format!(
                        "Error: {}\nMove: {}\nFEN: {}",
                        err.to_string(),
                        move_str,
                        self.fen()
                    ));
                }
            },
            Err(_) => {
                // parsing error that we handle later
            }
        };

        Err(format!(
            "Failed to parse move «{}»\nOnly moves in SAN or UCI formats allowed",
            move_str
        ))
    }

    pub fn reset() {
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

        self.history.push(fen);

        Ok(())
    }

    pub fn fen(&self) -> String {
        let fen = Fen::from_position(&self.chess, shakmaty::EnPassantMode::Legal);

        fen.to_string()
    }

    pub fn fen_at(&self, index: usize) -> Result<String, String> {
        if index >= self.history.len() {
            return Err(format!("Index out of bounds: {}", index));
        }

        let fen = &self.history[index];

        Ok(fen.to_string())
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
        let count = self.position_count.get(&self.hash);

        if count.is_some() {
            let count = count.unwrap();

            return *count >= 3;
        }

        false
    }

    pub fn is_draw(&self) -> bool {
        self.chess.is_stalemate()
            || self.chess.is_insufficient_material()
            || self.is_draw_by_fifty_moves()
            || self.is_threefold_repetition()
    }

    fn history(&self) {
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
    ) -> Result<Vec<String>, String> {
        let san_moves_vec = match parsing::uci_to_san(uci_moves, starting_fen) {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        };

        Ok(san_moves_vec)
    }

    /// converts Vec<string> of a  PV's into Vec of SAN moves
    /// PV is a string of UCI moves separated by a whitespace char, like "e2e4 e7e6 b1c3"
    pub fn uci_pv_to_san(
        &self,
        uci_moves: Vec<String>,
        starting_fen: Option<String>,
    ) -> Result<Vec<String>, String> {
        let san_moves_vec = match parsing::uci_pv_to_san(uci_moves, starting_fen) {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        };

        Ok(san_moves_vec)
    }
}
