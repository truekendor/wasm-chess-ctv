use shakmaty::{Chess, Move, Position, fen::Fen, san::San, uci::UciMove};

use wasm_bindgen::prelude::wasm_bindgen;

mod parsing;

#[wasm_bindgen]
struct WasmChess {
    chess: Chess,
    history: Vec<Fen>,
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

        Ok(WasmChess {
            history: vec![Fen::from_position(&chess, shakmaty::EnPassantMode::Always)],
            chess: chess,
        })
    }

    pub fn make_move(&mut self, move_str: &str) -> Result<(), String> {
        let internal_move = self.str_to_move(move_str);

        let internal_move: Move = match internal_move {
            Ok(val) => val,
            Err(err) => return Err(err),
        };

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

    fn str_to_move(&self, move_str: &str) -> Result<Move, String> {
        // if move is in a UCI format we play it and return
        // else we know that there is a parsing error
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

        // if we're here it means that the move is either illegal
        // or in a SAN format
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

    fn load() {
        todo!()
    }

    pub fn fen(&self) -> String {
        let fen = Fen::from_position(&self.chess, shakmaty::EnPassantMode::Legal);

        fen.to_string()
    }

    pub fn fen_at(&self, index: usize) -> Result<String, String> {
        if index > self.history.len() {
            return Err(format!("Index out of bounds: {}", index));
        }

        let fen = &self.history[index];

        Ok(fen.to_string())
    }

    fn load_pgn() {
        todo!()
    }

    fn get_headers() {
        todo!()
    }

    fn move_number() {
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
