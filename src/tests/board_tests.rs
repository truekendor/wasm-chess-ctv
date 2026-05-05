#[cfg(test)]
/// these tests are from chess.js test suite for board() method
///
/// @link https://github.com/jhlywa/chess.js/blob/master/__tests__/board.test.ts
pub mod board_tests {
    use crate::{
        WasmChess,
        tsify_structs::{PieceSymbol, others::*, *},
    };

    #[test]
    fn board_starting_pos_ok() {
        let chess = WasmChess::new(None).unwrap();

        let answer = vec![
            vec![
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::A8,
                    r#type: PieceSymbol::R,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::B8,
                    r#type: PieceSymbol::N,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::C8,
                    r#type: PieceSymbol::B,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::D8,
                    r#type: PieceSymbol::Q,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::E8,
                    r#type: PieceSymbol::K,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::F8,
                    r#type: PieceSymbol::B,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::G8,
                    r#type: PieceSymbol::N,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::H8,
                    r#type: PieceSymbol::R,
                }),
            ],
            vec![
                // pawns
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::A7,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::B7,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::C7,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::D7,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::E7,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::F7,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::G7,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::H7,
                    r#type: PieceSymbol::P,
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
                    color: others::ColorChar::W,
                    square: SquareStr::A2,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::B2,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::C2,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::D2,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::E2,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::F2,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::G2,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::H2,
                    r#type: PieceSymbol::P,
                }),
            ],
            vec![
                // pieces
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::A1,
                    r#type: PieceSymbol::R,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::B1,
                    r#type: PieceSymbol::N,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::C1,
                    r#type: PieceSymbol::B,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::D1,
                    r#type: PieceSymbol::Q,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::E1,
                    r#type: PieceSymbol::K,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::F1,
                    r#type: PieceSymbol::B,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::G1,
                    r#type: PieceSymbol::N,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::H1,
                    r#type: PieceSymbol::R,
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
                    square: SquareStr::A8,
                    color: others::ColorChar::B,
                    r#type: PieceSymbol::R,
                }),
                None,
                None,
                None,
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::E8,
                    r#type: PieceSymbol::K,
                }),
                None,
                None,
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::H8,
                    r#type: PieceSymbol::R,
                }),
            ],
            vec![
                // pawns
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::A7,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::B7,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::C7,
                    r#type: PieceSymbol::P,
                }),
                None,
                None,
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::F7,
                    r#type: PieceSymbol::P,
                }),
                None,
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::H7,
                    r#type: PieceSymbol::P,
                }),
            ],
            //
            vec![
                None,
                None,
                Some(SquareInfoObj {
                    color: others::ColorChar::B,
                    square: SquareStr::C6,
                    r#type: PieceSymbol::N,
                }),
                None,
                Some(SquareInfoObj {
                    square: SquareStr::E6,
                    color: others::ColorChar::B,
                    r#type: PieceSymbol::P,
                }),
                None,
                Some(SquareInfoObj {
                    square: SquareStr::G6,
                    color: others::ColorChar::B,
                    r#type: PieceSymbol::P,
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
                    square: SquareStr::C4,
                    color: others::ColorChar::W,
                    r#type: PieceSymbol::B,
                }),
                None,
                None,
                Some(SquareInfoObj {
                    square: SquareStr::F4,
                    color: others::ColorChar::W,
                    r#type: PieceSymbol::P,
                }),
                None,
                Some(SquareInfoObj {
                    square: SquareStr::H4,
                    color: others::ColorChar::B,
                    r#type: PieceSymbol::Q,
                }),
            ],
            //
            vec![
                None,
                None,
                Some(SquareInfoObj {
                    square: SquareStr::C3,
                    color: others::ColorChar::W,
                    r#type: PieceSymbol::N,
                }),
                Some(SquareInfoObj {
                    square: SquareStr::D3,
                    color: others::ColorChar::W,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    square: SquareStr::E3,
                    color: others::ColorChar::B,
                    r#type: PieceSymbol::B,
                }),
                None,
                Some(SquareInfoObj {
                    square: SquareStr::G3,
                    color: others::ColorChar::B,
                    r#type: PieceSymbol::N,
                }),
                None,
            ],
            //
            vec![
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::A2,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::B2,
                    r#type: PieceSymbol::P,
                }),
                None,
                None,
                None,
                None,
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::G2,
                    r#type: PieceSymbol::P,
                }),
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::H2,
                    r#type: PieceSymbol::P,
                }),
            ],
            vec![
                // pieces
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::A1,
                    r#type: PieceSymbol::R,
                }),
                None,
                None,
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::D1,
                    r#type: PieceSymbol::Q,
                }),
                None,
                None,
                None,
                Some(SquareInfoObj {
                    color: others::ColorChar::W,
                    square: SquareStr::H1,
                    r#type: PieceSymbol::K,
                }),
            ],
        ];

        let board_state = chess.board().board_matrix;

        pretty_assertions::assert_eq!(board_state, answer);
    }
}
