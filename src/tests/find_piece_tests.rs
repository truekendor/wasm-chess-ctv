// TODO: add more tests for find_piece

#[cfg(test)]
pub mod find_piece_tests {
    use crate::WasmChess;
    use crate::helpers::tsify::*;

    #[test]
    fn find_piece() {
        let chess = WasmChess::new(None).unwrap();

        let w_pawns = chess.find_piece("P".to_string()).unwrap();
        let b_pawns = chess.find_piece("p".to_string()).unwrap();

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
}
