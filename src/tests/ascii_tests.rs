#[cfg(test)]
mod ascii_tests {
    use crate::WasmChess;

    #[test]
    fn ascii_ok_default() {
        let wasm_chess = WasmChess::new(None).unwrap();

        let expected_output = vec![
            "   +------------------------+",
            " 8 | r  n  b  q  k  b  n  r |",
            " 7 | p  p  p  p  p  p  p  p |",
            " 6 | .  .  .  .  .  .  .  . |",
            " 5 | .  .  .  .  .  .  .  . |",
            " 4 | .  .  .  .  .  .  .  . |",
            " 3 | .  .  .  .  .  .  .  . |",
            " 2 | P  P  P  P  P  P  P  P |",
            " 1 | R  N  B  Q  K  B  N  R |",
            "   +------------------------+",
            "     a  b  c  d  e  f  g  h",
        ];

        pretty_assertions::assert_eq!(wasm_chess.ascii(), expected_output.join("\n"));
    }

    /// test taken from chess.js ascii test suite
    /// to assure that the output is consistent with chess.js
    ///
    /// @link https://github.com/jhlywa/chess.js/blob/master/__tests__/ascii.test.ts
    #[test]
    fn ascii_ok_set_position() {
        let fen = "r4rk1/4nqpp/1p1p4/2pPpp2/bPP1P3/R1B1NQ2/P4PPP/1R4K1 w - - 0 28".to_string();
        let wasm_chess = WasmChess::new(Some(fen)).unwrap();

        let expected_output = vec![
            "   +------------------------+",
            " 8 | r  .  .  .  .  r  k  . |",
            " 7 | .  .  .  .  n  q  p  p |",
            " 6 | .  p  .  p  .  .  .  . |",
            " 5 | .  .  p  P  p  p  .  . |",
            " 4 | b  P  P  .  P  .  .  . |",
            " 3 | R  .  B  .  N  Q  .  . |",
            " 2 | P  .  .  .  .  P  P  P |",
            " 1 | .  R  .  .  .  .  K  . |",
            "   +------------------------+",
            "     a  b  c  d  e  f  g  h",
        ];

        pretty_assertions::assert_eq!(wasm_chess.ascii(), expected_output.join("\n"));
    }
}
