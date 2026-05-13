use super::*;

#[wasm_bindgen]
impl WasmChess {
    // TODO: add tests for nags ?
    #[wasm_bindgen(js_name = "getNags")]
    pub fn get_nags(&self, fen: Option<FenString>) -> Vec<String> {
        let Some(pgn_result) = self.pgn_result.as_ref() else {
            return vec![];
        };

        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        pgn_result
            .nag_map
            .get(&fen_key)
            .cloned()
            .unwrap_or_else(Vec::new)
    }

    #[wasm_bindgen(js_name = "addNag")]
    pub fn add_nag(&mut self, nag: &str, fen: Option<FenString>) {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        let Some(pgn_result) = self.pgn_result.as_mut() else {
            return ();
        };

        let nags = pgn_result.nag_map.entry(fen_key.clone()).or_insert(vec![]);

        if !nags.contains(&fen_key) {
            nags.push(nag.to_string());
        }
    }

    #[wasm_bindgen(js_name = "setNags")]
    pub fn set_nags(&mut self, nags: Vec<String>, fen: Option<FenString>) {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        let Some(pgn_result) = self.pgn_result.as_mut() else {
            return ();
        };

        let _ = pgn_result.nag_map.insert(fen_key, nags);
    }

    #[wasm_bindgen(js_name = "removeNag")]
    pub fn remove_nag(&mut self, nag: String, fen: Option<FenString>) -> bool {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        let Some(pgn_result) = self.pgn_result.as_mut() else {
            return false;
        };

        let Some(nags) = pgn_result.nag_map.get_mut(&fen_key) else {
            return false;
        };

        let Some(index) = nags.iter().position(|el| el == &nag) else {
            return false;
        };

        nags.remove(index);
        true
    }

    #[wasm_bindgen(js_name = "removeNags")]
    pub fn remove_nags(&mut self, fen: Option<FenString>) -> Vec<NAGString> {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        let Some(pgn_result) = self.pgn_result.as_mut() else {
            return vec![];
        };
        let removed = pgn_result.nag_map.remove(&fen_key);

        removed.unwrap_or_else(|| Vec::new())
    }

    #[wasm_bindgen(js_name = "getSuffixAnnotation")]
    pub fn get_suffix_annotation(&self, fen: Option<FenString>) -> Option<SuffixString> {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        let Some(pgn_result) = self.pgn_result.as_ref() else {
            return None;
        };

        pgn_result.suffix_map.get(&fen_key).cloned()
    }

    // TODO: add custom types like type Suffix = String to avoid confusion
    #[wasm_bindgen(js_name = "setSuffixAnnotation")]
    pub fn set_suffix_annotation(
        &mut self,
        suffix: &str,
        fen: Option<FenString>,
    ) -> Result<(), String> {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        if !SuffixSymbol::str_is_valid_suffix(&suffix) {
            return Err(format!("Provided suffix is invalid: {}", suffix));
        };

        let pgn_result = self.pgn_result.get_or_insert_with(PGNResult::default);
        pgn_result.suffix_map.insert(fen_key, suffix.to_string());

        Ok(())
    }

    #[wasm_bindgen(js_name = "removeSuffixAnnotation")]
    pub fn remove_suffix_annotation(&mut self, fen: Option<FenString>) -> Option<SuffixString> {
        let fen_key = fen.unwrap_or_else(|| self.fen(None));

        let pgn_result = self.pgn_result.get_or_insert_with(PGNResult::default);

        pgn_result.suffix_map.remove(&fen_key)
    }
}
