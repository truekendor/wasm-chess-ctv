use super::*;

#[wasm_bindgen]
impl WasmChess {
    // TODO: add custom new line char to params
    #[wasm_bindgen(js_name = "loadPgn")]
    pub fn load_pgn(&mut self, pgn: &str) -> Result<(), String> {
        let pgn_headers = helpers::pgn_reader::parse_pgn(pgn);

        let (pgn_result, wasm_chess) =
            pgn_headers.map_err(|err| format!("Error loading pgn: {}", err))?;

        self.chess = wasm_chess.chess;
        self.hash = wasm_chess.hash;
        self.history = wasm_chess.history;
        self.repetition_table = wasm_chess.repetition_table;

        self.pgn_result = Some(pgn_result);

        return Ok(());
    }

    #[wasm_bindgen(js_name = "pgn")]
    pub fn pgn(&mut self, options: Option<PGNOptions>) -> String {
        let options = options.unwrap_or_else(|| PGNOptions {
            max_width: Some(0),
            newline: Some("\n".to_string()),
        });

        chess_to_pgn(self, options)
    }
}
