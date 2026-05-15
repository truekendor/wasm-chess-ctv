use std::io::{self};

use pgn_reader::Reader;

use super::*;

#[wasm_bindgen]
impl WasmChess {
    #[wasm_bindgen(js_name = "loadPgn")]
    pub fn load_pgn(&mut self, pgn: &str) -> Result<(), String> {
        let mut reader = Reader::new(io::Cursor::new(pgn));

        reader
            .read_game(self)
            .map_err(|err| err.to_string())?
            .unwrap_or(Ok(()))
    }

    #[wasm_bindgen(js_name = "pgn")]
    pub fn pgn(&mut self, options: Option<PGNOptions>) -> String {
        let options = options.unwrap_or_else(|| PGNOptions {
            max_width: None,
            newline: None,
        });

        chess_to_pgn(self, options)
    }
}
