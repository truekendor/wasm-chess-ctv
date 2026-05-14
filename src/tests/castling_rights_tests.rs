#[cfg(test)]
pub mod castling_rights_tests {
    use crate::{
        WasmChess,
        tsify_structs::others::{CastlingObj, ColorChar},
    };

    #[test]
    fn correct_rights_from_set_fen() {
        let no_castling_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Aa - 0 1";

        let chess = WasmChess::new(Some(no_castling_fen.to_string())).unwrap();

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::W),
            CastlingObj {
                king: Some(false),
                queen: Some(true)
            }
        );

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::B),
            CastlingObj {
                king: Some(false),
                queen: Some(true)
            }
        );
    }

    #[test]
    fn no_castling_after_rook_move() {
        let no_castling_fen = "rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w AHah - 0 1";

        let mut chess = WasmChess::new(Some(no_castling_fen.to_string())).unwrap();

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::W),
            CastlingObj {
                king: Some(true),
                queen: Some(true)
            }
        );

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::B),
            CastlingObj {
                king: Some(true),
                queen: Some(true)
            }
        );

        chess.make_move("Ra2").unwrap();
        chess.make_move("Rh7").unwrap();

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::W),
            CastlingObj {
                king: Some(true),
                queen: Some(false)
            }
        );
        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::B),
            CastlingObj {
                king: Some(false),
                queen: Some(true)
            }
        );
    }

    #[test]
    fn no_castling_after_king_move() {
        let no_castling_fen = "rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w AHah - 0 1";

        let mut chess = WasmChess::new(Some(no_castling_fen.to_string())).unwrap();

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::W),
            CastlingObj {
                king: Some(true),
                queen: Some(true)
            }
        );

        chess.make_move("Ke2").unwrap();

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::W),
            CastlingObj {
                king: Some(false),
                queen: Some(false)
            }
        );
    }
}
