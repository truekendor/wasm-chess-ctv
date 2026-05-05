// TODO these are more or less mock test cases
// TODO need to port chess.js test cases here

#[cfg(test)]
mod pgn_test {
    static PGN_FROM_WIKI: &'static str = r#"[Event "F/S Return Match"]
[Site "Belgrade, Serbia JUG"]
[Date "1992.11.04"]
[Round "29"]
[White "Fischer, Robert J."]
[Black "Spassky, Boris V."]
[Result "1/2-1/2"]

1.e4 e5 2.Nf3 Nc6 3.Bb5 {This opening is called the Ruy Lopez.} 3...a6
4.Ba4 Nf6 5.O-O Be7 6.Re1 b5 7.Bb3 d6 8.c3 O-O 9.h3 Nb8 10.d4 Nbd7
11.c4 c6 12.cxb5 axb5 13.Nc3 Bb7 14.Bg5 b4 15.Nb1 h6 16.Bh4 c5 17.dxe5
Nxe4 18.Bxe7 Qxe7 19.exd6 Qf6 20.Nbd2 Nxd6 21.Nc4 Nxc4 22.Bxc4 Nb6
23.Ne5 Rae8 24.Bxf7+ Rxf7 25.Nxf7 Rxe1+ 26.Qxe1 Kxf7 27.Qe3 Qg5 28.Qxg5
hxg5 29.b3 Ke6 30.a3 Kd6 31.axb4 cxb4 32.Ra5 Nd5 33.f3 Bc8 34.Kf2 Bf5
35.Ra7 g6 36.Ra6+ Kc5 37.Ke1 Nf4 38.g3 Nxh3 39.Kd2 Kb5 40.Rd6 Kc5 41.Ra6
Nf2 42.g4 Bd3 43.Re6 1/2-1/2"#;

