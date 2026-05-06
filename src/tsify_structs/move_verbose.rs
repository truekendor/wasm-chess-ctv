use serde::{Deserialize, Serialize};

use crate::tsify_structs::{PieceSymbol, SquareStr, others::ColorChar};

/// A verbose representation of a chess move, exposed to TypeScript as a plain object.
///
/// # API Differences from chess.js
///
/// Chess.js provides methods (`isCapture()`, `isEnPassant()`) while this port exposes
/// boolean fields directly on the object.
///
/// **Capture Detection:**
/// - `captured` (string | null | undefined): The piece that was taken (includes both regular and en passant captures)
/// - `isRegularCapture` (boolean): Matches chess.js `isCapture()` behavior exactly
///
/// ```typescript
/// if (move.captured) {
///     // A piece was captured (regular or en passant)
/// }
///
/// if (move.isRegularCapture) {
///     // Matches chess.js isCapture() - false for en passant
/// }
///
/// if (move.isEnPassant) {
///     // True only for en passant captures
/// }
/// ```
///
/// **Other fields directly mirror chess.js verbose move properties:**
/// - `san`, `lan`, `piece`, `color`, `promotion`
/// - `isBigPawn`, `isCastle`, `isKingsideCastle`, `isQueensideCastle`
/// - `from`, `to`, `before`, `after`

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct MoveVerbose {
    pub from: SquareStr,
    pub to: SquareStr,

    /// FEN before the move is played
    pub before: String,
    /// FEN after the move is played
    pub after: String,

    pub color: ColorChar,
    pub piece: PieceSymbol,
    pub captured: Option<PieceSymbol>,
    /// Matches chess.js `isCapture()` behavior.
    ///
    /// Returns `true` for regular captures (piece moves onto opponent-occupied square).
    /// Returns `false` for en passant captures, unlike checking `captured` which includes them.
    ///
    /// For exact chess.js `isCapture()` compatibility, use this field.
    pub is_regular_capture: bool,

    pub promotion: Option<String>,

    pub san: String,
    pub lan: String,

    pub is_en_passant: bool,
    /// Returns `true` for two-square pawn moves (e.g., e2e4)
    pub is_big_pawn: bool,

    pub is_castle: bool,
    pub is_kingside_castle: bool,
    pub is_queenside_castle: bool,
}
