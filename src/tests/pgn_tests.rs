/// these tests are taken from chess.js test suite for pgn() method
///
/// @link https://github.com/jhlywa/chess.js/blob/master/__tests__/pgn.test.ts
#[cfg(test)]
pub mod pgn_from_chess_test {
    use std::collections::HashMap;

    use crate::{
        WasmChess,
        tsify_structs::others::{ColorChar, PGNOptions},
    };

    #[test]
    fn pgn_works_removes_header() {
        let pgn = r#"
  [White "Paul Morphy"]
  [Black "Duke Karl / Count Isouard"]
  [fEn "1n2kb1r/p4ppp/4q3/4p1B1/4P3/8/PPP2PPP/2KR4 w k - 0 17"]

  17.Rd8# 1-0"#;

        let mut chess = WasmChess::new(None).unwrap();
        let mut chess2 = WasmChess::new(None).unwrap();

        let pgn2 = r#"
  [White "?"]
  [Black "Duke Karl / Count Isouard"]
  [fEn "1n2kb1r/p4ppp/4q3/4p1B1/4P3/8/PPP2PPP/2KR4 w k - 0 17"]

  17.Rd8# 1-0"#;

        chess.load_pgn(pgn).unwrap();
        chess2.load_pgn(pgn2).unwrap();

        chess.remove_header("White".to_string());

        pretty_assertions::assert_eq!(chess.get_headers(), chess2.get_headers());
    }

    #[test]
    fn pgn_works_begins_on_correct_turn_black() {
        let mut chess = WasmChess::new(None).unwrap();
        chess.load_pgn("1. e4").expect("PGN is correct");

        pretty_assertions::assert_eq!(chess.turn(), ColorChar::B);
    }

    #[test]
    fn pgn_works_begins_on_correct_turn_white() {
        let mut chess = WasmChess::new(None).unwrap();
        chess.load_pgn("1. e4 e5").expect("PGN is correct");

        pretty_assertions::assert_eq!(chess.turn(), ColorChar::W);
    }

    #[test]
    fn pgn_works_begins_on_correct_turn_when_empty() {
        let pgn = r#"
    [White "LichessAborter"]
    [Black "PoorSap"]"#;
        let mut chess = WasmChess::new(None).unwrap();
        chess.load_pgn(pgn).expect("PGN is correct");

        pretty_assertions::assert_eq!(chess.turn(), ColorChar::W);
    }

    #[test]
    fn pgn_works_removing_non_existent_header() {
        let pgn = r#"
  [White "LichessAborter"]
  [Black "PoorSap"]

  *"#;
        let mut chess = WasmChess::new(None).unwrap();
        chess.load_pgn(pgn).unwrap();

        let exists = chess.remove_header("Non-existent".to_string());

        pretty_assertions::assert_eq!(exists, false);
    }

    // TODO:
    // fix the formatting to be 100% compatible with chess.js
    #[test]
    fn list_of_positions_ok() {
        struct TestingPositions<'a> {
            moves: &'a str,
            header_tags: HashMap<&'a str, &'a str>,
            max_width: u32,
            new_line_char: Option<&'a str>,
            pgn: &'a str,
            final_fen: &'a str,
            starting_position: Option<&'a str>,
        }

