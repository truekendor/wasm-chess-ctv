/// these tests are taken from chess.js test suite for attackers() method
///
/// @link https://github.com/jhlywa/chess.js/blob/master/__tests__/attackers.test.ts

#[cfg(test)]
pub mod attackers_tests {
    use crate::{
        WasmChess,
        tsify_structs::{SquareStr, others::ColorChar},
    };

    fn aggregate_attacks(chess: &WasmChess, color: ColorChar) -> [u8; 64] {
        let mut attacks = [0; 64];

        for rank in (1..=8).rev() {
            for file in 'a'..='h' {
                let square_str = format!("{}{}", file, rank);
                let square = square_str.parse::<SquareStr>().unwrap();

                let file_idx = (file as u8 - b'a') as usize;

                // rank 8 -> 0
                // rank 1 -> 7
                let rank_idx = (8 - rank) as usize;

                let idx = rank_idx * 8 + file_idx;

                let attack_count = chess.attackers(square, Some(color.clone()));

                attacks[idx] = attack_count.len() as u8;
            }
        }

        attacks
    }

    #[test]
    fn attackers_ok_default_position() {
        let chess = WasmChess::new(None).unwrap();
        let result_w = aggregate_attacks(&chess, ColorChar::W);
        let result_b = aggregate_attacks(&chess, ColorChar::B);

        #[rustfmt::skip]
        let expected_white_attacker_count = vec![
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            2, 2, 3, 2, 2, 3, 2, 2,
            1, 1, 1, 4, 4, 1, 1, 1,
            0, 1, 1, 1, 1, 1, 1, 0,
        ];

        #[rustfmt::skip]
        let expected_black_attacker_count = vec![
            0, 1, 1, 1, 1, 1, 1, 0,
            1, 1, 1, 4, 4, 1, 1, 1,
            2, 2, 3, 2, 2, 3, 2, 2,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        pretty_assertions::assert_eq!(expected_white_attacker_count, result_w);
        pretty_assertions::assert_eq!(expected_black_attacker_count, result_b);
    }

    #[test]
    fn attackers_ok_middlegame_position() {
        let fen = "r3kb1r/1b3ppp/pqnppn2/1p6/4PBP1/PNN5/1PPQBP1P/2KR3R b kq - 0 1".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();
        let result_w = aggregate_attacks(&chess, ColorChar::W);
        let result_b = aggregate_attacks(&chess, ColorChar::B);

        #[rustfmt::skip]
        let expected_white_attacker_count = vec![
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 2, 0, 0, 0, 1,
            1, 2, 1, 3, 1, 2, 1, 1,
            1, 1, 1, 2, 1, 1, 1, 0,
            1, 1, 2, 3, 3, 1, 3, 0,
            1, 1, 2, 4, 2, 0, 0, 2,
            1, 2, 3, 5, 3, 3, 2, 1,
        ];

        #[rustfmt::skip]
        let expected_black_attacker_count = vec![
            1, 2, 2, 4, 2, 2, 2, 0,
            3, 1, 1, 2, 3, 1, 1, 2,
            3, 0, 2, 1, 1, 1, 2, 1,
            2, 2, 2, 2, 2, 1, 0, 1,
            1, 1, 1, 2, 1, 0, 1, 0,
            0, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        pretty_assertions::assert_eq!(expected_white_attacker_count, result_w);
        pretty_assertions::assert_eq!(expected_black_attacker_count, result_b);
    }

    #[test]
    fn all_covered() {
        let fen = "Q4K1k/1Q5p/2Q5/3Q4/4Q3/5Q2/6Q1/7Q w - - 0 1".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        let result_w = aggregate_attacks(&chess, ColorChar::W);
        let result_b = aggregate_attacks(&chess, ColorChar::B);

        #[rustfmt::skip]
        let expected_white_attacker_count: Vec<u8> = vec![
            1, 2, 3, 2, 4, 2, 3, 0,
            2, 2, 2, 3, 3, 4, 3, 3,
            3, 2, 2, 2, 3, 2, 3, 2,
            2, 3, 2, 2, 2, 3, 2, 3,
            3, 2, 3, 2, 2, 2, 3, 2,
            2, 3, 2, 3, 2, 2, 2, 3,
            3, 2, 3, 2, 3, 2, 2, 2,
            2, 3, 2, 3, 2, 3, 2, 1,
        ];

        #[rustfmt::skip]
        let expected_black_attacker_count: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 1, 1,
            0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        pretty_assertions::assert_eq!(expected_white_attacker_count, result_w);
        pretty_assertions::assert_eq!(expected_black_attacker_count, result_b);
    }

    #[test]
    fn return_val_depends_on_side_to_move() {
        let mut chess = WasmChess::new(None).unwrap();

        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::C3, None),
            vec![SquareStr::B1, SquareStr::B2, SquareStr::D2,],
        );
        pretty_assertions::assert_eq!(chess.attackers(SquareStr::C6, None), vec![],);

        chess.make_move("e4").unwrap();

        pretty_assertions::assert_eq!(chess.attackers(SquareStr::C3, None), vec![],);
        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::C6, None),
            vec![SquareStr::B7, SquareStr::D7, SquareStr::B8,],
        );

        chess.make_move("e5").unwrap();

        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::C3, None),
            vec![SquareStr::B1, SquareStr::B2, SquareStr::D2,]
        );
        pretty_assertions::assert_eq!(chess.attackers(SquareStr::C6, None), vec![],);
    }

    #[test]
    fn every_piece_attacking_another_piece() {
        let fen = "4k3/8/8/8/5Q2/5p1R/4PK2/4N2B w - - 0 1".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::F3, None),
            vec![
                SquareStr::E1,
                SquareStr::H1,
                SquareStr::E2,
                SquareStr::F2,
                SquareStr::H3,
                SquareStr::F4,
            ]
        );
    }

    #[test]
    fn every_piece_defending_empty_square() {
        let fen = "B3k3/8/8/2K4R/3QPN2/8/8/8 w - - 0 1".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::D5, Some(ColorChar::W)),
            vec![
                SquareStr::D4,
                SquareStr::E4,
                SquareStr::F4,
                SquareStr::C5,
                SquareStr::H5,
                SquareStr::A8,
            ]
        );
    }

    #[test]
    fn every_piece_defending_another_piece() {
        let fen = "2r5/1b1p4/1kp1q3/4n3/8/8/8/4K3 b - - 0 1".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::C6, None),
            vec![
                SquareStr::E5,
                SquareStr::B6,
                SquareStr::E6,
                SquareStr::B7,
                SquareStr::D7,
                SquareStr::C8,
            ]
        );
    }

    #[test]
    fn pinned_piece_attacks_and_defends() {
        let fen = "r1bqkbnr/ppp2ppp/2np4/1B2p3/3PP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 4".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::D4, Some(ColorChar::B)),
            vec![SquareStr::E5, SquareStr::C6,]
        );
        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::E5, Some(ColorChar::B)),
            vec![SquareStr::C6, SquareStr::D6,]
        );
    }

    #[test]
    fn kind_can_attack_defended_piece() {
        let fen = "3k4/8/8/8/3b4/3R4/4Pq2/4K3 w - - 0 1".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::F2, Some(ColorChar::W)),
            vec![SquareStr::E1]
        );
    }

    // Too much material
    // #[test]
    // fn lot_of_attackers() {
    //     let fen = "5k2/8/3N1N2/2NBQQN1/3R1R2/2NPRPN1/3N1N2/4K3 w - - 0 1".to_string();
    //     let chess = WasmChess::new(Some(fen)).unwrap();

    //     pretty_assertions::assert_eq!(
    //         chess.attackers(SquareStr::E4, Some(ColorChar::W)),
    //         vec![
    //         ]
    //     );
    // }

    #[test]
    fn no_attackers() {
        let chess = WasmChess::new(None).unwrap();

        pretty_assertions::assert_eq!(chess.attackers(SquareStr::E4, Some(ColorChar::W)), vec![]);
    }

    #[test]
    fn readme_tests() {
        let mut chess = WasmChess::new(None).unwrap();

        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::F3, None),
            vec![SquareStr::G1, SquareStr::E2, SquareStr::G2,]
        );
        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::E2, None),
            vec![SquareStr::D1, SquareStr::E1, SquareStr::F1, SquareStr::G1,]
        );
        pretty_assertions::assert_eq!(chess.attackers(SquareStr::F6, None), vec![]);

        chess.make_move("e4").unwrap();

        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::F6, None),
            vec![SquareStr::E7, SquareStr::G7, SquareStr::G8,]
        );
        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::F3, Some(ColorChar::W)),
            vec![SquareStr::D1, SquareStr::G1, SquareStr::G2,]
        );

        chess
            .load("4k3/4n3/8/8/8/8/4R3/4K3 w - - 0 1".to_string())
            .unwrap();

        pretty_assertions::assert_eq!(
            chess.attackers(SquareStr::C6, Some(ColorChar::B)),
            vec![SquareStr::E7,]
        );
    }
}
