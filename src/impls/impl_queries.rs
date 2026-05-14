use shakmaty::{Role, uci::UciMove};

use super::*;

#[wasm_bindgen]
impl WasmChess {
    pub fn fen(&self, force_en_passant_square: Option<bool>) -> FenString {
        let en_passant_mode = match force_en_passant_square {
            Some(true) => shakmaty::EnPassantMode::Always,
            Some(false) => shakmaty::EnPassantMode::Legal,
            None => shakmaty::EnPassantMode::Legal,
        };
        let fen = Fen::from_position(&self.chess, en_passant_mode);

        fen.to_string()
    }

    // TODO: add inline docs
    // add use cases ? maybe
    pub fn board_fen(&self) -> String {
        self.chess.board().board_fen().to_string()
    }

    pub fn board(&self) -> BoardMatrixReturnObj {
        const RANK_STRINGS: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];
        let mut result: BoardMatrix = Vec::with_capacity(8);
        let mut square_str = String::with_capacity(2);

        for rank in (1..=8).rev() {
            let mut row: BoardMatrixRow = Vec::with_capacity(8);

            for file in 'a'..='h' {
                square_str.clear();
                square_str.push(file);
                square_str.push_str(RANK_STRINGS[rank - 1 as usize]);

                let square = square_str.parse::<SquareStr>().unwrap(); // Safe because format is correct

                let shakmaty_square = SquareStr::to_shakmaty_sq(&square);
                let piece = self.chess.board().piece_at(shakmaty_square);

                let square_info = match piece {
                    Some(p) => {
                        let color = match p.color {
                            Color::White => ColorChar::W,
                            Color::Black => ColorChar::B,
                        };

                        Some(SquareInfoObj {
                            color,
                            square,
                            r#type: PieceSymbol::from_shakmaty_piece(&p),
                        })
                    }
                    None => None,
                };

                row.push(square_info);
            }

            result.push(row);
        }

