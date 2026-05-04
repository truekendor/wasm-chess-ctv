#[cfg(test)]
/// these tests are from chess.js test suite for board() method
///
/// @link https://github.com/jhlywa/chess.js/blob/master/__tests__/board.test.ts
pub mod board_tests {
    use crate::{
        WasmChess,
        helpers::tsify_structs::{self, SquareInfoObj},
    };

    #[test]
    fn board_starting_pos_ok() {
        let chess = WasmChess::new(None).unwrap();

        let answer = vec![
            vec![
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::A8,
                    r#type: tsify_structs::PieceSymbol::R,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::B8,
                    r#type: tsify_structs::PieceSymbol::N,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::C8,
                    r#type: tsify_structs::PieceSymbol::B,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::D8,
                    r#type: tsify_structs::PieceSymbol::Q,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::E8,
                    r#type: tsify_structs::PieceSymbol::K,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::F8,
                    r#type: tsify_structs::PieceSymbol::B,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::G8,
                    r#type: tsify_structs::PieceSymbol::N,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::H8,
                    r#type: tsify_structs::PieceSymbol::R,
                }),
            ],
            vec![
                // pawns
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::A7,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::B7,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::C7,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::D7,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::E7,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::F7,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::G7,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::H7,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
            ],
            //
            vec![None, None, None, None, None, None, None, None],
            //
            vec![None, None, None, None, None, None, None, None],
            //
            vec![None, None, None, None, None, None, None, None],
            //
            vec![None, None, None, None, None, None, None, None],
            //
            vec![
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::A2,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::B2,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::C2,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::D2,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::E2,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::F2,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::G2,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::H2,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
            ],
            vec![
                // pieces
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::A1,
                    r#type: tsify_structs::PieceSymbol::R,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::B1,
                    r#type: tsify_structs::PieceSymbol::N,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::C1,
                    r#type: tsify_structs::PieceSymbol::B,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::D1,
                    r#type: tsify_structs::PieceSymbol::Q,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::E1,
                    r#type: tsify_structs::PieceSymbol::K,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::F1,
                    r#type: tsify_structs::PieceSymbol::B,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::G1,
                    r#type: tsify_structs::PieceSymbol::N,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::H1,
                    r#type: tsify_structs::PieceSymbol::R,
                }),
            ],
        ];

        let board_state = chess.board().board_matrix;

        pretty_assertions::assert_eq!(answer, board_state);
    }

    #[test]
    fn board_set_pos_ok() {
        let fen = "r3k2r/ppp2p1p/2n1p1p1/8/2B2P1q/2NPb1n1/PP4PP/R2Q3K w kq - 0 8".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        let answer = vec![
            vec![
                Some(SquareInfoObj {
                    square: tsify_structs::SquareStr::A8,
                    color: tsify_structs::ColorChar::B,
                    r#type: tsify_structs::PieceSymbol::R,
                }),
                None,
                None,
                None,
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::E8,
                    r#type: tsify_structs::PieceSymbol::K,
                }),
                None,
                None,
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::H8,
                    r#type: tsify_structs::PieceSymbol::R,
                }),
            ],
            vec![
                // pawns
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::A7,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::B7,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::C7,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                None,
                None,
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::F7,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                None,
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::H7,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
            ],
            //
            vec![
                None,
                None,
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::B,
                    square: tsify_structs::SquareStr::C6,
                    r#type: tsify_structs::PieceSymbol::N,
                }),
                None,
                Some(SquareInfoObj {
                    square: tsify_structs::SquareStr::E6,
                    color: tsify_structs::ColorChar::B,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                None,
                Some(SquareInfoObj {
                    square: tsify_structs::SquareStr::G6,
                    color: tsify_structs::ColorChar::B,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                None,
            ],
            //
            vec![None, None, None, None, None, None, None, None],
            //
            vec![
                None,
                None,
                Some(SquareInfoObj {
                    square: tsify_structs::SquareStr::C4,
                    color: tsify_structs::ColorChar::W,
                    r#type: tsify_structs::PieceSymbol::B,
                }),
                None,
                None,
                Some(SquareInfoObj {
                    square: tsify_structs::SquareStr::F4,
                    color: tsify_structs::ColorChar::W,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                None,
                Some(SquareInfoObj {
                    square: tsify_structs::SquareStr::H4,
                    color: tsify_structs::ColorChar::B,
                    r#type: tsify_structs::PieceSymbol::Q,
                }),
            ],
            //
            vec![
                None,
                None,
                Some(SquareInfoObj {
                    square: tsify_structs::SquareStr::C3,
                    color: tsify_structs::ColorChar::W,
                    r#type: tsify_structs::PieceSymbol::N,
                }),
                Some(SquareInfoObj {
                    square: tsify_structs::SquareStr::D3,
                    color: tsify_structs::ColorChar::W,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    square: tsify_structs::SquareStr::E3,
                    color: tsify_structs::ColorChar::B,
                    r#type: tsify_structs::PieceSymbol::B,
                }),
                None,
                Some(SquareInfoObj {
                    square: tsify_structs::SquareStr::G3,
                    color: tsify_structs::ColorChar::B,
                    r#type: tsify_structs::PieceSymbol::N,
                }),
                None,
            ],
            //
            vec![
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::A2,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::B2,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                None,
                None,
                None,
                None,
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::G2,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::H2,
                    r#type: tsify_structs::PieceSymbol::P,
                }),
            ],
            vec![
                // pieces
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::A1,
                    r#type: tsify_structs::PieceSymbol::R,
                }),
                None,
                None,
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::D1,
                    r#type: tsify_structs::PieceSymbol::Q,
                }),
                None,
                None,
                None,
                Some(SquareInfoObj {
                    color: tsify_structs::ColorChar::W,
                    square: tsify_structs::SquareStr::H1,
                    r#type: tsify_structs::PieceSymbol::K,
                }),
            ],
        ];

        let board_state = chess.board().board_matrix;

        pretty_assertions::assert_eq!(board_state, answer);
    }
}
