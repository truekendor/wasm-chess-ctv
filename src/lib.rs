use std::collections::HashMap;

use ordermap::OrderMap;
use shakmaty::{
    Chess, Color, EnPassantMode, Move, Piece, Position, Setup, Square, fen::Fen, san::San,
    zobrist::Zobrist64,
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

struct EditablePosition {
    setup: Setup,
    validated: Option<Chess>,
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
    editable: Option<EditablePosition>,
    // TODO: delete later
    // editable_setup: Option<Setup>,
    // editable_chess_pos: Option<Chess>,
}

pub type FenString = String;
pub type SuffixString = String;

// todo: make nag and suffix u8/u16/u32 number ??
pub type NAGString = String;
pub type MoveString = String;

#[wasm_bindgen]
impl WasmChess {
    #[wasm_bindgen(constructor)]
    pub fn new(fen: Option<String>) -> Result<WasmChess, String> {
        let starting_fen: String = fen.unwrap_or(
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

        let repetition_table: HashMap<Zobrist64, i32> = HashMap::from([(zobrist_hash, 1)]);

        Ok(WasmChess {
            chess: chess,
            hash: zobrist_hash,
            repetition_table,
            history: vec![],
            pgn_result: None,
            seven_tag_roster: OrderMap::from([
                ("Event", "?"),
                ("Site", "?"),
                ("Date", "????.??.??"),
                ("Round", "?"),
                ("White", "?"),
                ("Black", "?"),
                ("Result", "*"),
            ]),
            editable: None,
        })
    }

    fn push_history_entry(&mut self, raw_move: Move, pos_before: Chess) {
        self.history.push(History {
            raw_move,

            turn: self.chess.turn().other(),

            fen_before: Fen::from_position(&pos_before, EnPassantMode::Legal),
            fen_after: Fen::from_position(&self.chess, EnPassantMode::Legal),

            position_before: pos_before,
            position_after: self.chess.clone(),
        });
    }

    /// resets to default starting position
    /// and clears all history and pgn related data
    pub fn reset(&mut self) {
        self.chess = Chess::default();

        self.reset_all();
    }

    /// Loads a position from a FEN string.
    ///
    /// This completely replaces the current game state and clears:
    /// - move history
    /// - repetition tracking
    /// - PGN comments
    /// - PGN headers
    ///
    /// # Errors
    ///
    /// Returns an error if the provided FEN is invalid.
    ///
    /// # chess.js Compatibility
    ///
    /// Behaves similarly to `chess.load()` from chess.js.
    ///
    /// # Examples
    ///
    /// ```js
    /// chess.load("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    /// ```
    pub fn load(
        &mut self,
        starting_fen: FenString,
        // NOTE:
        // I don't even know if we can just skip fen validation
        // {skip_validation: bool}
        // TODO: try add it anyway?
    ) -> Result<(), String> {
        let fen: Fen = starting_fen.parse::<Fen>().map_err(|err| {
            return format!("Invalid FEN '{starting_fen}': {err}");
        })?;

        self.set_fen(fen)?;

        Ok(())
    }

    // TODO: add Optional<PreserveHeaders> ??
    fn set_fen(&mut self, fen: Fen) -> Result<(), String> {
        self.chess = match fen.clone().into_position(shakmaty::CastlingMode::Chess960) {
            Ok(val) => val,
            Err(err) => {
                return Err(format!("Error {err}\nFEN: {fen}"));
            }
        };

        self.reset_all();

        Ok(())
    }

    fn reset_all(&mut self) {
        self.reset_history();
        self.reset_repetition_table_and_hash();
    }

    fn reset_history(&mut self) {
        self.pgn_result = None;
        self.history.clear();
    }

    fn reset_repetition_table_and_hash(&mut self) {
        let zobrist_hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);

        self.hash = zobrist_hash;
        self.repetition_table.clear();
        self.repetition_table.insert(zobrist_hash, 1);
    }
}
