use core::fmt;

use serde::{Deserialize, Serialize};
use shakmaty::{Chess, Move, Position, fen::Fen, san::San, uci::UciMove};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug)]
pub enum MoveParseError {
    IllegalMove { move_str: String, fen: String },
    InvalidFormat(String),
}

impl fmt::Display for MoveParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveParseError::IllegalMove { move_str, fen } => {
                write!(f, "Illegal move: {}\nFEN before move: {}", move_str, fen)
            }
            MoveParseError::InvalidFormat(msg) => write!(f, "Invalid move format: {}", msg),
        }
    }
}
impl std::error::Error for MoveParseError {}

#[derive(tsify::Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MovesAndError {
    pub moves: Vec<String>,
    pub message: Option<String>,
}

/// converts Vec of moves in SAN/LAN format, into Vec of SAN moves
#[wasm_bindgen(js_name = "toSan")]
pub fn to_san(moves: Vec<String>, starting_fen: Option<String>) -> MovesAndError {
    let starting_fen = starting_fen.unwrap_or_else(|| {
        Fen::from_position(&Chess::default(), shakmaty::EnPassantMode::Legal).to_string()
    });

    let mut san_moves_vec: Vec<String> = vec![];

    let fen: Fen = match starting_fen.parse() {
        Ok(val) => val,
        Err(err) => {
            return MovesAndError {
                moves: san_moves_vec,
                message: Some(err.to_string()),
            };
        }
    };

    let mut chess_pos: Chess = match fen.clone().into_position(shakmaty::CastlingMode::Chess960) {
        Ok(val) => val,
        Err(err) => {
            return MovesAndError {
                moves: san_moves_vec,
                message: Some(format!(
                    "Error converting FEN into position: {}\nStarting FEN: {}",
                    err,
                    fen.to_string()
                )),
            };
        }
    };

    for uci_move_str in moves {
        let internal_move: Move = match str_to_move(&uci_move_str, &chess_pos) {
            Ok(val) => val,
            Err(err) => {
                return MovesAndError {
                    moves: san_moves_vec,
                    message: Some(err.to_string()),
                };
            }
        };

        let san_move = San::from_move(&chess_pos, internal_move);

        chess_pos = match chess_pos.play(internal_move) {
            Ok(val) => val,
            Err(err) => {
                return MovesAndError {
                    moves: san_moves_vec,
                    message: Some(format!(
                        "{}\nAttempted to play: Fen: {}",
                        err.to_string(),
                        fen.to_string()
                    )),
                };
            }
        };

        san_moves_vec.push(san_move.to_string());
    }

    MovesAndError {
        moves: san_moves_vec,
        message: None,
    }
}

/// converts Vec of moves in SAN/LAN format, into Vec of UCI moves
#[wasm_bindgen(js_name = "toUci")]
pub fn to_uci(moves: Vec<String>, starting_fen: Option<String>) -> MovesAndError {
    let starting_fen = starting_fen.unwrap_or_else(|| {
        Fen::from_position(&Chess::default(), shakmaty::EnPassantMode::Legal).to_string()
    });

    let mut uci_moves_vec: Vec<String> = vec![];

    let fen: Fen = match starting_fen.parse() {
        Ok(val) => val,
        Err(err) => {
            return MovesAndError {
                moves: uci_moves_vec,
                message: Some(err.to_string()),
            };
        }
    };

    let mut chess_pos: Chess = match fen.clone().into_position(shakmaty::CastlingMode::Chess960) {
        Ok(val) => val,
        Err(err) => {
            return MovesAndError {
                moves: uci_moves_vec,
                message: Some(format!(
                    "Error converting FEN into position: {}\nStarting FEN: {}",
                    err,
                    fen.to_string()
                )),
            };
        }
    };

    for san_move_str in moves {
        let internal_move: Move = match str_to_move(&san_move_str, &chess_pos) {
            Ok(val) => val,
            Err(err) => {
                return MovesAndError {
                    moves: uci_moves_vec,
                    message: Some(err.to_string()),
                };
            }
        };

        let uci_move = UciMove::from_move(internal_move, shakmaty::CastlingMode::Chess960);

        chess_pos = match chess_pos.play(internal_move) {
            Ok(val) => val,
            Err(err) => {
                return MovesAndError {
                    moves: uci_moves_vec,
                    message: Some(format!(
                        "{}\nAttempted to play: Fen: {}",
                        err.to_string(),
                        fen.to_string()
                    )),
                };
            }
        };

        uci_moves_vec.push(uci_move.to_string());
    }

    MovesAndError {
        moves: uci_moves_vec,
        message: None,
    }
}

pub fn str_to_move(move_str: &str, chess: &Chess) -> Result<Move, MoveParseError> {
    // if move is in a UCI format we
    // try to return it immediately
    //
    // otherwise we know that there is a parsing error
    // because move is either in SAN format or illegal
    if let Ok(move_uci) = move_str.parse::<UciMove>() {
        match move_uci.to_move(chess) {
            Ok(valid_move) => return Ok(valid_move),
            Err(_) => {
                return Err(MoveParseError::IllegalMove {
                    move_str: move_str.to_string(),
                    fen: Fen::from_position(chess, shakmaty::EnPassantMode::Legal).to_string(),
                });
            }
        }
    }

    // Try SAN format if UCI parsing failed or UCI move was invalid
    if let Ok(move_san) = move_str.parse::<San>() {
        match move_san.to_move(chess) {
            Ok(valid_move) => return Ok(valid_move),
            Err(_) => {
                return Err(MoveParseError::IllegalMove {
                    move_str: move_str.to_string(),
                    fen: Fen::from_position(chess, shakmaty::EnPassantMode::Legal).to_string(),
                });
            }
        }
    }

    Err(MoveParseError::InvalidFormat(move_str.to_string()))
}

// TODO: return something like MovesAndError
fn to_internal_moves(moves: Vec<String>, starting_fen: Option<String>) -> Vec<Move> {
    let starting_fen = starting_fen.unwrap_or_else(|| {
        Fen::from_position(&Chess::default(), shakmaty::EnPassantMode::Legal).to_string()
    });

    let mut internal_moves_list: Vec<Move> = vec![];

    let fen: Fen = match starting_fen.parse() {
        Ok(val) => val,
        Err(_err) => return internal_moves_list,
    };

    let mut chess_pos: Chess = match fen.clone().into_position(shakmaty::CastlingMode::Chess960) {
        Ok(val) => val,
        Err(_err) => return internal_moves_list,
    };

    for move_str in moves {
        let internal_move: Move = match str_to_move(&move_str, &chess_pos) {
            Ok(val) => val,
            Err(_err) => return internal_moves_list,
        };

        if !chess_pos.is_legal(internal_move) {
            return internal_moves_list;
        }

        chess_pos.play_unchecked(internal_move);

        internal_moves_list.push(internal_move);
    }

    internal_moves_list
}
