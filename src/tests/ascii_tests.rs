// TODO: actually write some tests bruh
#[cfg(test)]
mod askii_tests {
    use crate::WasmChess;

    fn ascii_ok() {
        let mut wasm_chess = WasmChess::new(None).unwrap();

        print!("{}", wasm_chess.ascii());
    }
}
