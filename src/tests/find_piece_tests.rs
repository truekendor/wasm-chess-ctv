#[cfg(test)]
pub mod find_piece_tests {
    use crate::WasmChess;
    use crate::helpers::tsify_structs::*;

    #[test]
    fn find_pawns_from_str() {
        let chess = WasmChess::new(None).unwrap();

        let w_pawns = chess.find_piece_from_str("P").unwrap();
        let b_pawns = chess.find_piece_from_str("p").unwrap();

        let expected_white: Vec<SquareStr> = vec![
            SquareStr::A2,
            SquareStr::B2,
            SquareStr::C2,
            SquareStr::D2,
            SquareStr::E2,
            SquareStr::F2,
            SquareStr::G2,
            SquareStr::H2,
        ];

        let expected_black: Vec<SquareStr> = vec![
            SquareStr::A7,
            SquareStr::B7,
            SquareStr::C7,
            SquareStr::D7,
            SquareStr::E7,
            SquareStr::F7,
            SquareStr::G7,
            SquareStr::H7,
        ];

        pretty_assertions::assert_eq!(w_pawns, expected_white);
        pretty_assertions::assert_eq!(b_pawns, expected_black);
    }

    #[test]
    fn find_pawns_from_obj() {
        let chess = WasmChess::new(None).unwrap();

        let w_pawns = chess
            .find_piece_from_obj(PieceObj {
                r#type: PieceSymbol::P,
                color: ColorChar::W,
            })
            .unwrap();
        let b_pawns = chess
            .find_piece_from_obj(PieceObj {
                r#type: PieceSymbol::P,
                color: ColorChar::B,
            })
            .unwrap();

        let expected_white: Vec<SquareStr> = vec![
            SquareStr::A2,
            SquareStr::B2,
            SquareStr::C2,
            SquareStr::D2,
            SquareStr::E2,
            SquareStr::F2,
            SquareStr::G2,
            SquareStr::H2,
        ];

        let expected_black: Vec<SquareStr> = vec![
            SquareStr::A7,
            SquareStr::B7,
            SquareStr::C7,
            SquareStr::D7,
            SquareStr::E7,
            SquareStr::F7,
            SquareStr::G7,
            SquareStr::H7,
        ];

        pretty_assertions::assert_eq!(w_pawns, expected_white);
        pretty_assertions::assert_eq!(b_pawns, expected_black);
    }

    #[test]
    fn find_missing_piece_from_obj() {
        let chess = WasmChess::new(Some(
            "8/6p1/8/2k4p/1R3P1P/Pp2K1P1/r7/8 w - - 1 44".to_string(),
        ))
        .unwrap();

        let w_queen = chess
            .find_piece_from_obj(PieceObj {
                r#type: PieceSymbol::Q,
                color: ColorChar::W,
            })
            .unwrap();

        pretty_assertions::assert_eq!(w_queen.len(), 0);
    }

    #[test]
    fn errors_on_invalid_piece() {
        let chess = WasmChess::new(None).unwrap();

        let bad_piece = chess.find_piece_from_str("bad_piece");

        assert!(bad_piece.is_err());
    }
}
