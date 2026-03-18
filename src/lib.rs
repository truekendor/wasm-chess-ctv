use std::usize;

use shakmaty::{Chess, Position, fen::Fen, san::San, uci::UciMove};

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

#[wasm_bindgen]
struct WasmChess {
    chess: Chess,
}

#[wasm_bindgen]
impl WasmChess {
    pub fn new() -> WasmChess {
        WasmChess {
            chess: Chess::default(),
        }
    }

    // currently not doing anything
    fn make_move(&mut self, uci_move_str: &str) -> Result<(), String> {
        let ascii: &[u8] = uci_move_str.as_bytes();

        let move_uci: UciMove = match UciMove::from_ascii(ascii) {
            Ok(val) => val,
            Err(err) => {
                console::log_1(&format!("{}", err).into());
                return Err(err.to_string());
            }
        };

        let internal_move = match move_uci.to_move(&self.chess) {
            Ok(val) => val,
            Err(err) => {
                console::log_1(&format!("{}", err).into());
                return Err(err.to_string());
            }
        };

        self.chess = self
            .chess
            .clone()
            .play(internal_move)
            .map_err(|e| format!("Failed to make move: {}", e))?;

        Ok(())
    }

    pub fn uci_to_san(
        &mut self,
        uci_moves: Vec<String>,
        starting_fen: Option<String>,
    ) -> Result<Vec<String>, String> {
        let starting_fen = starting_fen.unwrap_or_else(|| {
            console::log_1(&format!("Argument for starting FEN was not provided.\nAttempting use FEN of a stating position",).into());

            Fen::from_position(&Chess::default(), shakmaty::EnPassantMode::Legal).to_string()
        });

        let mut san_moves_vec: Vec<String> = vec![];

        let fen: Fen = match starting_fen.parse() {
            Ok(val) => val,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        let mut chess_pos: Chess = match fen.clone().into_position(shakmaty::CastlingMode::Chess960)
        {
            Ok(val) => val,
            Err(err) => {
                return Err(format!(
                    "Error converting FEN into position: {}\nPassed FEN: {}",
                    err,
                    fen.to_string()
                ));
            }
        };

        for uci_move_str in uci_moves {
            let ascii_move = uci_move_str.as_bytes();

            let move_uci: UciMove = match UciMove::from_ascii(ascii_move) {
                Ok(val) => val,
                Err(err) => {
                    return Err(format!(
                        "Error converting UCI from ascii\nError msg: {}. Attempted conversion: {}",
                        err, uci_move_str
                    ));
                }
            };

            let internal_move = match move_uci.to_move(&chess_pos) {
                Ok(val) => val,
                Err(err) => {
                    return Err(format!(
                        "{}. Failed UCI move conversion: {}",
                        err, uci_move_str
                    ));
                }
            };

            let san_move = San::from_move(&chess_pos, internal_move);
            san_moves_vec.push(san_move.to_string());

            chess_pos = match chess_pos.play(internal_move) {
                Ok(val) => val,
                Err(err) => {
                    return Err(format!(
                        "{}\nAttempted to play: {}\nFen: {}",
                        err,
                        internal_move.to_string(),
                        fen.to_string()
                    ));
                }
            };
        }

        Ok(san_moves_vec)
    }

    // converts Vec<string> of a PV's in a format of "e2e4 e7e5 ..." into Vec of SAN moves
    pub fn uci_pv_to_san(
        &mut self,
        uci_moves: Vec<String>,
        starting_fen: Option<String>,
    ) -> Result<Vec<String>, String> {
        let starting_fen = starting_fen.unwrap_or(
            Fen::from_position(&Chess::default(), shakmaty::EnPassantMode::Legal).to_string(),
        );

        let mut san_moves_vec: Vec<String> = vec![];

        let fen: Fen = match starting_fen.parse() {
            Ok(val) => val,
            Err(err) => {
                console::log_1(&format!("{}", err).into());
                return Err(err.to_string());
            }
        };

        let mut chess_pos: Chess = match fen.into_position(shakmaty::CastlingMode::Chess960) {
            Ok(val) => val,
            Err(err) => {
                console::log_1(&format!("{}", err).into());
                return Err(err.to_string());
            }
        };

        for uci_move_str in uci_moves {
            let mut chess_pos_pv = chess_pos.clone();
            let mut pv_vec: Vec<String> = vec![];
            let mut first_move = true;

            for pv_uci_move in uci_move_str.split_ascii_whitespace() {
                let ascii_move = pv_uci_move.as_bytes();

                let move_uci: UciMove = match UciMove::from_ascii(ascii_move) {
                    Ok(val) => val,
                    Err(err) => {
                        console::log_1(
                            &format!("{}. Attempted conversion: {}", err, uci_move_str).into(),
                        );
                        return Err(err.to_string());
                    }
                };

                let internal_move = match move_uci.to_move(&chess_pos_pv) {
                    Ok(val) => val,
                    Err(err) => {
                        console::log_1(
                            &format!("{}\nIllegal internal move: {}", err, uci_move_str).into(),
                        );
                        return Err(err.to_string());
                    }
                };

                let san_move = San::from_move(&chess_pos_pv, internal_move);
                pv_vec.push(san_move.to_string());

                chess_pos_pv = match chess_pos_pv.play(internal_move) {
                    Ok(val) => val,
                    Err(err) => {
                        console::log_1(
                            &format!(
                                "{}\nAttempted move play: {}",
                                err,
                                internal_move.to_string()
                            )
                            .into(),
                        );
                        return Err(err.to_string());
                    }
                };

                if first_move {
                    chess_pos = chess_pos_pv.clone();
                    first_move = false;
                }
            }

            let result_san_str = pv_vec.join(" ");
            san_moves_vec.push(result_san_str);
        }

        Ok(san_moves_vec)
    }
}
