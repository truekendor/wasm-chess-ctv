use serde::{Deserialize, Serialize};
use shakmaty::Role;

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

    pub fn from_shakmaty_piece_role(role: &Role) -> Self {
        match role {
            Role::Pawn => PieceSymbol::P,
            Role::Knight => PieceSymbol::N,
            Role::Bishop => PieceSymbol::B,
            Role::Rook => PieceSymbol::R,
            Role::Queen => PieceSymbol::Q,
            Role::King => PieceSymbol::K,
        }
    }

    pub fn from_str(str: &str) -> Option<Self> {
        match str {
            "p" => Some(PieceSymbol::P),
            "n" => Some(PieceSymbol::N),
            "b" => Some(PieceSymbol::B),
            "r" => Some(PieceSymbol::R),
            "q" => Some(PieceSymbol::Q),
            "k" => Some(PieceSymbol::K),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PieceSymbol::P => "p",
            PieceSymbol::N => "n",
            PieceSymbol::B => "b",
            PieceSymbol::R => "r",
            PieceSymbol::Q => "q",
            PieceSymbol::K => "k",
        }
    }
}
