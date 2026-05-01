// TODO: rename file and module
// TODO: add more tests for edge cases, e.g. en passant, promotion, castling, ambiguous moves, etc.
// TODO: decompose into multiple test modules

#[cfg(test)]
pub mod test {
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
        todo!()
    }

    fn test_make_move_with_promotion() {
        todo!()
    }

    #[test]
    fn test_history_san_recording() {
        let mut chess = WasmChess::new(None).unwrap();

        assert_eq!(chess.history_san().len(), 0);

        chess.make_move("e2e4").unwrap();
        chess.make_move("e7e5").unwrap();
        chess.make_move("g1f3").unwrap();

        let history = chess.history_san();

        assert_eq!(history.len(), 3);
        assert_eq!(history[0], "e4");
        assert_eq!(history[1], "e5");
        assert_eq!(history[2], "Nf3");

        chess.undo().unwrap();
        let history_after_undo = chess.history_san();
        assert_eq!(history_after_undo.len(), 2);
    }

    #[test]
    fn test_history_uci_recording() {
        let mut chess = WasmChess::new(None).unwrap();

        assert_eq!(chess.history_san().len(), 0);

        chess.make_move("e2e4").unwrap();
        chess.make_move("e7e5").unwrap();
        chess.make_move("g1f3").unwrap();

        let history = chess.history_uci();

        assert_eq!(history.len(), 3);
        assert_eq!(history[0], "e2e4");
        assert_eq!(history[1], "e7e5");
        assert_eq!(history[2], "g1f3");

        chess.undo().unwrap();
        let history_after_undo = chess.history_san();
        assert_eq!(history_after_undo.len(), 2);
    }

    #[test]
    // TODO
    fn test_history_verbose() {
        let mut chess = WasmChess::new(None).unwrap();

        chess.make_move("e4").unwrap();
        chess.make_move("e5").unwrap();
        chess.make_move("Nf3").unwrap();
        chess.make_move("Nc6").unwrap();
        chess.make_move("d4").unwrap();
        chess.make_move("Nxd4").unwrap();

        let verbose = chess.history_verbose().map_err(|err| {
            println!("Error getting verbose history: {}", err);
        });

        verbose.iter().for_each(|el| println!("{:#?}", el));
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
        assert_eq!(chess.history_san().len(), 0);
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

    #[test]
    fn false_ambiguous_move() {
        let fen_str = String::from("8/1Q2bk2/P2p2p1/2pPp3/2P1P3/2N2n2/2KN1q2/8 w - - 1 61");
        let mut chess = WasmChess::new(Some(fen_str)).unwrap();

        chess.make_move("Nb1").unwrap();
    }

    #[test]
    fn move_illegal() {
        let fen_str = String::from("8/1Q2bk2/P2p2p1/2pPp3/2P1P3/2N2n2/2KN1q2/8 w - - 1 61");
        let mut chess = WasmChess::new(Some(fen_str)).unwrap();

        let result = chess.make_move("Ndb1");

        assert!(result.is_err())
    }

    #[test]
    fn ambiguous_move() {
        let fen_str = String::from("8/1Q2bk2/P2p2p1/2pPp3/2P1P3/2N2n2/2KN4/8 w - - 1 61");
        let mut chess = WasmChess::new(Some(fen_str)).unwrap();

        let result = chess.make_move("Nb1");

        assert!(result.is_err());
    }
}
