use shakmaty::{Chess, Color, Piece, Position, Square, san::San, uci::UciMove};
use wasm_bindgen::prelude::wasm_bindgen;

// TODO
#[wasm_bindgen]
pub struct StrictMove {
    from: Square,
    to: Square,
    color: Color,
    piece: Piece,
    captured: Option<Piece>,
    promotion: Option<Piece>,
}

pub fn uci(chess: &Chess) -> Vec<String> {
    let legal_moves: Vec<String> = chess
        .legal_moves()
        .iter()
        .map(|el| {
            let uci_move = UciMove::from_move(*el, shakmaty::CastlingMode::Chess960);
            return uci_move.to_string();
        })
        .collect();

    legal_moves
}

pub fn san(chess: &Chess) -> Vec<String> {
    let legal_moves: Vec<String> = chess
        .legal_moves()
        .iter()
        .map(|el| {
            let san_move = San::from_move(chess, *el);

            return san_move.to_string();
        })
        .collect();

    legal_moves
}

pub fn strict(chess: &Chess) -> Vec<StrictMove> {
    todo!()
}
