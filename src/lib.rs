use std::{collections::HashMap, str::FromStr};

use ordermap::OrderMap;
use shakmaty::{
    Chess, Color, EnPassantMode, Move, Piece, Position, Square, fen::Fen, san::San,
    zobrist::Zobrist64,
};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::helpers::{
    parsing,
    pgn_reader::PGNResult,
    tsify_structs::{
        CastlingObj, ColorChar, CommentsObj, HeadersObj, MoveFromSquares, MoveObject, MoveVerbose,
        OkOrError, PieceObj, PieceSymbol, PrunedCommentsObj, SquareColor, SquareStr, SuffixSymbol,
    },
};

mod helpers;
mod tests;

#[derive(Clone, Debug)]
struct History {
    internal_move: Move,

    move_number: u32,
    half_moves: u32,
    turn: Color,

    fen_before: Fen,
    fen_after: Fen,

    position_before: Chess,
    position_after: Chess,
}

#[wasm_bindgen]
pub struct WasmChess {
    chess: Chess,
    history: Vec<History>,
    hash: Zobrist64,
    position_count: HashMap<Zobrist64, i32>,
    // TODO: rename
    pgn_result: Option<PGNResult>,
}

// TODO: use them
pub type FenString = String;
pub type SuffixString = String;

// todo: make nag u8/u16/u32 number ??
pub type NAGString = String;

// TODO: add docs