        // TODO: add rest of the positions
        // from @link https://github.com/jhlywa/chess.js/blob/master/__tests__/pgn.test.ts
        let positions: Vec<TestingPositions> = vec![
            TestingPositions {
                moves: r#"d4 d5 Nf3 Nc6 e3 e6 Bb5 g5 O-O Qf6 Nc3 Bd7 Bxc6 Bxc6 Re1 O-O-O a4
        Bb4 a5 b5 axb6 axb6 Ra8+ Kd7 Ne5+ Kd6 Rxd8+ Qxd8 Nxf7+ Ke7 Nxd5+ Qxd5
        c3 Kxf7 Qf3+ Qxf3 gxf3 Bxf3 cxb4 e5 dxe5 Ke6 b3 Kxe5 Bb2+ Ke4 Bxh8 Nf6
        Bxf6 h5 Bxg5 Bg2 Kxg2 Kf5 Bh4 Kg4 Bg3 Kf5 e4+ Kg4 e5 h4 Bxh4 Kxh4 e6 c5
        bxc5 bxc5 e7 c4 bxc4 Kg4 e8=Q Kf5 Qe5+ Kg4 Re4#"#,
                header_tags: HashMap::from([
                    ("White", "Jeff Hlywa"),
                    ("Black", "Steve Bragg"),
                    ("Result", "1-0"),
                    ("GreatestGameEverPlayed?", "True"),
                ]),
                max_width: 19,
                new_line_char: Some("<br />"),
                final_fen: "8/8/8/4Q3/2P1R1k1/8/5PKP/8 b - - 4 39",
                pgn: r#"[Event "?"]<br />[Site "?"]<br />[Date "????.??.??"]<br />[Round "?"]<br />[White "Jeff Hlywa"]<br />[Black "Steve Bragg"]<br />[Result "1-0"]<br />[GreatestGameEverPlayed? "True"]<br /><br />1. d4 d5 2. Nf3 Nc6<br />3. e3 e6 4. Bb5 g5<br />5. O-O Qf6<br />6. Nc3 Bd7<br />7. Bxc6 Bxc6<br />8. Re1 O-O-O<br />9. a4 Bb4 10. a5 b5<br />11. axb6 axb6<br />12. Ra8+ Kd7<br />13. Ne5+ Kd6<br />14. Rxd8+ Qxd8<br />15. Nxf7+ Ke7<br />16. Nxd5+ Qxd5<br />17. c3 Kxf7<br />18. Qf3+ Qxf3<br />19. gxf3 Bxf3<br />20. cxb4 e5<br />21. dxe5 Ke6<br />22. b3 Kxe5<br />23. Bb2+ Ke4<br />24. Bxh8 Nf6<br />25. Bxf6 h5<br />26. Bxg5 Bg2<br />27. Kxg2 Kf5<br />28. Bh4 Kg4<br />29. Bg3 Kf5<br />30. e4+ Kg4<br />31. e5 h4<br />32. Bxh4 Kxh4<br />33. e6 c5<br />34. bxc5 bxc5<br />35. e7 c4<br />36. bxc4 Kg4<br />37. e8=Q Kf5<br />38. Qe5+ Kg4<br />39. Re4# 1-0"#,
                starting_position: None,
            },
            TestingPositions {
                moves: r#"c4 e6 Nf3 d5 d4 Nf6 Nc3 Be7 Bg5 O-O e3 h6 Bh4 b6 cxd5 Nxd5 Bxe7
        Qxe7 Nxd5 exd5 Rc1 Be6 Qa4 c5 Qa3 Rc8 Bb5 a6 dxc5 bxc5 O-O Ra7 Be2 Nd7
        Nd4 Qf8 Nxe6 fxe6 e4 d4 f4 Qe7 e5 Rb8 Bc4 Kh8 Qh3 Nf8 b3 a5 f5 exf5
        Rxf5 Nh7 Rcf1 Qd8 Qg3 Re7 h4 Rbb7 e6 Rbc7 Qe5 Qe8 a4 Qd8 R1f2 Qe8 R2f3
        Qd8 Bd3 Qe8 Qe4 Nf6 Rxf6 gxf6 Rxf6 Kg8 Bc4 Kh8 Qf4"#,
                header_tags: HashMap::from([
                    ("Event", "Reykjavik WCh"),
                    ("Site", "Reykjavik WCh"),
                    ("Date", "1972.01.07"),
                    ("EventDate", "?"),
                    ("Round", "6"),
                    ("Result", "1-0"),
                    ("White", "Robert James Fischer"),
                    ("Black", "Boris Spassky"),
                    ("ECO", "D59"),
                    ("WhiteElo", "?"),
                    ("BlackElo", "?"),
                    ("PlyCount", "81"),
                ]),
                max_width: 65,
                starting_position: None,
                final_fen: "4q2k/2r1r3/4PR1p/p1p5/P1Bp1Q1P/1P6/6P1/6K1 b - - 4 41",

                pgn: r#"[Event "Reykjavik WCh"]
[Site "Reykjavik WCh"]
[Date "1972.01.07"]
[Round "6"]
[White "Robert James Fischer"]
[Black "Boris Spassky"]
[Result "1-0"]
[WhiteElo "?"]
[BlackElo "?"]
[EventDate "?"]
[ECO "D59"]
[PlyCount "81"]

1. c4 e6 2. Nf3 d5 3. d4 Nf6 4. Nc3 Be7 5. Bg5 O-O 6. e3 h6
7. Bh4 b6 8. cxd5 Nxd5 9. Bxe7 Qxe7 10. Nxd5 exd5 11. Rc1 Be6
12. Qa4 c5 13. Qa3 Rc8 14. Bb5 a6 15. dxc5 bxc5 16. O-O Ra7
17. Be2 Nd7 18. Nd4 Qf8 19. Nxe6 fxe6 20. e4 d4 21. f4 Qe7
22. e5 Rb8 23. Bc4 Kh8 24. Qh3 Nf8 25. b3 a5 26. f5 exf5
27. Rxf5 Nh7 28. Rcf1 Qd8 29. Qg3 Re7 30. h4 Rbb7 31. e6 Rbc7
32. Qe5 Qe8 33. a4 Qd8 34. R1f2 Qe8 35. R2f3 Qd8 36. Bd3 Qe8
37. Qe4 Nf6 38. Rxf6 gxf6 39. Rxf6 Kg8 40. Bc4 Kh8 41. Qf4 1-0"#,
                new_line_char: None,
            },
        ];

        positions.iter().for_each(|position| {
            let mut chess = WasmChess::new(None).unwrap();

            if let Some(starting_pos) = position.starting_position {
                chess.load(starting_pos.to_string()).unwrap();
            }

            let moves = position.moves.split_whitespace();
            let moves_count = &moves.clone().count();

            moves.for_each(|move_str| {
                chess.make_move(move_str).unwrap();
            });

            pretty_assertions::assert_eq!(*moves_count as u32, chess.move_number());
            pretty_assertions::assert_eq!(chess.fen(None), position.final_fen);

            position.header_tags.iter().for_each(|(key, value)| {
                chess.set_header(key.to_string(), value.to_string());
            });

            let options = PGNOptions {
                max_width: Some(position.max_width as usize),
                newline: match position.new_line_char {
                    Some(val) => Some(val.to_string()),
                    None => Some("\n".to_string()),
                },
            };

            let pgn = chess.pgn(Some(options));

            pretty_assertions::assert_eq!(position.pgn, pgn);
        });
    }
}