    static PGN_WITH_NAGS: &'static str = r#"
    [Event "CCC 25 Double-Fischer: Finals"]
[Site "https://www.chess.com/computer-chess-championship"]
[Date "2026.03.27"]
[Round "19"]
[White "Reckless"]
[Black "Stockfish"]
[Result "1/2-1/2"]
[SetUp "1"]
[FEN "nqbrkbrn/pppppp1p/6p1/8/7P/8/PPPPPPP1/BBNRNKRQ w GDgd - 0 2"]
[Variant "Chess960"]
[GameDuration "00:12:40"]
[GameStartTime "2026-03-27T13:25:35 -0700"]
[GameEndTime "2026-03-27T13:38:16 -0700"]
[PlyCount "99"]
[Termination "normal"]
[TimeControl "300+2"]

2. h5! $18 {+1.17/30 21.193s, tl=282.807s, latency=0.188s, n=1656310213, sd=50, nps=104117663, hashfull=229, tbhits=55, pv="h4h5 d7d5 g2g4 c7c6 b2b3 f8g7 d2d4 h7h6 e2e3 g6g5 c2c4 e8g8 c4d5 c6d5 c1e2 a8c7 h1g2 f7f5 g4f5 c8f5 b1f5 f8f5 e1d3 b8c8 a1b2 c7e6 b2a3 c8c2 f1e1 c2a2 g2g4 d8f8 a3e7 f8f7 e7d6 a2b3 f2f4 b3b6 d6e5 g7e5 f4e5"}
d5? {-1.02/35 56.623s, tl=247.377s, latency=0.000s, n=4788469447, sd=67, nps=84567568, hashfull=903, tbhits=10071, pv="d7d5 b2b3 c7c6 g2g4 f8g7 d2d4 f7f5 g4g5 a8c7 h5h6 g7f8 c1d3 h8f7 e1f3 e7e5 d4e5 c7e6 h1h4 c6c5 d3f4 e6f4 h4f4 f8e7 c2c3 e8g8 b1c2 f8e8 c3c4 d5d4 e2e3 d4e3 f4e3 d8d1 c2d1 f5f4"}
3. b3$3 {+1.20/32 15.818s, tl=268.989s, latency=0.000s, n=1679002170, sd=73, nps=106141573, hashfull=238, tbhits=3119, pv="b2b3 c7c6 c2c4 f8g7 a1g7 g8g7 h5g6 h8g6 c4d5 d8d5 h1h6 e8f8 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 b1g6 g7g6 g3f4 g6g7 e2e4 d5d6 d2d4 f8g8 c1e2 c6c5 d4d5 f7f5 e2g3 b8f8 f4e5 f8f6 e5f6 e7f6 h1h4 g7f7 h4h5 f5e4 g3e4 d7g4 e4d6 g4h5 d1c1 f7e7 d6b7 c7d5 b7c5 h5e2 f1g1"}
c6 {-0.99/29 3.679s, tl=245.698s, latency=0.000s, n=397528982, sd=56, nps=108053542, hashfull=68, tbhits=328, pv="c7c6 g2g4 f8g7 d2d4 f7f5 g4g5 a8c7 h5h6 g7f8 c1d3 h8f7 e1f3 e7e5 d4e5 f8e7 d3f4 c7e6 h1h4 e6f4 h4f4 c6c5 b3b4 b7b6 a2a4 e8g8 a1b2 f8e8 b1a2 c8e6 c2c3 b8c8"}
4. c4!? {+1.19/35 16.144s, tl=254.845s, latency=0.001s, n=1799980062, sd=66, nps=111499580, hashfull=263, tbhits=224, pv="c2c4 f8g7 a1g7 g8g7 h5g6 h8g6 c4d5 d8d5 h1h6 e8f8 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 c1d3 g6e5 g3f4 e5g6 f4h6 c7e8 e1c2 e8f6 c2e3 d5d4 d3e1 f6g8 h6h2 b8h2 h1h2 g8f6 e1f3 d4d6 f3g5 d6d4 g2g3 h7h5 g5f3 d4d6 f1e1 g7g8 d1c1 d7e6 h2h1 d6d7 b1g6 f7g6 h1h4 f8g7 d2d3 d7d6 h4a4 g8a8 f3g5 e6g8 e1d2"}
Bg7 {-0.89/30 4.666s, tl=243.032s, latency=0.001s, n=425937676, sd=59, nps=91304968, hashfull=116, tbhits=1356, pv="f8g7 a1g7 g8g7 h5g6 h8g6 c4d5 d8d5 h1h6 e8f8 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 c1d3 g6e5 g3f4 e5g6 f4e3 d7f5 e1f3 d5d8 d3c5 c7d5 e3h6 f5b1 d1b1 d5f6 d2d4 f8g8 g2g3 f6g4 h6c1 e7e5 d4e5 g6e5 c1f4 e5f3 f4f3 b8e5 c5d3 e5f6 f3f6 g4f6"}
5. Bxg7 $5 {+1.25/32 4.893s, tl=251.952s, latency=0.001s, n=634402185, sd=60, nps=129658090, hashfull=63, tbhits=62, pv="a1g7 g8g7 h5g6 h8g6 c4d5 d8d5 h1h6 e8f8 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 c1d3 g6e5 g3f4 e5g6 f4h6 c7e8 e1c2 e8f6 c2e3 d5d4 d3e1 f6g8 h6h2 b8h2 h1h2 g8f6 e1f3 d4d6 f3g5 d6d4 g2g3 h7h5 g5f3 d4d6 e3c4 d6d5 e2e4 d5b5 d1e1 g7h7 e4e5 f6g4 h2h1 h7h6 d2d4 d7e6 b1e4 e6d5 f3h4 h6h8 h4g2 d5c4 b3c4"}
Rxg7 $2 {-0.91/32 8.357s, tl=236.675s, latency=0.000s, n=799614202, sd=53, nps=95681967, hashfull=182, tbhits=9645, pv="g8g7 c4d5 d8d5 h5g6 h8g6 h1h6 e8f8 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 c1d3 b8d8 g3e3 e7e5 d3c5 d7c8 b1g6 f7g6 e1f3 c7b5 f3g5 d8e7 c5e4 c8f5 d2d3 f8g8 f2f3 d5d4"}
6. hxg6 $4 {+1.00/36 28.024s, tl=225.928s, latency=0.001s, n=2994118813, sd=59, nps=106848380, hashfull=405, tbhits=1049, pv="h5g6 h8g6 c4d5 d8d5 h1h6 e8f8 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 b1e4 d5d6 c1d3 f7f5 e4f3 f8g8 d3e5 b8d8 e5g6 g7g6 g3e5 d8f8 e1d3 g6h6 h1h6 f8h6 e2e3 b7b6 b3b4 d7e6 f1g1 h6g7 e5f4 c7d5 f4h2 d5f6 d3e5 f6e4 d2d3 e4g5 g1f1 e6a2 h2f4 a2b3 d1c1 d6h6 f1e2 g5f7 d3d4 f7e5 d4e5 h6e6"}
Nxg6$1 {-0.86/30 5.212s, tl=233.463s, latency=0.000s, n=507732918, sd=73, nps=97416139, hashfull=119, tbhits=708, pv="h8g6 h1h6 e8f8 c4d5 d8d5 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 b1e4 d5d6 c1d3 b8d8 d3e5 g6e5 g3e5 f8g8 e5h2 f7f5 e4d3 c7d5 f2f3 b7b5 g2g4 f5g4 d3h7 g8f8 h7e4 g4g3"}
7. Qh6 $6 {+1.03/35 34.088s, tl=193.840s, latency=0.001s, n=3653550737, sd=69, nps=107180400, hashfull=460, tbhits=4577, pv="h1h6 e8f8 c4d5 d8d5 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 b1e4 d5d6 c1d3 f7f5 e4f3 f8g8 d3e5 b8d8 e5g6 g7g6 g3e5 d8f8 e1d3 g6h6 h1h6 f8h6 e2e3 b7b6 b3b4 d6d3 e5c7 h6d6 c7a7 d3d2 d1d2 d6d2 g2g3 b6b5 a7b8 g8f7 b8d8 f7e6 a2a3 d2d3 f1g2 d3a3 g3g4 a3d3 d8f8 e6d6 f8b8 d6e6 g4g5 d3c3 b8g8 e6d6 g8h7 c3b4 h7h2 f5f4 g5g6 d7e6 e3f4 d6d7 g6g7"}
Kf8 {-0.86/32 7.341s, tl=228.122s, latency=-0.001s, n=732199533, sd=55, nps=99727531, hashfull=166, tbhits=8246, pv="e8f8 c4d5 d8d5 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 b1e4 d5d6 c1d3 b8d8 g3e3 b7b6 e4g6 d6g6 d3e5 g6e6 e1f3 f7f6 e5d7 d8d7 e3f4 e6d6 d2d3 c7d5 f4d2 e7e5"}"#;

