#[cfg(test)]
pub mod fen_tests {
    use crate::WasmChess;
    use std::usize;

    #[test]
    fn test_new_game_custom_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let fen_no_ep = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
        let chess = WasmChess::new(Some(fen.to_string())).unwrap();

        pretty_assertions::assert_eq!(chess.fen(Some(true)), fen);
        pretty_assertions::assert_eq!(chess.fen(None), fen_no_ep);
        pretty_assertions::assert_eq!(chess.turn(), "b");
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

    /// this test and tests below are taken from chess.js test suite to verify that the FEN output is consistent with chess.js
    ///
    ///  @link https://github.com/jhlywa/chess.js/blob/master/__tests__/fen.test.ts
    pub mod chess_js_fen_tests {
        use super::*;

        #[test]
        fn ep_square_only_if_legal() {
            let mut wasm_chess =
                WasmChess::new(Some("4k3/8/8/8/5p2/8/4P3/4K3 w - - 0 1".to_string())).unwrap();

            wasm_chess.make_move("e4").unwrap();

            pretty_assertions::assert_eq!(
                wasm_chess.fen(None),
                "4k3/8/8/8/4Pp2/8/8/4K3 b - e3 0 1"
            );
        }

        #[test]
        fn ep_square_only_if_legal_pinned_first() {
            let mut wasm_chess =
                WasmChess::new(Some("5k2/8/8/8/5p2/8/4P3/4KR2 w - - 0 1".to_string())).unwrap();

            wasm_chess.make_move("e4").unwrap();

            pretty_assertions::assert_eq!(
                wasm_chess.fen(None),
                "5k2/8/8/8/4Pp2/8/8/4KR2 b - - 0 1"
            );
        }

        #[test]
        fn ep_square_only_if_legal_pinned_second() {
            let mut wasm_chess = WasmChess::new(Some(
                "rnb1kbn1/p1p1pp2/PpPp2qr/5Pp1/8/R1P4p/1PK1P1PP/1NBQ1BNR b - - 0 1".to_string(),
            ))
            .unwrap();

            wasm_chess.make_move("e5").unwrap();

            pretty_assertions::assert_eq!(
                wasm_chess.fen(None),
                "rnb1kbn1/p1p2p2/PpPp2qr/4pPp1/8/R1P4p/1PK1P1PP/1NBQ1BNR w - - 0 2"
            );
        }

        #[test]
        fn allow_ep_square_by_option_pinned() {
            let mut wasm_chess = WasmChess::new(Some(
                "rnb1kbn1/p1p1pp2/PpPp2qr/5Pp1/8/R1P4p/1PK1P1PP/1NBQ1BNR b - - 0 1".to_string(),
            ))
            .unwrap();

            wasm_chess.make_move("e5").unwrap();

            pretty_assertions::assert_eq!(
                wasm_chess.fen(Some(true)),
                "rnb1kbn1/p1p2p2/PpPp2qr/4pPp1/8/R1P4p/1PK1P1PP/1NBQ1BNR w - e6 0 2"
            );
        }

        #[test]
        fn force_en_passant_square_by_option() {
            let mut wasm_chess = WasmChess::new(None).unwrap();

            wasm_chess.make_move("e4").unwrap();
            let fen_with_ep = wasm_chess.fen(Some(true));
            let fen_with_without_ep = wasm_chess.fen(Some(false));
            let fen_with_without_ep_none = wasm_chess.fen(None);

            pretty_assertions::assert_eq!(
                fen_with_ep,
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
            );

            pretty_assertions::assert_eq!(
                fen_with_without_ep,
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
            );

            pretty_assertions::assert_eq!(
                fen_with_without_ep_none,
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
            );
        }
    }
}
