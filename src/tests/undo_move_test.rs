#[cfg(test)]
mod undo_logic_test {
    use crate::WasmChess;
    use crate::tsify_structs::{MoveVerbose, SquareStr, others::*};

    #[test]
    fn test_undo_after_two_moves() {
        let mut wasm_chess = WasmChess::new(None).unwrap();
        let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        wasm_chess.make_move("e4").unwrap();
        wasm_chess.make_move("e7e5").unwrap();

        pretty_assertions::assert_eq!(wasm_chess.fen_at(0).unwrap(), starting_fen);

        pretty_assertions::assert_eq!(
            wasm_chess.fen_at(1).unwrap(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
        );

        let move_str = wasm_chess.undo().unwrap();

        pretty_assertions::assert_eq!(
            move_str,
            MoveVerbose {
                from: SquareStr::E7,
                to: SquareStr::E5,
                before: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".to_string(),
                after: "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".to_string(),
                color: ColorChar::B,
                piece: "p".to_string(),
                captured: None,
                promotion: None,
                san: "e5".to_string(),
                lan: "e7e5".to_string(),
                is_en_passant: false,
                is_castle: false,
            }
        );

        pretty_assertions::assert_eq!(
            wasm_chess.fen_at(1),
            Some("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".to_string())
        );
        pretty_assertions::assert_eq!(wasm_chess.fen_at(0).unwrap(), starting_fen);

        let move_str = wasm_chess.undo().unwrap();
        pretty_assertions::assert_eq!(
            move_str,
            MoveVerbose {
                from: SquareStr::E2,
                to: SquareStr::E4,
                before: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
                after: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".to_string(),
                color: ColorChar::W,
                piece: "p".to_string(),
                captured: None,
                promotion: None,
                san: "e4".to_string(),
                lan: "e2e4".to_string(),
                is_en_passant: false,
                is_castle: false,
            }
        );

        let null_undo_result = wasm_chess.undo();
        pretty_assertions::assert_eq!(null_undo_result, None);
    }

    #[test]
    fn test_undo() {
        let mut chess = WasmChess::new(None).unwrap();

        chess.make_move("e2e4").unwrap();
        chess.make_move("e7e5").unwrap();

        pretty_assertions::assert_eq!(chess.history.len(), 2);
        pretty_assertions::assert_eq!(chess.repetition_table.len(), 3);

        let undo_result = chess.undo();

        pretty_assertions::assert_eq!(chess.history.len(), 1);
        pretty_assertions::assert_eq!(chess.repetition_table.len(), 2);

        pretty_assertions::assert_eq!(undo_result.unwrap().san, "e5".to_string());
        pretty_assertions::assert_eq!(chess.turn(), ColorChar::B);
        pretty_assertions::assert_eq!(chess.fullmoves(), 1);

        // Undo again
        let undo_result2 = chess.undo();
        assert!(undo_result2.is_some());
        pretty_assertions::assert_eq!(undo_result2.unwrap().san, "e4".to_string());
        pretty_assertions::assert_eq!(
            chess.fen(None),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );

        // Undo when no moves left
        assert!(chess.undo().is_none());
        assert!(chess.undo().is_none());

        pretty_assertions::assert_eq!(chess.history.len(), 0);
        // we always have starting position in position count, so it should never be 0
        pretty_assertions::assert_eq!(chess.repetition_table.len(), 1);
    }
}
