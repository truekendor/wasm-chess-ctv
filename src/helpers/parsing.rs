use core::fmt;

use serde::{Deserialize, Serialize};
use shakmaty::{Chess, Color, Move, Position, Role, fen::Fen, san::San, uci::UciMove};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    MoveString,
    tsify_structs::{MoveVerbose, PieceSymbol, SquareStr, others::ColorChar},
};

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
    pub moves: Vec<MoveString>,
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

/// Converts a raw chess move into a verbose move object containing comprehensive move metadata.
///
/// # Important Safety Note
/// **Move validation is the caller's responsibility.** This function plays the move unchecked
/// using `play_unchecked()`, assuming the move is already validated as legal by the caller.
/// Passing an illegal move may result in an invalid board state or panics.
///
/// # Mutation Note
/// This function does not mutate the original `WasmChess` struct or the provided `chess_pos`
/// reference. The position is cloned internally, and all mutations happen on the clone.
/// The original position remains unchanged.
///
/// # Parameters
/// - `raw_move`: The raw move to convert and apply to the position
/// - `chess_pos`: The current chess position before the move is played
///
/// # Returns
/// A `MoveVerbose` struct containing:
/// - Algebraic notation (SAN and LAN/UCI formats)
/// - Piece and capture information
/// - Castle detection flags
/// - En passant detection
/// - FEN strings of the board state before and after the move
/// - Square coordinates (from/to)
/// - Promotion piece (if any)
///
/// # Note
/// Only standard chess and Chess960 positions are supported. The function will panic if
/// `raw_move.from()` returns `None`, which shouldn't happen for standard chess variants.
pub fn verbose_move_from_raw_move(raw_move: Move, chess_pos: &Chess) -> MoveVerbose {
    let mut chess_pos = chess_pos.clone();

    let fen_before = Fen::from_position(&chess_pos, shakmaty::EnPassantMode::Legal);

    let promotion: Option<String> = raw_move.promotion().map(|val| val.char().to_string());
    let captured_piece: Option<PieceSymbol> = raw_move
        .capture()
        .map(|role| PieceSymbol::from_shakmaty_piece_role(&role));
    let from_sq = raw_move
        .from()
        .expect("Only standard chess and chess960 is supported, from() should always return Some");

    let color_shorthand = match chess_pos.turn() {
        Color::White => ColorChar::W,
        Color::Black => ColorChar::B,
    };

    let piece = PieceSymbol::from_shakmaty_piece_role(&raw_move.role());
    let san_move = San::from_move(&chess_pos, raw_move);

    let CastleData {
        is_castle,
        is_kingside_castle,
        is_queenside_castle,
    } = castle_data_from_san_move(&san_move);

    chess_pos.play_unchecked(raw_move);
    let fen_after = Fen::from_position(&chess_pos, shakmaty::EnPassantMode::Legal);

    let san_string = san_to_san_plus(&san_move, &chess_pos);

    let from = SquareStr::from_shakmaty_sq(&from_sq);
    let to = SquareStr::from_shakmaty_sq(&raw_move.to());

    let is_big_pawn = is_two_square_pawn_move(&raw_move);

    MoveVerbose {
        from,
        to,
        promotion,
        lan: raw_move
            .to_uci(shakmaty::CastlingMode::Chess960)
            .to_string(),
        san: san_string,
        piece,
        captured: captured_piece,
        is_regular_capture: raw_move.is_capture() && !raw_move.is_en_passant(),

        color: color_shorthand,
        before: fen_before.to_string(),
        after: fen_after.to_string(),

        is_en_passant: raw_move.is_en_passant(),
        is_castle,
        is_big_pawn,
        is_kingside_castle,
        is_queenside_castle,
    }
}

pub fn is_two_square_pawn_move(mov: &Move) -> bool {
    // Only pawns can make "big pawn" moves
    if mov.role() != Role::Pawn {
        return false;
    }

    let from = match mov.from() {
        Some(sq) => sq,
        None => return false,
    };
    let to = mov.to();

    // TODO:  probably just check if from() rank is 2 or 7 and to() rank is 4 or 5?

    let rank_diff = (to.rank() as i8 - from.rank() as i8).abs();

    rank_diff == 2
}

pub fn san_to_san_plus(san_move: &San, pos_after: &Chess) -> String {
    let mut san_string = format!("{}", san_move);
    if pos_after.is_checkmate() {
        san_string.push_str("#");
    } else if pos_after.is_check() {
        san_string.push_str("+");
    }

    san_string
}

pub fn castle_data_from_san_move(san_move: &San) -> CastleData {
    match san_move {
        San::Castle(castling_side) => match castling_side {
            shakmaty::CastlingSide::KingSide => CastleData {
                is_castle: true,
                is_queenside_castle: false,
                is_kingside_castle: true,
            },
            shakmaty::CastlingSide::QueenSide => CastleData {
                is_castle: true,
                is_queenside_castle: true,
                is_kingside_castle: false,
            },
        },
        _ => CastleData {
            is_castle: false,
            is_kingside_castle: false,
            is_queenside_castle: false,
        },
    }
}

#[derive(Default)]
pub struct CastleData {
    pub is_castle: bool,
    pub is_queenside_castle: bool,
    pub is_kingside_castle: bool,
}
