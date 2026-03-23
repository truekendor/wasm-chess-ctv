use serde::{Deserialize, Serialize};
use shakmaty::{Chess, Move, Position, fen::Fen, san::San, uci::UciMove};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ErrorWithValue {
    moves: Vec<String>,
    message: String,
}

/// converts Vec of uci moves `Vec<["e2e4", "e7e5", ...]>`, into Vec of SAN moves
pub fn uci_to_san(
    uci_moves: Vec<String>,
    starting_fen: Option<String>,
) -> Result<Vec<String>, ErrorWithValue> {
    let starting_fen = starting_fen.unwrap_or_else(|| {
        // console::log_1(&format!("Argument for starting FEN was not provided.\nAttempting use FEN of a stating position",).into());
        Fen::from_position(&Chess::default(), shakmaty::EnPassantMode::Legal).to_string()
    });

    let mut san_moves_vec: Vec<String> = vec![];

    let fen: Fen = match starting_fen.parse() {
        Ok(val) => val,
        Err(err) => {
            return Err(ErrorWithValue {
                moves: san_moves_vec,
                message: err.to_string(),
            });
        }
    };

    let mut chess_pos: Chess = match fen.clone().into_position(shakmaty::CastlingMode::Chess960) {
        Ok(val) => val,
        Err(err) => {
            return Err(ErrorWithValue {
                message: format!(
                    "Error converting FEN into position: {}\nStarting FEN: {}",
                    err,
                    fen.to_string()
                ),
                moves: san_moves_vec,
            });
        }
    };

    for uci_move_str in uci_moves {
        let internal_move: Move = match str_to_move(&uci_move_str, &chess_pos) {
            Ok(val) => val,
            Err(err) => {
                return Err(ErrorWithValue {
                    moves: san_moves_vec,
                    message: format!("{}\nStarting FEN: {}", err.to_string(), fen.to_string()),
                });
            }
        };

        let san_move = San::from_move(&chess_pos, internal_move);
        san_moves_vec.push(san_move.to_string());

        chess_pos = match chess_pos.play(internal_move) {
            Ok(val) => val,
            Err(err) => {
                return Err(ErrorWithValue {
                    moves: san_moves_vec,
                    message: format!(
                        "{}\nAttempted to play: {}\nFen: {}",
                        err,
                        internal_move.to_string(),
                        fen.to_string()
                    ),
                });
            }
        };
    }

    Ok(san_moves_vec)
}

/// ! unused
/// converts Vec<string> of a  PV's into Vec of SAN moves
/// PV is a string of UCI moves separated by a whitespace char, like "e2e4 e7e6 b1c3"
pub fn uci_pv_to_san(
    uci_moves: Vec<String>,
    starting_fen: Option<String>,
) -> Result<Vec<String>, ErrorWithValue> {
    let starting_fen = starting_fen.unwrap_or(
        // console::log_1(&format!("Argument for starting FEN was not provided.\nAttempting use FEN of a stating position",).into());
        Fen::from_position(&Chess::default(), shakmaty::EnPassantMode::Legal).to_string(),
    );

    let mut san_moves_vec: Vec<String> = vec![];

    let fen: Fen = match starting_fen.parse() {
        Ok(val) => val,
        Err(err) => {
            return Err(ErrorWithValue {
                moves: san_moves_vec,
                message: err.to_string(),
            });
        }
    };

    let mut chess_pos: Chess = match fen.clone().into_position(shakmaty::CastlingMode::Chess960) {
        Ok(val) => val,
        Err(err) => {
            return Err(ErrorWithValue {
                message: format!(
                    "Error converting FEN into position: {}\nStarting FEN: {}",
                    err,
                    fen.to_string()
                ),
                moves: san_moves_vec,
            });
        }
    };

    for uci_move_str in uci_moves {
        let mut chess_pos_pv = chess_pos.clone();
        let mut pv_vec: Vec<String> = vec![];
        let mut first_move = true;

        for pv_uci_move in uci_move_str.split_ascii_whitespace() {
            let internal_move: Move = match str_to_move(&pv_uci_move, &chess_pos) {
                Ok(val) => val,
                Err(err) => {
                    return Err(ErrorWithValue {
                        moves: san_moves_vec,
                        message: format!("{}\nStarting FEN: {}", err.to_string(), fen.to_string()),
                    });
                }
            };

            let san_move = San::from_move(&chess_pos_pv, internal_move);
            pv_vec.push(san_move.to_string());

            chess_pos_pv = match chess_pos_pv.play(internal_move) {
                Ok(val) => val,
                Err(err) => {
                    return Err(ErrorWithValue {
                        moves: san_moves_vec,
                        message: format!(
                            "{}\nAttempted to play: {}\nFen: {}",
                            err,
                            internal_move.to_string(),
                            fen.to_string()
                        ),
                    });
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

pub fn str_to_move(move_str: &str, chess: &Chess) -> Result<Move, String> {
    // if move is in a UCI format we immediately
    // try to return it
    // otherwise we know that there is a parsing error
    // because move is either in SAN format or illegal
    if let Ok(move_uci) = move_str.parse::<UciMove>() {
        match move_uci.to_move(chess) {
            Ok(valid_move) => return Ok(valid_move),
            Err(err) => {
                return Err(format!(
                    "Error: {}\nMove: {}\nFEN before move: {}",
                    err.to_string(),
                    move_str,
                    Fen::from_position(chess, shakmaty::EnPassantMode::Legal).to_string()
                ));
            }
        }
    }

    // Try SAN format if UCI parsing failed or UCI move was invalid
    if let Ok(move_san) = move_str.parse::<San>() {
        match move_san.to_move(chess) {
            Ok(valid_move) => return Ok(valid_move),
            Err(err) => {
                return Err(format!(
                    "Error: {}\nMove: {}\nFEN before move: {}",
                    err.to_string(),
                    move_str,
                    Fen::from_position(chess, shakmaty::EnPassantMode::Legal).to_string()
                ));
            }
        }
    }

    Err(format!(
        "Failed to parse move «{}»\nOnly SAN and UCI formats are allowed",
        move_str
    ))
}
