use ordermap::OrderMap;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::tsify_structs::square_str::SquareStr;

#[derive(tsify::Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct HeadersObj {
    #[tsify(type = "Map<string, string>")]
    pub headers_data: OrderMap<String, String>,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq, Display)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub enum SquareColor {
    Light,
    Dark,
}

#[derive(tsify::Tsify, Serialize, Deserialize, PartialEq, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MoveObject {
    pub from: SquareStr,
    pub to: SquareStr,
    #[tsify(optional)]
    pub promotion: Option<String>,
}

#[derive(tsify::Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MoveFromSquares {
    pub from: SquareStr,
    pub to: SquareStr,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct CommentsObj {
    pub fen: String,
    #[tsify(optional)]
    pub comment: Option<String>,
    #[tsify(optional)]
    pub suffix_annotation: Option<String>,
    pub nags: Vec<String>,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct PrunedCommentsObj {
    pub fen: String,
    pub comment: String,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CastlingObj {
    pub king: bool,
    pub queen: bool,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub enum ColorChar {
    W,
    B,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub enum PieceSymbol {
    P,
    N,
    B,
    R,
    Q,
    K,
}

impl PieceSymbol {
    pub fn from_shakmaty_piece(piece: &shakmaty::Piece) -> Self {
        match piece.role {
            shakmaty::Role::Pawn => PieceSymbol::P,
            shakmaty::Role::Knight => PieceSymbol::N,
            shakmaty::Role::Bishop => PieceSymbol::B,
            shakmaty::Role::Rook => PieceSymbol::R,
            shakmaty::Role::Queen => PieceSymbol::Q,
            shakmaty::Role::King => PieceSymbol::K,
        }
    }
}

#[derive(tsify::Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PieceObj {
    pub r#type: PieceSymbol,
    pub color: ColorChar,
}

// this is like a custom result
#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct OkOrError<T> {
    pub ok: Option<T>,
    pub err: Option<String>,
}

// TODO: use it later

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct SquareInfoObj {
    pub square: SquareStr,
    pub r#type: PieceSymbol,
    pub color: ColorChar,
}
