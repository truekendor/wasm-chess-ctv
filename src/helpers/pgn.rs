use shakmaty::{Position, san::San};

use crate::{
    WasmChess,
    helpers::{parsing::san_to_san_plus, pgn_reader::PGNResult},
    tsify_structs::others::PGNOptions,
};

pub fn chess_to_pgn(wasm_chess: &mut WasmChess, options: PGNOptions) -> String {
    let newline = options.newline.unwrap_or("\n".to_string());
    let newline_char = newline.as_str();
    let max_line_width = options.max_width.unwrap_or(0);

    let mut move_chunks: Vec<String> = Vec::new();
    let mut header_string = String::new();

    let initial_fen = if wasm_chess.history.is_empty() {
        wasm_chess.fen(None)
    } else {
        wasm_chess.history[0].fen_before.clone().to_string()
    };

    let pgn_result = wasm_chess.pgn_result.get_or_insert_with(PGNResult::default);

    for (key, value) in &pgn_result.headers {
        header_string.push_str(&format!("[{key} \"{value}\"]{newline_char}"));
    }

    if !pgn_result.headers.is_empty() {
        header_string.push_str(newline_char);
    }

    if let Some(comment) = pgn_result.comments_map.get(&initial_fen) {
        move_chunks.push(format!("{{{comment}}}"));
    }

    for (index, history_entry) in wasm_chess.history.iter().enumerate() {
        let move_number = history_entry.position_before.fullmoves();

        let san = San::from_move(&history_entry.position_before, history_entry.raw_move);
        let san_plus = san_to_san_plus(&san, &history_entry.position_after);
        let mut move_text = san_plus;

        let prefix = if index == 0 {
            match history_entry.turn {
                shakmaty::Color::White => format!("{move_number}."),
                shakmaty::Color::Black => format!("{move_number}..."),
            }
        } else if index % 2 == 0 {
            format!("{move_number}.")
        } else {
            String::new()
        };

        if prefix.is_empty() {
            if let Some(last) = move_chunks.last_mut() {
                last.push(' ');
                last.push_str(&move_text);
            }
        } else {
            move_chunks.push(format!("{prefix} {move_text}"));
        }

        let fen_after = history_entry.fen_after.to_string();

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

        if let Some(comment) = pgn_result.comments_map.get(&fen_after) {
            let comment = comment.split_whitespace().collect::<Vec<_>>().join(" ");
            move_chunks.push(format!("{{{comment}}}"));
        }
    }

    if let Some(pgn_result) = wasm_chess.pgn_result.as_ref() {
        let result = match pgn_result.known_outcome {
            Some(shakmaty::KnownOutcome::Decisive { winner }) => match winner {
                shakmaty::Color::White => "1-0",
                shakmaty::Color::Black => "0-1",
            },
            Some(shakmaty::KnownOutcome::Draw) => "1/2-1/2",
            None => pgn_result
                .headers
                .get("Result")
                .map(|s| s.as_str())
                .unwrap_or("*"),
        };

        move_chunks.push(result.to_string());
    }

    // No wrapping
    if max_line_width == 0 {
        return format!("{header_string}{}", move_chunks.join(" "))
            .trim()
            .to_string();
    }

    let mut result_string = header_string;
    let mut current_line_width = 0;

    for token in move_chunks {
        let token_len = token.chars().count();

        let required_width = if current_line_width == 0 {
            token_len
        } else {
            1 + token_len
        };

        if current_line_width > 0 && ((current_line_width + required_width) > max_line_width) {
            result_string.push_str(newline_char);
            result_string.push_str(&token);
            current_line_width = token_len;
        } else {
            if current_line_width > 0 {
                result_string.push(' ');
                current_line_width += 1;
            }

            result_string.push_str(&token);
            current_line_width += token_len;
        }
    }

    result_string.trim().to_string()
}