    use pgn_reader::Reader;

    use crate::{WasmChess, helpers::pgn_reader::PGNResult};

    use crate::tsify_structs::others::*;

    use std::{
        fs::{self},
        io,
    };

    #[test]
    fn headers() {
        let mut reader = Reader::new(io::Cursor::new(PGN_FROM_WIKI));
        let mut pgn_parser = PGNResult::default();
        reader.read_game(&mut pgn_parser).unwrap();

        pretty_assertions::assert_eq!(*pgn_parser.headers.get("Round").unwrap(), "29".to_owned());
        assert!(pgn_parser.headers.contains_key("Site"));
        assert!(pgn_parser.headers.contains_key("White"));
        assert!(pgn_parser.headers.contains_key("Black"));
        // pretty_assertions::assert_eq!(pgn_parser.move_list.len(), 85);

        assert!(!pgn_parser.headers.contains_key("Variant"));
    }

    #[test]
    fn load_and_read_valid_pgn() {
        let pgn = fs::read("./src/tests/pgn/1.pgn").unwrap();

        let mut reader: Reader<io::Cursor<Vec<u8>>> = Reader::new(io::Cursor::new(pgn));
        let mut pgn_headers = PGNResult::default();

        reader.read_game(&mut pgn_headers).unwrap();
    }

