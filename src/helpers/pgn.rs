use shakmaty::san::San;

use crate::{WasmChess, tsify_structs::others::ColorChar};

pub fn chess_to_pgn(wasm_chess: &WasmChess) -> String {
    let mut pgn = String::new();

    // Headers
    if let Some(pgn_result) = &wasm_chess.pgn_result {
        for (key, value) in &pgn_result.headers {
            pgn.push_str(&format!("[{key} \"{value}\"]\n"));
        }

        if !pgn_result.headers.is_empty() {
            pgn.push('\n');
        }
    }

    // Initial position comment
    if let Some(pgn_result) = &wasm_chess.pgn_result {
        let initial_fen = if wasm_chess.history.is_empty() {
            wasm_chess.fen(None)
        } else {
            wasm_chess.history[0].fen_before.clone().to_string()
        };

        if let Some(comment) = pgn_result.comments_map.get(&initial_fen) {
            pgn.push_str(&format!("{{{comment}}} "));
        }
    }

    // Moves
    for (index, entry) in wasm_chess.history.iter().enumerate() {
        let move_number = (index / 2) + 1;

        // White move
        if index % 2 == 0 {
            pgn.push_str(&format!("{move_number}. "));
        }
        // Black move at beginning variation style position
        else if index == 1 {
            pgn.push_str(&format!("{move_number}... "));
        }

        let san = San::from_move(&entry.position_before, entry.raw_move);
        let fen_after = entry.fen_after.to_string();

        pgn.push_str(&san.to_string());

        if let Some(pgn_result) = &wasm_chess.pgn_result {
            // NAGs
            if let Some(nags) = pgn_result.nag_map.get(&fen_after) {
                for nag in nags {
                    pgn.push(' ');
                    pgn.push_str(nag);
                }
            }

            // Suffix annotation
            if let Some(suffix) = pgn_result.suffix_map.get(&fen_after) {
                pgn.push(' ');
                pgn.push_str(suffix);
            }

            // Comment
            if let Some(comment) = pgn_result.comments_map.get(&fen_after) {
                pgn.push(' ');
                pgn.push_str(&format!("{{{comment}}}"));
            }
        }

        pgn.push(' ');
    }

    // Result
    let result = if wasm_chess.is_checkmate() {
        match wasm_chess.turn() {
            ColorChar::W => "0-1",
            ColorChar::B => "1-0",
        }
    } else if wasm_chess.is_draw() {
        "1/2-1/2"
    } else {
        "*"
    };

    pgn.push_str(result);

    pgn.trim().to_string()
}
