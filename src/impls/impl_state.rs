use super::*;

#[wasm_bindgen]
impl WasmChess {
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

    #[wasm_bindgen(js_name = "isStalemate")]
    pub fn is_stalemate(&self) -> bool {
        self.chess.is_stalemate()
    }

    #[wasm_bindgen(js_name = "isDraw")]
    pub fn is_draw(&self) -> bool {
        self.chess.is_stalemate()
            || self.chess.is_insufficient_material()
            || self.is_draw_by_fifty_moves()
            || self.is_threefold_repetition()
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
        self.repetition_table
            .get(&self.hash)
            .is_some_and(|&val| val >= 3)
    }

    pub fn fullmoves(&self) -> u32 {
        let move_number = &self.chess.fullmoves();

        move_number.get()
    }

    pub fn halfmoves(&self) -> u32 {
        self.chess.halfmoves()
    }

    pub fn perft(&self, depth: usize) -> u64 {
        shakmaty::perft(&self.chess, depth as u32)
    }
}
