/// these tests are from chess.js test suite
///
/// @link https://github.com/jhlywa/chess.js/blob/master/__tests__/history.test.ts
#[cfg(test)]
pub mod history_test_chess_js {
    use crate::WasmChess;

    use crate::tsify_structs::{others::*, *};

    #[test]
    fn history() {
        let mut chess = WasmChess::new(None).unwrap();

        let final_fen = "4q2k/2r1r3/4PR1p/p1p5/P1Bp1Q1P/1P6/6P1/6K1 b - - 4 41";
        let moves = [
            "c4", "e6", "Nf3", "d5", "d4", "Nf6", "Nc3", "Be7", "Bg5", "O-O", "e3", "h6", "Bh4",
            "b6", "cxd5", "Nxd5", "Bxe7", "Qxe7", "Nxd5", "exd5", "Rc1", "Be6", "Qa4", "c5", "Qa3",
            "Rc8", "Bb5", "a6", "dxc5", "bxc5", "O-O", "Ra7", "Be2", "Nd7", "Nd4", "Qf8", "Nxe6",
            "fxe6", "e4", "d4", "f4", "Qe7", "e5", "Rb8", "Bc4", "Kh8", "Qh3", "Nf8", "b3", "a5",
            "f5", "exf5", "Rxf5", "Nh7", "Rcf1", "Qd8", "Qg3", "Re7", "h4", "Rbb7", "e6", "Rbc7",
            "Qe5", "Qe8", "a4", "Qd8", "R1f2", "Qe8", "R2f3", "Qd8", "Bd3", "Qe8", "Qe4", "Nf6",
            "Rxf6", "gxf6", "Rxf6", "Kg8", "Bc4", "Kh8", "Qf4",
        ];

        moves.iter().for_each(|m| {
            chess.make_move(m).unwrap();
        });

        pretty_assertions::assert_eq!(chess.fen(None), final_fen.to_string());

        let answer: Vec<MoveVerbose> = vec![
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::C2,
                to: SquareStr::C4,

                san: "c4".to_string(),
                lan: "c2c4".to_string(),
                before: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
                after: "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: true,

                is_castle: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,
                from: SquareStr::E7,
                to: SquareStr::E6,

                san: "e6".to_string(),
                lan: "e7e6".to_string(),
                before: "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
                after: "rnbqkbnr/pppp1ppp/4p3/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 2".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::N,

                from: SquareStr::G1,
                to: SquareStr::F3,

                san: "Nf3".to_string(),
                lan: "g1f3".to_string(),
                before: "rnbqkbnr/pppp1ppp/4p3/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 2".to_string(),
                after: "rnbqkbnr/pppp1ppp/4p3/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 1 2".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,
                from: SquareStr::D7,

                to: SquareStr::D5,

                san: "d5".to_string(),
                lan: "d7d5".to_string(),
                before: "rnbqkbnr/pppp1ppp/4p3/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 1 2"
                    .to_string(),
                after: "rnbqkbnr/ppp2ppp/4p3/3p4/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 3"
                    .to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: true,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::D2,

                to: SquareStr::D4,

                san: "d4".to_string(),
                lan: "d2d4".to_string(),
                before: "rnbqkbnr/ppp2ppp/4p3/3p4/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 3"
                    .to_string(),
                after: "rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 3"
                    .to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: true,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::N,
                from: SquareStr::G8,

                to: SquareStr::F6,

                san: "Nf6".to_string(),
                lan: "g8f6".to_string(),
                before: "rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 3"
                    .to_string(),
                after: "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 1 4"
                    .to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::N,
                from: SquareStr::B1,

                to: SquareStr::C3,

                san: "Nc3".to_string(),
                lan: "b1c3".to_string(),
                before: "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 1 4"
                    .to_string(),
                after: "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 2 4"
                    .to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::B,
                from: SquareStr::F8,

                to: SquareStr::E7,

                san: "Be7".to_string(),
                lan: "f8e7".to_string(),
                before: "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 2 4"
                    .to_string(),
                after: "rnbqk2r/ppp1bppp/4pn2/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R w KQkq - 3 5"
                    .to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::B,
                from: SquareStr::C1,

                to: SquareStr::G5,

                san: "Bg5".to_string(),
                lan: "c1g5".to_string(),
                before: "rnbqk2r/ppp1bppp/4pn2/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R w KQkq - 3 5"
                    .to_string(),
                after: "rnbqk2r/ppp1bppp/4pn2/3p2B1/2PP4/2N2N2/PP2PPPP/R2QKB1R b KQkq - 4 5"
                    .to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::K,
                from: SquareStr::E8,

                to: SquareStr::H8,

                san: "O-O".to_string(),
                // TODO: fix Chess960 discrepancy
                lan: "e8h8".to_string(),
                before: "rnbqk2r/ppp1bppp/4pn2/3p2B1/2PP4/2N2N2/PP2PPPP/R2QKB1R b KQkq - 4 5"
                    .to_string(),
                after: "rnbq1rk1/ppp1bppp/4pn2/3p2B1/2PP4/2N2N2/PP2PPPP/R2QKB1R w KQ - 5 6"
                    .to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: true,
                is_kingside_castle: true,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::E2,

                to: SquareStr::E3,

                san: "e3".to_string(),
                lan: "e2e3".to_string(),
                before: "rnbq1rk1/ppp1bppp/4pn2/3p2B1/2PP4/2N2N2/PP2PPPP/R2QKB1R w KQ - 5 6"
                    .to_string(),
                after: "rnbq1rk1/ppp1bppp/4pn2/3p2B1/2PP4/2N1PN2/PP3PPP/R2QKB1R b KQ - 0 6"
                    .to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,
                from: SquareStr::H7,

                to: SquareStr::H6,

                san: "h6".to_string(),
                lan: "h7h6".to_string(),
                before: "rnbq1rk1/ppp1bppp/4pn2/3p2B1/2PP4/2N1PN2/PP3PPP/R2QKB1R b KQ - 0 6"
                    .to_string(),
                after: "rnbq1rk1/ppp1bpp1/4pn1p/3p2B1/2PP4/2N1PN2/PP3PPP/R2QKB1R w KQ - 0 7"
                    .to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::B,
                from: SquareStr::G5,

                to: SquareStr::H4,

                san: "Bh4".to_string(),
                lan: "g5h4".to_string(),
                before: "rnbq1rk1/ppp1bpp1/4pn1p/3p2B1/2PP4/2N1PN2/PP3PPP/R2QKB1R w KQ - 0 7"
                    .to_string(),
                after: "rnbq1rk1/ppp1bpp1/4pn1p/3p4/2PP3B/2N1PN2/PP3PPP/R2QKB1R b KQ - 1 7"
                    .to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,
                from: SquareStr::B7,

                to: SquareStr::B6,

                san: "b6".to_string(),
                lan: "b7b6".to_string(),
                before: "rnbq1rk1/ppp1bpp1/4pn1p/3p4/2PP3B/2N1PN2/PP3PPP/R2QKB1R b KQ - 1 7"
                    .to_string(),
                after: "rnbq1rk1/p1p1bpp1/1p2pn1p/3p4/2PP3B/2N1PN2/PP3PPP/R2QKB1R w KQ - 0 8"
                    .to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::C4,

                to: SquareStr::D5,

                san: "cxd5".to_string(),
                lan: "c4d5".to_string(),
                before: "rnbq1rk1/p1p1bpp1/1p2pn1p/3p4/2PP3B/2N1PN2/PP3PPP/R2QKB1R w KQ - 0 8"
                    .to_string(),
                after: "rnbq1rk1/p1p1bpp1/1p2pn1p/3P4/3P3B/2N1PN2/PP3PPP/R2QKB1R b KQ - 0 8"
                    .to_string(),
                promotion: None,
                captured: Some(PieceSymbol::P),
                is_regular_capture: true,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::N,
                from: SquareStr::F6,
                to: SquareStr::D5,

                san: "Nxd5".to_string(),
                lan: "f6d5".to_string(),
                before: "rnbq1rk1/p1p1bpp1/1p2pn1p/3P4/3P3B/2N1PN2/PP3PPP/R2QKB1R b KQ - 0 8"
                    .to_string(),
                after: "rnbq1rk1/p1p1bpp1/1p2p2p/3n4/3P3B/2N1PN2/PP3PPP/R2QKB1R w KQ - 0 9"
                    .to_string(),
                promotion: None,
                captured: Some(PieceSymbol::P),
                is_regular_capture: true,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::B,
                from: SquareStr::H4,
                to: SquareStr::E7,

                san: "Bxe7".to_string(),
                lan: "h4e7".to_string(),
                before: "rnbq1rk1/p1p1bpp1/1p2p2p/3n4/3P3B/2N1PN2/PP3PPP/R2QKB1R w KQ - 0 9"
                    .to_string(),
                after: "rnbq1rk1/p1p1Bpp1/1p2p2p/3n4/3P4/2N1PN2/PP3PPP/R2QKB1R b KQ - 0 9"
                    .to_string(),
                promotion: None,
                captured: Some(PieceSymbol::B),
                is_regular_capture: true,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::Q,
                from: SquareStr::D8,
                to: SquareStr::E7,

                san: "Qxe7".to_string(),
                lan: "d8e7".to_string(),
                before: "rnbq1rk1/p1p1Bpp1/1p2p2p/3n4/3P4/2N1PN2/PP3PPP/R2QKB1R b KQ - 0 9"
                    .to_string(),
                after: "rnb2rk1/p1p1qpp1/1p2p2p/3n4/3P4/2N1PN2/PP3PPP/R2QKB1R w KQ - 0 10"
                    .to_string(),
                promotion: None,
                captured: Some(PieceSymbol::B),
                is_regular_capture: true,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::N,
                from: SquareStr::C3,

                to: SquareStr::D5,

                san: "Nxd5".to_string(),
                lan: "c3d5".to_string(),
                before: "rnb2rk1/p1p1qpp1/1p2p2p/3n4/3P4/2N1PN2/PP3PPP/R2QKB1R w KQ - 0 10"
                    .to_string(),
                after: "rnb2rk1/p1p1qpp1/1p2p2p/3N4/3P4/4PN2/PP3PPP/R2QKB1R b KQ - 0 10"
                    .to_string(),
                promotion: None,
                captured: Some(PieceSymbol::N),
                is_regular_capture: true,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,
                from: SquareStr::E6,
                to: SquareStr::D5,

                san: "exd5".to_string(),
                lan: "e6d5".to_string(),
                before: "rnb2rk1/p1p1qpp1/1p2p2p/3N4/3P4/4PN2/PP3PPP/R2QKB1R b KQ - 0 10"
                    .to_string(),
                after: "rnb2rk1/p1p1qpp1/1p5p/3p4/3P4/4PN2/PP3PPP/R2QKB1R w KQ - 0 11".to_string(),
                promotion: None,
                captured: Some(PieceSymbol::N),
                is_regular_capture: true,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::R,
                from: SquareStr::A1,

                to: SquareStr::C1,

                san: "Rc1".to_string(),
                lan: "a1c1".to_string(),
                before: "rnb2rk1/p1p1qpp1/1p5p/3p4/3P4/4PN2/PP3PPP/R2QKB1R w KQ - 0 11".to_string(),
                after: "rnb2rk1/p1p1qpp1/1p5p/3p4/3P4/4PN2/PP3PPP/2RQKB1R b K - 1 11".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::B,
                from: SquareStr::C8,

                to: SquareStr::E6,

                san: "Be6".to_string(),
                lan: "c8e6".to_string(),
                before: "rnb2rk1/p1p1qpp1/1p5p/3p4/3P4/4PN2/PP3PPP/2RQKB1R b K - 1 11".to_string(),
                after: "rn3rk1/p1p1qpp1/1p2b2p/3p4/3P4/4PN2/PP3PPP/2RQKB1R w K - 2 12".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::Q,
                from: SquareStr::D1,

                to: SquareStr::A4,

                san: "Qa4".to_string(),
                lan: "d1a4".to_string(),
                before: "rn3rk1/p1p1qpp1/1p2b2p/3p4/3P4/4PN2/PP3PPP/2RQKB1R w K - 2 12".to_string(),
                after: "rn3rk1/p1p1qpp1/1p2b2p/3p4/Q2P4/4PN2/PP3PPP/2R1KB1R b K - 3 12".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,
                from: SquareStr::C7,

                to: SquareStr::C5,

                san: "c5".to_string(),
                lan: "c7c5".to_string(),
                before: "rn3rk1/p1p1qpp1/1p2b2p/3p4/Q2P4/4PN2/PP3PPP/2R1KB1R b K - 3 12"
                    .to_string(),
                after: "rn3rk1/p3qpp1/1p2b2p/2pp4/Q2P4/4PN2/PP3PPP/2R1KB1R w K - 0 13".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: true,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::Q,
                from: SquareStr::A4,

                to: SquareStr::A3,

                san: "Qa3".to_string(),
                lan: "a4a3".to_string(),
                before: "rn3rk1/p3qpp1/1p2b2p/2pp4/Q2P4/4PN2/PP3PPP/2R1KB1R w K - 0 13".to_string(),
                after: "rn3rk1/p3qpp1/1p2b2p/2pp4/3P4/Q3PN2/PP3PPP/2R1KB1R b K - 1 13".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::R,
                from: SquareStr::F8,

                to: SquareStr::C8,

                san: "Rc8".to_string(),
                lan: "f8c8".to_string(),
                before: "rn3rk1/p3qpp1/1p2b2p/2pp4/3P4/Q3PN2/PP3PPP/2R1KB1R b K - 1 13".to_string(),
                after: "rnr3k1/p3qpp1/1p2b2p/2pp4/3P4/Q3PN2/PP3PPP/2R1KB1R w K - 2 14".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::B,
                from: SquareStr::F1,

                to: SquareStr::B5,

                san: "Bb5".to_string(),
                lan: "f1b5".to_string(),
                before: "rnr3k1/p3qpp1/1p2b2p/2pp4/3P4/Q3PN2/PP3PPP/2R1KB1R w K - 2 14".to_string(),
                after: "rnr3k1/p3qpp1/1p2b2p/1Bpp4/3P4/Q3PN2/PP3PPP/2R1K2R b K - 3 14".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,
                from: SquareStr::A7,

                to: SquareStr::A6,

                san: "a6".to_string(),
                lan: "a7a6".to_string(),
                before: "rnr3k1/p3qpp1/1p2b2p/1Bpp4/3P4/Q3PN2/PP3PPP/2R1K2R b K - 3 14".to_string(),
                after: "rnr3k1/4qpp1/pp2b2p/1Bpp4/3P4/Q3PN2/PP3PPP/2R1K2R w K - 0 15".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::D4,
                to: SquareStr::C5,

                san: "dxc5".to_string(),
                lan: "d4c5".to_string(),
                before: "rnr3k1/4qpp1/pp2b2p/1Bpp4/3P4/Q3PN2/PP3PPP/2R1K2R w K - 0 15".to_string(),
                after: "rnr3k1/4qpp1/pp2b2p/1BPp4/8/Q3PN2/PP3PPP/2R1K2R b K - 0 15".to_string(),
                promotion: None,
                captured: Some(PieceSymbol::P),
                is_regular_capture: true,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,

                from: SquareStr::B6,
                to: SquareStr::C5,

                san: "bxc5".to_string(),
                lan: "b6c5".to_string(),
                before: "rnr3k1/4qpp1/pp2b2p/1BPp4/8/Q3PN2/PP3PPP/2R1K2R b K - 0 15".to_string(),
                after: "rnr3k1/4qpp1/p3b2p/1Bpp4/8/Q3PN2/PP3PPP/2R1K2R w K - 0 16".to_string(),

                promotion: None,
                captured: Some(PieceSymbol::P),
                is_regular_capture: true,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::K,
                from: SquareStr::E1,

                to: SquareStr::H1,

                san: "O-O".to_string(),
                // NOTE:
                // discrepancy in produced LAN move
                // because we use Chess960 castling format by default
                // TODO: fix?
                lan: "e1h1".to_string(),
                before: "rnr3k1/4qpp1/p3b2p/1Bpp4/8/Q3PN2/PP3PPP/2R1K2R w K - 0 16".to_string(),
                after: "rnr3k1/4qpp1/p3b2p/1Bpp4/8/Q3PN2/PP3PPP/2R2RK1 b - - 1 16".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: true,
                is_kingside_castle: true,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::R,
                from: SquareStr::A8,

                to: SquareStr::A7,

                san: "Ra7".to_string(),
                lan: "a8a7".to_string(),
                before: "rnr3k1/4qpp1/p3b2p/1Bpp4/8/Q3PN2/PP3PPP/2R2RK1 b - - 1 16".to_string(),
                after: "1nr3k1/r3qpp1/p3b2p/1Bpp4/8/Q3PN2/PP3PPP/2R2RK1 w - - 2 17".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::B,
                from: SquareStr::B5,

                to: SquareStr::E2,

                san: "Be2".to_string(),
                lan: "b5e2".to_string(),
                before: "1nr3k1/r3qpp1/p3b2p/1Bpp4/8/Q3PN2/PP3PPP/2R2RK1 w - - 2 17".to_string(),
                after: "1nr3k1/r3qpp1/p3b2p/2pp4/8/Q3PN2/PP2BPPP/2R2RK1 b - - 3 17".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::N,
                from: SquareStr::B8,

                to: SquareStr::D7,

                san: "Nd7".to_string(),
                lan: "b8d7".to_string(),
                before: "1nr3k1/r3qpp1/p3b2p/2pp4/8/Q3PN2/PP2BPPP/2R2RK1 b - - 3 17".to_string(),
                after: "2r3k1/r2nqpp1/p3b2p/2pp4/8/Q3PN2/PP2BPPP/2R2RK1 w - - 4 18".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::N,
                from: SquareStr::F3,

                to: SquareStr::D4,

                san: "Nd4".to_string(),
                lan: "f3d4".to_string(),
                before: "2r3k1/r2nqpp1/p3b2p/2pp4/8/Q3PN2/PP2BPPP/2R2RK1 w - - 4 18".to_string(),
                after: "2r3k1/r2nqpp1/p3b2p/2pp4/3N4/Q3P3/PP2BPPP/2R2RK1 b - - 5 18".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::Q,
                from: SquareStr::E7,

                to: SquareStr::F8,

                san: "Qf8".to_string(),
                lan: "e7f8".to_string(),
                before: "2r3k1/r2nqpp1/p3b2p/2pp4/3N4/Q3P3/PP2BPPP/2R2RK1 b - - 5 18".to_string(),
                after: "2r2qk1/r2n1pp1/p3b2p/2pp4/3N4/Q3P3/PP2BPPP/2R2RK1 w - - 6 19".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::N,
                from: SquareStr::D4,
                to: SquareStr::E6,

                san: "Nxe6".to_string(),
                lan: "d4e6".to_string(),
                before: "2r2qk1/r2n1pp1/p3b2p/2pp4/3N4/Q3P3/PP2BPPP/2R2RK1 w - - 6 19".to_string(),
                after: "2r2qk1/r2n1pp1/p3N2p/2pp4/8/Q3P3/PP2BPPP/2R2RK1 b - - 0 19".to_string(),
                promotion: None,
                captured: Some(PieceSymbol::B),
                is_regular_capture: true,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,
                from: SquareStr::F7,
                to: SquareStr::E6,

                san: "fxe6".to_string(),
                lan: "f7e6".to_string(),
                before: "2r2qk1/r2n1pp1/p3N2p/2pp4/8/Q3P3/PP2BPPP/2R2RK1 b - - 0 19".to_string(),
                after: "2r2qk1/r2n2p1/p3p2p/2pp4/8/Q3P3/PP2BPPP/2R2RK1 w - - 0 20".to_string(),
                promotion: None,
                captured: Some(PieceSymbol::N),
                is_regular_capture: true,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::E3,
                to: SquareStr::E4,

                san: "e4".to_string(),
                lan: "e3e4".to_string(),
                before: "2r2qk1/r2n2p1/p3p2p/2pp4/8/Q3P3/PP2BPPP/2R2RK1 w - - 0 20".to_string(),
                after: "2r2qk1/r2n2p1/p3p2p/2pp4/4P3/Q7/PP2BPPP/2R2RK1 b - - 0 20".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,
                from: SquareStr::D5,
                to: SquareStr::D4,

                san: "d4".to_string(),
                lan: "d5d4".to_string(),
                before: "2r2qk1/r2n2p1/p3p2p/2pp4/4P3/Q7/PP2BPPP/2R2RK1 b - - 0 20".to_string(),
                after: "2r2qk1/r2n2p1/p3p2p/2p5/3pP3/Q7/PP2BPPP/2R2RK1 w - - 0 21".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_castle: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::F2,
                to: SquareStr::F4,

                san: "f4".to_string(),
                lan: "f2f4".to_string(),
                before: "2r2qk1/r2n2p1/p3p2p/2p5/3pP3/Q7/PP2BPPP/2R2RK1 w - - 0 21".to_string(),
                after: "2r2qk1/r2n2p1/p3p2p/2p5/3pPP2/Q7/PP2B1PP/2R2RK1 b - - 0 21".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: true,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::Q,
                from: SquareStr::F8,
                to: SquareStr::E7,

                san: "Qe7".to_string(),
                lan: "f8e7".to_string(),
                before: "2r2qk1/r2n2p1/p3p2p/2p5/3pPP2/Q7/PP2B1PP/2R2RK1 b - - 0 21".to_string(),
                after: "2r3k1/r2nq1p1/p3p2p/2p5/3pPP2/Q7/PP2B1PP/2R2RK1 w - - 1 22".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::E4,
                to: SquareStr::E5,

                san: "e5".to_string(),
                lan: "e4e5".to_string(),
                before: "2r3k1/r2nq1p1/p3p2p/2p5/3pPP2/Q7/PP2B1PP/2R2RK1 w - - 1 22".to_string(),
                after: "2r3k1/r2nq1p1/p3p2p/2p1P3/3p1P2/Q7/PP2B1PP/2R2RK1 b - - 0 22".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::R,
                from: SquareStr::C8,
                to: SquareStr::B8,

                san: "Rb8".to_string(),
                lan: "c8b8".to_string(),
                before: "2r3k1/r2nq1p1/p3p2p/2p1P3/3p1P2/Q7/PP2B1PP/2R2RK1 b - - 0 22".to_string(),
                after: "1r4k1/r2nq1p1/p3p2p/2p1P3/3p1P2/Q7/PP2B1PP/2R2RK1 w - - 1 23".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::B,
                from: SquareStr::E2,
                to: SquareStr::C4,

                san: "Bc4".to_string(),
                lan: "e2c4".to_string(),
                before: "1r4k1/r2nq1p1/p3p2p/2p1P3/3p1P2/Q7/PP2B1PP/2R2RK1 w - - 1 23".to_string(),
                after: "1r4k1/r2nq1p1/p3p2p/2p1P3/2Bp1P2/Q7/PP4PP/2R2RK1 b - - 2 23".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::K,
                from: SquareStr::G8,
                to: SquareStr::H8,

                san: "Kh8".to_string(),
                lan: "g8h8".to_string(),
                before: "1r4k1/r2nq1p1/p3p2p/2p1P3/2Bp1P2/Q7/PP4PP/2R2RK1 b - - 2 23".to_string(),
                after: "1r5k/r2nq1p1/p3p2p/2p1P3/2Bp1P2/Q7/PP4PP/2R2RK1 w - - 3 24".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::Q,
                from: SquareStr::A3,
                to: SquareStr::H3,

                san: "Qh3".to_string(),
                lan: "a3h3".to_string(),
                before: "1r5k/r2nq1p1/p3p2p/2p1P3/2Bp1P2/Q7/PP4PP/2R2RK1 w - - 3 24".to_string(),
                after: "1r5k/r2nq1p1/p3p2p/2p1P3/2Bp1P2/7Q/PP4PP/2R2RK1 b - - 4 24".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::N,
                from: SquareStr::D7,
                to: SquareStr::F8,

                san: "Nf8".to_string(),
                lan: "d7f8".to_string(),
                before: "1r5k/r2nq1p1/p3p2p/2p1P3/2Bp1P2/7Q/PP4PP/2R2RK1 b - - 4 24".to_string(),
                after: "1r3n1k/r3q1p1/p3p2p/2p1P3/2Bp1P2/7Q/PP4PP/2R2RK1 w - - 5 25".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::B2,

                to: SquareStr::B3,

                san: "b3".to_string(),
                lan: "b2b3".to_string(),
                before: "1r3n1k/r3q1p1/p3p2p/2p1P3/2Bp1P2/7Q/PP4PP/2R2RK1 w - - 5 25".to_string(),
                after: "1r3n1k/r3q1p1/p3p2p/2p1P3/2Bp1P2/1P5Q/P5PP/2R2RK1 b - - 0 25".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,
                from: SquareStr::A6,
                to: SquareStr::A5,

                san: "a5".to_string(),
                lan: "a6a5".to_string(),
                before: "1r3n1k/r3q1p1/p3p2p/2p1P3/2Bp1P2/1P5Q/P5PP/2R2RK1 b - - 0 25".to_string(),
                after: "1r3n1k/r3q1p1/4p2p/p1p1P3/2Bp1P2/1P5Q/P5PP/2R2RK1 w - - 0 26".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::F4,
                to: SquareStr::F5,

                san: "f5".to_string(),
                lan: "f4f5".to_string(),
                before: "1r3n1k/r3q1p1/4p2p/p1p1P3/2Bp1P2/1P5Q/P5PP/2R2RK1 w - - 0 26".to_string(),
                after: "1r3n1k/r3q1p1/4p2p/p1p1PP2/2Bp4/1P5Q/P5PP/2R2RK1 b - - 0 26".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,
                from: SquareStr::E6,
                to: SquareStr::F5,

                san: "exf5".to_string(),
                lan: "e6f5".to_string(),
                before: "1r3n1k/r3q1p1/4p2p/p1p1PP2/2Bp4/1P5Q/P5PP/2R2RK1 b - - 0 26".to_string(),
                after: "1r3n1k/r3q1p1/7p/p1p1Pp2/2Bp4/1P5Q/P5PP/2R2RK1 w - - 0 27".to_string(),
                promotion: None,
                captured: Some(PieceSymbol::P),
                is_regular_capture: true,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::R,
                from: SquareStr::F1,
                to: SquareStr::F5,

                san: "Rxf5".to_string(),
                lan: "f1f5".to_string(),
                before: "1r3n1k/r3q1p1/7p/p1p1Pp2/2Bp4/1P5Q/P5PP/2R2RK1 w - - 0 27".to_string(),
                after: "1r3n1k/r3q1p1/7p/p1p1PR2/2Bp4/1P5Q/P5PP/2R3K1 b - - 0 27".to_string(),
                promotion: None,
                captured: Some(PieceSymbol::P),
                is_regular_capture: true,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::N,
                from: SquareStr::F8,
                to: SquareStr::H7,

                san: "Nh7".to_string(),
                lan: "f8h7".to_string(),
                before: "1r3n1k/r3q1p1/7p/p1p1PR2/2Bp4/1P5Q/P5PP/2R3K1 b - - 0 27".to_string(),
                after: "1r5k/r3q1pn/7p/p1p1PR2/2Bp4/1P5Q/P5PP/2R3K1 w - - 1 28".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::R,
                from: SquareStr::C1,
                to: SquareStr::F1,

                san: "Rcf1".to_string(),
                lan: "c1f1".to_string(),
                before: "1r5k/r3q1pn/7p/p1p1PR2/2Bp4/1P5Q/P5PP/2R3K1 w - - 1 28".to_string(),
                after: "1r5k/r3q1pn/7p/p1p1PR2/2Bp4/1P5Q/P5PP/5RK1 b - - 2 28".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::Q,
                from: SquareStr::E7,
                to: SquareStr::D8,

                san: "Qd8".to_string(),
                lan: "e7d8".to_string(),
                before: "1r5k/r3q1pn/7p/p1p1PR2/2Bp4/1P5Q/P5PP/5RK1 b - - 2 28".to_string(),
                after: "1r1q3k/r5pn/7p/p1p1PR2/2Bp4/1P5Q/P5PP/5RK1 w - - 3 29".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,
                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::Q,
                from: SquareStr::H3,
                to: SquareStr::G3,

                san: "Qg3".to_string(),
                lan: "h3g3".to_string(),
                before: "1r1q3k/r5pn/7p/p1p1PR2/2Bp4/1P5Q/P5PP/5RK1 w - - 3 29".to_string(),
                after: "1r1q3k/r5pn/7p/p1p1PR2/2Bp4/1P4Q1/P5PP/5RK1 b - - 4 29".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::R,
                from: SquareStr::A7,
                to: SquareStr::E7,

                san: "Re7".to_string(),
                lan: "a7e7".to_string(),
                before: "1r1q3k/r5pn/7p/p1p1PR2/2Bp4/1P4Q1/P5PP/5RK1 b - - 4 29".to_string(),
                after: "1r1q3k/4r1pn/7p/p1p1PR2/2Bp4/1P4Q1/P5PP/5RK1 w - - 5 30".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::H2,
                to: SquareStr::H4,

                san: "h4".to_string(),
                lan: "h2h4".to_string(),
                before: "1r1q3k/4r1pn/7p/p1p1PR2/2Bp4/1P4Q1/P5PP/5RK1 w - - 5 30".to_string(),
                after: "1r1q3k/4r1pn/7p/p1p1PR2/2Bp3P/1P4Q1/P5P1/5RK1 b - - 0 30".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: true,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::R,
                from: SquareStr::B8,
                to: SquareStr::B7,

                san: "Rbb7".to_string(),
                lan: "b8b7".to_string(),
                before: "1r1q3k/4r1pn/7p/p1p1PR2/2Bp3P/1P4Q1/P5P1/5RK1 b - - 0 30".to_string(),
                after: "3q3k/1r2r1pn/7p/p1p1PR2/2Bp3P/1P4Q1/P5P1/5RK1 w - - 1 31".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::E5,
                to: SquareStr::E6,

                san: "e6".to_string(),
                lan: "e5e6".to_string(),
                before: "3q3k/1r2r1pn/7p/p1p1PR2/2Bp3P/1P4Q1/P5P1/5RK1 w - - 1 31".to_string(),
                after: "3q3k/1r2r1pn/4P2p/p1p2R2/2Bp3P/1P4Q1/P5P1/5RK1 b - - 0 31".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::R,
                from: SquareStr::B7,
                to: SquareStr::C7,

                san: "Rbc7".to_string(),
                lan: "b7c7".to_string(),
                before: "3q3k/1r2r1pn/4P2p/p1p2R2/2Bp3P/1P4Q1/P5P1/5RK1 b - - 0 31".to_string(),
                after: "3q3k/2r1r1pn/4P2p/p1p2R2/2Bp3P/1P4Q1/P5P1/5RK1 w - - 1 32".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::Q,
                from: SquareStr::G3,

                to: SquareStr::E5,

                san: "Qe5".to_string(),
                lan: "g3e5".to_string(),
                before: "3q3k/2r1r1pn/4P2p/p1p2R2/2Bp3P/1P4Q1/P5P1/5RK1 w - - 1 32".to_string(),
                after: "3q3k/2r1r1pn/4P2p/p1p1QR2/2Bp3P/1P6/P5P1/5RK1 b - - 2 32".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::Q,
                from: SquareStr::D8,
                to: SquareStr::E8,

                san: "Qe8".to_string(),
                lan: "d8e8".to_string(),
                before: "3q3k/2r1r1pn/4P2p/p1p1QR2/2Bp3P/1P6/P5P1/5RK1 b - - 2 32".to_string(),
                after: "4q2k/2r1r1pn/4P2p/p1p1QR2/2Bp3P/1P6/P5P1/5RK1 w - - 3 33".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::A2,
                to: SquareStr::A4,

                san: "a4".to_string(),
                lan: "a2a4".to_string(),
                before: "4q2k/2r1r1pn/4P2p/p1p1QR2/2Bp3P/1P6/P5P1/5RK1 w - - 3 33".to_string(),
                after: "4q2k/2r1r1pn/4P2p/p1p1QR2/P1Bp3P/1P6/6P1/5RK1 b - - 0 33".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: true,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::Q,
                from: SquareStr::E8,
                to: SquareStr::D8,

                san: "Qd8".to_string(),
                lan: "e8d8".to_string(),
                before: "4q2k/2r1r1pn/4P2p/p1p1QR2/P1Bp3P/1P6/6P1/5RK1 b - - 0 33".to_string(),
                after: "3q3k/2r1r1pn/4P2p/p1p1QR2/P1Bp3P/1P6/6P1/5RK1 w - - 1 34".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::R,
                from: SquareStr::F1,
                to: SquareStr::F2,

                san: "R1f2".to_string(),
                lan: "f1f2".to_string(),
                before: "3q3k/2r1r1pn/4P2p/p1p1QR2/P1Bp3P/1P6/6P1/5RK1 w - - 1 34".to_string(),
                after: "3q3k/2r1r1pn/4P2p/p1p1QR2/P1Bp3P/1P6/5RP1/6K1 b - - 2 34".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::Q,
                from: SquareStr::D8,
                to: SquareStr::E8,

                san: "Qe8".to_string(),
                lan: "d8e8".to_string(),
                before: "3q3k/2r1r1pn/4P2p/p1p1QR2/P1Bp3P/1P6/5RP1/6K1 b - - 2 34".to_string(),
                after: "4q2k/2r1r1pn/4P2p/p1p1QR2/P1Bp3P/1P6/5RP1/6K1 w - - 3 35".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::R,
                from: SquareStr::F2,

                to: SquareStr::F3,

                san: "R2f3".to_string(),
                lan: "f2f3".to_string(),
                before: "4q2k/2r1r1pn/4P2p/p1p1QR2/P1Bp3P/1P6/5RP1/6K1 w - - 3 35".to_string(),
                after: "4q2k/2r1r1pn/4P2p/p1p1QR2/P1Bp3P/1P3R2/6P1/6K1 b - - 4 35".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::Q,
                from: SquareStr::E8,

                to: SquareStr::D8,

                san: "Qd8".to_string(),
                lan: "e8d8".to_string(),
                before: "4q2k/2r1r1pn/4P2p/p1p1QR2/P1Bp3P/1P3R2/6P1/6K1 b - - 4 35".to_string(),
                after: "3q3k/2r1r1pn/4P2p/p1p1QR2/P1Bp3P/1P3R2/6P1/6K1 w - - 5 36".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::B,
                from: SquareStr::C4,

                to: SquareStr::D3,

                san: "Bd3".to_string(),
                lan: "c4d3".to_string(),
                before: "3q3k/2r1r1pn/4P2p/p1p1QR2/P1Bp3P/1P3R2/6P1/6K1 w - - 5 36".to_string(),
                after: "3q3k/2r1r1pn/4P2p/p1p1QR2/P2p3P/1P1B1R2/6P1/6K1 b - - 6 36".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::Q,
                from: SquareStr::D8,
                to: SquareStr::E8,

                san: "Qe8".to_string(),
                lan: "d8e8".to_string(),
                before: "3q3k/2r1r1pn/4P2p/p1p1QR2/P2p3P/1P1B1R2/6P1/6K1 b - - 6 36".to_string(),
                after: "4q2k/2r1r1pn/4P2p/p1p1QR2/P2p3P/1P1B1R2/6P1/6K1 w - - 7 37".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::Q,
                from: SquareStr::E5,
                to: SquareStr::E4,

                san: "Qe4".to_string(),
                lan: "e5e4".to_string(),
                before: "4q2k/2r1r1pn/4P2p/p1p1QR2/P2p3P/1P1B1R2/6P1/6K1 w - - 7 37".to_string(),
                after: "4q2k/2r1r1pn/4P2p/p1p2R2/P2pQ2P/1P1B1R2/6P1/6K1 b - - 8 37".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::N,
                from: SquareStr::H7,
                to: SquareStr::F6,

                san: "Nf6".to_string(),
                lan: "h7f6".to_string(),
                before: "4q2k/2r1r1pn/4P2p/p1p2R2/P2pQ2P/1P1B1R2/6P1/6K1 b - - 8 37".to_string(),
                after: "4q2k/2r1r1p1/4Pn1p/p1p2R2/P2pQ2P/1P1B1R2/6P1/6K1 w - - 9 38".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::R,
                from: SquareStr::F5,
                to: SquareStr::F6,

                san: "Rxf6".to_string(),
                lan: "f5f6".to_string(),
                before: "4q2k/2r1r1p1/4Pn1p/p1p2R2/P2pQ2P/1P1B1R2/6P1/6K1 w - - 9 38".to_string(),
                after: "4q2k/2r1r1p1/4PR1p/p1p5/P2pQ2P/1P1B1R2/6P1/6K1 b - - 0 38".to_string(),
                promotion: None,
                captured: Some(PieceSymbol::N),
                is_regular_capture: true,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,
                from: SquareStr::G7,
                to: SquareStr::F6,

                san: "gxf6".to_string(),
                lan: "g7f6".to_string(),
                before: "4q2k/2r1r1p1/4PR1p/p1p5/P2pQ2P/1P1B1R2/6P1/6K1 b - - 0 38".to_string(),
                after: "4q2k/2r1r3/4Pp1p/p1p5/P2pQ2P/1P1B1R2/6P1/6K1 w - - 0 39".to_string(),
                promotion: None,
                captured: Some(PieceSymbol::R),
                is_regular_capture: true,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::R,
                from: SquareStr::F3,
                to: SquareStr::F6,

                san: "Rxf6".to_string(),
                lan: "f3f6".to_string(),
                before: "4q2k/2r1r3/4Pp1p/p1p5/P2pQ2P/1P1B1R2/6P1/6K1 w - - 0 39".to_string(),
                after: "4q2k/2r1r3/4PR1p/p1p5/P2pQ2P/1P1B4/6P1/6K1 b - - 0 39".to_string(),
                promotion: None,
                captured: Some(PieceSymbol::P),
                is_regular_capture: true,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::K,
                from: SquareStr::H8,
                to: SquareStr::G8,

                san: "Kg8".to_string(),
                lan: "h8g8".to_string(),
                before: "4q2k/2r1r3/4PR1p/p1p5/P2pQ2P/1P1B4/6P1/6K1 b - - 0 39".to_string(),
                after: "4q1k1/2r1r3/4PR1p/p1p5/P2pQ2P/1P1B4/6P1/6K1 w - - 1 40".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::B,
                from: SquareStr::D3,

                to: SquareStr::C4,

                san: "Bc4".to_string(),
                lan: "d3c4".to_string(),
                before: "4q1k1/2r1r3/4PR1p/p1p5/P2pQ2P/1P1B4/6P1/6K1 w - - 1 40".to_string(),
                after: "4q1k1/2r1r3/4PR1p/p1p5/P1BpQ2P/1P6/6P1/6K1 b - - 2 40".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::K,
                from: SquareStr::G8,

                to: SquareStr::H8,

                san: "Kh8".to_string(),
                lan: "g8h8".to_string(),
                before: "4q1k1/2r1r3/4PR1p/p1p5/P1BpQ2P/1P6/6P1/6K1 b - - 2 40".to_string(),
                after: "4q2k/2r1r3/4PR1p/p1p5/P1BpQ2P/1P6/6P1/6K1 w - - 3 41".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::Q,
                from: SquareStr::E4,

                to: SquareStr::F4,

                san: "Qf4".to_string(),
                lan: "e4f4".to_string(),
                before: "4q2k/2r1r3/4PR1p/p1p5/P1BpQ2P/1P6/6P1/6K1 w - - 3 41".to_string(),
                after: "4q2k/2r1r3/4PR1p/p1p5/P1Bp1Q1P/1P6/6P1/6K1 b - - 4 41".to_string(),
                promotion: None,
                captured: None,
                is_regular_capture: false,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
        ];

        let history = chess.history_verbose();

        let history_san = chess.history_san();
        let history_lan = chess.history_uci();

        pretty_assertions::assert_eq!(moves.len(), history.len());
        pretty_assertions::assert_eq!(answer.len(), history.len());

        pretty_assertions::assert_eq!(answer, history);

        answer.iter().enumerate().for_each(|(i, el)| {
            pretty_assertions::assert_eq!(history_san[i], el.san);
            pretty_assertions::assert_eq!(history_lan[i], el.lan);
        });
    }

