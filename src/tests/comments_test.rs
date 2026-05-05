#[cfg(test)]
pub mod comments_test {
    use crate::WasmChess;
    use crate::tsify_structs::{others::*, *};

    #[test]
    fn captures_multiple_suffixes_and_comments_ok() {
        let pgn = r#"1. c4 {English Opening} 
        e5!? {Aggressive} 
        2. Nf3!! {Best Move} 
        Nc6?? {Blunder} *"#;

        let mut chess = WasmChess::new(None).unwrap();

        chess.load_pgn(pgn).unwrap();

        let fen1 = "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq - 0 1";
        let fen2 = "rnbqkbnr/pppp1ppp/8/4p3/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 2";
        let fen3 = "rnbqkbnr/pppp1ppp/8/4p3/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 1 2";
        let fen4 = "r1bqkbnr/pppp1ppp/2n5/4p3/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 2 3";

        pretty_assertions::assert_eq!(chess.fen(None), fen4);

        let answers: Vec<CommentsObj> = vec![
            CommentsObj {
                comment: Some("English Opening".to_string()),
                fen: fen1.to_string(),
                nags: vec![],
                suffix_annotation: None,
            },
            CommentsObj {
                comment: Some("Aggressive".to_string()),
                fen: fen2.to_string(),
                nags: vec![],
                suffix_annotation: Some("!?".to_string()),
            },
            CommentsObj {
                comment: Some("Best Move".to_string()),
                fen: fen3.to_string(),
                nags: vec![],
                suffix_annotation: Some("!!".to_string()),
            },
            CommentsObj {
                comment: Some("Blunder".to_string()),
                fen: fen4.to_string(),
                nags: vec![],
                suffix_annotation: Some("??".to_string()),
            },
        ];

        let comments = chess.get_comments();

        pretty_assertions::assert_eq!(comments.len(), answers.len());
        pretty_assertions::assert_eq!(answers, comments);
    }

    #[test]
    fn correctly_handles_comments_and_suffixes() {
        let pgn = "1. c4 {Comment for c4} e5!? {Comment and Suffix for e5} 2. Nf3!! Nc6 *";
        let mut chess = WasmChess::new(None).unwrap();

        chess.load_pgn(pgn).unwrap();

        let fen_c4 = "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq - 0 1";
        let fen_e5 = "rnbqkbnr/pppp1ppp/8/4p3/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 2";
        let fen_nf3 = "rnbqkbnr/pppp1ppp/8/4p3/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 1 2";
        let fen_nc6 = "r1bqkbnr/pppp1ppp/2n5/4p3/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 2 3";

        pretty_assertions::assert_eq!(chess.fen(None), fen_nc6);

        let comments = chess.get_comments();
        let answers: Vec<CommentsObj> = vec![
            CommentsObj {
                comment: Some("Comment for c4".to_string()),
                fen: fen_c4.to_string(),
                nags: vec![],
                suffix_annotation: None,
            },
            CommentsObj {
                comment: Some("Comment and Suffix for e5".to_string()),
                fen: fen_e5.to_string(),
                nags: vec![],
                suffix_annotation: Some("!?".to_string()),
            },
            CommentsObj {
                comment: None,
                fen: fen_nf3.to_string(),
                nags: vec![],
                suffix_annotation: Some("!!".to_string()),
            },
        ];

        pretty_assertions::assert_eq!(comments.len(), answers.len());
        pretty_assertions::assert_eq!(comments, answers);
    }

    // #[test]
    // TODO: add this
    fn handles_manually_set_suffix_for_current_fen() {
        let mut chess = WasmChess::new(None).unwrap();
        chess.make_move("g3").unwrap();

        // chess.set
    }

    #[cfg(test)]

    pub mod manipulate_comments {
        use super::*;

        #[test]
        fn no_comments() {
            let mut chess = WasmChess::new(None).unwrap();

            pretty_assertions::assert_eq!(chess.get_comment(), None);
            pretty_assertions::assert_eq!(chess.get_comments().len(), 0);

            chess.make_move("e4").unwrap();

            pretty_assertions::assert_eq!(chess.get_comment(), None);
            pretty_assertions::assert_eq!(chess.get_comments().len(), 0);
        }

