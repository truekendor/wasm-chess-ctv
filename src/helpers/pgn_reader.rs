use ordermap::OrderMap;
use shakmaty::{CastlingMode, Chess, fen::Fen};
use std::{io, ops::ControlFlow};

use pgn_reader::{RawTag, Reader, SanPlus, Visitor};

use crate::WasmChess;

#[derive(Debug, Default, Clone)]
pub struct PGNResult {
    pub headers: OrderMap<String, String>,
    pub starting_fen: Fen,

    pub comments_map: OrderMap<String, String>,
    pub suffix_map: OrderMap<String, String>,
    pub nag_map: OrderMap<String, Vec<String>>,
}

const SUFFIX_LIST: [&str; 6] = ["!", "?", "!!", "??", "!?", "?!"];

impl Visitor for PGNResult {
    type Tags = ();
    type Movetext = WasmChess;
    type Output = Result<WasmChess, String>;

    fn begin_tags(&mut self) -> ControlFlow<Self::Output, Self::Tags> {
        self.comments_map = OrderMap::new();
        self.suffix_map = OrderMap::new();
        self.nag_map = OrderMap::new();

        self.headers = OrderMap::from([
            ("Event".to_string(), "?".to_string()),
            ("Site".to_string(), "?".to_string()),
            ("Date".to_string(), "????.??.??".to_string()),
            ("Round".to_string(), "?".to_string()),
            ("White".to_string(), "?".to_string()),
            ("Black".to_string(), "?".to_string()),
            ("Result".to_string(), "*".to_string()),
        ]);

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

        if name.to_ascii_uppercase() == b"FEN" {
            let fen = match Fen::from_ascii(value.as_bytes()) {
                Ok(fen) => fen,
                Err(err) => {
                    return ControlFlow::Break(Err(format!("Error parsing fen from PGN: {}", err)));
                }
            };
            match fen.clone().into_position::<Chess>(CastlingMode::Chess960) {
                Ok(pos) => {
                    self.starting_fen = fen;
                    pos
                }
                Err(err) => {
                    return ControlFlow::Break(Err(format!(
                        "Position error: {} for FEN: {}",
                        err, fen
                    )));
                }
            };
        };

        if let Ok(value) = tag_val {
            self.headers.insert(tag_key, value.to_string());

            return ControlFlow::Continue(());
        };

        ControlFlow::Break(Err(format!("Error reading PGN. ")))
    }

    fn begin_movetext(&mut self, _tags: Self::Tags) -> ControlFlow<Self::Output, Self::Movetext> {
        ControlFlow::Continue(WasmChess::new(Some(self.starting_fen.to_string())).unwrap())
    }

    fn san(&mut self, chess: &mut Self::Movetext, san_plus: SanPlus) -> ControlFlow<Self::Output> {
        match chess.make_move(&san_plus.san.to_string()) {
            Ok(_) => {
                return ControlFlow::Continue(());
            }
            Err(err) => ControlFlow::Break(Err(format!("{}", err))),
        }
    }

    fn nag(
        &mut self,
        wasm_chess: &mut Self::Movetext,
        nag: pgn_reader::Nag,
    ) -> ControlFlow<Self::Output> {
        let nag = nag.to_string();

        let fen_key =
            Fen::from_position(&wasm_chess.chess, shakmaty::EnPassantMode::Legal).to_string();

        let nag_str = nag.as_str();
        match nag_str {
            "$1" | "$2" | "$3" | "$4" | "$5" | "$6" => {
                let number_part = &nag_str[1..];

                let number = number_part.parse::<u32>();

                if let Ok(nag_number) = number {
                    let suffix_number = nag_number - 1;

                    if suffix_number >= SUFFIX_LIST.len() as u32 {
                        return ControlFlow::Continue(());
                    }

                    let char = SUFFIX_LIST[suffix_number as usize];

                    self.suffix_map.insert(fen_key.clone(), char.to_owned());
                }

                return ControlFlow::Continue(());
            }
            _ => (),
        }

        self.nag_map.entry(fen_key).or_insert(Vec::new()).push(nag);

        ControlFlow::Continue(())
    }

    fn comment(
        &mut self,
        wasm_chess: &mut Self::Movetext,
        comment: pgn_reader::RawComment<'_>,
    ) -> ControlFlow<Self::Output> {
        let raw_comment = comment;
        let comment = str::from_utf8(&raw_comment.as_bytes());

        if let Ok(val) = comment {
            let fen_key =
                Fen::from_position(&wasm_chess.chess, shakmaty::EnPassantMode::Legal).to_string();

            self.comments_map.insert(fen_key, val.to_string());

            return ControlFlow::Continue(());
        }

        ControlFlow::Break(Err(format!(
            "Error parsing comment from PGN: {:?}",
            raw_comment
        )))
    }

    fn end_game(&mut self, wasm_chess: Self::Movetext) -> Self::Output {
        return Ok(wasm_chess);
    }
}

pub fn parse_pgn(pgn: &str) -> Result<(PGNResult, WasmChess), String> {
    let mut reader = Reader::new(io::Cursor::new(pgn));
    let mut pgn_headers = PGNResult::default();

    match reader.read_game(&mut pgn_headers) {
        Ok(chess) => {
            let chess = chess.unwrap().unwrap();

            return Ok((pgn_headers, chess));
        }
        Err(err) => {
            return Err(err.to_string());
        }
    };
}