// TODO: find what String can be replaced with &str without lifetime and do it

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

        let position_count: HashMap<Zobrist64, i32> = HashMap::from([(zobrist_hash, 1)]);

        Ok(WasmChess {
            chess: chess,
            hash: zobrist_hash,
            position_count,
            history: vec![],
            pgn_result: None,
        })
    }

    #[wasm_bindgen(js_name = "move")]
    pub fn make_move(&mut self, move_str: &str) -> Result<(), String> {
        let internal_move =
            helpers::parsing::str_to_move(move_str, &self.chess).map_err(|err| {
                return err.to_string();
            })?;

        if !self.chess.is_legal(internal_move) {
            return Err(format!(
                "Illegal move: {}\nFEN: {}",
                move_str,
                self.fen(None)
            ));
        }

        let pos_before = self.chess.clone();

        self.chess.play_unchecked(internal_move);
        self.push_history_entry(internal_move, pos_before);

        self.hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);
        *self.position_count.entry(self.hash).or_insert(0) += 1;

        return Ok(());
    }

    #[wasm_bindgen(js_name = "moveFromObj")]
    pub fn make_move_from_obj(&mut self, move_obj: MoveObject) -> Result<(), String> {
        let mut uci_str = format!("{}{}", move_obj.from, move_obj.to);

        match move_obj.promotion {
            Some(val) => {
                uci_str.push_str(&val.to_lowercase());
            }
            None => (),
        };

        self.make_move(&uci_str)
    }

    /// resets to default starting position
    /// and clears all history and pgn related data
    pub fn reset(&mut self) {
        self.chess = Chess::default();

        self.reset_all();
    }

    pub fn load(
        &mut self,
        starting_fen: String,
        // TODO: I don't even know if we can just skip fen validation
        // {skip_validation: bool}
    ) -> Result<(), String> {
        self.reset_history();

        let fen: Fen = starting_fen.parse::<Fen>().map_err(|err| {
            return format!(
                "Error parsing fen string\nError message: {}\n«{}» is not a valid fen",
                err, starting_fen
            );
        })?;

        self.chess = fen
            .clone()
            .into_position(shakmaty::CastlingMode::Chess960)
            .map_err(|err| {
                return format!(
                    "Error converting FEN into chess position\nError message: {}\nFEN: {}",
                    err, fen
                );
            })?;

        self.reset_pos_count_and_hash();

        Ok(())
    }

    fn reset_all(&mut self) {
        self.reset_history();
        self.reset_pos_count_and_hash();
    }

    fn reset_history(&mut self) {
        self.pgn_result = None;
        self.history.clear();
    }

    fn reset_pos_count_and_hash(&mut self) {
        let zobrist_hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);

        self.hash = zobrist_hash;
        self.position_count.clear();
        self.position_count.insert(zobrist_hash, 1);
    }

    pub fn fen(&self, force_en_passant_square: Option<bool>) -> String {
        let en_passant_mode = match force_en_passant_square {
            Some(true) => shakmaty::EnPassantMode::Always,
            Some(false) => shakmaty::EnPassantMode::Legal,
            None => shakmaty::EnPassantMode::Legal,
        };
        let fen = Fen::from_position(&self.chess, en_passant_mode);

        fen.to_string()
    }

    // more docs because this method not present in chess.js
    /// ## Returns the FEN string at a specific move index.
    ///
    /// ## Parameters
    /// * `index` - The move index (0-based):
    ///   - `0` - Starting position (before any moves)
    ///   - `1` - Position after first move
    ///   - `2` - Position after second move, etc.
    ///
    /// ## Returns
    /// * `Some(String)` - The FEN string at the requested position
    /// * `None` - If `index` exceeds total moves played
    ///
    /// ## Example
    /// ```
    /// assert!(chess.fen_at(0).is_some());  // Starting position always available
    /// assert_eq!(chess.fen_at(0), Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string())); // starting position
    ///
    /// // After 1.e4
    /// assert_eq!(chess.fen_at(1), Some("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".to_string()));
    /// ```
    #[wasm_bindgen(js_name = "fenAt")]
    pub fn fen_at(&self, index: usize) -> Option<String> {
        match index {
            0 => {
                let starting_fen = match self.history.len() > 0 {
                    false => self.fen(None),
                    true => self.history[0].fen_before.to_string(),
                };

                Some(starting_fen)
            }
            idx => {
                if idx <= self.history.len() {
                    Some(self.history[idx - 1].fen_after.to_string())
                } else {
                    None
                }
            }
        }
    }

    // TODO: write test for it
    /// Returns the move at a specific index.
    ///
    /// # Parameters
    /// * `index` - The move index (0-based):
    ///   - `0` - Returns `None` (no move at starting position)
    ///   - `1` - First move played
    ///   - `2` - Second move played, etc.
    ///
    /// # Returns
    /// * `Some(MoveObject)` - The move at the requested index
    /// * `None` - If `index` is 0 or exceeds total moves played
    #[wasm_bindgen(js_name = "moveAt")]
    pub fn move_at(&self, index: usize) -> Option<MoveObject> {
        match index {
            0 => None,
            idx if idx <= self.history.len() => {
                let history_entry = &self.history[idx - 1];
                let internal_move = history_entry.internal_move;
                let promotion = internal_move.promotion().map(|m| m.char().to_string());

                let from = internal_move.from()?;
                let to = internal_move.to();

                let from = from.to_string().to_lowercase().parse::<SquareStr>().ok()?;
                let to = to.to_string().to_lowercase().parse::<SquareStr>().ok()?;

                Some(MoveObject {
                    from,
                    to,
                    promotion,
                })
            }
            _ => None,
        }
    }

    // TODO: write tests for it
    /// Returns which side to move at a specific index.
    ///
    /// # Parameters
    /// * `index` - The position index (0-based):
    ///   - `0` - Starting position (White's turn for default starting position)
    ///   - `1` - Turn after first move (Black's turn for default starting position)
    ///   - `2` - Turn after second move, etc.
    ///
    /// # Returns
    /// * `Some(ColorChar)` - The side to move at the requested position
    /// * `None` - If `index` exceeds total history length
    #[wasm_bindgen(js_name = "turnAt")]
    pub fn turn_at(&self, index: usize) -> Option<ColorChar> {
        match index {
            0 => {
                let turn = match self.history.is_empty() {
                    false => self.history[0].turn,
                    true => self.chess.turn(),
                };
                Some(match turn {
                    Color::White => ColorChar::W,
                    Color::Black => ColorChar::B,
                })
            }
            idx => {
                if idx <= self.history.len() {
                    let turn = self.history[idx - 1].turn;
                    Some(match turn {
                        Color::White => ColorChar::B,
                        Color::Black => ColorChar::W,
                    })
                } else {
                    None
                }
            }
        }
    }

    // TODO: write tests
    #[wasm_bindgen(js_name = "isAttacked")]
    pub fn is_attacked(&self, square: SquareStr, attacked_by_side: Option<ColorChar>) -> bool {
        let Ok(square) = Square::from_str(&square.to_string().to_lowercase()) else {
            return false;
        };

        let get_attackers = |color: Color| -> Vec<Square> {
            self.chess
                .board()
                .attacks_to(square, color, self.chess.board().by_color(color))
                .into_iter()
                .collect()
        };

        match attacked_by_side {
            Some(ColorChar::W) => !get_attackers(Color::White).is_empty(),
            Some(ColorChar::B) => !get_attackers(Color::Black).is_empty(),
            None => {
                let turn = self.chess.turn();
                !get_attackers(turn).is_empty()
            }
        }
    }

    // TODO: make static/move to some other mod?
    pub fn fen_is_valid(&self, fen: String) -> OkOrError<String> {
        match fen.parse::<Fen>() {
            Ok(fen) => OkOrError {
                ok: Some(fen.to_string()),
                err: None,
            },
            Err(err) => OkOrError {
                ok: None,
                err: Some(err.to_string()),
            },
        }
    }

    pub fn undo(&mut self) -> Option<MoveVerbose> {
        let last = match self.history.pop() {
            Some(h) => h,
            None => return None,
        };

        if let Some(count) = self.position_count.get_mut(&self.hash) {
            *count -= 1;
            if *count <= 0 {
                self.position_count.remove(&self.hash);
            }
        }
        self.chess = last.position_before;
        self.hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);

        self.position_count.entry(self.hash).or_insert(1);

        let move_verbose: Option<MoveVerbose> =
            helpers::parsing::verbose_move_object_from_internal_move(
                last.internal_move,
                &self.chess,
            )
            .ok();

        move_verbose
    }

    #[wasm_bindgen(js_name = "legalMovesUCI")]
    pub fn legal_moves_uci(&self) -> Vec<String> {
        helpers::legal_moves::uci(&self.chess)
    }

    #[wasm_bindgen(js_name = "legalMovesSAN")]
    pub fn legal_moves_san(&self) -> Vec<String> {
        helpers::legal_moves::san(&self.chess)
    }

    /// # API Discrepancy: chess.js Compatibility
    ///
    /// This implementation differs from chess.js in how it handles the
    /// en passant square in verbose move objects.
    ///
    /// |      Aspect       ||           chess.js             ||      This Implementation      |
    /// |-------------------||--------------------------------||-------------------------------|
    /// | En passant square || Always included when available || Only included for legal moves |
    ///
    /// **TODO:** Evaluate whether to align with chess.js behavior in a future release.
    #[wasm_bindgen(js_name = "legalMovesVerbose")]
    pub fn legal_moves_verbose(&self) -> Vec<MoveVerbose> {
        helpers::legal_moves::verbose(&self.chess)
    }

    pub fn perft(&self, depth: usize) -> u64 {
        shakmaty::perft(&self.chess, depth as u32)
    }

    #[wasm_bindgen(js_name = "moveNumber")]
    pub fn move_number(&self) -> u32 {
        return self.fullmoves();
    }

    pub fn fullmoves(&self) -> u32 {
        let move_number = &self.chess.fullmoves();

        move_number.get()
    }

    pub fn halfmoves(&self) -> u32 {
        self.chess.halfmoves()
    }

    pub fn length(&self) -> u32 {
        return self.history.len() as u32;
    }

    #[wasm_bindgen(js_name = "squareColor")]
    pub fn square_color(&self, square: SquareStr) -> Option<SquareColor> {
        let sq_string = square.to_string();
        let square = Square::from_ascii(sq_string.as_bytes()).ok()?;

        Some(if square.is_light() {
            SquareColor::Light
        } else {
            SquareColor::Dark
        })
    }

    #[wasm_bindgen(js_name = "findPieceFromString")]
    pub fn find_piece_from_str(&self, piece: &str) -> Result<Vec<SquareStr>, String> {
        let piece = piece.trim();

        if piece.len() != 1 {
            return Err(format!("Error: unexpected piece length: {}", piece.len()));
        }

        let piece_char = piece
            .chars()
            .next()
            .ok_or_else(|| "Empty piece string".to_string())?;

        let piece_type = Piece::from_char(piece_char).ok_or_else(|| {
            format!(
                "Error parsing piece char: \"{}\" into a valid piece type",
                piece
            )
        })?;

        let mut squares_with_piece = Vec::new();

        for (sq, p) in self.chess.board().iter() {
            if p == piece_type {
                let square_str = sq.to_string().to_lowercase();
                let square = SquareStr::from_str(&square_str)
                    .map_err(|_| format!("Failed to parse square: {}", square_str))?;
                squares_with_piece.push(square);
            }
        }

        Ok(squares_with_piece)
    }

    #[wasm_bindgen(js_name = "findPieceFromPieceObject")]
    pub fn find_piece_from_obj(&self, piece: PieceObj) -> Result<Vec<SquareStr>, String> {
        let piece_char: char = match piece.r#type {
            PieceSymbol::P => 'p',
            PieceSymbol::N => 'n',
            PieceSymbol::B => 'b',
            PieceSymbol::R => 'r',
            PieceSymbol::Q => 'q',
            PieceSymbol::K => 'k',
        };

        let piece_char = match piece.color {
            ColorChar::W => piece_char.to_ascii_uppercase(),
            ColorChar::B => piece_char.to_ascii_lowercase(),
        };

        let piece_type = Piece::from_char(piece_char).ok_or_else(|| {
            format!(
                "Error parsing piece char: \"{:#?}\" into a valid piece type",
                piece.r#type
            )
        })?;

        let mut squares_with_piece: Vec<SquareStr> = vec![];

        for (sq, p) in self.chess.board().iter() {
            if p == piece_type {
                let square_str = sq.to_string().to_lowercase();
                let square = SquareStr::from_str(&square_str)
                    .map_err(|_| format!("Failed to parse square: {}", square_str))?;
                squares_with_piece.push(square);
            }
        }

        Ok(squares_with_piece)
    }

    pub fn hash(&self) -> u64 {
        return self.hash.0;
    }

    #[wasm_bindgen(js_name = "getCastlingRights")]
    pub fn get_castling_rights(&self, color_char: ColorChar) -> Result<CastlingObj, String> {
        let castles_bitboard = &self.chess.castles().castling_rights();

        match color_char {
            ColorChar::W => {
                let queenside = castles_bitboard.contains(Square::A1);
                let kingside = castles_bitboard.contains(Square::H1);

                return Ok(CastlingObj {
                    king: kingside,
                    queen: queenside,
                });
            }
            ColorChar::B => {
                let queenside = castles_bitboard.contains(Square::A8);
                let kingside = castles_bitboard.contains(Square::H8);

                return Ok(CastlingObj {
                    king: kingside,
                    queen: queenside,
                });
            }
        };
    }

    #[wasm_bindgen(js_name = "isGameOver")]
    pub fn is_game_over(&self) -> bool {
        self.chess.is_game_over() || self.is_draw()
    }

    #[wasm_bindgen(js_name = "isCheck")]
    pub fn is_check(&self) -> bool {
        self.chess.is_check()
    }

    #[wasm_bindgen(js_name = "isCheckmate")]
    pub fn is_checkmate(&self) -> bool {
        self.chess.is_checkmate()
    }

    #[wasm_bindgen(js_name = "isDrawByFiftyMoves")]
    pub fn is_draw_by_fifty_moves(&self) -> bool {
        self.chess.halfmoves() >= 100
    }

    #[wasm_bindgen(js_name = "isInsufficientMaterial")]
    pub fn is_insufficient_material(&self) -> bool {
        self.chess.is_insufficient_material()
    }

    #[wasm_bindgen(js_name = "isThreefoldRepetition")]
    pub fn is_threefold_repetition(&self) -> bool {
        self.position_count
            .get(&self.hash)
            .is_some_and(|&val| val >= 3)
    }

    #[wasm_bindgen(js_name = "isDraw")]
    pub fn is_draw(&self) -> bool {
        self.chess.is_stalemate()
            || self.chess.is_insufficient_material()
            || self.is_draw_by_fifty_moves()
            || self.is_threefold_repetition()
    }

    pub fn turn(&self) -> ColorChar {
        match self.chess.turn() {
            Color::White => ColorChar::W,
            Color::Black => ColorChar::B,
        }
    }

    #[wasm_bindgen(js_name = "isStalemate")]
    pub fn is_stalemate(&self) -> bool {
        self.chess.is_stalemate()
    }

    #[wasm_bindgen(js_name = "isPromotion")]
    pub fn is_promotion(&self, move_obj: MoveFromSquares) -> bool {
        let move_str = format!("{}{}{}", move_obj.from, move_obj.to, "n");

        parsing::str_to_move(&move_str, &self.chess)
            .map(|internal_move| internal_move.is_promotion())
            .unwrap_or(false)
    }

    // TODO: idk what chess.js does
    pub fn board(&self) -> Vec<String> {
        Square::ALL
            .iter()
            .map(|sq| {
                let piece = self.chess.board().piece_at(*sq);

                match piece {
                    Some(p) => p.char().to_string(),
                    None => " ".to_string(),
                }
            })
            .collect::<Vec<String>>()
    }

    pub fn ascii(&self) -> String {
        let border: &str = "   +------------------------+\n";
        let letters: &str = "     a  b  c  d  e  f  g  h\n";
        let mut ascii_str = String::with_capacity(328);

        ascii_str.push_str(border);

        for rank in (0..8).rev() {
            ascii_str.push_str(&format!(" {} |", rank + 1));

            for file in 0..8 {
                let sq = Square::from_coords(
                    shakmaty::File::new(file as u32),
                    shakmaty::Rank::new(rank as u32),
                );

                let piece = self.chess.board().piece_at(sq);

                match piece {
                    Some(p) => {
                        let symbol = p.char();
                        ascii_str.push(' ');
                        ascii_str.push(symbol);
                        ascii_str.push(' ');
                    }
                    None => {
                        ascii_str.push_str(" . ");
                    }
                }
            }

            ascii_str.push_str(&format!("|\n"));
        }

        ascii_str.push_str(border);
        ascii_str.push_str(letters.trim_end());

        ascii_str
    }

    // TODO:  return PieceObj !
    pub fn get(&self, square: SquareStr) -> Option<String> {
        let sq_string = square.to_string();

        let square = match Square::from_ascii(sq_string.as_bytes()) {
            Ok(val) => val,
            Err(_err) => {
                return None;
            }
        };

        let piece = self.chess.board().piece_at(square);
        let char = match piece {
            Some(p) => p.char(),
            None => {
                return None;
            }
        };

        Some(char.to_string())
    }

    // TODO: port chess js tests for this bad boy
    pub fn attackers(
        &self,
        square: SquareStr,
        attacked_by_side: Option<ColorChar>,
    ) -> Result<Vec<String>, String> {
        let square =
            Square::from_str(&square.to_string().to_lowercase()).map_err(|err| err.to_string())?;

        let get_attackers = |color: Color| -> Vec<Square> {
            self.chess
                .board()
                .attacks_to(square, color, self.chess.board().by_color(color))
                .into_iter()
                .collect()
        };

        let w_attackers = get_attackers(Color::White);
        let b_attackers = get_attackers(Color::Black);

        let squares = match attacked_by_side {
            None => match self.chess.turn() {
                Color::White => w_attackers,
                Color::Black => b_attackers,
            },
            Some(ColorChar::W) => w_attackers,
            Some(ColorChar::B) => b_attackers,
        };

        Ok(squares.into_iter().map(|el| el.to_string()).collect())
    }

    #[wasm_bindgen(js_name = "historySAN")]
    pub fn history_san(&self) -> Vec<String> {
        self.history
            .iter()
            .map(|history| {
                let san_move = San::from_move(&history.position_before, history.internal_move);
                san_move.to_string()
            })
            .collect()
    }

    #[wasm_bindgen(js_name = "historyUCI")]
    pub fn history_uci(&self) -> Vec<String> {
        self.history
            .iter()
            .map(|h| {
                let uci_move = h.internal_move.to_uci(shakmaty::CastlingMode::Chess960);

                uci_move.to_string()
            })
            .collect()
    }

    #[wasm_bindgen(js_name = "historyVerbose")]
    pub fn history_verbose(&self) -> Vec<MoveVerbose> {
        let moves_verbose: Vec<MoveVerbose> = self
            .history
            .iter()
            .map(|history_entry| {
                let internal_move = history_entry.internal_move;

                let promotion: Option<String> =
                    internal_move.promotion().map(|val| val.char().to_string());

                let captured_piece: Option<String> =
                    internal_move.capture().map(|val| val.char().to_string());

                let san_move = San::from_move(&history_entry.position_before, internal_move);

                let fen_after = Fen::from_position(
                    &history_entry.position_after,
                    shakmaty::EnPassantMode::Legal,
                );
                let color_shorthand = match history_entry.turn {
                    Color::White => ColorChar::W,
                    Color::Black => ColorChar::B,
                };

                let from_sq = internal_move.from().expect(
                    "Only standard chess and chess960 is supported, from() should always return Some",
                );

                MoveVerbose {
                    from: from_sq.to_string(),
                    to: internal_move.to().to_string(),
                    promotion,
                    lan: internal_move
                        .to_uci(shakmaty::CastlingMode::Chess960)
                        .to_string(),
                    san: san_move.to_string(),
                    piece: internal_move.role().char().to_string(),
                    captured: captured_piece,

                    color: color_shorthand,
                    before: history_entry.fen_before.to_string(),
                    after: fen_after.to_string(),

                    is_en_passant: internal_move.is_en_passant(),
                    is_castle: internal_move.is_castle(),
                }
            })
            .collect();

        moves_verbose
    }

    fn push_history_entry(&mut self, internal_move: Move, pos_before: Chess) {
        self.history.push(History {
            internal_move,

            move_number: self.fullmoves(),
            half_moves: self.halfmoves(),
            turn: self.chess.turn().other(),

            fen_before: Fen::from_position(&pos_before, EnPassantMode::Legal),
            fen_after: Fen::from_position(&self.chess, EnPassantMode::Legal),

            position_before: pos_before,
            position_after: self.chess.clone(),
        });
    }

    // TODO: add Optional<PreserveHeaders> ??
    fn set_fen(&mut self, fen: Fen) -> Result<(), String> {
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

        self.reset_all();

        Ok(())
    }

    #[wasm_bindgen(js_name = "loadPgn")]
    pub fn load_pgn(&mut self, pgn: &str) -> Result<(), String> {
        let pgn_headers = helpers::pgn_reader::parse_pgn(pgn);

        let (pgn_result, wasm_chess) =
            pgn_headers.map_err(|err| format!("Error loading pgn: {}", err))?;

        self.chess = wasm_chess.chess;
        self.hash = wasm_chess.hash;
        self.history = wasm_chess.history;
        self.position_count = wasm_chess.position_count;

        self.pgn_result = Some(pgn_result);

        return Ok(());
    }

    #[wasm_bindgen(js_name = "getHeaders")]
    pub fn get_headers(&self) -> HeadersObj {
        match self.pgn_result.as_ref() {
            Some(pgn_result) => HeadersObj {
                headers_data: pgn_result.headers.clone(),
            },
            None => HeadersObj {
                headers_data: OrderMap::new(),
            },
        }
    }

    #[wasm_bindgen(js_name = "setHeader")]
    pub fn set_header(&mut self, key: String, value: String) -> HeadersObj {
        if self.pgn_result.is_none() {
            self.pgn_result = Some(PGNResult::default());
        };

        match self.pgn_result.as_mut() {
            Some(val) => {
                val.headers = OrderMap::new();
                val.headers.insert(key, value);
            }
            None => (),
        }

        self.get_headers()
    }

    #[wasm_bindgen(js_name = "removeHeader")]
    pub fn remove_header(&mut self, key: String) -> bool {
        self.pgn_result
            .as_mut()
            .map(|pgn| pgn.headers.remove(&key).is_some())
            .unwrap_or(false)
    }

    #[wasm_bindgen(js_name = "removeComment")]
    pub fn remove_comment(&mut self) -> Option<String> {
        let current_fen = self.fen(None);

        let pgn_result = self.pgn_result.as_mut()?;
        let comment = pgn_result.comments_map.get(&current_fen).cloned()?;
        pgn_result.comments_map.remove(&current_fen);
        Some(comment)
    }

    #[wasm_bindgen(js_name = "removeComments")]
    pub fn remove_comments(&mut self) -> Vec<PrunedCommentsObj> {
        let Some(pgn_result) = self.pgn_result.as_mut() else {
            return vec![];
        };

        pgn_result
            .comments_map
            .drain(..)
            .map(|(key, value)| PrunedCommentsObj {
                fen: key,
                comment: value,
            })
            .collect()
    }

    #[wasm_bindgen(js_name = "getComment")]
    pub fn get_comment(&self) -> Option<String> {
        let Some(pgn_result) = self.pgn_result.as_ref() else {
            return None;
        };

        let comments = pgn_result.comments_map.get(&self.fen(None));

        comments.cloned()
    }

    // TODO: suffix annotations not working for now (deprecated ?j)
    #[wasm_bindgen(js_name = "getComments")]
    pub fn get_comments(&mut self) -> Vec<CommentsObj> {
        let mut comments_vec: Vec<CommentsObj> = vec![];

        let Some(pgn_result) = self.pgn_result.as_ref() else {
            return comments_vec;
        };

        let initial_fen_dev = match self.history.len() {
            0 => self.fen(None),
            _ => self.history[0].fen_before.to_string(),
        };

        let comment_obj = self.get_comment_object(initial_fen_dev, pgn_result);

        if let Some(comment_obj) = comment_obj {
            comments_vec.push(comment_obj);
        }

        if self.history.len() == 0 {
            return comments_vec;
        }

        self.history.iter().for_each(|history_entry| {
            let fen_str = history_entry.fen_after.to_string();

            let comment_obj = self.get_comment_object(fen_str, pgn_result);

            if let Some(comment_obj) = comment_obj {
                comments_vec.push(comment_obj);
            }
        });

        comments_vec
    }

    #[wasm_bindgen(js_name = "setComment")]
    pub fn set_comment(&mut self, comment: &str) {
        let fen = self.fen(None);
        let pgn_result = self.pgn_result.get_or_insert_with(PGNResult::default);

        pgn_result
            .comments_map
            .insert(fen, comment.replace('{', "[").replace('}', "]"));
    }

    fn get_comment_object(&self, fen_str: String, pgn_result: &PGNResult) -> Option<CommentsObj> {
        let comment_str = pgn_result.comments_map.get(&fen_str);
        let suffix: Option<String> = pgn_result.suffix_map.get(&fen_str).cloned();
        let nags = match pgn_result.nag_map.get(&fen_str) {
            Some(val) => val.clone(),
            None => vec![],
        };

        if comment_str.is_some() || suffix.is_some() || nags.len() > 0 {
            return Some(CommentsObj {
                fen: fen_str,
                comment: comment_str.cloned(),
                suffix_annotation: suffix,
                nags,
            });
        }

        return None;
    }

    // TODO: add tests for nags ?
    #[wasm_bindgen(js_name = "getNags")]
    pub fn get_nags(&self, fen: Option<String>) -> Vec<String> {
        let Some(pgn_result) = self.pgn_result.as_ref() else {
            return vec![];
        };

        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        pgn_result
            .nag_map
            .get(&fen_key)
            .cloned()
            .unwrap_or_else(Vec::new)
    }

    #[wasm_bindgen(js_name = "addNag")]
    pub fn add_nag(&mut self, nag: &str, fen: Option<String>) {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        let Some(pgn_result) = self.pgn_result.as_mut() else {
            return ();
        };

        let nags = pgn_result.nag_map.entry(fen_key.clone()).or_insert(vec![]);

        if !nags.contains(&fen_key) {
            nags.push(nag.to_string());
        }
    }

    #[wasm_bindgen(js_name = "setNags")]
    pub fn set_nags(&mut self, nags: Vec<String>, fen: Option<String>) {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        let Some(pgn_result) = self.pgn_result.as_mut() else {
            return ();
        };

        let _ = pgn_result.nag_map.insert(fen_key, nags);
    }

    #[wasm_bindgen(js_name = "removeNag")]
    pub fn remove_nag(&mut self, nag: String, fen: Option<String>) -> bool {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        let Some(pgn_result) = self.pgn_result.as_mut() else {
            return false;
        };

        let Some(nags) = pgn_result.nag_map.get_mut(&fen_key) else {
            return false;
        };

        let Some(index) = nags.iter().position(|el| el == &nag) else {
            return false;
        };

        nags.remove(index);
        true
    }

    #[wasm_bindgen(js_name = "removeNags")]
    pub fn remove_nags(&mut self, fen: Option<String>) -> Vec<String> {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        let Some(pgn_result) = self.pgn_result.as_mut() else {
            return vec![];
        };
        let removed = pgn_result.nag_map.remove(&fen_key);

        removed.unwrap_or_else(|| Vec::new())
    }

    #[wasm_bindgen(js_name = "getSuffixAnnotation")]
    pub fn get_suffix_annotation(&self, fen: Option<String>) -> Option<SuffixString> {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        let Some(pgn_result) = self.pgn_result.as_ref() else {
            return None;
        };

        pgn_result.suffix_map.get(&fen_key).cloned()
    }

    // TODO: add custom types like type Suffix = String to avoid confusion
    #[wasm_bindgen(js_name = "setSuffixAnnotation")]

    pub fn set_suffix_annotation(
        &mut self,
        suffix: &str,
        fen: Option<FenString>,
    ) -> Result<(), String> {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        if !SuffixSymbol::str_is_valid_suffix(&suffix) {
            return Err(format!("Provided suffix is invalid: {}", suffix));
        };

        let pgn_result = self.pgn_result.get_or_insert_with(PGNResult::default);
        pgn_result.suffix_map.insert(fen_key, suffix.to_string());

        Ok(())
    }

    #[wasm_bindgen(js_name = "removeSuffixAnnotation")]
    pub fn remove_suffix_annotation(&mut self, fen: Option<FenString>) -> Option<SuffixString> {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        let pgn_result = self.pgn_result.get_or_insert_with(PGNResult::default);

        pgn_result.suffix_map.remove(&fen_key)
    }
}
