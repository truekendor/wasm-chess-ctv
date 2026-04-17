use shakmaty::{CastlingMode, Chess, fen::Fen};
use std::{collections::HashMap, io, ops::ControlFlow};

use pgn_reader::{RawTag, Reader, SanPlus, Visitor};

#[derive(Debug, Default, Clone)]
pub struct PGNHeaders {
    pub headers: HashMap<String, String>,
    pub comments: Vec<String>,
    pub move_list: Vec<String>,
    pub starting_fen: Fen,
}

impl Visitor for PGNHeaders {
    type Tags = ();
    type Output = ();
    type Movetext = ();

    fn begin_tags(&mut self) -> ControlFlow<Self::Output, Self::Tags> {
        ControlFlow::Continue(())
    }

    fn tag(
        &mut self,
        _tags: &mut Self::Tags,
        name: &[u8],
        value: RawTag<'_>,
    ) -> ControlFlow<Self::Output> {
        let tag_key: String = name.iter().map(|b| *b as char).collect();
        let tag_val = str::from_utf8(value.as_bytes());

        if name == b"FEN" {
            let fen = match Fen::from_ascii(value.as_bytes()) {
                Ok(fen) => fen,
                Err(err) => {
                    eprintln!("{} ({:?})", err, value);
                    return ControlFlow::Break(());
                }
            };
            let pos: Chess = match fen.clone().into_position(CastlingMode::Chess960) {
                Ok(pos) => {
                    self.starting_fen = fen;
                    pos
                }
                Err(err) => {
                    eprintln!("{}", err);
                    return ControlFlow::Break(());
                }
            };
        };

        if let Ok(value) = tag_val {
            self.headers.insert(tag_key, value.to_string());

            return ControlFlow::Continue(());
        };

        ControlFlow::Break(())
    }

    fn comment(
        &mut self,
        _movetext: &mut Self::Movetext,
        comment: pgn_reader::RawComment<'_>,
    ) -> ControlFlow<Self::Output> {
        let comment = str::from_utf8(comment.as_bytes());

        if let Ok(val) = comment {
            self.comments.push(val.to_string());

            return ControlFlow::Continue(());
        }

        ControlFlow::Break(())
    }

    fn san(
        &mut self,
        movetext: &mut Self::Movetext,
        san_plus: SanPlus,
    ) -> ControlFlow<Self::Output> {
        let san_move = san_plus.san.to_string();
        self.move_list.push(san_move);

        ControlFlow::Continue(())
    }

    fn begin_movetext(&mut self, _tags: Self::Tags) -> ControlFlow<Self::Output, Self::Movetext> {
        ControlFlow::Continue(())
    }

    fn end_game(&mut self, _movetext: Self::Movetext) -> Self::Output {
        return ();
    }
}

pub fn parse_pgn(pgn: String) -> Result<PGNHeaders, String> {
    let mut reader = Reader::new(io::Cursor::new(pgn));
    let mut pgn_headers = PGNHeaders::default();

    match reader.read_game(&mut pgn_headers) {
        Ok(_) => {
            return Ok(pgn_headers);
        }
        Err(err) => {
            return Err(err.to_string());
        }
    };
}
