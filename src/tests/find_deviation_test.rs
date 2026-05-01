#[cfg(test)]
pub mod diverge_transpose_test {
    use shakmaty::{Chess, fen::Fen};

    use crate::helpers::find_deviation::{DivergeData, TranspositionDataEntry, find_deviation};

    #[test]
    fn immediate_diverge() {
        let start_pos =
            Fen::from_position(&Chess::default(), shakmaty::EnPassantMode::Legal).to_string();

        let move_list_main: Vec<String> = vec![
            "d4", "d5", "c4", "e6", "Nc3", "Nf6", "Bd2", "a6", "Bc1", "a5",
        ]
        .iter()
        .map(|sss| sss.to_string())
        .collect();

        let move_list_reverse: Vec<String> = vec![
            "c4", "e6", "Nc3", "Nf6", "d4", "d5", "Bd2", "a5", "Bc1", "Ng8",
        ]
        .iter()
        .map(|sss| sss.to_string())
        .collect();

        let result = find_deviation(start_pos, move_list_main, move_list_reverse);

        assert_eq!(
            result,
            vec![
                TranspositionDataEntry {
                    move_index: 5,
                    diverge_data: None,
                },
                TranspositionDataEntry {
                    move_index: 6,
                    diverge_data: None,
                },
                TranspositionDataEntry {
                    move_index: 7,
                    diverge_data: Some(DivergeData {
                        move_san: "a5".to_string(),
                        move_index: 7
                    }),
                },
            ]
        );
    }

    #[test]
    fn diverge_after_one_move() {
        let start_pos =
            Fen::from_position(&Chess::default(), shakmaty::EnPassantMode::Legal).to_string();

        let move_list_main: Vec<String> =
            vec!["e4", "e6", "d4", "d5", "exd5", "exd5", "Nf3", "Nf6", "a3"]
                .iter()
                .map(|sss| sss.to_string())
                .collect();

        let move_list_reverse: Vec<String> = vec![
            "e4", "e5", "Nf3", "Nf6", "Nxe5", "d6", "Nf3", "Nxe4", "d3", "Nf6", "d4", "d5", "a4",
        ]
        .iter()
        .map(|sss| sss.to_string())
        .collect();

        let result = find_deviation(start_pos, move_list_main, move_list_reverse);

        assert_eq!(
            result,
            vec![
                TranspositionDataEntry {
                    move_index: 0,
                    diverge_data: None,
                },
                TranspositionDataEntry {
                    move_index: 1,
                    diverge_data: Some(DivergeData {
                        move_san: "e5".into(),
                        move_index: 1,
                    }),
                },
                TranspositionDataEntry {
                    move_index: 7,
                    diverge_data: None,
                },
                TranspositionDataEntry {
                    move_index: 8,
                    diverge_data: Some(DivergeData {
                        move_san: "a4".into(),
                        move_index: 12,
                    }),
                },
            ]
        );
    }
}