    #[test]
    fn pgn_loads_correctly() {
        let pgn = fs::read("./src/tests/pgn/1.pgn").unwrap();

        let mut reader: Reader<io::Cursor<Vec<u8>>> = Reader::new(io::Cursor::new(pgn.clone()));
        let mut pgn_headers = PGNResult::default();

        reader.read_game(&mut pgn_headers).unwrap();

        let mut wasm_chess = WasmChess::new(None).unwrap();
        wasm_chess
            .load_pgn(String::from_utf8(pgn).unwrap().as_str())
            .unwrap();

        pretty_assertions::assert_eq!(
            wasm_chess.fen_at(0).unwrap(),
            "nqbrkbrn/pppppp1p/6p1/8/7P/8/PPPPPPP1/BBNRNKRQ w KQkq - 0 2"
        );
    }

    #[test]
    fn begins_on_correct_turn_w() {
        let pgn = r#"1. e4 e5"#;

        let mut wasm_chess = WasmChess::new(None).unwrap();

        wasm_chess.load_pgn(pgn).unwrap();
        let turn = wasm_chess.turn();
        pretty_assertions::assert_eq!(turn, ColorChar::W);
    }

    #[test]
    fn begins_on_correct_turn_b() {
        let pgn = r#"1. e4"#;

        let mut wasm_chess = WasmChess::new(None).unwrap();

        wasm_chess.load_pgn(pgn).unwrap();
        let turn = wasm_chess.turn();
        pretty_assertions::assert_eq!(turn, ColorChar::B);
    }

    #[test]
    fn begins_on_correct_turn_w_empty() {
        let pgn = r#"  
[White "LichessAborter"]
[Black "PoorSap"]

*"#;

        let mut wasm_chess = WasmChess::new(None).unwrap();

        wasm_chess.load_pgn(pgn).unwrap();
        let turn = wasm_chess.turn();
        pretty_assertions::assert_eq!(turn, ColorChar::W);
    }

    #[test]
    // TODO move to own test file
    fn remove_header_ok() {
        let mut wasm_chess = WasmChess::new(None).unwrap();
        wasm_chess.load_pgn(PGN_FROM_WIKI).unwrap();

        // Check before removal
        assert!(
            wasm_chess
                .pgn_result
                .as_mut()
                .unwrap()
                .headers
                .contains_key("White")
        );

        wasm_chess.remove_header("White".to_owned());

        // Check after removal
        assert!(
            !wasm_chess
                .pgn_result
                .as_mut()
                .unwrap()
                .headers
                .contains_key("White")
        );
    }

    #[test]
    fn set_header_ok() {
        let mut wasm_chess = WasmChess::new(None).unwrap();
        wasm_chess.load_pgn(PGN_FROM_WIKI).unwrap();

        let arbitrary_tag = "MyTag";

        assert!(
            !wasm_chess
                .pgn_result
                .as_mut()
                .unwrap()
                .headers
                .contains_key(arbitrary_tag)
        );

        let _ = wasm_chess.set_header(arbitrary_tag.to_owned(), "MyValue".to_owned());

        assert!(
            wasm_chess
                .pgn_result
                .as_mut()
                .unwrap()
                .headers
                .contains_key(arbitrary_tag)
        );
    }

