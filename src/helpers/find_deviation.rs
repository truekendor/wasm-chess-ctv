use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use shakmaty::{Chess, Move, Position, fen::Fen, zobrist::Zobrist64};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::helpers::parsing::{str_to_move, to_san};

pub struct InternalMovesAndHash {
    zobrist_hash: Vec<Zobrist64>,
    san_moves: Vec<String>,
    // TODO:
    // err_message: Option<String>
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct TranspositionDataEntry {
    pub move_index: u32,
    #[tsify(optional)]
    pub diverge_data: Option<DivergeData>,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct DivergeData {
    pub move_san: String,
    pub move_index: u32,
}

#[wasm_bindgen(js_name = "findDeviation")]
pub fn find_deviation(
    starting_fen: String,
    move_list_current: Vec<String>,
    move_list_reverse: Vec<String>,
) -> Vec<TranspositionDataEntry> {
    let mut same_positions_list: Vec<TranspositionDataEntry> = Vec::new();

    if move_list_current.len() == 0 || move_list_reverse.len() == 0 {
        return same_positions_list;
    }

    let result_current = get_hash_and_san(move_list_current, Some(starting_fen.clone()));
    let result_reverse = get_hash_and_san(move_list_reverse, Some(starting_fen.clone()));

    // TODO: rename from "fen" set
    let reverse_zobrist_set: HashSet<&Zobrist64> =
        HashSet::from_iter(result_reverse.zobrist_hash.iter());

    let current_san_moves = result_current.san_moves;
    let reverse_san_moves = result_reverse.san_moves;

    // todo: rename
    let current_fen_list = result_current.zobrist_hash;
    let reverse_fen_list = &result_reverse.zobrist_hash;

    let mut was_same_pos = &current_san_moves[0] == &reverse_san_moves[0];
    let mut prev_zobrist_hash: Zobrist64 = Zobrist64::default();

    current_fen_list
        .iter()
        .enumerate()
        .for_each(|(index, hash)| {
            if reverse_zobrist_set.contains(hash) {
                same_positions_list.push(TranspositionDataEntry {
                    diverge_data: None,
                    move_index: index as u32,
                });

                was_same_pos = true;
            } else if was_same_pos {
                let diverge_move_index = &reverse_fen_list
                    .iter()
                    .rposition(|el| {
                        return el == &prev_zobrist_hash;
                    })
                    .unwrap_or(index);

                let rev_index = diverge_move_index + 1;
                let move_san = &reverse_san_moves[rev_index];

                same_positions_list.push(TranspositionDataEntry {
                    move_index: index as u32,
                    diverge_data: Some(DivergeData {
                        move_san: move_san.to_string(),
                        move_index: rev_index as u32,
                    }),
                });

                was_same_pos = false;
            }

            prev_zobrist_hash = *hash;
        });

    same_positions_list
}

fn get_hash_and_san(moves: Vec<String>, starting_fen: Option<String>) -> InternalMovesAndHash {
    let starting_fen = starting_fen.unwrap_or_else(|| {
        Fen::from_position(&Chess::default(), shakmaty::EnPassantMode::Legal).to_string()
    });

    let mut zobrist_hash_list: Vec<Zobrist64> = vec![];
    let result = to_san(moves.clone(), Some(starting_fen.clone()));
    let san_moves = result.moves;

    let fen: Fen = match starting_fen.parse() {
        Ok(val) => val,
        Err(_err) => {
            return InternalMovesAndHash {
                zobrist_hash: zobrist_hash_list,
                san_moves,
            };
        }
    };

    let mut chess_pos: Chess = match fen.clone().into_position(shakmaty::CastlingMode::Chess960) {
        Ok(val) => val,
        Err(_err) => {
            return InternalMovesAndHash {
                zobrist_hash: zobrist_hash_list,
                san_moves,
            };
        }
    };

    for move_str in moves {
        let internal_move: Move = match str_to_move(&move_str, &chess_pos) {
            Ok(val) => val,
            Err(_err) => {
                return InternalMovesAndHash {
                    zobrist_hash: zobrist_hash_list,
                    san_moves,
                };
            }
        };

        if !chess_pos.is_legal(internal_move) {
            return InternalMovesAndHash {
                zobrist_hash: zobrist_hash_list,
                san_moves,
            };
        }

        chess_pos.play_unchecked(internal_move);

        let hash: Zobrist64 = chess_pos.zobrist_hash(shakmaty::EnPassantMode::Legal);
        zobrist_hash_list.push(hash);
    }

    InternalMovesAndHash {
        zobrist_hash: zobrist_hash_list,
        san_moves,
    }
}
