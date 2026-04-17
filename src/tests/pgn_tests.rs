// TODO these are more or less mock test cases
// TODO need to port chess.jss test cases here

#[cfg(test)]
mod pgn_test {
    static TEST_PGN_1: &str = r#"[Event "F/S Return Match"]
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

    use pgn_reader::Reader;

    use crate::{WasmChess, pgn_loader::pgn_reader::PGNHeaders};

    use std::{
        fs::{self},
        io,
    };

    #[test]
    // TODO: make an actual test
    fn headers() {
        let mut reader = Reader::new(io::Cursor::new(TEST_PGN_1));
        let mut pgn_parser = PGNHeaders::default();
        reader.read_game(&mut pgn_parser).unwrap();

        assert_eq!(*pgn_parser.headers.get("Round").unwrap(), "29".to_owned());
        assert!(pgn_parser.headers.contains_key("Site"));
        assert!(pgn_parser.headers.contains_key("White"));
        assert!(pgn_parser.headers.contains_key("Black"));
        assert_eq!(pgn_parser.move_list.len(), 85);

        assert!(!pgn_parser.headers.contains_key("Variant"));
    }

    #[test]
    fn load_and_read_valid_pgn() {
        let pgn = fs::read("./src/tests/pgn/1.pgn").unwrap();

        let mut reader: Reader<io::Cursor<Vec<u8>>> = Reader::new(io::Cursor::new(pgn));
        let mut pgn_headers = PGNHeaders::default();

        reader.read_game(&mut pgn_headers).unwrap();
    }

    #[test]
    fn pgn_loads_correctly() {
        let pgn = fs::read("./src/tests/pgn/1.pgn").unwrap();

        let mut reader: Reader<io::Cursor<Vec<u8>>> = Reader::new(io::Cursor::new(pgn));
        let mut pgn_headers = PGNHeaders::default();

        reader.read_game(&mut pgn_headers).unwrap();

        let starting_fen = pgn_headers.starting_fen;

        let mut wasm_chess = WasmChess::new(Some(starting_fen.to_string())).unwrap();

        pgn_headers.move_list.iter().for_each(|el| {
            // TODO add error handling
            wasm_chess
                .make_move(el)
                .expect("Unexpected panic on valid FEN");
        });

        assert_eq!(
            starting_fen.to_string(),
            "nqbrkbrn/pppppp1p/6p1/8/7P/8/PPPPPPP1/BBNRNKRQ w KQkq - 0 2"
        );

        // TODO:
        wasm_chess.history_san().unwrap().iter().for_each(|m| {
            // println!("inner move: {}", m);
        });
    }

    #[test]
    fn begins_on_correct_turn_w() {
        let pgn = r#"1. e4 e5"#;

        let mut wasm_chess = WasmChess::new(None).unwrap();

        wasm_chess.load_pgn(pgn.to_owned()).unwrap();
        let turn = wasm_chess.turn();
        assert_eq!(turn, "w");
    }

    #[test]
    fn begins_on_correct_turn_b() {
        let pgn = r#"1. e4"#;

        let mut wasm_chess = WasmChess::new(None).unwrap();

        wasm_chess.load_pgn(pgn.to_owned()).unwrap();
        let turn = wasm_chess.turn();
        assert_eq!(turn, "b");
    }

    #[test]
    fn begins_on_correct_turn_w_empty() {
        let pgn = r#"  
[White "LichessAborter"]
[Black "PoorSap"]

*"#;

        let mut wasm_chess = WasmChess::new(None).unwrap();

        wasm_chess.load_pgn(pgn.to_owned()).unwrap();
        let turn = wasm_chess.turn();
        assert_eq!(turn, "w");
    }

    // #[test]
    // TODO:
    fn remove_header_works() {
        let mut wasm_chess = WasmChess::new(None).unwrap();
        wasm_chess.load_pgn(TEST_PGN_1.to_owned()).unwrap();
    }
}
