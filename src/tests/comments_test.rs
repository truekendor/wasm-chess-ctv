/// tests taken from chess.js test suite for comments
///
/// @link https://github.com/jhlywa/chess.js/blob/master/__tests__/comments.test.ts
#[cfg(test)]
pub mod comments_test {
    use crate::WasmChess;
    use crate::tsify_structs::others::*;

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

    #[test]
    fn handles_manually_set_suffix_only_for_current_fen() {
        let mut chess = WasmChess::new(None).unwrap();
        chess.make_move("g3").unwrap();

        chess.set_suffix_annotation("?!", None).unwrap();

        let comments_result = chess.get_comments();

        pretty_assertions::assert_eq!(
            comments_result[0],
            CommentsObj {
                fen: chess.fen(None),
                comment: None,
                suffix_annotation: Some("?!".to_string()),
                nags: vec![]
            }
        )
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

            pretty_assertions::assert_eq!(chess.pgn(None).ends_with("1. e4 *"), true);
        }

        #[test]
        fn set_comments_initial_pos() {
            let mut chess = WasmChess::new(None).unwrap();
            let comment_str = "Starting position";

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
            );

            pretty_assertions::assert_eq!(
                chess
                    .pgn(None)
                    .ends_with(format!("{{{}}} *", comment_str).as_str()),
                true
            );
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
            );

            pretty_assertions::assert_eq!(
                chess
                    .pgn(None)
                    .ends_with(format!("1. e4 {{{}}} e5 *", comment_str).as_str()),
                true
            );
        }

        #[test]
        fn comment_for_last_move() {
            let mut chess = WasmChess::new(None).unwrap();
            chess.make_move("e4").unwrap();
            chess.make_move("e6").unwrap();

            let comment_str = "dubious move";
            chess.set_comment(comment_str);

            pretty_assertions::assert_eq!(chess.get_comment().unwrap(), comment_str);

            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![CommentsObj {
                    comment: Some(comment_str.to_string()),
                    fen: chess.fen(None),
                    nags: vec![],
                    suffix_annotation: None
                }]
            );

            pretty_assertions::assert_eq!(
                chess
                    .pgn(None)
                    .ends_with(format!("1. e4 e6 {{{}}} *", comment_str).as_str()),
                true
            );
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
            let first_comment_str = "starting position";
            chess.set_comment(first_comment_str);

            pretty_assertions::assert_eq!(chess.get_comment().unwrap(), first_comment_str);
            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![CommentsObj {
                    comment: Some(first_comment_str.to_string()),
                    fen: initial_fen.clone(),
                    nags: vec![],
                    suffix_annotation: None
                }]
            );

            pretty_assertions::assert_eq!(
                chess
                    .pgn(None)
                    .ends_with(format!("{{{}}} *", first_comment_str).as_str()),
                true
            );

            chess.make_move("e4").unwrap();

            let e4_fen = chess.fen(None);
            let second_comment_str = "good move";

            chess.set_comment(second_comment_str);

            pretty_assertions::assert_eq!(chess.get_comment().unwrap(), second_comment_str);
            pretty_assertions::assert_eq!(chess.get_comments().len(), 2);
            pretty_assertions::assert_eq!(
                chess.get_comments(),
                vec![
                    CommentsObj {
                        comment: Some(first_comment_str.to_string()),
                        fen: initial_fen.clone(),
                        nags: vec![],
                        suffix_annotation: None
                    },
                    CommentsObj {
                        comment: Some(second_comment_str.to_string()),
                        fen: e4_fen.clone(),
                        nags: vec![],
                        suffix_annotation: None
                    }
                ]
            );

            pretty_assertions::assert_eq!(
                chess.pgn(None).ends_with(
                    format!(
                        "{{{}}} 1. e4 {{{}}} *",
                        first_comment_str, second_comment_str
                    )
                    .as_str()
                ),
                true
            );

            chess.make_move("e6").unwrap();
            let third_comment_str = "dubious move";

            chess.set_comment(third_comment_str);
            let e6_fen = chess.fen(None);

            pretty_assertions::assert_eq!(chess.get_comment().unwrap(), third_comment_str);
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
                        comment: Some(third_comment_str.to_string()),
                        fen: e6_fen,
                        nags: vec![],
                        suffix_annotation: None
                    }
                ]
            );

            pretty_assertions::assert_eq!(
                chess.pgn(None).ends_with(
                    format!(
                        "{{{}}} 1. e4 {{{}}} e6 {{{}}} *",
                        first_comment_str, second_comment_str, third_comment_str
                    )
                    .as_str()
                ),
                true
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

            pretty_assertions::assert_eq!(
                chess
                    .pgn(None)
                    .ends_with("{starting position} 1. e4 {good move} e6 *",),
                true
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
            );

            pretty_assertions::assert_eq!(chess.pgn(None).ends_with("1. e4 e6 *"), true);
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

            pretty_assertions::assert_eq!(chess.pgn(None).ends_with("1. d4 {positional} *"), true);
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

            chess.load(chess.fen(None).as_str(), None).unwrap();

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

    #[cfg(test)]
    pub mod format_comments {
        use crate::WasmChess;

        #[test]
        fn test_all() {
            struct CommentsFormatTest<'a> {
                pub name: &'a str,
                pub input: &'a str,
                pub output: &'a str,
            }

            let tests: Vec<CommentsFormatTest> = vec![
                CommentsFormatTest {
                    name: "bracket comments",
                    input: "1. e4 {good move} e5 {classical response}",
                    output: "1. e4 {good move} e5 {classical response}",
                },
                // CommentsFormatTest {
                //     name: "semicolon comments",
                //     input: "1. e4 e5; romantic era\n 2. Nf3 Nc6; common continuation",
                //     output: "1. e4 e5 {romantic era} 2. Nf3 Nc6 {common continuation}",
                // },
                // CommentsFormatTest {
                //     name: "bracket and semicolon comments",
                //     input: "1. e4 {good!} e5; standard response\n 2. Nf3 Nc6 {common}",
                //     output: "1. e4 {good!} e5 {standard response} 2. Nf3 Nc6 {common}",
                // },
                CommentsFormatTest {
                    name: "bracket comments with newlines",
                    input: "1. e4 {good\nmove} e5 {classical\nresponse}",
                    output: "1. e4 {good move} e5 {classical response}",
                },
                CommentsFormatTest {
                    name: "initial comment",
                    input: "{ great game }\n1. e4 e5",
                    output: "{ great game } 1. e4 e5",
                },
                // CommentsFormatTest {
                //     name: "initial comment with black starting first",
                //     input: r#"[SetUp "1"]\n[FEN "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq - 0 1"]\n\n{ great game } 1. ... Nc6""#,
                //     output: r#"[SetUp "1"]\n[FEN "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq - 0 1"]\n\n{ great game } 1. ... Nc6""#,
                // },
                CommentsFormatTest {
                    name: "empty bracket comment",
                    input: "1. e4 {}",
                    output: "1. e4 {}",
                },
                // CommentsFormatTest {
                //     name: "empty semicolon comment",
                //     input: "1. e4;\ne5",
                //     output: "1. e4 {} e5",
                // },
                CommentsFormatTest {
                    name: "unicode comment",
                    input: "1. e4 {Δ, Й, ק ,م, ๗, あ, 叶, 葉, and 말}",
                    output: "1. e4 {Δ, Й, ק ,م, ๗, あ, 叶, 葉, and 말}",
                },
                // CommentsFormatTest {
                //     name: "semicolon in bracket comment",
                //     input: "1. e4 { a classic; well-studied } e5",
                //     output: "1. e4 { a classic; well-studied } e5",
                // },
                // CommentsFormatTest {
                //     name: "bracket in semicolon comment",
                //     input: "1. e4 e5 ; a classic {well-studied}",
                //     output: "1. e4 e5 {a classic {well-studied}}",
                // },
                CommentsFormatTest {
                    name: "markers in bracket comment",
                    input: "1. e4 e5 {($1) 1. e4 is good}",
                    output: "1. e4 e5 {($1) 1. e4 is good}",
                },
                // CommentsFormatTest {
                //     name: "markers in semicolon comment",
                //     input: "1. e4 e5; ($1) 1. e4 is good",
                //     output: "1. e4 e5 {($1) 1. e4 is good}",
                // },
                // PROBLEM ? DEFAULTS ARE NOT NEEDED
                //                 CommentsFormatTest {
                //                     name: "comment before black to move",
                //                     input: r#"
                //                 [SetUp "1"]
                //                 [FEN "r4rk1/p1nq1pp1/1p1pp2p/8/P2PR3/1QP2N2/1P3PPP/R5K1 b - - 0 16"]

                //                 {test comment} 16...Rfb8"#,
                //                     output: r#"[SetUp "1"]
                // [FEN "r4rk1/p1nq1pp1/1p1pp2p/8/P2PR3/1QP2N2/1P3PPP/R5K1 b - - 0 16"]

                // {test comment} 16. ... Rfb8"#,
                // },
            ];

            tests.iter().for_each(|test_entry| {
                let mut chess = WasmChess::new(None).unwrap();

                chess
                    .load_pgn(test_entry.input)
                    .expect("Test entries are all correct");

                let msg = format!("Test failed, {}", test_entry.name);

                let pgn = chess.pgn(None);

                pretty_assertions::assert_eq!(
                    pgn.ends_with(format!("{} *", test_entry.output).as_str()),
                    true,
                    "ERROR: {}",
                    msg,
                );
            });
        }
    }
}
