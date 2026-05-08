#[cfg(test)]
pub mod header_tests {
    use ordermap::OrderMap;

    use crate::{WasmChess, tsify_structs::others::HeadersObj};

    #[test]
    fn set_header_should_add_or_update_header() {
        let mut chess = WasmChess::new(None).unwrap();

        chess.set_header("Event".to_string(), "Test Event".to_string());
        chess.set_header("Site".to_string(), "Test Site".to_string());

        let answer = HeadersObj {
            headers_data: OrderMap::from([
                ("Event".to_string(), "Test Event".to_string()),
                ("Site".to_string(), "Test Site".to_string()),
                ("Date".to_string(), "????.??.??".to_string()),
                ("Round".to_string(), "?".to_string()),
                ("White".to_string(), "?".to_string()),
                ("Black".to_string(), "?".to_string()),
                ("Result".to_string(), "*".to_string()),
            ]),
        };

        let headers = chess.get_headers().headers_data;

        pretty_assertions::assert_eq!(headers, answer.headers_data);
    }

    #[test]
    fn remove_header_should_remove_if_exists() {
        let mut chess = WasmChess::new(None).unwrap();
        chess.set_header("Event".to_string(), "Test Event".to_string());
        chess.set_header("Site".to_string(), "Test Site".to_string());

        let removed = chess.remove_header("Event".to_string());
        assert!(removed);

        let answer = HeadersObj {
            headers_data: OrderMap::from([
                ("Event".to_string(), "?".to_string()),
                ("Site".to_string(), "Test Site".to_string()),
                ("Date".to_string(), "????.??.??".to_string()),
                ("Round".to_string(), "?".to_string()),
                ("White".to_string(), "?".to_string()),
                ("Black".to_string(), "?".to_string()),
                ("Result".to_string(), "*".to_string()),
            ]),
        };

        let headers = chess.get_headers().headers_data;

        pretty_assertions::assert_eq!(headers, answer.headers_data);

        let non_existent = chess.remove_header("NonExistent".to_string());

        assert!(!non_existent);
    }

    // we do not have "null" headers so just skip that part
    #[test]
    fn get_headers_returns_custom_headers() {
        let mut chess = WasmChess::new(None).unwrap();

        chess.set_header("Event".to_string(), "Test Event".to_string());
        chess.set_header("Site".to_string(), "Test Site".to_string());
        chess.set_header("Opening".to_string(), "Santasiere's Folly".to_string());

        let answer = HeadersObj {
            headers_data: OrderMap::from([
                ("Event".to_string(), "Test Event".to_string()),
                ("Site".to_string(), "Test Site".to_string()),
                ("Date".to_string(), "????.??.??".to_string()),
                ("Round".to_string(), "?".to_string()),
                ("White".to_string(), "?".to_string()),
                ("Black".to_string(), "?".to_string()),
                ("Result".to_string(), "*".to_string()),
                ("Opening".to_string(), "Santasiere's Folly".to_string()),
            ]),
        };

        let headers = chess.get_headers().headers_data;

        pretty_assertions::assert_eq!(headers, answer.headers_data);
    }

    #[test]
    fn get_headers_lots_of_custom_before_seven_tag_roster() {
        let pgn = r#"
[Custom1 "Custom1"]
[Custom2 "Custom2"]
[Custom3 "Custom3"]
[Custom4 "Custom4"]
[Custom5 "Custom5"]
[Custom6 "Custom6"]
[Custom7 "Custom7"]
[Custom8 "Custom8"]
[Custom9 "Custom9"]

[Event "CCC 25 Double-Fischer: Finals"]
[Site "https://www.chess.com/computer-chess-championship"]
[Date "2026.03.27"]
[Round "19"]
[White "Reckless"]
[Black "Stockfish"]
[Result "1/2-1/2"]
[FEN "nqbrkbrn/pppppp1p/6p1/8/7P/8/PPPPPPP1/BBNRNKRQ w GDgd - 0 2"]
[SetUp "1"]


"#;

        let mut chess = WasmChess::new(None).unwrap();

        chess.load_pgn(pgn).unwrap();

        let answer = HeadersObj {
            headers_data: OrderMap::from([
                (
                    "Event".to_string(),
                    "CCC 25 Double-Fischer: Finals".to_string(),
                ),
                (
                    "Site".to_string(),
                    "https://www.chess.com/computer-chess-championship".to_string(),
                ),
                ("Date".to_string(), "2026.03.27".to_string()),
                ("Round".to_string(), "19".to_string()),
                ("White".to_string(), "Reckless".to_string()),
                ("Black".to_string(), "Stockfish".to_string()),
                ("Result".to_string(), "1/2-1/2".to_string()),
                //
                ("Custom1".to_string(), "Custom1".to_string()),
                ("Custom2".to_string(), "Custom2".to_string()),
                ("Custom3".to_string(), "Custom3".to_string()),
                ("Custom4".to_string(), "Custom4".to_string()),
                ("Custom5".to_string(), "Custom5".to_string()),
                ("Custom6".to_string(), "Custom6".to_string()),
                ("Custom7".to_string(), "Custom7".to_string()),
                ("Custom8".to_string(), "Custom8".to_string()),
                ("Custom9".to_string(), "Custom9".to_string()),
                (
                    "FEN".to_string(),
                    "nqbrkbrn/pppppp1p/6p1/8/7P/8/PPPPPPP1/BBNRNKRQ w GDgd - 0 2".to_string(),
                ),
                ("SetUp".to_string(), "1".to_string()),
            ]),
        };

        let headers = chess.get_headers().headers_data;

        pretty_assertions::assert_eq!(headers, answer.headers_data);
    }
}