        #[test]
        fn set_comments_initial_pos() {
            let comment_str = "Starting position";
            let mut chess = WasmChess::new(None).unwrap();

            pretty_assertions::assert_eq!(chess.get_comment(), None);
            pretty_assertions::assert_eq!(chess.get_comments().len(), 0);

            chess.set_comment(comment_str);

            pretty_assertions::assert_eq!(chess.get_comment(), Some(comment_str.to_string()));
            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![CommentsObj {
                    comment: Some(comment_str.to_string()),
                    fen: chess.fen(None).to_string(),
                    nags: vec![],
                    suffix_annotation: None
                }]
            )
        }

        #[test]
        fn set_comments_first_move() {
            let comment_str = "Good move";
            let mut chess = WasmChess::new(None).unwrap();

            pretty_assertions::assert_eq!(chess.get_comment(), None);
            pretty_assertions::assert_eq!(chess.get_comments().len(), 0);

            chess.make_move("e4").unwrap();
            let c4_fen = chess.fen(None);

            chess.set_comment(comment_str);

            pretty_assertions::assert_eq!(chess.get_comment(), Some(comment_str.to_string()));
            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![CommentsObj {
                    comment: Some(comment_str.to_string()),
                    fen: c4_fen.to_string(),
                    nags: vec![],
                    suffix_annotation: None
                }]
            );

