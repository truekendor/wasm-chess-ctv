// TODO actually add tests

#[cfg(test)]
pub mod test {
    use std::usize;

    use crate::WasmChess;

    #[test]
    fn test_history_index_out_of_bound() {
        let wasm_chess = WasmChess::new(None).unwrap();

        assert_eq!(wasm_chess.fen_at(100), wasm_chess.fen());
        assert_eq!(wasm_chess.fen_at(usize::MAX), wasm_chess.fen());
        assert_eq!(wasm_chess.fen_at(0), wasm_chess.fen());
    }
}