    #[test]
    fn history_pgn_setup() {
        let pgn = r#"[SetUp "1"]
    [FEN "r1bqk1nr/pppp1ppp/2n5/4p3/1bB1P3/2P2N2/P2P1PPP/RNBQK2R b KQkq - 0 1"]

    1. ... Ba5 2. O-O d6
    3. d4"#;

        let mut chess = WasmChess::new(None).unwrap();

        chess.load_pgn(pgn).unwrap();

        let moves = vec![
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::B,
                from: SquareStr::B4,
                to: SquareStr::A5,
                lan: "b4a5".to_string(),
                san: "Ba5".to_string(),
                before: "r1bqk1nr/pppp1ppp/2n5/4p3/1bB1P3/2P2N2/P2P1PPP/RNBQK2R b KQkq - 0 1"
                    .to_string(),
                after: "r1bqk1nr/pppp1ppp/2n5/b3p3/2B1P3/2P2N2/P2P1PPP/RNBQK2R w KQkq - 1 2"
                    .to_string(),
                captured: None,
                is_regular_capture: false,
                promotion: None,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::K,
                from: SquareStr::E1,
                to: SquareStr::H1,
                lan: "e1h1".to_string(),
                san: "O-O".to_string(),
                before: "r1bqk1nr/pppp1ppp/2n5/b3p3/2B1P3/2P2N2/P2P1PPP/RNBQK2R w KQkq - 1 2"
                    .to_string(),
                after: "r1bqk1nr/pppp1ppp/2n5/b3p3/2B1P3/2P2N2/P2P1PPP/RNBQ1RK1 b kq - 2 2"
                    .to_string(),
                captured: None,
                is_regular_capture: false,
                promotion: None,

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: true,
                is_kingside_castle: true,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::B,
                piece: PieceSymbol::P,
                from: SquareStr::D7,
                to: SquareStr::D6,
                lan: "d7d6".to_string(),
                san: "d6".to_string(),
                before: "r1bqk1nr/pppp1ppp/2n5/b3p3/2B1P3/2P2N2/P2P1PPP/RNBQ1RK1 b kq - 2 2"
                    .to_string(),
                after: "r1bqk1nr/ppp2ppp/2np4/b3p3/2B1P3/2P2N2/P2P1PPP/RNBQ1RK1 w kq - 0 3"
                    .to_string(),
                captured: None,
                is_regular_capture: false,
                promotion: None,

                is_en_passant: false,
                is_castle: false,
                is_big_pawn: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                color: ColorChar::W,
                piece: PieceSymbol::P,
                from: SquareStr::D2,
                to: SquareStr::D4,
                lan: "d2d4".to_string(),
                san: "d4".to_string(),
                before: "r1bqk1nr/ppp2ppp/2np4/b3p3/2B1P3/2P2N2/P2P1PPP/RNBQ1RK1 w kq - 0 3"
                    .to_string(),
                after: "r1bqk1nr/ppp2ppp/2np4/b3p3/2BPP3/2P2N2/P4PPP/RNBQ1RK1 b kq - 0 3"
                    .to_string(),
                captured: None,
                is_regular_capture: false,
                promotion: None,

                is_en_passant: false,
                is_big_pawn: true,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
        ];

        pretty_assertions::assert_eq!(moves, chess.history_verbose())
    }

