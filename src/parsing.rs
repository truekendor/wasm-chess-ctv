use core::fmt;

use shakmaty::{Chess, Move, Position, fen::Fen, san::San, uci::UciMove};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct MovesAndError {
    moves: Vec<String>,
    message: Option<String>,
}

#[wasm_bindgen]
impl MovesAndError {
    #[wasm_bindgen(getter)]
    pub fn moves(&self) -> Vec<String> {
        self.moves.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> Option<String> {
        self.message.clone()
    }
}

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

/// converts Vec of uci moves `Vec<["e2e4", "e7e5", ...]>`, into Vec of SAN moves
pub fn uci_to_san(uci_moves: Vec<String>, starting_fen: Option<String>) -> MovesAndError {
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

    for uci_move_str in uci_moves {
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

pub fn str_to_move(move_str: &str, chess: &Chess) -> Result<Move, MoveParseError> {
    // if move is in a UCI format we immediately
    // try to return it
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
