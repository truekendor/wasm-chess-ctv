use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(tsify::Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct HeadersObj {
    pub headers_data: HashMap<String, String>,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct MoveVerbose {
    pub before: String,
    pub after: String,

    pub captured: Option<String>,
    pub color: AttackedBySide,

    pub piece: String,
    pub from: String,
    pub to: String,
    pub promotion: Option<String>,

    pub san: String,
    pub lan: String,

    pub is_en_passant: bool,

    // for now we do not distinguish between kingside and queenside castle
    pub is_castle: bool,
}

impl MoveVerbose {
    pub fn is_capture(&self) -> bool {
        return self.captured.is_some();
    }

    pub fn is_promotion(&self) -> bool {
        return self.promotion.is_some();
    }

    // pub fn is_big_pawn() -> bool {
    //     todo!()
    // }

    // pub fn is_null_move() -> bool {
    //     todo!()
    // }
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum AttackedBySide {
    W,
    B,
    Both,
}

#[derive(tsify::Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MoveObject {
    pub from: String,
    pub to: String,
    #[tsify(optional)]
    pub promotion: Option<String>,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct CommentsObj {
    pub fen: String,
    #[tsify(optional)]
    pub comment: Option<String>,
    #[tsify(optional)]
    pub suffix_annotation: Option<String>,
    pub nags: Vec<u32>,
}
