use super::*;

#[wasm_bindgen]
impl WasmChess {
    // TODO: make static/move to some other mod?
    // TODO: add js_name
    #[wasm_bindgen(js_name = "validateFen")]
    pub fn validate_fen(&self, fen: String) -> OkOrError<bool> {
        match fen.parse::<Fen>() {
            Ok(_) => OkOrError {
                ok: true,
                err: None,
            },
            Err(err) => OkOrError {
                ok: false,
                err: Some(err.to_string()),
            },
        }
    }
}
