#[cfg(test)]
pub mod fen_tests {
    use crate::WasmChess;
    use std::usize;

    #[test]
    fn test_new_game_custom_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let chess = WasmChess::new(Some(fen.to_string())).unwrap();

        assert_eq!(chess.fen(), fen);
        assert_eq!(chess.turn(), "b");
    }

    fn test_check_detection() {
        todo!()
    }

    #[test]
    fn test_stalemate() {
        let mut chess = WasmChess::new(Some("6bk/R7/7K/8/8/8/8/8 w - - 0 1".to_string())).unwrap();

        assert!(!chess.is_draw());
        chess.make_move("a7a8").unwrap();
        // King has no legal moves but not in check = stalemate
        assert!(chess.is_draw());
        assert!(!chess.is_checkmate());
    }

    #[test]
    fn test_invalid_fen_returns_error() {
        let invalid_fen = "invalid fen string";
        let result = WasmChess::new(Some(invalid_fen.to_string()));

        assert!(result.is_err());
        assert!(result.is_err_and(|err| { err.contains("Error parsing fen string") }));
    }

    // #[test]
    fn test_fifty_move_rule() {
        todo!()
    }

    #[test]
    fn test_threefold_repetition() {
        let mut chess = WasmChess::new(None).unwrap();

        assert!(!chess.is_threefold_repetition());

        // Create a repeating position sequence
        let moves = ["g1f3", "g8f6", "f3g1", "f6g8"];

        for _ in 0..3 {
            for mv in moves.iter() {
                chess.make_move(mv).unwrap();
            }
        }

        // After three repetitions, should be detected
        assert!(chess.is_threefold_repetition());
        assert!(chess.is_game_over());
    }

    #[test]
    fn test_get_piece_at_square() {
        let chess = WasmChess::new(None).unwrap();

        assert_eq!(chess.get("e2".to_string()).unwrap(), "P");
        assert_eq!(chess.get("e7".to_string()).unwrap(), "p");
        assert_eq!(chess.get("a1".to_string()).unwrap(), "R");
        assert_eq!(chess.get("h8".to_string()).unwrap(), "r");
        assert!(chess.get("e4".to_string()).is_none());
    }

    #[test]
    fn test_fen_at_index_before_any_moves() {
        let wasm_chess = WasmChess::new(None).unwrap();

        assert!(wasm_chess.fen_at(0).is_none());
        assert!(wasm_chess.fen_at(100).is_none());
    }

    #[test]
    fn test_fen_at() {
        let mut wasm_chess = WasmChess::new(None).unwrap();

        wasm_chess.make_move("e2e4").unwrap();
        wasm_chess.make_move("e7e5").unwrap();
        wasm_chess.make_move("g1f3").unwrap();
        wasm_chess.make_move("h7h6").unwrap();

        assert_eq!(
            wasm_chess.fen_at(0).unwrap(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
        assert_eq!(
            wasm_chess.fen_at(1).unwrap(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
        );
        assert_eq!(
            wasm_chess.fen_at(2).unwrap(),
            "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2"
        );
        assert_eq!(
            wasm_chess.fen_at(3).unwrap(),
            "rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2"
        );

        assert!(wasm_chess.fen_at(4).is_none());
        assert!(wasm_chess.fen_at(10000).is_none());
        assert!(wasm_chess.fen_at(usize::MAX).is_none());
    }
}
