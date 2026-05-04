use ordermap::OrderMap;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(tsify::Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct HeadersObj {
    #[tsify(type = "Map<string, string>")]
    pub headers_data: OrderMap<String, String>,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct MoveVerbose {
    pub from: String,
    pub to: String,

    /// fen before move is played
    pub before: String,
    /// fen after move is played
    pub after: String,

    pub color: ColorChar,
    pub piece: String,
    pub captured: Option<String>,

    pub promotion: Option<String>,

    pub san: String,
    pub lan: String,

    pub is_en_passant: bool,
    // for now we do not distinguish between kingside and queenside castle
    pub is_castle: bool,
    // TODO: add `is_kingside_castle` and `is_queenside_castle`
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

// copy of a shakmaty's Square enum for tsify types
#[rustfmt::skip]
#[derive(tsify::Tsify, Serialize, Deserialize, Debug, Display, EnumString, IntoStaticStr, PartialEq, Eq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum SquareStr {
    A1 = 1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub enum ColorChar {
    W,
    B,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug)]
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
#[derive(tsify::Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub enum SuffixSymbol {
    Exclam,
    Question,
    DoubleExclam,
    ExclamQuestion,
    QuestionExclam,
    DoubleQuestion,
}

impl SuffixSymbol {
    fn as_str(&self) -> &'static str {
        match self {
            SuffixSymbol::Exclam => "!",
            SuffixSymbol::Question => "?",
            SuffixSymbol::DoubleExclam => "!!",
            SuffixSymbol::ExclamQuestion => "!?",
            SuffixSymbol::QuestionExclam => "?!",
            SuffixSymbol::DoubleQuestion => "??",
        }
    }

    pub fn from_str(str: &str) -> Option<SuffixSymbol> {
        match str {
            "!" | "$1" => Some(SuffixSymbol::Exclam),
            "?" | "$2" => Some(SuffixSymbol::Question),
            "!!" | "$3" => Some(SuffixSymbol::DoubleExclam),
            "??" | "$4" => Some(SuffixSymbol::DoubleQuestion),
            "!?" | "$5" => Some(SuffixSymbol::ExclamQuestion),
            "?!" | "$6" => Some(SuffixSymbol::QuestionExclam),
            _ => None,
        }
    }

    pub fn str_is_valid_suffix(str: &str) -> bool {
        match str {
            "!" | "$1" | "?" | "$2" | "!!" | "$3" | "??" | "$4" | "!?" | "$5" | "?!" | "$6" => true,
            _ => false,
        }
    }
}
