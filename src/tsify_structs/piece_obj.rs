use serde::{Deserialize, Serialize};
use shakmaty::{Color, Piece};

use crate::tsify_structs::{PieceSymbol, others::ColorChar};

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PieceObj {
    pub r#type: PieceSymbol,
    pub color: ColorChar,
}

impl PieceObj {
    pub fn to_shakmaty_piece(&self) -> Piece {
        Piece {
            color: match self.color {
                ColorChar::W => Color::White,
                ColorChar::B => Color::Black,
            },
            role: match self.r#type {
                PieceSymbol::P => shakmaty::Role::Pawn,
                PieceSymbol::N => shakmaty::Role::Knight,
                PieceSymbol::B => shakmaty::Role::Bishop,
                PieceSymbol::R => shakmaty::Role::Rook,
                PieceSymbol::Q => shakmaty::Role::Queen,
                PieceSymbol::K => shakmaty::Role::King,
            },
        }
    }

    pub fn from_shakmaty_piece(piece: &Piece) -> Self {
        PieceObj {
            color: match piece.color {
                Color::White => ColorChar::W,
                Color::Black => ColorChar::B,
            },
            r#type: PieceSymbol::from_shakmaty_piece_role(&piece.role),
        }
    }
}