    // TODO uncomment when implemented
    #[test]
    fn comments_ok() {
        let mut wasm_chess = WasmChess::new(None).unwrap();

        let _ = wasm_chess.load_pgn(PGN_WITH_NAGS).unwrap();
        let comments = wasm_chess.get_comments();

        let answer = vec![
            CommentsObj {
                fen: "nqbrkbrn/pppppp1p/6p1/7P/8/8/PPPPPPP1/BBNRNKRQ b KQkq - 0 2".to_string(),
                nags: vec![
                    "$18".to_string()
                ],
                comment: Some("+1.17/30 21.193s, tl=282.807s, latency=0.188s, n=1656310213, sd=50, nps=104117663, hashfull=229, tbhits=55, pv=\"h4h5 d7d5 g2g4 c7c6 b2b3 f8g7 d2d4 h7h6 e2e3 g6g5 c2c4 e8g8 c4d5 c6d5 c1e2 a8c7 h1g2 f7f5 g4f5 c8f5 b1f5 f8f5 e1d3 b8c8 a1b2 c7e6 b2a3 c8c2 f1e1 c2a2 g2g4 d8f8 a3e7 f8f7 e7d6 a2b3 f2f4 b3b6 d6e5 g7e5 f4e5\"".to_string()),
                suffix_annotation: Some("!".to_string())
            },
            CommentsObj {
                fen: "nqbrkbrn/ppp1pp1p/6p1/3p3P/8/8/PPPPPPP1/BBNRNKRQ w KQkq - 0 3".to_string(),
                nags: vec![],
                comment: Some("-1.02/35 56.623s, tl=247.377s, latency=0.000s, n=4788469447, sd=67, nps=84567568, hashfull=903, tbhits=10071, pv=\"d7d5 b2b3 c7c6 g2g4 f8g7 d2d4 f7f5 g4g5 a8c7 h5h6 g7f8 c1d3 h8f7 e1f3 e7e5 d4e5 c7e6 h1h4 c6c5 d3f4 e6f4 h4f4 f8e7 c2c3 e8g8 b1c2 f8e8 c3c4 d5d4 e2e3 d4e3 f4e3 d8d1 c2d1 f5f4\"".to_string()),
                suffix_annotation: Some("?".to_string())
            },
            CommentsObj {
                fen: "nqbrkbrn/ppp1pp1p/6p1/3p3P/8/1P6/P1PPPPP1/BBNRNKRQ b KQkq - 0 3".to_string(),
                nags: vec![],
                comment: Some("+1.20/32 15.818s, tl=268.989s, latency=0.000s, n=1679002170, sd=73, nps=106141573, hashfull=238, tbhits=3119, pv=\"b2b3 c7c6 c2c4 f8g7 a1g7 g8g7 h5g6 h8g6 c4d5 d8d5 h1h6 e8f8 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 b1g6 g7g6 g3f4 g6g7 e2e4 d5d6 d2d4 f8g8 c1e2 c6c5 d4d5 f7f5 e2g3 b8f8 f4e5 f8f6 e5f6 e7f6 h1h4 g7f7 h4h5 f5e4 g3e4 d7g4 e4d6 g4h5 d1c1 f7e7 d6b7 c7d5 b7c5 h5e2 f1g1\"".to_string()),
                suffix_annotation: Some("!!".to_string())
            },
            CommentsObj {
                fen: "nqbrkbrn/pp2pp1p/2p3p1/3p3P/8/1P6/P1PPPPP1/BBNRNKRQ w KQkq - 0 4".to_string(),
                nags: vec![],
                comment: Some("-0.99/29 3.679s, tl=245.698s, latency=0.000s, n=397528982, sd=56, nps=108053542, hashfull=68, tbhits=328, pv=\"c7c6 g2g4 f8g7 d2d4 f7f5 g4g5 a8c7 h5h6 g7f8 c1d3 h8f7 e1f3 e7e5 d4e5 f8e7 d3f4 c7e6 h1h4 e6f4 h4f4 c6c5 b3b4 b7b6 a2a4 e8g8 a1b2 f8e8 b1a2 c8e6 c2c3 b8c8\"".to_string()),
                suffix_annotation: None
            },
            CommentsObj {
                fen: "nqbrkbrn/pp2pp1p/2p3p1/3p3P/2P5/1P6/P2PPPP1/BBNRNKRQ b KQkq - 0 4".to_string(),
                nags: vec![],
                comment: Some("+1.19/35 16.144s, tl=254.845s, latency=0.001s, n=1799980062, sd=66, nps=111499580, hashfull=263, tbhits=224, pv=\"c2c4 f8g7 a1g7 g8g7 h5g6 h8g6 c4d5 d8d5 h1h6 e8f8 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 c1d3 g6e5 g3f4 e5g6 f4h6 c7e8 e1c2 e8f6 c2e3 d5d4 d3e1 f6g8 h6h2 b8h2 h1h2 g8f6 e1f3 d4d6 f3g5 d6d4 g2g3 h7h5 g5f3 d4d6 f1e1 g7g8 d1c1 d7e6 h2h1 d6d7 b1g6 f7g6 h1h4 f8g7 d2d3 d7d6 h4a4 g8a8 f3g5 e6g8 e1d2\"".to_string()),
                suffix_annotation: Some("!?".to_string())
            },
            CommentsObj {
                fen: "nqbrk1rn/pp2ppbp/2p3p1/3p3P/2P5/1P6/P2PPPP1/BBNRNKRQ w KQkq - 1 5".to_string(),
                nags: vec![],
                comment: Some("-0.89/30 4.666s, tl=243.032s, latency=0.001s, n=425937676, sd=59, nps=91304968, hashfull=116, tbhits=1356, pv=\"f8g7 a1g7 g8g7 h5g6 h8g6 c4d5 d8d5 h1h6 e8f8 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 c1d3 g6e5 g3f4 e5g6 f4e3 d7f5 e1f3 d5d8 d3c5 c7d5 e3h6 f5b1 d1b1 d5f6 d2d4 f8g8 g2g3 f6g4 h6c1 e7e5 d4e5 g6e5 c1f4 e5f3 f4f3 b8e5 c5d3 e5f6 f3f6 g4f6\"".to_string()),
                suffix_annotation: None
            },
            CommentsObj {
                fen: "nqbrk1rn/pp2ppBp/2p3p1/3p3P/2P5/1P6/P2PPPP1/1BNRNKRQ b KQkq - 0 5".to_string(),
                nags: vec![],
                comment: Some("+1.25/32 4.893s, tl=251.952s, latency=0.001s, n=634402185, sd=60, nps=129658090, hashfull=63, tbhits=62, pv=\"a1g7 g8g7 h5g6 h8g6 c4d5 d8d5 h1h6 e8f8 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 c1d3 g6e5 g3f4 e5g6 f4h6 c7e8 e1c2 e8f6 c2e3 d5d4 d3e1 f6g8 h6h2 b8h2 h1h2 g8f6 e1f3 d4d6 f3g5 d6d4 g2g3 h7h5 g5f3 d4d6 e3c4 d6d5 e2e4 d5b5 d1e1 g7h7 e4e5 f6g4 h2h1 h7h6 d2d4 d7e6 b1e4 e6d5 f3h4 h6h8 h4g2 d5c4 b3c4\"".to_string()),
                suffix_annotation: Some("!?".to_string())
            },
            CommentsObj {
                fen: "nqbrk2n/pp2pprp/2p3p1/3p3P/2P5/1P6/P2PPPP1/1BNRNKRQ w KQq - 0 6".to_string(),
                nags: vec![],
                comment: Some("-0.91/32 8.357s, tl=236.675s, latency=0.000s, n=799614202, sd=53, nps=95681967, hashfull=182, tbhits=9645, pv=\"g8g7 c4d5 d8d5 h5g6 h8g6 h1h6 e8f8 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 c1d3 b8d8 g3e3 e7e5 d3c5 d7c8 b1g6 f7g6 e1f3 c7b5 f3g5 d8e7 c5e4 c8f5 d2d3 f8g8 f2f3 d5d4\"".to_string()),
                suffix_annotation: Some("?".to_string())
            },
            CommentsObj {
                fen: "nqbrk2n/pp2pprp/2p3P1/3p4/2P5/1P6/P2PPPP1/1BNRNKRQ b KQq - 0 6".to_string(),
                nags: vec![],
                comment: Some("+1.00/36 28.024s, tl=225.928s, latency=0.001s, n=2994118813, sd=59, nps=106848380, hashfull=405, tbhits=1049, pv=\"h5g6 h8g6 c4d5 d8d5 h1h6 e8f8 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 b1e4 d5d6 c1d3 f7f5 e4f3 f8g8 d3e5 b8d8 e5g6 g7g6 g3e5 d8f8 e1d3 g6h6 h1h6 f8h6 e2e3 b7b6 b3b4 d7e6 f1g1 h6g7 e5f4 c7d5 f4h2 d5f6 d3e5 f6e4 d2d3 e4g5 g1f1 e6a2 h2f4 a2b3 d1c1 d6h6 f1e2 g5f7 d3d4 f7e5 d4e5 h6e6\"".to_string()),
                suffix_annotation: Some("??".to_string())
            },
            CommentsObj {
                fen: "nqbrk3/pp2pprp/2p3n1/3p4/2P5/1P6/P2PPPP1/1BNRNKRQ w KQq - 0 7".to_string(),
                nags: vec![],
                comment: Some("-0.86/30 5.212s, tl=233.463s, latency=0.000s, n=507732918, sd=73, nps=97416139, hashfull=119, tbhits=708, pv=\"h8g6 h1h6 e8f8 c4d5 d8d5 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 b1e4 d5d6 c1d3 b8d8 d3e5 g6e5 g3e5 f8g8 e5h2 f7f5 e4d3 c7d5 f2f3 b7b5 g2g4 f5g4 d3h7 g8f8 h7e4 g4g3\"".to_string()),
                suffix_annotation: Some("!".to_string())
            },
            CommentsObj {
                fen: "nqbrk3/pp2pprp/2p3nQ/3p4/2P5/1P6/P2PPPP1/1BNRNKR1 b KQq - 1 7".to_string(),
                nags: vec![],
                comment: Some("+1.03/35 34.088s, tl=193.840s, latency=0.001s, n=3653550737, sd=69, nps=107180400, hashfull=460, tbhits=4577, pv=\"h1h6 e8f8 c4d5 d8d5 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 b1e4 d5d6 c1d3 f7f5 e4f3 f8g8 d3e5 b8d8 e5g6 g7g6 g3e5 d8f8 e1d3 g6h6 h1h6 f8h6 e2e3 b7b6 b3b4 d6d3 e5c7 h6d6 c7a7 d3d2 d1d2 d6d2 g2g3 b6b5 a7b8 g8f7 b8d8 f7e6 a2a3 d2d3 f1g2 d3a3 g3g4 a3d3 d8f8 e6d6 f8b8 d6e6 g4g5 d3c3 b8g8 e6d6 g8h7 c3b4 h7h2 f5f4 g5g6 d7e6 e3f4 d6d7 g6g7\"".to_string()),
                suffix_annotation: Some("?!".to_string())
            },
            CommentsObj {
                fen: "nqbr1k2/pp2pprp/2p3nQ/3p4/2P5/1P6/P2PPPP1/1BNRNKR1 w KQ - 2 8".to_string(),
                nags: vec![],
                comment: Some("-0.86/32 7.341s, tl=228.122s, latency=-0.001s, n=732199533, sd=55, nps=99727531, hashfull=166, tbhits=8246, pv=\"e8f8 c4d5 d8d5 g1h1 a8c7 h6h2 c8g4 h2g3 g4d7 b1e4 d5d6 c1d3 b8d8 g3e3 b7b6 e4g6 d6g6 d3e5 g6e6 e1f3 f7f6 e5d7 d8d7 e3f4 e6d6 d2d3 c7d5 f4d2 e7e5\"".to_string()),
                suffix_annotation: None
            }
        ];

        pretty_assertions::assert_eq!(comments.len(), answer.len());
        pretty_assertions::assert_eq!(answer, comments);
    }

