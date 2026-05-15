use super::*;

#[wasm_bindgen]
impl WasmChess {
    #[wasm_bindgen(js_name = "getHeaders")]
    pub fn get_headers(&mut self) -> HeadersObj {
        self.populate_seven_tag_roster();
        let pgn_result = self.pgn_result.get_or_insert_with(|| PGNResult::default());

        pgn_result.reorder_headers();

        HeadersObj {
            headers_data: pgn_result.headers.clone(),
        }
    }

    #[wasm_bindgen(js_name = "setHeader")]
    pub fn set_header(&mut self, key: String, value: String) -> HeadersObj {
        self.populate_seven_tag_roster();

        let pgn_result = self.pgn_result.get_or_insert_with(PGNResult::default);

        pgn_result.headers.insert(key, value);

        self.get_headers()
    }

    #[wasm_bindgen(js_name = "removeHeader")]
    pub fn remove_header(&mut self, key: String) -> bool {
        self.pgn_result
            .as_mut()
            .map(|pgn| {
                if let Some(val) = self.seven_tag_roster.get(&key.clone().as_str()) {
                    pgn.headers.insert(key, val.to_string());

                    return true;
                }
                pgn.headers.remove(&key).is_some()
            })
            .unwrap_or(false)
    }

    fn populate_seven_tag_roster(&mut self) {
        let pgn_result = self.pgn_result.get_or_insert_with(PGNResult::default);

        if pgn_result.headers.len() < 1 {
            self.seven_tag_roster.iter().for_each(|(key, val)| {
                pgn_result
                    .headers
                    .entry(key.to_string())
                    .or_insert(val.to_string());
            });
        }
    }
}
