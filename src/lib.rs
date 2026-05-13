use std::collections::HashMap;

use ordermap::OrderMap;
use shakmaty::{
    Chess, Color, EnPassantMode, FromSetup, Move, Piece, Position, Setup, Square, fen::Fen,
    san::San, zobrist::Zobrist64,
};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    helpers::{
        ascii,
        parsing::{self, san_to_san_plus, verbose_move_from_raw_move},
        pgn::chess_to_pgn,
        pgn_reader::PGNResult,
    },
    tsify_structs::{
        BoardMatrix, BoardMatrixReturnObj, BoardMatrixRow, MoveVerbose, PieceObj, PieceSymbol,
        SquareStr, SuffixSymbol,
        others::{
            CastlingObj, ColorChar, CommentsObj, HeadersObj, LegalMovesFilterOptions,
            MoveFromSquares, MoveObject, OkOrError, PGNOptions, PreserveHeaders, PrunedCommentsObj,
            SquareColor, SquareInfoObj,
        },
    },
};

mod helpers;
mod impls;
mod tests;
mod tsify_structs;

/// TODOs global
/// add helper for fen parsing
/// move board(), get_comments(), load_pgn() pgn() out of WasmChess struct
///
/// add current_or_initial_fen() ?
///
/// change legalMoves(UCI/SAN/Verbose) to
/// legalMoves(Option<Mode::Verbose/San/Uci >)
///
/// missing chess.js methods
/// moves(), pgn()
///
/// NOTES: not supported: nullmoves, excessive material
/// direct board manipulation: clear(), put(), remove(), (setTurn() ? may be possible rn),
/// setCastlingRights, clear

#[derive(Clone, Debug)]
struct History {
    raw_move: Move,

    fen_before: Fen,
    fen_after: Fen,
    turn: Color,

    position_before: Chess,
    position_after: Chess,
}

#[wasm_bindgen]
pub struct WasmChess {
    chess: Chess,
    history: Vec<History>,
    hash: Zobrist64,
    repetition_table: HashMap<Zobrist64, i32>,
    // TODO: rename
    pgn_result: Option<PGNResult>,
    seven_tag_roster: OrderMap<&'static str, &'static str>,

    // TODO: implement board manip methods using this
    // NOTES:
    // i think any manip operation should make this
    // field to be Some and any attempt at make_move
    // will try and re-assemble chess position from this setup
    // and make it None on success
    // TODO: related
    // update this setup after pgn_load, and other such methods
    // TODO: make these two one struct since they are coupled
    editable_setup: Option<Setup>,
    editable_chess_pos: Option<Chess>,
}

pub type FenString = String;
pub type SuffixString = String;

// todo: make nag and suffix u8/u16/u32 number ??
pub type NAGString = String;
pub type MoveString = String;

#[wasm_bindgen]
impl WasmChess {
    // TODO: make static/move to some other mod?
    // TODO: add js_name
    #[wasm_bindgen(js_name = "validateFen")]
    pub fn validate_fen(&self, fen: FenString) -> OkOrError<bool> {
        match fen.parse::<Fen>() {
            Ok(_) => OkOrError {
                ok: true,
                err: None,
            },
            Err(err) => OkOrError {
                ok: false,
                err: Some(err.to_string()),
            },
        }
    }

    pub fn ascii(&self) -> String {
        ascii::from_board(&self.chess.board())
    }
}
