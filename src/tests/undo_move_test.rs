#[cfg(test)]
mod undo_logic_test {
    use crate::WasmChess;

    #[test]
    fn test_undo_after_two_moves() {
        let mut wasm_chess = WasmChess::new(None).unwrap();
        let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        wasm_chess.make_move("e2e4").unwrap();
        wasm_chess.make_move("e7e5").unwrap();

        assert_eq!(wasm_chess.fen_at(0).unwrap(), starting_fen);

        assert_eq!(
            wasm_chess.fen_at(1).unwrap(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
        );

        let move_str = wasm_chess.undo().unwrap();
        // TODO wtf is this move format. Change it later to UCI?
        assert_eq!(move_str, "e7-e5");

        assert_eq!(wasm_chess.fen_at(1), None);
        assert_eq!(wasm_chess.fen_at(0).unwrap(), starting_fen);

        let move_str = wasm_chess.undo().unwrap();
        assert_eq!(move_str, "e2-e4");
    }

    #[test]
    fn test_undo() {
        let mut chess = WasmChess::new(None).unwrap();

        assert!(chess.make_move("e2e4").is_ok());
        assert!(chess.make_move("e7e5").is_ok());

        let undo_result = chess.undo();
        assert!(undo_result.is_ok());
        assert_eq!(undo_result.unwrap(), "e7-e5");

        assert_eq!(chess.turn(), "b");
        assert_eq!(chess.fullmoves(), 1);

        // Undo again
        let undo_result2 = chess.undo();
        assert!(undo_result2.is_ok());
        assert_eq!(undo_result2.unwrap(), "e2-e4");
        assert_eq!(
            chess.fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );

        // Undo when no moves left
        assert!(chess.undo().is_err());
    }
}
