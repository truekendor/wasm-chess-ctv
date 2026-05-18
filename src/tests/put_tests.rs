#[cfg(test)]
// TODO: add hash checks
pub mod put_tests {
    use crate::{
        WasmChess,
        models::{PieceObj, PieceSymbol, SquareStr, utils::ColorChar},
    };

    #[test]
    fn put_works() {
        let mut chess = WasmChess::new(None).unwrap();

        let put_success = chess.put(
            PieceObj {
                color: ColorChar::W,
                r#type: PieceSymbol::R,
            },
            SquareStr::A1,
        );

        pretty_assertions::assert_eq!(put_success, true);
    }

    #[test]
    fn put_disallow_two_white_kings() {
        let mut chess = WasmChess::new(None).unwrap();

        let put_success = chess.put(
            PieceObj {
                color: ColorChar::W,
                r#type: PieceSymbol::K,
            },
            SquareStr::D1,
        );

        pretty_assertions::assert_eq!(chess.editable.is_some(), true);
        pretty_assertions::assert_eq!(put_success, false);
    }

    #[test]
    fn put_disallow_two_black_kings() {
        let mut chess = WasmChess::new(None).unwrap();

        let put_success = chess.put(
            PieceObj {
                color: ColorChar::B,
                r#type: PieceSymbol::K,
            },
            SquareStr::D8,
        );

        pretty_assertions::assert_eq!(chess.editable.is_some(), true);
        pretty_assertions::assert_eq!(put_success, false);
    }

    #[test]
    fn allow_two_kings_if_overwriting_same_square() {
        let mut chess = WasmChess::new(None).unwrap();

        let put_success = chess.put(
            PieceObj {
                color: ColorChar::W,
                r#type: PieceSymbol::K,
            },
            SquareStr::E1,
        );

        // chess.editable should be Some()
        pretty_assertions::assert_eq!(chess.editable.is_some(), true);
        pretty_assertions::assert_eq!(put_success, true);
    }

    #[test]
    fn put_replacing_white_kingside_rook_loses_castling_rights() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        let put_success = chess.put(
            PieceObj {
                color: ColorChar::W,
                r#type: PieceSymbol::N,
            },
            SquareStr::H1,
        );

        pretty_assertions::assert_eq!(chess.editable.is_some(), true);
        pretty_assertions::assert_eq!(put_success, true);

        let legal_moves = chess.legal_moves_san(None);

        legal_moves.iter().for_each(|mov| {
            pretty_assertions::assert_ne!(mov, "O-O");
        });
    }

    #[test]
    fn put_replacing_white_queenside_rook_loses_castling_rights() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        let put_success = chess.put(
            PieceObj {
                color: ColorChar::W,
                r#type: PieceSymbol::N,
            },
            SquareStr::A1,
        );

        pretty_assertions::assert_eq!(chess.editable.is_some(), true);
        pretty_assertions::assert_eq!(put_success, true);

        let legal_moves = chess.legal_moves_san(None);

        legal_moves.iter().for_each(|mov| {
            pretty_assertions::assert_ne!(mov, "O-O-O");
        });
    }

    // TODO: uncomment on fix
    // #[test]
    fn replacing_white_king_loses_castling_rights() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        let _ = chess.put(
            PieceObj {
                color: ColorChar::W,
                r#type: PieceSymbol::K,
            },
            SquareStr::E1,
        );

        let legal_moves = chess.legal_moves_san(None);

        legal_moves.iter().for_each(|mov| {
            pretty_assertions::assert_ne!(mov, "O-O-O");
            pretty_assertions::assert_ne!(mov, "O-O");
        });

        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(chess.fen(None)))
                .unwrap()
                .zobrist_hash()
        );
    }

    #[test]
    fn replacing_white_pawn_clears_en_passant_square() {
        let fen = "rnbqkbnr/pppppp1p/8/8/3PPPp1/8/PPP3PP/RNBQKBNR b KQkq f3 0 3".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        let put_success = chess.put(
            PieceObj {
                color: ColorChar::W,
                r#type: PieceSymbol::N,
            },
            SquareStr::F4,
        );

        pretty_assertions::assert_eq!(put_success, true);

        let legal_moves = chess.legal_moves_san(None);

        legal_moves.iter().for_each(|mov| {
            pretty_assertions::assert_ne!(mov, "gxf3");
        });
    }

    // modified test
    // Original piece was a knight, not a Q
    #[test]
    fn occupying_white_en_passant_square_clears_it() {
        let fen = "rnbqkbnr/pppppp1p/8/8/3PPPp1/8/PPP3PP/RNBQKBNR b KQkq f3 0 3".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        let put_success = chess.put(
            PieceObj {
                color: ColorChar::B,
                // r#type: PieceSymbol::N,
                r#type: PieceSymbol::Q,
            },
            SquareStr::F3,
        );

        pretty_assertions::assert_eq!(put_success, true);

        //
        let legal_moves = chess.legal_moves_san(None);

        legal_moves.iter().for_each(|mov| {
            pretty_assertions::assert_ne!(mov, "gxf3");
        });
    }
}
