use std::{collections::HashMap, str::FromStr};

use shakmaty::{
    Board, Chess, Color, Move, Piece, Position, Square, fen::Fen, san::San, zobrist::Zobrist64,
};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::helpers::{
    pgn_reader::PGNResult,
    tsify::{AttackedBySide, CommentsObj, HeadersObj, MoveObject, MoveVerbose},
};

mod helpers;
mod tests;

#[derive(Clone)]
struct History {
    internal_move: Move,
    // maybe drop fen from history and just
    // calculate it on the fly when needed
    fen: Fen,
    move_number: u32,
    half_moves: u32,
    turn: Color,
    position: Chess,
}

#[wasm_bindgen]
struct WasmChess {
    chess: Chess,
    history: Vec<History>,
    hash: Zobrist64,
    position_count: HashMap<Zobrist64, i32>,
    // TODO: rename
    pgn_result: Option<PGNResult>,
    seven_tag_roster: HashMap<String, String>,
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

        // TODO do we need this??
        let seven_tag_roster: HashMap<String, String> = HashMap::from([
            ("Event".to_owned(), "?".to_owned()),
            ("Site".to_owned(), "?".to_owned()),
            ("Date".to_owned(), "????.??.??".to_owned()),
            ("Round".to_owned(), "?".to_owned()),
            ("White".to_owned(), "?".to_owned()),
            ("Black".to_owned(), "?".to_owned()),
            ("Result".to_owned(), "*".to_owned()),
        ]);

