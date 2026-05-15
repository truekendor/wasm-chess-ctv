/// these tests taken from chess.js test suite for pgn() method
///
/// @link https://github.com/jhlywa/chess.js/blob/master/__tests__/pgn.test.ts
#[cfg(test)]
pub mod pgn_from_chess_test {
    use std::{collections::HashMap, fs};

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
        let game_zero_pgn = file_to_string("./src/tests/pgn/chessjs-0.pgn");
        let game_one_pgn = file_to_string("./src/tests/pgn/chessjs-1.pgn");
        let game_two_pgn = file_to_string("./src/tests/pgn/chessjs-2.pgn");
        let game_three_pgn = file_to_string("./src/tests/pgn/chessjs-3.pgn");

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
                pgn: &game_zero_pgn,
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

                pgn: &game_one_pgn,
                new_line_char: None,
            },
            TestingPositions {
                starting_position: None,
                final_fen: "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3",
                moves: "f3 e5 g4 Qh4#",
                header_tags: HashMap::from([("Result", "0-1")]),
                max_width: 1,
                pgn: &game_two_pgn,
                new_line_char: None,
            },
            TestingPositions {
                starting_position: Some(
                    "r1bqk1nr/pppp1ppp/2n5/4p3/1bB1P3/2P2N2/P2P1PPP/RNBQK2R b KQkq - 0 1",
                ),
                final_fen: "r1bqk1nr/ppp2ppp/2np4/b3p3/2BPP3/2P2N2/P4PPP/RNBQ1RK1 b kq - 0 3",
                moves: "Ba5 O-O d6 d4",
                header_tags: HashMap::new(),
                max_width: 20,
                pgn: &game_three_pgn,
                new_line_char: None,
            },
        ];

        positions.iter().for_each(|position| {
            let mut chess = WasmChess::new(None).unwrap();

            if let Some(starting_pos) = position.starting_position {
                chess.load(starting_pos, None).unwrap();
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
                    None => Some("\r\n".to_string()),
                },
            };

            let pgn = chess.pgn(Some(options));
            // TODO:
            // delete
            // println!("pgn() output: {:#?}", pgn);
            // println!("expected    : {:#?}", position.pgn);

            pretty_assertions::assert_eq!(position.pgn.trim(), pgn);
        });
    }

    fn file_to_string(path: &str) -> String {
        let vec = fs::read(path).unwrap();

        String::from_utf8(vec).expect("PGN chars are all valid in this case")
    }
}
