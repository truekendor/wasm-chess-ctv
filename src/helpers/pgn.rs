use shakmaty::{Position, san::San};

use crate::{
    WasmChess,
    helpers::pgn_reader::PGNResult,
    tsify_structs::others::{ColorChar, PGNOptions},
};

pub fn chess_to_pgn(wasm_chess: &mut WasmChess, options: PGNOptions) -> String {
    let newline = options.newline.unwrap_or("\n".to_string());
    let newline = newline.as_str();
    let max_width = options.max_width.unwrap_or(0);

    let mut tokens: Vec<String> = Vec::new();
    let mut header_string = String::new();

    // TODO: rewtite to `let pgn_result = &wasm_chess.pgn_result`

    let initial_fen = if wasm_chess.history.is_empty() {
        wasm_chess.fen(None)
    } else {
        wasm_chess.history[0].fen_before.clone().to_string()
    };

    let pgn_result = wasm_chess.pgn_result.get_or_insert_with(PGNResult::default);

    for (key, value) in &pgn_result.headers {
        header_string.push_str(&format!("[{key} \"{value}\"]{newline}"));
    }

    if !pgn_result.headers.is_empty() {
        header_string.push_str(newline);
    }

    if let Some(comment) = pgn_result.comments_map.get(&initial_fen) {
        tokens.push(format!("{{{comment}}}"));
    }

    // Moves
    for (index, history_entry) in wasm_chess.history.iter().enumerate() {
        let move_number = history_entry.position_before.fullmoves();

        if index == 0 {
            match history_entry.turn {
                shakmaty::Color::White => tokens.push(format!("{move_number}.")),
                shakmaty::Color::Black => tokens.push(format!("{move_number}...")),
            };
        } else if index % 2 == 0 {
            tokens.push(format!("{move_number}."));
        };

        let san = San::from_move(&history_entry.position_before, history_entry.raw_move);
        let fen_after = history_entry.fen_after.to_string();

        let mut move_text = san.to_string();

        if history_entry.position_after.is_checkmate() {
            move_text.push_str("#");
        } else if history_entry.position_after.is_check() {
            move_text.push_str("+");
        }

        // NAGs
        if let Some(nags) = pgn_result.nag_map.get(&fen_after) {
            for nag in nags {
                move_text.push(' ');
                move_text.push_str(nag);
            }
        }

        // Suffix annotation
        if let Some(suffix) = pgn_result.suffix_map.get(&fen_after) {
            move_text.push(' ');
            move_text.push_str(suffix);
        }

        tokens.push(move_text);

        if let Some(comment) = pgn_result.comments_map.get(&fen_after) {
            let comment = comment.replace("\n", " ");
            // let comment = comment.trim();
            tokens.push(format!("{{{comment}}}"));
        }
    }

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

    tokens.push(result.to_string());

    // No wrapping
    if max_width == 0 {
        return format!("{header_string}{}", tokens.join(" "))
            .trim()
            .to_string();
    }

    // Wrap lines like chess.js
    let mut result_string = header_string;
    let mut current_line_width = 0;

    for token in tokens {
        let token_len = token.chars().count();

        // if current_line_width == 0 {
        //     result_string.push_str(&token);
        //     current_line_width = token_len;
        // } else
        if current_line_width + token_len > max_width {
            result_string.push_str(newline);
            result_string.push_str(&token);
            current_line_width = token_len;
            continue;
        }

        if current_line_width == 0 {
            result_string.push_str(&token);
            current_line_width = token_len;
        } else {
            result_string.push(' ');
            result_string.push_str(&token);
            current_line_width += 1 + token_len;
        }
    }

    result_string.trim().to_string()
}