    #[test]
    fn comments_ok_second() {
        let comment_fen =
            "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3".to_string();
        let mut wasm_chess = WasmChess::new(None).unwrap();

        wasm_chess.load_pgn(PGN_FROM_WIKI).unwrap();
        let comments = wasm_chess.get_comments();

        pretty_assertions::assert_eq!(
            vec![CommentsObj {
                comment: Some("This opening is called the Ruy Lopez.".to_string()),
                fen: comment_fen,
                nags: vec![],
                suffix_annotation: None
            }],
            comments
        );
    }

    #[test]
    fn remove_comment_from_pgn_ok() {
        let comment_fen =
            "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3".to_string();
        let mut wasm_chess = WasmChess::new(None).unwrap();

        wasm_chess.load_pgn(PGN_FROM_WIKI).unwrap();
        let comments = wasm_chess.get_comments();

        pretty_assertions::assert_eq!(
            vec![CommentsObj {
                comment: Some("This opening is called the Ruy Lopez.".to_string()),
                fen: comment_fen,
                nags: vec![],
                suffix_annotation: None
            }],
            comments
        );

        while wasm_chess.history.len() > 5 {
            wasm_chess.undo().unwrap();
        }

        let comment = wasm_chess.remove_comment();

        assert!(comment.is_some());
        pretty_assertions::assert_eq!(comment.unwrap(), "This opening is called the Ruy Lopez.");

        let comment = wasm_chess.remove_comment();
        assert!(comment.is_none());
    }

