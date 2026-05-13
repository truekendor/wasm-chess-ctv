use super::*;

#[wasm_bindgen]
impl WasmChess {
    #[wasm_bindgen(js_name = "historySan")]
    pub fn history_san(&self) -> Vec<String> {
        self.history
            .iter()
            .map(|history| {
                let san_move = San::from_move(&history.position_before, history.raw_move);

                san_to_san_plus(&san_move, &history.position_after)
            })
            .collect()
    }

    #[wasm_bindgen(js_name = "historyUci")]
    pub fn history_uci(&self) -> Vec<String> {
        self.history
            .iter()
            .map(|h| {
                let uci_move = h.raw_move.to_uci(shakmaty::CastlingMode::Chess960);

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
                let internal_move = history_entry.raw_move;

                let move_verbose = parsing::verbose_move_from_raw_move(
                    internal_move,
                    &history_entry.position_before,
                );

                move_verbose
            })
            .collect();

        moves_verbose
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
                let internal_move = history_entry.raw_move;
                let promotion = internal_move
                    .promotion()
                    .map(|role| PieceSymbol::from_shakmaty_piece_role(&role));

                let from = internal_move.from()?;
                let to = internal_move.to();

                let from = SquareStr::from_shakmaty_sq(&from);
                let to = SquareStr::from_shakmaty_sq(&to);

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
    #[wasm_bindgen(js_name = "sideToMoveAt")]
    pub fn side_to_move_at(&self, index: usize) -> Option<ColorChar> {
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

    #[wasm_bindgen(js_name = "moveNumber")]
    pub fn move_number(&self) -> u32 {
        return self.history.len() as u32;
    }

    pub fn length(&self) -> u32 {
        return self.history.len() as u32;
    }
}
