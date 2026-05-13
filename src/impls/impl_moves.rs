use super::*;

#[wasm_bindgen]
impl WasmChess {
    /// Makes a move using SAN or UCI notation.
    ///
    /// Accepted formats include:
    /// - SAN (`Nf3`, `Qxe5+`, `O-O`)
    /// - UCI (`e2e4`, `g1f3`, `g7h8q`)
    ///
    /// On success the move is applied, repetition state is updated,
    /// and the move is appended to history.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the move cannot be parsed
    /// - the move is illegal in the current position
    ///
    /// # chess.js Compatibility
    ///
    /// Compatible with `chess.move()` string input behavior.
    ///
    /// # Examples
    ///
    /// ```js
    /// chess.move("e4")
    /// chess.move("Nf3")
    /// chess.move("e2e4")
    /// ```
    #[wasm_bindgen(js_name = "move")]
    pub fn make_move(&mut self, move_str: &str) -> Result<MoveVerbose, String> {
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
        let verbose = verbose_move_from_raw_move(internal_move, &pos_before);

        self.chess.play_unchecked(internal_move);
        self.push_history_entry(internal_move, pos_before);

        self.hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);
        *self.repetition_table.entry(self.hash).or_insert(0) += 1;

        return Ok(verbose);
    }

    // TODO:
    // add docs about what it does
    // make public
    // add js_name
    // add tests
    /// Parses and validates a move without modifying the current position.
    ///
    /// Unlike [`move`](#method.make_move), this method does **not**
    /// update:
    /// - the board state
    /// - move history
    /// - repetition tracking
    /// - hashes or PGN state
    ///
    /// This is useful for:
    /// - validating user input
    /// - previewing move metadata
    /// - checking legality before committing a move
    /// - UI hover / drag previews
    ///
    /// Accepted formats include:
    /// - SAN (`Nf3`, `Qxe5+`, `O-O`)
    /// - UCI (`e2e4`, `g1f3`, `g7h8q`)
    ///
    /// # Returns
    ///
    /// Returns a [`MoveVerbose`] object describing the move if it is legal.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the move cannot be parsed
    /// - the move is illegal in the current position
    ///
    /// # Examples
    ///
    /// ```js
    /// chess.simulateMove("e4")
    /// chess.simulateMove("Nf3")
    /// chess.simulateMove("e2e4")
    /// ```
    fn simulate_move(&self, move_str: &str) -> Result<MoveVerbose, String> {
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

        let verbose_move = verbose_move_from_raw_move(internal_move, &self.chess);

        return Ok(verbose_move);
    }

    /// Plays multiple moves sequentially from the current position.
    ///
    /// Each move is processed using [`make_move`](#method.make_move),
    /// meaning all normal validation, history tracking, repetition
    /// updates, and verbose move generation still apply.
    ///
    /// Accepted move formats:
    /// - SAN (`e4`, `Nf3`, `O-O`)
    /// - UCI (`e2e4`, `g1f3`)
    ///
    /// # Atomicity
    ///
    /// This method is **not atomic**.
    ///
    /// If a move in the batch fails:
    /// - all previously applied moves remain applied
    /// - processing stops immediately
    /// - the error is returned
    ///
    /// # Returns
    ///
    /// Returns a vector of [`MoveVerbose`] objects corresponding
    /// to each successfully played move.
    ///
    /// # Errors
    ///
    /// Returns an error if any move:
    /// - cannot be parsed
    /// - is illegal in the current position
    ///
    /// # Examples
    ///
    /// ```js
    /// chess.playMovesBatch(["e4", "e5", "Nf3", "Nc6"])
    ///
    /// chess.playMovesBatch([
    ///   "e2e4",
    ///   "e7e5",
    ///   "g1f3"
    /// ])
    /// ```
    #[wasm_bindgen(js_name = "playMovesBatch")]
    pub fn play_moves_batch(&mut self, moves: Vec<MoveString>) -> Result<Vec<MoveVerbose>, String> {
        moves
            .iter()
            .map(|move_str| {
                return self.make_move(move_str);
            })
            .collect::<Result<Vec<MoveVerbose>, String>>()
    }

    #[wasm_bindgen(js_name = "moveFromObj")]
    pub fn make_move_from_obj(&mut self, move_obj: MoveObject) -> Result<MoveVerbose, String> {
        let mut move_str = String::with_capacity(5);
        move_str.push_str(&move_obj.from.as_str());
        move_str.push_str(&move_obj.to.as_str());

        if let Some(val) = move_obj.promotion {
            move_str.push_str(val.as_str());
        }

        self.make_move(&move_str)
    }

    /// Undoes the last move.
    ///
    /// Restores:
    /// - board state
    /// - side to move
    /// - castling rights
    /// - en passant state
    /// - repetition tracking
    ///
    /// Returns the undone move in verbose format.
    ///
    /// Returns `None` if the game history is empty.
    ///
    /// # chess.js Compatibility
    ///
    /// Behaves similarly to `chess.undo()`.
    pub fn undo(&mut self) -> Option<MoveVerbose> {
        let last = match self.history.pop() {
            Some(h) => h,
            None => return None,
        };

        if let Some(count) = self.repetition_table.get_mut(&self.hash) {
            *count -= 1;
            if *count <= 0 {
                self.repetition_table.remove(&self.hash);
            }
        }
        self.chess = last.position_before;
        self.hash = self.chess.zobrist_hash(shakmaty::EnPassantMode::Legal);

        self.repetition_table.entry(self.hash).or_insert(1);

        let move_verbose: MoveVerbose =
            helpers::parsing::verbose_move_from_raw_move(last.raw_move, &self.chess);

        Some(move_verbose)
    }
}