    #[test]
    fn it_reads_dfrc_pgn() {
        let pgn = r#"[Variant "From Position"]
[FEN "bbnnrkrq/pppppppp/8/8/8/8/PPPPPPPP/RQBKRBNN w KQkq - 0 1"]

1. e4 e5 2. Nf3 Nd6 3. Bc4 Nc6 4. O-O O-O-O"#;

        let mut chess = WasmChess::new(None).unwrap();

        chess.load_pgn(pgn).expect("Pgn ok");

        pretty_assertions::assert_eq!(
            chess.fen(None),
            "bbkr2rq/pppp1ppp/2nn4/4p3/2B1P3/5N2/PPPP1PPP/RQB2RKN w - - 6 5".to_string()
        );
    }

    #[test]
    fn castles_from_dfrc_setup() {
        let pgn = r#"[Variant "From Position"]
[FEN "bbnnrkrq/pppppppp/8/8/8/8/PPPPPPPP/RQBKRBNN w KQkq - 0 1"]

1. e4"#;

        let fen_last = "bbkr2rq/pppp1ppp/2nn4/4p3/2B1P3/5N2/PPPP1PPP/RQB2RKN w - - 6 5".to_string();
        let mut chess = WasmChess::new(None).unwrap();

        chess.load_pgn(pgn).expect("Pgn ok");

        chess.make_move("e5").unwrap();
        chess.make_move("Nf3").unwrap();
        chess.make_move("Nd6").unwrap();
        chess.make_move("Bc4").unwrap();
        chess.make_move("Nc6").unwrap();

        // NOTE: not supported
        // TODO: add support ?
        // chess.make_move("o-o").unwrap();
        // chess.make_move("o-o-o").unwrap();
        // chess.make_move("0-0").unwrap();
        // chess.make_move("0-0-0").unwrap();

        chess.make_move("O-O").unwrap();
        chess.make_move("O-O-O").unwrap();

        pretty_assertions::assert_eq!(chess.fen(None), fen_last);
        chess.undo();
        chess.undo();

        // uci format castle
        chess.make_move("d1e1").unwrap();
        chess.make_move("f8e8").unwrap();

        pretty_assertions::assert_eq!(chess.fen(None), fen_last);
    }
}