    #[test]
    fn history_with_queenside_castle() {
        let pgn = r#"
        [Variant "From Position"]
[FEN "bbnnrkrq/pppppppp/8/8/8/8/PPPPPPPP/RQBKRBNN w KQkq - 0 1"]

1. e4 e5 2. Nf3 Nd6 3. Bc4 Nc6 4. O-O O-O-O"#;

        let mut chess = WasmChess::new(None).unwrap();
        chess.load_pgn(pgn).unwrap();

        let answer = vec![
            MoveVerbose {
                from: SquareStr::E2,
                to: SquareStr::E4,
                before: "bbnnrkrq/pppppppp/8/8/8/8/PPPPPPPP/RQBKRBNN w KQkq - 0 1".to_string(),
                after: "bbnnrkrq/pppppppp/8/8/4P3/8/PPPP1PPP/RQBKRBNN b KQkq - 0 1".to_string(),
                color: ColorChar::W,
                piece: PieceSymbol::P,
                captured: None,
                is_regular_capture: false,
                promotion: None,
                san: "e4".to_string(),
                lan: "e2e4".to_string(),
                is_en_passant: false,
                is_big_pawn: true,
                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                from: SquareStr::E7,
                to: SquareStr::E5,
                before: "bbnnrkrq/pppppppp/8/8/4P3/8/PPPP1PPP/RQBKRBNN b KQkq - 0 1".to_string(),
                after: "bbnnrkrq/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RQBKRBNN w KQkq - 0 2".to_string(),
                color: ColorChar::B,
                piece: PieceSymbol::P,
                captured: None,
                is_regular_capture: false,
                promotion: None,
                san: "e5".to_string(),
                lan: "e7e5".to_string(),

                is_en_passant: false,
                is_big_pawn: true,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                from: SquareStr::G1,
                to: SquareStr::F3,
                before: "bbnnrkrq/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RQBKRBNN w KQkq - 0 2".to_string(),
                after: "bbnnrkrq/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RQBKRB1N b KQkq - 1 2".to_string(),
                color: ColorChar::W,
                piece: PieceSymbol::N,
                captured: None,
                is_regular_capture: false,
                promotion: None,
                san: "Nf3".to_string(),
                lan: "g1f3".to_string(),

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                from: SquareStr::C8,
                to: SquareStr::D6,
                before: "bbnnrkrq/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RQBKRB1N b KQkq - 1 2"
                    .to_string(),
                after: "bb1nrkrq/pppp1ppp/3n4/4p3/4P3/5N2/PPPP1PPP/RQBKRB1N w KQkq - 2 3"
                    .to_string(),
                color: ColorChar::B,
                piece: PieceSymbol::N,
                captured: None,
                is_regular_capture: false,
                promotion: None,
                san: "Nd6".to_string(),
                lan: "c8d6".to_string(),

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                from: SquareStr::F1,
                to: SquareStr::C4,
                before: "bb1nrkrq/pppp1ppp/3n4/4p3/4P3/5N2/PPPP1PPP/RQBKRB1N w KQkq - 2 3"
                    .to_string(),
                after: "bb1nrkrq/pppp1ppp/3n4/4p3/2B1P3/5N2/PPPP1PPP/RQBKR2N b KQkq - 3 3"
                    .to_string(),
                color: ColorChar::W,
                piece: PieceSymbol::B,
                captured: None,
                is_regular_capture: false,
                promotion: None,
                san: "Bc4".to_string(),
                lan: "f1c4".to_string(),

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                from: SquareStr::D8,
                to: SquareStr::C6,
                before: "bb1nrkrq/pppp1ppp/3n4/4p3/2B1P3/5N2/PPPP1PPP/RQBKR2N b KQkq - 3 3"
                    .to_string(),
                after: "bb2rkrq/pppp1ppp/2nn4/4p3/2B1P3/5N2/PPPP1PPP/RQBKR2N w KQkq - 4 4"
                    .to_string(),
                color: ColorChar::B,
                piece: PieceSymbol::N,
                captured: None,
                is_regular_capture: false,
                promotion: None,
                san: "Nc6".to_string(),
                lan: "d8c6".to_string(),

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: false,
                is_kingside_castle: false,
                is_queenside_castle: false,
            },
            MoveVerbose {
                from: SquareStr::D1,
                to: SquareStr::E1,
                before: "bb2rkrq/pppp1ppp/2nn4/4p3/2B1P3/5N2/PPPP1PPP/RQBKR2N w KQkq - 4 4"
                    .to_string(),
                after: "bb2rkrq/pppp1ppp/2nn4/4p3/2B1P3/5N2/PPPP1PPP/RQB2RKN b kq - 5 4"
                    .to_string(),
                color: ColorChar::W,
                piece: PieceSymbol::K,
                captured: None,
                is_regular_capture: false,
                promotion: None,
                san: "O-O".to_string(),
                lan: "d1e1".to_string(),

                is_en_passant: false,
                is_big_pawn: false,

                is_castle: true,
                is_kingside_castle: true,
                is_queenside_castle: false,
            },
            MoveVerbose {
                from: SquareStr::F8,
                to: SquareStr::E8,
                before: "bb2rkrq/pppp1ppp/2nn4/4p3/2B1P3/5N2/PPPP1PPP/RQB2RKN b kq - 5 4"
                    .to_string(),
                after: "bbkr2rq/pppp1ppp/2nn4/4p3/2B1P3/5N2/PPPP1PPP/RQB2RKN w - - 6 5".to_string(),
                color: ColorChar::B,
                piece: PieceSymbol::K,
                captured: None,
                is_regular_capture: false,
                promotion: None,
                san: "O-O-O".to_string(),
                lan: "f8e8".to_string(),

                is_en_passant: false,
                is_big_pawn: false,
                is_castle: true,

                is_kingside_castle: false,
                is_queenside_castle: true,
            },
        ];

        pretty_assertions::assert_eq!(chess.history_verbose(), answer);
    }
}
