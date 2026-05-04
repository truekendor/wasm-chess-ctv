#[cfg(test)]
pub mod attackers_tests {
    use crate::{
        WasmChess,
        tsify_structs::{SquareStr, others::ColorChar},
    };

    fn aggregate_attacks(chess: &WasmChess, color: ColorChar) -> [u8; 64] {
        let mut attackers_vec = [0; 64];

        for rank in (1..=8).rev() {
            for file in 'a'..='h' {
                let square_str = format!("{}{}", file, rank);
                let square = square_str.parse::<SquareStr>().unwrap();

                let file_idx = (file as u8 - b'a') as usize;
                let rank_idx = (rank - 1) as usize;
                let idx = rank_idx * 8 + 7 - file_idx;

                let attack_count = chess.attackers_unblocked(square.clone(), Some(color.clone()));

                attackers_vec[idx] = attack_count.len() as u8;
            }
        }

        attackers_vec.reverse();

        return attackers_vec;
    }

    #[test]
    fn attackers_unblocked_ok_default_position() {
        let chess = WasmChess::new(None).unwrap();
        let result_w = aggregate_attacks(&chess, ColorChar::W);
        let result_b = aggregate_attacks(&chess, ColorChar::B);

        #[rustfmt::skip]
        let answer_w = vec![
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
        let answer_b = vec![
            0, 1, 1, 1, 1, 1, 1, 0,
            1, 1, 1, 4, 4, 1, 1, 1,
            2, 2, 3, 2, 2, 3, 2, 2,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        pretty_assertions::assert_eq!(answer_w, result_w);
        pretty_assertions::assert_eq!(answer_b, result_b);
    }

    #[test]
    fn attacker_unblocked_ok_middlegame_position() {
        let fen = "r3kb1r/1b3ppp/pqnppn2/1p6/4PBP1/PNN5/1PPQBP1P/2KR3R b kq - 0 1".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();
        let result_w = aggregate_attacks(&chess, ColorChar::W);
        let result_b = aggregate_attacks(&chess, ColorChar::B);

        #[rustfmt::skip]
        let answer_w = vec![
            0, 1, 0, 1, 0, 0, 0, 0,
            0, 0, 1, 1, 0, 0, 0, 0,
            1, 0, 0, 2, 0, 0, 0, 1,
            1, 2, 1, 3, 1, 2, 1, 1,
            1, 1, 1, 2, 1, 1, 1, 0,
            1, 1, 2, 3, 3, 1, 3, 0,
            1, 1, 2, 4, 2, 0, 0, 2,
            1, 2, 3, 5, 3, 3, 2, 1,
        ];

        #[rustfmt::skip]
        let answer_b = vec![
            1, 2, 2, 4, 2, 2, 2, 0,
            3, 1, 1, 2, 3, 1, 1, 2,
            3, 0, 2, 1, 1, 1, 2, 1,
            2, 2, 2, 2, 2, 1, 0, 1,
            1, 1, 1, 2, 1, 0, 1, 0,
            0, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 0,
        ];

        let hmm = chess.attackers_unblocked(SquareStr::G2, Some(ColorChar::B));

        println!("{:#?}", hmm);

        pretty_assertions::assert_eq!(answer_w, result_w);
        pretty_assertions::assert_eq!(answer_b, result_b);
    }

    #[test]
    fn all_covered() {
        let fen = "Q4K1k/1Q5p/2Q5/3Q4/4Q3/5Q2/6Q1/7Q w - - 0 1".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        let result_w = aggregate_attacks(&chess, ColorChar::W);
        let result_b = aggregate_attacks(&chess, ColorChar::B);

        #[rustfmt::skip]
        let answer_b: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 1, 1,
            0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        pretty_assertions::assert_eq!(answer_b, result_b);
    }
}