        BoardMatrixReturnObj {
            board_matrix: result,
        }
    }

    pub fn turn(&self) -> ColorChar {
        match self.chess.turn() {
            Color::White => ColorChar::W,
            Color::Black => ColorChar::B,
        }
    }

    /// Checks if a move from the given squares would result in a promotion.
    ///
    /// # Returns
    /// `true` if the move would promote a pawn, `false` otherwise
    #[wasm_bindgen(js_name = "isPromotion")]
    pub fn is_promotion(&self, move_obj: MoveFromSquares) -> bool {
        let mut move_str = String::with_capacity(5);
        move_str.push_str(&move_obj.from.as_str());
        move_str.push_str(&move_obj.to.as_str());

        // # Note
        // Uses knight as a temporary promotion piece to validate the move.
        // This works because any promotion piece would indicate a valid promotion.
        move_str.push('n');

        parsing::str_to_move(&move_str, &self.chess)
            .map(|internal_move| internal_move.is_promotion())
            .unwrap_or(false)
    }

    pub fn get(&self, square: SquareStr) -> Option<PieceObj> {
        let square = square.to_shakmaty_sq();

        let Some(piece) = self.chess.board().piece_at(square) else {
            return None;
        };

        let piece_obj = PieceObj::from_shakmaty_piece(&piece);

        Some(piece_obj)
    }

    #[wasm_bindgen(js_name = "findPiece")]
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
                let square = SquareStr::from_shakmaty_sq(&sq);
                squares_with_piece.push(square);
            }
        }

        Ok(squares_with_piece)
    }

    #[wasm_bindgen(js_name = "findPieceByType")]
    pub fn find_piece_from_obj(&self, piece: PieceObj) -> Vec<SquareStr> {
        let piece_type = piece.to_shakmaty_piece();

        let mut squares_with_piece: Vec<SquareStr> = vec![];

        for (sq, p) in self.chess.board().iter() {
            if p == piece_type {
                let square = SquareStr::from_shakmaty_sq(&sq);

                squares_with_piece.push(square);
            }
        }

        squares_with_piece
    }

    #[wasm_bindgen(js_name = "squareColor")]
    pub fn square_color(&self, square: SquareStr) -> Option<SquareColor> {
        let square = SquareStr::to_shakmaty_sq(&square);

        Some(if square.is_light() {
            SquareColor::Light
        } else {
            SquareColor::Dark
        })
    }

    // TODO:
    // write tests
    // consider changing default behavior if None is provided by returning
    // true if any side given square
    // i don't like state coupling, but it is the way chess.js implemented it
    #[wasm_bindgen(js_name = "isAttacked")]
    pub fn is_attacked(&self, square: SquareStr, attacked_by_side: Option<ColorChar>) -> bool {
        let square = SquareStr::to_shakmaty_sq(&square);

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

    // TODO:
    // consider changing default behavior if None is provided by returning
    // true if any side given square
    // i don't like state coupling, but it is the way chess.js implemented it
    pub fn attackers(
        &self,
        square: SquareStr,
        attacked_by_side: Option<ColorChar>,
    ) -> Vec<SquareStr> {
        let square = square.to_shakmaty_sq();

        let get_attackers = |color: Color| -> Vec<Square> {
            self.chess
                .board()
                .attacks_to(
                    square,
                    color,
                    self.chess.board().occupied(), // .without(square)
                )
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

        squares
            .into_iter()
            .map(|sq| SquareStr::from_shakmaty_sq(&sq))
            .collect()
    }

    #[wasm_bindgen(js_name = "legalMovesUci")]
    pub fn legal_moves_uci(
        &self,
        filter_options: Option<LegalMovesFilterOptions>,
    ) -> Vec<MoveString> {
        let filter_options = unwrap_filter_options(&filter_options);

        let legal_moves: Vec<String> = self
            .chess
            .legal_moves()
            .iter()
            .filter_map(|mov| {
                let move_filtered = filter_move(mov, &filter_options);
                if move_filtered {
                    return None;
                }

                let uci_move = UciMove::from_move(*mov, shakmaty::CastlingMode::Chess960);
                return Some(uci_move.to_string());
            })
            .collect();

        legal_moves
    }

    #[wasm_bindgen(js_name = "legalMovesSan")]
    pub fn legal_moves_san(
        &self,
        filter_options: Option<LegalMovesFilterOptions>,
    ) -> Vec<MoveString> {
        let filter_options = unwrap_filter_options(&filter_options);

        let legal_moves: Vec<String> = self
            .chess
            .legal_moves()
            .iter()
            .filter_map(|mov| {
                let mut chess_clone = self.chess.clone();

                let move_filtered = filter_move(mov, &filter_options);
                if move_filtered {
                    return None;
                }

                let san_move = San::from_move(&self.chess, *mov);
                chess_clone.play_unchecked(*mov);

                let san_str = san_to_san_plus(&san_move, &chess_clone);

                return Some(san_str);
            })
            .collect();

        legal_moves
    }

    /// # API Discrepancy: chess.js Compatibility
    ///
    /// This implementation differs from chess.js in how it handles the
    /// en passant square in verbose move objects.
    ///
    /// |      Aspect       |           chess.js             |      This Implementation      |
    /// |-------------------|--------------------------------|-------------------------------|
    /// | En passant square | Always included when available | Only included for legal moves |
    ///
    /// **TODO:** Evaluate whether to align with chess.js behavior in a future release.
    #[wasm_bindgen(js_name = "legalMovesVerbose")]
    pub fn legal_moves_verbose(
        &self,
        filter_options: Option<LegalMovesFilterOptions>,
    ) -> Vec<MoveVerbose> {
        let filter_options = unwrap_filter_options(&filter_options);

        let legal_moves: Vec<MoveVerbose> = self
            .chess
            .legal_moves()
            .iter()
            .filter_map(|raw_move| {
                let move_filtered = filter_move(raw_move, &filter_options);
                if move_filtered {
                    return None;
                }

                let move_verbose = verbose_move_from_raw_move(*raw_move, &self.chess);

                Some(move_verbose)
            })
            .collect();

        legal_moves
    }

    #[wasm_bindgen(js_name = "getCastlingRights")]
    pub fn get_castling_rights(&self, color_char: ColorChar) -> CastlingObj {
        let castles_bitboard = &self.chess.castles().castling_rights();

        match color_char {
            ColorChar::W => {
                let queenside = castles_bitboard.contains(Square::A1);
                let kingside = castles_bitboard.contains(Square::H1);

                return CastlingObj {
                    king: Some(kingside),
                    queen: Some(queenside),
                };
            }
            ColorChar::B => {
                let queenside = castles_bitboard.contains(Square::A8);
                let kingside = castles_bitboard.contains(Square::H8);

                return CastlingObj {
                    king: Some(kingside),
                    queen: Some(queenside),
                };
            }
        };
    }

    #[wasm_bindgen(js_name = "zobristHash")]
    pub fn zobrist_hash(&self) -> u64 {
        return self.hash.0;
    }
}

struct FilterOptions {
    pub square: Option<Square>,
    pub piece: Option<Role>,
}

fn unwrap_filter_options(options: &Option<LegalMovesFilterOptions>) -> FilterOptions {
    let filter_square_option: Option<Square> = match options.as_ref() {
        Some(val) => {
            if let Some(square) = &val.from_square {
                Some(square.to_shakmaty_sq())
            } else {
                None
            }
        }
        None => None,
    };

    let filter_piece_option: Option<Role> = match options.as_ref() {
        Some(val) => {
            let p = val.piece.as_ref();

            if let Some(oo) = p {
                Some(oo.to_shakmaty_piece_role())
            } else {
                None
            }
        }
        None => None,
    };

    FilterOptions {
        square: filter_square_option,
        piece: filter_piece_option,
    }
}

fn filter_move(mov: &Move, options: &FilterOptions) -> bool {
    if let Some(filter_square) = options.square {
        let from = mov.from();

        let Some(from) = from else {
            return true;
        };

        if filter_square != from {
            return true;
        }
    }

    if let Some(filter_piece) = options.piece {
        if mov.role() != filter_piece {
            return true;
        }
    }

    false
}
