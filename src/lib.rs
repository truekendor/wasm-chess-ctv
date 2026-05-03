use std::{collections::HashMap, str::FromStr};

use shakmaty::{
    Chess, Color, EnPassantMode, Move, Piece, Position, Square, fen::Fen, san::San,
    zobrist::Zobrist64,
};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::helpers::{
    parsing,
    pgn_reader::PGNResult,
    tsify::{
        CastlingObj, ColorChar, CommentsObj, HeadersObj, MoveAlgebraic, MoveObject, MoveVerbose,
        PieceObj, SquareColor, SquareStr,
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
    ///
    /// TODO: need to double-check what is does in chess.js
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

    // as for now the api of this is strange since
    // without any moves played it will return `None`
    // maybe it is an OK behavior
    #[wasm_bindgen(js_name = "fenAt")]
    pub fn fen_at(&self, index: usize) -> Option<String> {
        if index >= self.history.len() {
            return None;
        }

        let fen = &self.history[index].fen_before;

        Some(fen.to_string())
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

    // ! API discrepancy
    // TODO: change it later?
    // this version of verbose moves is a bit different from chess.js since it
    // marks en passant square only if it is a legal move,
    // while chess.js always marks en passant square if it is available
    #[wasm_bindgen(js_name = "legalMovesVerbose")]
    pub fn legal_moves_verbose(&self) -> Vec<MoveVerbose> {
        helpers::legal_moves::verbose(&self.chess)
    }

    pub fn perft(&self, depth: usize) -> u64 {
        shakmaty::perft(&self.chess, depth as u32)
    }

    #[wasm_bindgen(js_name = "moveNumber")]
    /// same as wasm_chess.fullmoves()
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

    // TODO:
    pub fn length(&self) -> u32 {
        return self.history.len() as u32;
    }

    // TODO: custom CTV function?
    // #[wasm_bindgen(js_name = "turnAt")]
    fn turn_at(&self, index: usize) -> u32 {
        if index as u32 >= self.length() {
            return 0;
        }

        return 0;
    }

    // TODO: custom CTV function?
    // TODO: change to Result<MoveObject, String?>
    // #[wasm_bindgen(js_name = "moveAt")]
    fn move_at(&self, index: usize) -> Option<MoveObject> {
        // TODO: index == 0 is fine
        if index == 0 || index as u32 >= self.length() {
            return None;
        }

        let internal_move = &self.history[index].internal_move;
        let promotion = internal_move.promotion().map(|m| m.char().to_string());

        let from = match internal_move.from() {
            Some(val) => val,
            None => {
                return None;
            }
        };

        let from = from.to_string().to_lowercase().parse::<SquareStr>();
        let to = internal_move
            .to()
            .to_string()
            .to_lowercase()
            .parse::<SquareStr>();

        let from = match from {
            Ok(val) => val,
            Err(_err) => {
                return None;
            }
        };

        let to = match to {
            Ok(val) => val,
            Err(_err) => {
                return None;
            }
        };

        Some(MoveObject {
            from,
            to,
            promotion,
        })
    }

    #[wasm_bindgen(js_name = "squareColor")]
    pub fn square_color(&self, square: SquareStr) -> Option<SquareColor> {
        let sq_string = square.to_string();

        let square = match Square::from_ascii(sq_string.as_bytes()) {
            Ok(val) => val,
            Err(_err) => {
                return None;
            }
        };

        let square_color = match square.is_light() {
            true => SquareColor::Light,
            false => SquareColor::Dark,
        };

        Some(square_color)
    }

    #[wasm_bindgen(js_name = "findPieceFromString")]
    pub fn find_piece_from_str(&self, piece: String) -> Result<Vec<SquareStr>, String> {
        let mut squares_with_piece: Vec<SquareStr> = vec![];

        let piece = piece.trim();
        if piece.len() > 1 {
            return Err(format!("Error: unexpected piece type: {}", piece));
        }

        // Validate piece type and color
        let piece_char = piece
            .chars()
            .next()
            .ok_or_else(|| format!("Empty piece string"))?;

        let piece_type = match Piece::from_char(piece_char) {
            Some(p) => p,
            None => {
                return Err(format!(
                    "Error parsing piece char: \"{}\" into a valid piece type",
                    piece
                ));
            }
        };

        self.chess.board().iter().for_each(|(sq, p)| {
            if p == piece_type {
                squares_with_piece.push(sq.to_string().to_lowercase().parse().unwrap());
            }
        });

        Ok(squares_with_piece)
    }

    #[wasm_bindgen(js_name = "findPieceFromPieceObject")]
    pub fn find_piece_from_obj(&self, piece: PieceObj) -> Result<Vec<SquareStr>, String> {
        let mut squares_with_piece: Vec<SquareStr> = vec![];

        let piece_char: char = match piece.r#type {
            helpers::tsify::PieceSymbol::P => 'p',
            helpers::tsify::PieceSymbol::N => 'n',
            helpers::tsify::PieceSymbol::B => 'b',
            helpers::tsify::PieceSymbol::R => 'r',
            helpers::tsify::PieceSymbol::Q => 'q',
            helpers::tsify::PieceSymbol::K => 'k',
            _ => {
                return Err(format!(
                    "Unknown piece type\nInput piece type: {:#?}\nInput color: {:#?}",
                    piece.r#type, piece.color
                ));
            }
        };

        let piece_char: char = match piece.color {
            ColorChar::W => piece_char.to_ascii_uppercase(),
            ColorChar::B => piece_char.to_ascii_lowercase(),
        };

        let piece_type = match Piece::from_char(piece_char) {
            Some(p) => p,
            None => {
                return Err(format!(
                    "Error parsing piece char: \"{:#?}\" into a valid piece type",
                    piece.r#type
                ));
            }
        };

        self.chess.board().iter().for_each(|(sq, p)| {
            if p == piece_type {
                squares_with_piece.push(sq.to_string().to_lowercase().parse().unwrap());
            }
        });

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
        match self.position_count.get(&self.hash) {
            Some(val) => {
                return *val >= 3;
            }
            None => false,
        }
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
    pub fn is_promotion(&self, move_obj: MoveAlgebraic) -> bool {
        let move_str = format!("{}{}{}", move_obj.from, move_obj.to, "n");

        let internal_move: Move = match parsing::str_to_move(&move_str.as_str(), &self.chess) {
            Ok(val) => val,
            Err(_) => {
                return false;
            }
        };

        internal_move.is_promotion()
    }

    pub fn board(&self) -> Vec<String> {
        let result: Vec<String> = Square::ALL
            .iter()
            .map(|sq| {
                let piece = self.chess.board().piece_at(*sq);

                match piece {
                    Some(p) => p.char().to_string(),
                    None => " ".to_string(),
                }
            })
            .collect();

        result
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

    // TODO: return PieceObj
    pub fn get(&self, square: String) -> Option<String> {
        let sq: shakmaty::Square = square.parse().ok()?;
        let piece = self.chess.board().piece_at(sq);
        let char = match piece {
            Some(p) => p.char(),
            None => {
                return None;
            }
        };

        Some(char.to_string())
    }

    // TODO: TEST bevcause we derive bunh of bs with strum
    pub fn attackers(
        &self,
        square: SquareStr,
        attacked_by_side: Option<ColorChar>,
    ) -> Result<Vec<String>, String> {
        let square = Square::from_str(&square.to_string().to_lowercase());

        if let Err(err) = square {
            return Err(err.to_string());
        }

        let square = square.unwrap();

        let mut squares: Vec<Square> = vec![];

        let mut w_attackers: Vec<Square> = self
            .chess
            .board()
            .attacks_to(square, Color::White, self.chess.board().white())
            .into_iter()
            .map(|square| {
                return square;
            })
            .collect();

        let mut b_attackers: Vec<Square> = self
            .chess
            .board()
            .attacks_to(square, Color::Black, self.chess.board().black())
            .into_iter()
            .map(|square| {
                return square;
            })
            .collect();

        if attacked_by_side.is_none() {
            match self.chess.turn() {
                Color::White => {
                    squares.append(&mut w_attackers);
                }
                Color::Black => {
                    squares.append(&mut b_attackers);
                }
            }
        } else {
            match attacked_by_side.unwrap() {
                ColorChar::W => {
                    squares.append(&mut w_attackers);
                }
                ColorChar::B => {
                    squares.append(&mut b_attackers);
                }
            }
        }

        let string_result: Vec<String> = squares.iter().map(|el| el.to_string()).collect();

        return Ok(string_result);
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

                let from_sq = internal_move
                    .from()
                    .expect("Optional only for chess variants");

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
    pub fn load_pgn(&mut self, pgn: String) -> Result<(), String> {
        let pgn_headers = helpers::pgn_reader::parse_pgn(pgn);

        if let Err(pgn_error) = pgn_headers {
            return Err(format!("Error loading pgn: {}", pgn_error));
        }

        let (pgn_result, wasm_chess) = pgn_headers.unwrap();

        self.chess = wasm_chess.chess;
        self.hash = wasm_chess.hash;
        self.history = wasm_chess.history;
        self.position_count = wasm_chess.position_count;

        self.pgn_result = Some(pgn_result);

        return Ok(());
    }

    #[wasm_bindgen(js_name = "getHeaders")]
    pub fn get_headers(&self) -> HeadersObj {
        if self.pgn_result.is_none() {
            return HeadersObj {
                headers_data: HashMap::new(),
            };
        }

        return HeadersObj {
            headers_data: self.pgn_result.clone().unwrap().headers,
        };
    }

    #[wasm_bindgen(js_name = "setHeader")]
    pub fn set_header(&mut self, key: String, value: String) -> HeadersObj {
        if self.pgn_result.is_none() {
            self.pgn_result = Some(PGNResult::default());
        };

        match self.pgn_result.as_mut() {
            Some(val) => {
                val.headers = HashMap::new();
                val.headers.insert(key, value);
            }
            None => (),
        }

        self.get_headers()
    }

    #[wasm_bindgen(js_name = "removeHeader")]
    pub fn remove_header(&mut self, key: String) -> bool {
        if self.pgn_result.is_some() {
            let val = self.pgn_result.as_mut().unwrap().headers.remove(&key);

            return val.is_some();
        }

        return false;
    }

    #[wasm_bindgen(js_name = "removeComment")]
    pub fn remove_comment(&mut self) -> Option<String> {
        if self.pgn_result.is_none() {
            return None;
        }

        let current_fen = self.fen(None);
        let pgn_result = self.pgn_result.as_mut().unwrap();

        let comment = pgn_result.comments_map.get(&current_fen).cloned();

        if comment.is_none() {
            return None;
        };

        pgn_result.comments_map.remove(&current_fen);

        comment
    }

    // TODO: suffix annotations not working for now
    #[wasm_bindgen(js_name = "getComments")]
    pub fn get_comments(&mut self) -> Vec<CommentsObj> {
        let mut comments_vec: Vec<CommentsObj> = vec![];

        if self.pgn_result.is_none() {
            return comments_vec;
        }

        let pgn_result = self.pgn_result.as_ref().unwrap();

        self.history
            .iter()
            .enumerate()
            .for_each(|(index, history_entry)| {
                let fen_str = history_entry.fen_after.to_string();

                let comment_str = pgn_result.comments_map.get(&fen_str);
                let suffix: Option<String> = pgn_result.suffix_map.get(&fen_str).cloned();
                let nags = match pgn_result.nag_map.get(&fen_str) {
                    Some(val) => val.clone(),
                    None => vec![],
                };

                if comment_str.is_some() || suffix.is_some() || nags.len() > 0 {
                    comments_vec.push(CommentsObj {
                        fen: fen_str,
                        comment: comment_str.cloned(),
                        suffix_annotation: suffix,
                        nags,
                    });
                }
            });

        comments_vec
    }

    #[wasm_bindgen(js_name = "setComment")]
    pub fn set_comment(&mut self, comment: String) {
        if self.pgn_result.is_none() {
            self.pgn_result = Some(PGNResult::default());
        }

        let fen = self.fen(None);
        let pgn_result = self.pgn_result.as_mut().unwrap();

        pgn_result
            .comments_map
            .insert(fen, comment.replace('{', "[").replace('}', "]"));
    }
}