        Ok(WasmChess {
            chess: chess,
            hash: zobrist_hash,
            position_count,
            history: vec![],
            pgn_result: None,
            seven_tag_roster,
        })
    }

    #[wasm_bindgen(js_name = "move")]
    pub fn make_move(&mut self, move_str: &str) -> Result<(), String> {
        let internal_move =
            helpers::parsing::str_to_move(move_str, &self.chess).map_err(|err| {
                return err.to_string();
            })?;

        if !self.chess.is_legal(internal_move) {
            return Err(format!("Illegal move: {}\nFEN: {}", move_str, self.fen()));
        }

        self.push_history_entry(internal_move);

        self.chess.play_unchecked(internal_move);

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

    pub fn fen(&self) -> String {
        let fen = Fen::from_position(&self.chess, shakmaty::EnPassantMode::Always);

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

        let fen = &self.history[index].fen;

        Some(fen.to_string())
    }

    pub fn undo(&mut self) -> Result<String, String> {
        let last = match self.history.pop() {
            Some(h) => h,
            None => return Err("No moves to undo".to_string()),
        };

        if let Some(count) = self.position_count.get_mut(&self.hash) {
            *count -= 1;
            if *count <= 0 {
                self.position_count.remove(&self.hash);
            }
        }
        self.chess = last.position;

        self.hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);

        self.position_count.entry(self.hash).or_insert(1);

        Ok(last.internal_move.to_string())
    }

    #[wasm_bindgen(js_name = "legalMovesUCI")]
    pub fn legal_moves_uci(&self) -> Vec<String> {
        helpers::legal_moves::uci(&self.chess)
    }

    #[wasm_bindgen(js_name = "legalMovesSAN")]
    pub fn legal_moves_san(&self) -> Vec<String> {
        helpers::legal_moves::san(&self.chess)
    }

    // #[wasm_bindgen(js_name = "legalMovesVerbose")]
    fn legal_moves_verbose(&self) -> Vec<MoveVerbose> {
        todo!()
    }

    pub fn perft(&self, depth: usize) -> u64 {
        shakmaty::perft(&self.chess, depth as u32)
    }

    pub fn fullmoves(&self) -> u32 {
        let move_number = &self.chess.fullmoves();

        move_number.get()
    }

    pub fn halfmoves(&self) -> u32 {
        self.chess.halfmoves()
    }

    #[wasm_bindgen(js_name = "isGameOver")]
    pub fn is_game_over(&self) -> bool {
        self.chess.is_game_over() || self.is_draw_by_fifty_moves() || self.is_threefold_repetition()
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

    pub fn turn(&self) -> String {
        match self.chess.turn() {
            Color::White => "w".to_string(),
            Color::Black => "b".to_string(),
        }
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

            // ascii_str.push_str(&format!("| {}\n", rank + 1));
            ascii_str.push_str(&format!("|\n"));
        }

        ascii_str.push_str(border);
        ascii_str.push_str(letters);

        ascii_str
    }

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

    pub fn attackers(
        &self,
        square: String,
        attacked_by_side: Option<AttackedBySide>,
    ) -> Result<Vec<String>, String> {
        let square = Square::from_str(&square);

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
                AttackedBySide::W => {
                    squares.append(&mut w_attackers);
                }
                AttackedBySide::B => {
                    squares.append(&mut b_attackers);
                }
                AttackedBySide::Both => {
                    squares.append(&mut w_attackers);
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
                let san_move = San::from_move(&history.position, history.internal_move);

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
    pub fn history_verbose(&self) -> Result<Vec<MoveVerbose>, String> {
        if self.history.len() == 0 {
            return Ok(vec![]);
        }

        let first_entry = &self.history[0];

        let mut chess: Chess = first_entry
            .fen
            .clone()
            .into_position(shakmaty::CastlingMode::Chess960)
            .map_err(|err| {
                return format!(
                    "Error converting FEN into chess position\nError message: {}\nFEN: {}",
                    err,
                    first_entry.fen.to_string()
                );
            })?;

        let moves_verbose: Result<Vec<MoveVerbose>, String> = self
            .history
            .iter()
            .rev()
            .map(|history_entry| -> Result<MoveVerbose, String> {
                let internal_move = history_entry.internal_move;

                let promotion: Option<String> =
                    internal_move.promotion().map(|val| val.char().to_string());

                let captured_piece: Option<String> =
                    internal_move.capture().map(|val| val.char().to_string());

                let san_move = San::from_move(&history_entry.position, internal_move);
                chess.play_unchecked(internal_move);

                let fen_after = Fen::from_position(&chess, shakmaty::EnPassantMode::Legal);
                let color_shorthand = match history_entry.turn {
                    Color::White => AttackedBySide::W,
                    Color::Black => AttackedBySide::B,
                };

                let from_sq = internal_move.from();

                if from_sq.is_none() {
                    return Err("unable to get square info from move".to_string());
                }

                Ok(MoveVerbose {
                    from: from_sq.unwrap().to_string(),
                    to: internal_move.to().to_string(),
                    promotion,
                    lan: internal_move
                        .to_uci(shakmaty::CastlingMode::Chess960)
                        .to_string(),
                    san: san_move.to_string(),
                    piece: internal_move.role().char().to_string(),
                    captured: captured_piece,

                    color: color_shorthand,
                    before: history_entry.fen.to_string(),
                    after: fen_after.to_string(),

                    is_en_passant: internal_move.is_en_passant(),
                    is_castle: internal_move.is_castle(),
                })
            })
            .rev()
            .collect();

        if moves_verbose.is_err() {
            return Err(moves_verbose.err().unwrap());
        }

        Ok(moves_verbose.unwrap())
    }

    fn push_history_entry(&mut self, internal_move: Move) {
        self.history.push(History {
            internal_move,
            fen: Fen::from_position(&self.chess, shakmaty::EnPassantMode::Legal),

            move_number: self.fullmoves(),
            half_moves: self.halfmoves(),
            turn: self.chess.turn(),
            position: self.chess.clone(),
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

        let pgn_result = pgn_headers.unwrap();

        let starting_fen = pgn_result.starting_fen.clone();
        let moves_list = pgn_result.move_list.iter();

        self.set_fen(starting_fen)?;

        for san_move in moves_list {
            self.make_move(san_move)?;
        }

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

    // #[wasm_bindgen(js_name = "getComments")]
    fn get_comments(&self) -> Vec<CommentsObj> {
        // if self.pgn_result.is_none() {
        //     return None;
        // }

        // let comments = &self.pgn_result.as_ref().unwrap().comments;
        // return Some(comments.to_vec());

        todo!("Comments API is not implemented yet");
    }

    #[wasm_bindgen(js_name = "removeHeader")]
    pub fn remove_header(&mut self, key: String) -> bool {
        if self.pgn_result.is_some() {
            let val = self.pgn_result.as_mut().unwrap().headers.remove(&key);

            return val.is_some();
        }

        return false;
    }

    fn set_comment() {
        todo!()
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
}
