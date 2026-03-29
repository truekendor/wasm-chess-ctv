#[cfg(test)]
pub mod test {
    use std::usize;

    use crate::WasmChess;

    #[test]
    fn test_new_game_initial_position() {
        let chess = WasmChess::new(None).unwrap();

        assert_eq!(
            chess.fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
        assert_eq!(chess.turn(), "w");
        assert_eq!(chess.fullmoves(), 1);
        assert_eq!(chess.halfmoves(), 0);
        assert!(!chess.is_game_over());
        assert!(!chess.is_check());
        assert!(!chess.is_checkmate());
    }

    #[test]
    fn test_make_move_uci() {
        let mut chess = WasmChess::new(None).unwrap();

        // Valid moves
        assert!(chess.make_move("e2e4").is_ok());
        assert_eq!(
            chess.fen(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
        );
        assert_eq!(chess.turn(), "b");
        assert_eq!(chess.fullmoves(), 1);
        assert_eq!(chess.halfmoves(), 0);

        assert!(chess.make_move("e7e5").is_ok());
        assert_eq!(chess.turn(), "w");
        assert_eq!(chess.fullmoves(), 2);

        // Invalid move
        let result = chess.make_move("e2e4");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Illegal move"));
    }

    #[test]
    fn test_game_over_conditions() {
        let mut chess = WasmChess::new(None).unwrap();

        assert!(!chess.is_game_over());

        // Fool's mate
        chess.make_move("f2f3").unwrap();
        chess.make_move("e7e5").unwrap();
        chess.make_move("g2g4").unwrap();
        chess.make_move("d8h4").unwrap();

        assert!(chess.is_game_over());
        assert!(chess.is_checkmate());
        assert!(chess.is_check());
    }

    // #[test]
    fn test_make_move_from_object() {
        todo!("add serde kson to dependencies")
        // let mut chess = WasmChess::new(None).unwrap();

        // let move_obj = json!({
        //     "from": "e2",
        //     "to": "e4"
        // });

        // assert!(chess.make_move_from_obj(move_obj.into()).is_ok());
        // assert_eq!(
        //     chess.fen(),
        //     "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
        // );
    }

    fn test_make_move_with_promotion() {
        todo!()
    }

    #[test]
    fn test_history_recording() {
        let mut chess = WasmChess::new(None).unwrap();

        assert_eq!(chess.history().unwrap().len(), 0);

        chess.make_move("e2e4").unwrap();
        chess.make_move("e7e5").unwrap();
        chess.make_move("g1f3").unwrap();

        let history = chess.history().unwrap();
        // TODO BRUH the move format is insane
        // todo will change to SAN
        assert_eq!(history.len(), 3);
        assert_eq!(history[0], "e2-e4");
        assert_eq!(history[1], "e7-e5");
        assert_eq!(history[2], "Ng1-f3");

        chess.undo().unwrap();
        let history_after_undo = chess.history().unwrap();
        assert_eq!(history_after_undo.len(), 2);
    }

    // #[test]
    fn test_history_verbose() {
        // let mut chess = WasmChess::new(None).unwrap();

        // chess.make_move("e2e4").unwrap();

        // let verbose = chess.history_verbose().unwrap();
        // assert_eq!(verbose.len(), 1);
        // assert!(verbose[0].contains("e2-e4"));
        // assert!(verbose[0].contains("fen:"));
        // assert!(verbose[0].contains("turn:"));
    }

    #[test]
    fn test_position_count_for_repetition() {
        let mut chess = WasmChess::new(None).unwrap();

        // Position should be counted once initially
        assert!(!chess.is_threefold_repetition());

        // Repeat same position twice more
        chess.make_move("g1f3").unwrap();
        chess.make_move("g8f6").unwrap();
        chess.make_move("f3g1").unwrap();
        chess.make_move("f6g8").unwrap();

        // After 2nd repetition, should still be false (needs 3)
        assert!(!chess.is_threefold_repetition());

        chess.make_move("g1f3").unwrap();
        chess.make_move("g8f6").unwrap();
        chess.make_move("f3g1").unwrap();
        chess.make_move("f6g8").unwrap();

        // After 3rd repetition, should be true
        assert!(chess.is_threefold_repetition());
    }

    #[test]
    fn test_halfmoves_counter() {
        let mut chess = WasmChess::new(None).unwrap();

        assert_eq!(chess.halfmoves(), 0);
        chess.make_move("e2e4").unwrap();
        assert_eq!(chess.halfmoves(), 0); // Pawn move resets counter
        chess.make_move("e7e5").unwrap();
        assert_eq!(chess.halfmoves(), 0); // Pawn move resets counter

        chess.make_move("g1f3").unwrap();
        assert_eq!(chess.halfmoves(), 1);
        chess.make_move("g8f6").unwrap();
        assert_eq!(chess.halfmoves(), 2);
    }

    #[test]
    fn test_reset() {
        let mut chess = WasmChess::new(Some(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        ))
        .unwrap();
        chess.make_move("a2a3").unwrap_or(());

        chess.reset();

        assert_eq!(
            chess.fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
        assert_eq!(chess.history().unwrap().len(), 0);
    }

    #[test]
    fn test_load() {
        let mut chess = WasmChess::new(None).unwrap();

        let fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2";
        assert!(chess.load(fen.to_string()).is_ok());
        assert_eq!(chess.fen(), fen);

        // Test invalid FEN
        let result = chess.load("invalid".to_string());
        assert!(result.is_err());
    }

    #[cfg(test)]
    mod undo_logic_test {
        use super::*;

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
}
