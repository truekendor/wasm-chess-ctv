#[cfg(test)]
pub mod move_tests {
    use crate::{
        WasmChess,
        tsify_structs::{PieceSymbol, SquareStr},
    };

    #[test]
    fn move_works_standard_algebraic_notation() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
        let fen_next = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".to_string();

        let mut chess = WasmChess::new(Some(fen.clone())).unwrap();

        let e4_move = chess.make_move("e4").unwrap();

        pretty_assertions::assert_eq!(e4_move.captured, None);
        pretty_assertions::assert_eq!(e4_move.promotion, None);
        pretty_assertions::assert_eq!(e4_move.after, chess.fen(None));
        pretty_assertions::assert_eq!(chess.fen(None), fen_next);

        pretty_assertions::assert_eq!(e4_move.is_en_passant, false);
        pretty_assertions::assert_eq!(e4_move.is_big_pawn, true);

        pretty_assertions::assert_eq!(e4_move.is_castle, false);
        pretty_assertions::assert_eq!(e4_move.is_kingside_castle, false);
        pretty_assertions::assert_eq!(e4_move.is_queenside_castle, false);

        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen_next)).unwrap().zobrist_hash()
        );

        chess.undo();

        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen)).unwrap().zobrist_hash()
        );
    }

    #[test]
    fn move_works_standard_algebraic_notation_mates() {
        let fen = "7k/3R4/3p2Q1/6Q1/2N1N3/8/8/3R3K w - - 0 1".to_string();
        let fen_next = "3R3k/8/3p2Q1/6Q1/2N1N3/8/8/3R3K b - - 1 1".to_string();

        let mut chess = WasmChess::new(Some(fen.clone())).unwrap();

        let rook_mate_move = chess.make_move("Rd8#").unwrap();

        pretty_assertions::assert_eq!(rook_mate_move.captured, None);
        pretty_assertions::assert_eq!(rook_mate_move.promotion, None);

        pretty_assertions::assert_eq!(rook_mate_move.after, chess.fen(None));
        pretty_assertions::assert_eq!(chess.fen(None), fen_next);

        pretty_assertions::assert_eq!(rook_mate_move.is_en_passant, false);
        pretty_assertions::assert_eq!(rook_mate_move.is_regular_capture, false);
        pretty_assertions::assert_eq!(rook_mate_move.is_big_pawn, false);

        pretty_assertions::assert_eq!(rook_mate_move.is_castle, false);
        pretty_assertions::assert_eq!(rook_mate_move.is_kingside_castle, false);
        pretty_assertions::assert_eq!(rook_mate_move.is_queenside_castle, false);

        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen_next)).unwrap().zobrist_hash()
        );

        chess.undo();

        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen)).unwrap().zobrist_hash()
        );
    }

    #[test]
    fn move_works_standard_algebraic_white_en_passant() {
        let fen = "rnbqkbnr/pp3ppp/2pp4/4pP2/4P3/8/PPPP2PP/RNBQKBNR w KQkq e6 0 1".to_string();
        let fen_next = "rnbqkbnr/pp3ppp/2ppP3/8/4P3/8/PPPP2PP/RNBQKBNR b KQkq - 0 1".to_string();

        let mut chess = WasmChess::new(Some(fen.clone())).unwrap();

        let rook_mate_move = chess.make_move("fxe6").unwrap();

        assert!(rook_mate_move.captured.is_some());
        pretty_assertions::assert_eq!(rook_mate_move.captured, Some(PieceSymbol::P));
        pretty_assertions::assert_eq!(rook_mate_move.promotion, None);

        pretty_assertions::assert_eq!(rook_mate_move.after, chess.fen(None));
        pretty_assertions::assert_eq!(chess.fen(None), fen_next);

        pretty_assertions::assert_eq!(rook_mate_move.is_en_passant, true);
        pretty_assertions::assert_eq!(rook_mate_move.is_regular_capture, false);
        pretty_assertions::assert_eq!(rook_mate_move.is_big_pawn, false);

        pretty_assertions::assert_eq!(rook_mate_move.is_castle, false);
        pretty_assertions::assert_eq!(rook_mate_move.is_kingside_castle, false);
        pretty_assertions::assert_eq!(rook_mate_move.is_queenside_castle, false);

        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen_next)).unwrap().zobrist_hash()
        );

        chess.undo();

        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen)).unwrap().zobrist_hash()
        );
    }

    #[test]
    fn move_works_standard_algebraic_black_en_passant() {
        let fen = "rnbqkbnr/pppp2pp/8/4p3/4Pp2/2PP4/PP3PPP/RNBQKBNR b KQkq e3 0 1".to_string();
        let fen_next = "rnbqkbnr/pppp2pp/8/4p3/8/2PPp3/PP3PPP/RNBQKBNR w KQkq - 0 2".to_string();

        let mut chess = WasmChess::new(Some(fen.clone())).unwrap();

        let rook_mate_move = chess.make_move("fxe3").unwrap();

        assert!(rook_mate_move.captured.is_some());
        pretty_assertions::assert_eq!(rook_mate_move.captured, Some(PieceSymbol::P));
        pretty_assertions::assert_eq!(rook_mate_move.promotion, None);

        pretty_assertions::assert_eq!(rook_mate_move.after, chess.fen(None));
        pretty_assertions::assert_eq!(chess.fen(None), fen_next);

        pretty_assertions::assert_eq!(rook_mate_move.is_en_passant, true);
        pretty_assertions::assert_eq!(rook_mate_move.is_regular_capture, false);
        pretty_assertions::assert_eq!(rook_mate_move.is_big_pawn, false);

        pretty_assertions::assert_eq!(rook_mate_move.is_castle, false);
        pretty_assertions::assert_eq!(rook_mate_move.is_kingside_castle, false);
        pretty_assertions::assert_eq!(rook_mate_move.is_queenside_castle, false);

        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen_next)).unwrap().zobrist_hash()
        );

        chess.undo();

        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen)).unwrap().zobrist_hash()
        );
    }

    #[test]
    fn move_works_standard_algebraic_notation_pin_disambiguates_piece() {
        let fen = "r2qkbnr/ppp2ppp/2n5/1B2pQ2/4P3/8/PPP2PPP/RNB1K2R b KQkq - 3 7".to_string();
        let fen_next = "r2qkb1r/ppp1nppp/2n5/1B2pQ2/4P3/8/PPP2PPP/RNB1K2R w KQkq - 4 8".to_string();

        let mut chess = WasmChess::new(Some(fen.clone())).unwrap();

        let mov = chess.make_move("Ne7").unwrap();

        pretty_assertions::assert_eq!(mov.from, SquareStr::G8);
        pretty_assertions::assert_eq!(mov.to, SquareStr::E7);
        pretty_assertions::assert_eq!(mov.piece, PieceSymbol::N);

        pretty_assertions::assert_eq!(&mov.after, &fen_next);
        pretty_assertions::assert_eq!(&mov.after, &chess.fen(None));
        pretty_assertions::assert_eq!(mov.is_regular_capture, false);
        pretty_assertions::assert_eq!(mov.promotion.is_some(), false);

        pretty_assertions::assert_eq!(mov.is_en_passant, false);
        pretty_assertions::assert_eq!(mov.is_big_pawn, false);

        pretty_assertions::assert_eq!(mov.is_castle, false);
        pretty_assertions::assert_eq!(mov.is_kingside_castle, false);
        pretty_assertions::assert_eq!(mov.is_queenside_castle, false);

        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen_next)).unwrap().zobrist_hash()
        );
        chess.undo();
        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen)).unwrap().zobrist_hash()
        );
    }

    #[test]
    fn move_works_standard_algebraic_overly_disabig_piece() {
        let fen = "r2qkbnr/ppp2ppp/2n5/1B2pQ2/4P3/8/PPP2PPP/RNB1K2R b KQkq - 3 7".to_string();
        let fen_next = "r2qkb1r/ppp1nppp/2n5/1B2pQ2/4P3/8/PPP2PPP/RNB1K2R w KQkq - 4 8".to_string();

        let mut chess = WasmChess::new(Some(fen.clone())).unwrap();

        let mov = chess.make_move("Nge7").unwrap();

        pretty_assertions::assert_eq!(mov.from, SquareStr::G8);
        pretty_assertions::assert_eq!(mov.to, SquareStr::E7);
        pretty_assertions::assert_eq!(mov.piece, PieceSymbol::N);

        pretty_assertions::assert_eq!(&mov.after, &fen_next);
        pretty_assertions::assert_eq!(&mov.after, &chess.fen(None));
        pretty_assertions::assert_eq!(mov.is_regular_capture, false);
        pretty_assertions::assert_eq!(mov.promotion.is_some(), false);

        pretty_assertions::assert_eq!(mov.is_en_passant, false);
        pretty_assertions::assert_eq!(mov.is_big_pawn, false);

        pretty_assertions::assert_eq!(mov.is_castle, false);
        pretty_assertions::assert_eq!(mov.is_kingside_castle, false);
        pretty_assertions::assert_eq!(mov.is_queenside_castle, false);

        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen_next)).unwrap().zobrist_hash()
        );
        chess.undo();
        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen)).unwrap().zobrist_hash()
        );
    }

    // TODO: port rest of the tests
}
