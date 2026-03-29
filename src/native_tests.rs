// TODO actually add tests

#[cfg(test)]
pub mod test {
    use std::usize;

    use crate::WasmChess;

    #[test]
    fn test_fen_at_index_before_any_moves() {
        let wasm_chess = WasmChess::new(None).unwrap();

        assert_eq!(
            wasm_chess.fen_at(0).unwrap(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_fen_at_index_in_bound() {
        let mut wasm_chess = WasmChess::new(None).unwrap();

        wasm_chess.make_move("e2e4").unwrap();
        wasm_chess.make_move("e7e5").unwrap();
        wasm_chess.make_move("g1f3").unwrap();

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
    }

    #[test]
    fn test_fen_at_index_out_of_bound() {
        let mut wasm_chess = WasmChess::new(None).unwrap();

        wasm_chess.make_move("e2e4").unwrap();
        wasm_chess.make_move("e7e5").unwrap();
        wasm_chess.make_move("g1f3").unwrap();

        assert_eq!(wasm_chess.fen_at(3), None);
        assert_eq!(wasm_chess.fen_at(10000), None);
        assert_eq!(wasm_chess.fen_at(usize::MAX), None);
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
    }
}