            chess.make_move("e5").unwrap();
            pretty_assertions::assert_eq!(chess.get_comment(), None);
            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![CommentsObj {
                    comment: Some(comment_str.to_string()),
                    fen: c4_fen.to_string(),
                    nags: vec![],
                    suffix_annotation: None
                }]
            )

            // TODO: add .pgn()
        }

        #[test]
        fn comment_with_bracket() {
            let mut chess = WasmChess::new(None).unwrap();
            chess.set_comment("{starting position}");

            pretty_assertions::assert_eq!(
                chess.get_comment().unwrap(),
                "[starting position]".to_string()
            );
        }

        #[test]
        fn comment_for_everything() {
            let mut chess = WasmChess::new(None).unwrap();

            let initial_fen = chess.fen(None);
            chess.set_comment("starting position");

            pretty_assertions::assert_eq!(chess.get_comment().unwrap(), "starting position");
            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![CommentsObj {
                    comment: Some("starting position".to_string()),
                    fen: initial_fen.clone(),
                    nags: vec![],
                    suffix_annotation: None
                }]
            );
            // TODO: insert .pgn() here

            chess.make_move("e4").unwrap();

            let e4_fen = chess.fen(None);
            chess.set_comment("good move");

            pretty_assertions::assert_eq!(chess.get_comment().unwrap(), "good move");
            pretty_assertions::assert_eq!(chess.get_comments().len(), 2);
            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![
                    CommentsObj {
                        comment: Some("starting position".to_string()),
                        fen: initial_fen.clone(),
                        nags: vec![],
                        suffix_annotation: None
                    },
                    CommentsObj {
                        comment: Some("good move".to_string()),
                        fen: e4_fen.clone(),
                        nags: vec![],
                        suffix_annotation: None
                    }
                ]
            );

            chess.make_move("e6").unwrap();
            chess.set_comment("dubious move");
            let e6_fen = chess.fen(None);

            pretty_assertions::assert_eq!(chess.get_comment().unwrap(), "dubious move");
            pretty_assertions::assert_eq!(chess.get_comments().len(), 3);
            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![
                    CommentsObj {
                        comment: Some("starting position".to_string()),
                        fen: initial_fen.clone(),
                        nags: vec![],
                        suffix_annotation: None
                    },
                    CommentsObj {
                        comment: Some("good move".to_string()),
                        fen: e4_fen,
                        nags: vec![],
                        suffix_annotation: None
                    },
                    CommentsObj {
                        comment: Some("dubious move".to_string()),
                        fen: e6_fen,
                        nags: vec![],
                        suffix_annotation: None
                    }
                ]
            );
        }

        #[test]
        fn remove_comment() {
            let mut chess = WasmChess::new(None).unwrap();

            pretty_assertions::assert_eq!(chess.remove_comment(), None);
            pretty_assertions::assert_eq!(chess.remove_comments(), vec![]);

            let init_fen = chess.fen(None);

            chess.set_comment("starting position");
            chess.make_move("e4").unwrap();

            let e4_fen = chess.fen(None);
            chess.set_comment("good move");

            chess.make_move("e6").unwrap();
            chess.set_comment("dubious move");
            let e6_fen = chess.fen(None);

            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![
                    CommentsObj {
                        fen: init_fen.clone(),
                        comment: Some("starting position".to_string()),
                        nags: vec![],
                        suffix_annotation: None
                    },
                    CommentsObj {
                        fen: e4_fen.clone(),
                        comment: Some("good move".to_string()),
                        nags: vec![],
                        suffix_annotation: None
                    },
                    CommentsObj {
                        fen: e6_fen,
                        comment: Some("dubious move".to_string()),
                        nags: vec![],
                        suffix_annotation: None
                    },
                ]
            );

            pretty_assertions::assert_eq!(
                chess.remove_comment().unwrap(),
                "dubious move".to_string()
            );

            pretty_assertions::assert_eq!(chess.remove_comment(), None);

            let removed = chess.remove_comments();

            pretty_assertions::assert_eq!(removed.len(), 2);
            pretty_assertions::assert_eq!(
                removed,
                vec![
                    PrunedCommentsObj {
                        fen: init_fen,
                        comment: "starting position".to_string(),
                    },
                    PrunedCommentsObj {
                        fen: e4_fen,
                        comment: "good move".to_string(),
                    },
                ]
            )
        }

        #[test]
        fn prune_comments() {
            let mut chess = WasmChess::new(None).unwrap();
            chess.make_move("e4").unwrap();
            chess.set_comment("tactical");

            chess.undo();
            chess.make_move("d4").unwrap();
            chess.set_comment("positional");

            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![CommentsObj {
                    comment: Some("positional".to_string()),
                    fen: chess.fen(None),
                    nags: vec![],
                    suffix_annotation: None
                }]
            );

            println!("PGN: {}", chess.pgn());
            // assert !(chess.pgn().ends_with("1. d4 {positional} *"));
        }

        #[test]
        fn comments_clear_after_load() {
            let mut chess = WasmChess::new(None).unwrap();
            chess.make_move("e4").unwrap();
            chess.set_comment("good move");

            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![CommentsObj {
                    comment: Some("good move".to_string()),
                    fen: chess.fen(None),
                    nags: vec![],
                    suffix_annotation: None
                }]
            );

            chess.load(chess.fen(None)).unwrap();

            pretty_assertions::assert_eq!(chess.get_comments(), vec![]);
        }

        #[test]
        fn comments_clear_after_pgn_load() {
            let mut chess = WasmChess::new(None).unwrap();
            chess.make_move("e4").unwrap();
            chess.set_comment("good move");

            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![CommentsObj {
                    comment: Some("good move".to_string()),
                    fen: chess.fen(None),
                    nags: vec![],
                    suffix_annotation: None
                }]
            );

            chess.load_pgn("1. e4").unwrap();

            pretty_assertions::assert_eq!(chess.get_comments(), vec![]);
        }

        #[test]
        fn comments_clear_after_reset() {
            let mut chess = WasmChess::new(None).unwrap();
            chess.make_move("e4").unwrap();
            chess.set_comment("good move");

            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![CommentsObj {
                    comment: Some("good move".to_string()),
                    fen: chess.fen(None),
                    nags: vec![],
                    suffix_annotation: None
                }]
            );

            chess.reset();
            pretty_assertions::assert_eq!(chess.get_comments(), vec![]);
        }

        fn clear_comment_on_clear() {
            // TODO
            // chess.clear()
        }
    }
}
