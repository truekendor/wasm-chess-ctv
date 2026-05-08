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
}
