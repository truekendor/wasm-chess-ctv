#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use shakmaty::fen::Fen;

    use crate::{WasmChess, helpers::tsify_structs::ColorChar};

    #[test]
    fn test_turn_at_after_moves() {
        let mut board = WasmChess::new(None).unwrap();

        // Make a move: e4
        board.make_move("e4").unwrap();

        // After 1 move, it should be Black's turn
        pretty_assertions::assert_eq!(board.turn_at(0), Some(ColorChar::W)); // Starting position
        pretty_assertions::assert_eq!(board.turn_at(1), Some(ColorChar::B)); // After e4
        pretty_assertions::assert_eq!(board.turn_at(2), None); // Beyond move count
    }

    #[test]
    fn test_turn_at_after_two_moves() {
        let mut board = WasmChess::new(None).unwrap();

        // Make moves: e4 e5
        board.make_move("e4").unwrap();
        board.make_move("e5").unwrap();

        // After 2 moves, White's turn again
        assert_eq!(board.turn_at(0), Some(ColorChar::W)); // Starting position
        assert_eq!(board.turn_at(1), Some(ColorChar::B)); // After e4
        assert_eq!(board.turn_at(2), Some(ColorChar::W)); // After e5
    }

    #[test]
    fn test_turn_at_invalid_index() {
        let mut board = WasmChess::new(None).unwrap();

        // Empty board (no moves)
        assert_eq!(board.turn_at(0), Some(ColorChar::W));
        assert_eq!(board.turn_at(1), None);
        assert_eq!(board.turn_at(5), None);

        board.make_move("e4").unwrap();

        // After one move
        assert_eq!(board.turn_at(2), None);
        assert_eq!(board.turn_at(99), None);
    }

    #[test]
    fn test_move_at_starting_position() {
        let board = WasmChess::new(None).unwrap();

        // No move at index 0
        pretty_assertions::assert_eq!(board.move_at(0), None);
    }

    #[test]
    fn test_move_at_first_move() {
        let mut board = WasmChess::new(None).unwrap();

        // Make first move: e4
        board.make_move("e4").unwrap();

        let move_obj = board.move_at(1);
        assert!(move_obj.is_some());

        let move_obj = move_obj.unwrap();
        pretty_assertions::assert_eq!(move_obj.from.to_string(), "e2");
        pretty_assertions::assert_eq!(move_obj.to.to_string(), "e4");
        pretty_assertions::assert_eq!(move_obj.promotion, None);
    }

    #[test]
    fn test_move_at_second_move() {
        let mut board = WasmChess::new(None).unwrap();

        // Make moves: e4 e5
        board.make_move("e4").unwrap();
        board.make_move("e5").unwrap();

        let first_move = board.move_at(1);
        assert!(first_move.is_some());
        pretty_assertions::assert_eq!(first_move.unwrap().to.to_string(), "e4");

        let second_move = board.move_at(2);
        assert!(second_move.is_some());
        pretty_assertions::assert_eq!(second_move.unwrap().to.to_string(), "e5");
    }

    #[test]
    fn test_move_at_promotion() {
        let starting_fen = "rnbqkbnr/pppppppP/8/8/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1".to_string();
        let mut board = WasmChess::new(Some(starting_fen)).unwrap();

        // Make promotion move: h7h8=Q
        board.make_move("h7g8q").unwrap();

        let move_obj = board.move_at(1);
        assert!(move_obj.is_some());

        let move_obj = move_obj.unwrap();
        pretty_assertions::assert_eq!(move_obj.promotion, Some("q".to_string()));
    }

    #[test]
    fn test_move_at_invalid_index() {
        let mut board = WasmChess::new(None).unwrap();
        board.make_move("e4").unwrap();

        // Index out of bounds
        pretty_assertions::assert_eq!(board.move_at(0), None);
        pretty_assertions::assert_eq!(board.move_at(2), None);
        pretty_assertions::assert_eq!(board.move_at(100), None);

        board.reset();
        pretty_assertions::assert_eq!(board.move_at(1), None);
    }

    #[test]
    fn test_move_at_after_reseting() {
        let mut board = WasmChess::new(None).unwrap();
        board.make_move("e4").unwrap();
        board.make_move("e5").unwrap();

        assert!(board.move_at(0).is_none());
        assert!(board.move_at(1).is_some());

        board.reset();
        pretty_assertions::assert_eq!(board.move_at(0), None);
        pretty_assertions::assert_eq!(board.move_at(1), None);
    }

    #[test]
    fn test_move_at_after_setting_fen() {
        let mut board = WasmChess::new(None).unwrap();
        let starting_fen = board.fen(None);
        board.make_move("e4").unwrap();
        board.make_move("e5").unwrap();

        assert!(board.move_at(0).is_none());
        assert!(board.move_at(1).is_some());

        board
            .set_fen(Fen::from_str(&starting_fen).unwrap())
            .unwrap();
        pretty_assertions::assert_eq!(board.move_at(0), None);
        pretty_assertions::assert_eq!(board.move_at(1), None);
    }

    #[test]
    fn test_move_at_consistency_with_history() {
        let mut board = WasmChess::new(None).unwrap();

        // Play a sequence of moves
        let moves = ["e2e4", "e7e5", "g1f3", "b8c6"];

        for (i, move_str) in moves.iter().enumerate() {
            board.make_move(move_str).unwrap();

            // Check that move_at(i+1) returns the move we just made
            let move_obj = board.move_at(i + 1);
            assert!(move_obj.is_some());

            // Verify the move can be converted back to UCI format
            let move_uci = format!(
                "{}{}",
                move_obj.as_ref().unwrap().from,
                move_obj.as_ref().unwrap().to
            );
            pretty_assertions::assert_eq!(move_uci, *move_str);
        }
    }

    // #[test]
    fn test_move_at_castling() {
        // TODO:
    }

    // #[test]
    fn test_move_at_en_passant() {
        //
    }
}
