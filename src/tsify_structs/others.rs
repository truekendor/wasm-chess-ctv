use ordermap::OrderMap;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::tsify_structs::{PieceSymbol, square_str::SquareStr};

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
    pub promotion: Option<PieceSymbol>,
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

// this is like a custom result
#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct OkOrError<T> {
    #[tsify(type = "T")]
    pub ok: T,
    pub err: Option<String>,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct SquareInfoObj {
    pub square: SquareStr,
    pub r#type: PieceSymbol,
    pub color: ColorChar,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct PreserveHeaders {
    pub preserve_headers: bool,
}
