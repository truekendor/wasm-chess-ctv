use ordermap::OrderMap;
use shakmaty::{CastlingMode, Chess, KnownOutcome, fen::Fen};
use std::ops::ControlFlow;

use pgn_reader::{RawTag, SanPlus, Visitor};

use crate::{WasmChess, tsify_structs::others::PreserveHeaders};

#[derive(Debug, Default, Clone)]
pub struct PGNResult {
    pub headers: OrderMap<String, String>,
    pub starting_fen: Fen,

    pub comments_map: OrderMap<String, String>,
    pub suffix_map: OrderMap<String, String>,
    pub nag_map: OrderMap<String, Vec<String>>,

    pub known_outcome: Option<KnownOutcome>,
}

const SUFFIX_LIST: [&str; 6] = ["!", "?", "!!", "??", "!?", "?!"];

const SEVEN_TAG_ROSTER: [&str; 7] = ["Event", "Site", "Date", "Round", "White", "Black", "Result"];
const SUPPLEMENTAL_TAGS: [(&str, Option<&str>); 30] = [
    ("WhiteTitle", None),
    ("BlackTitle", None),
    ("WhiteElo", None),
    ("BlackElo", None),
    ("WhiteUSCF", None),
    ("BlackUSCF", None),
    ("WhiteNA", None),
    ("BlackNA", None),
    ("WhiteType", None),
    ("BlackType", None),
    ("EventDate", None),
    ("EventSponsor", None),
    ("Section", None),
    ("Stage", None),
    ("Board", None),
    ("Opening", None),
    ("Variation", None),
    ("SubVariation", None),
    ("ECO", None),
    ("NIC", None),
    ("Time", None),
    ("UTCTime", None),
    ("UTCDate", None),
    ("TimeControl", None),
    ("SetUp", None),
    ("FEN", None),
    ("Termination", None),
    ("Annotator", None),
    ("Mode", None),
    ("PlyCount", None),
];

impl Visitor for WasmChess {
    type Tags = ();
    type Movetext = ();
    type Output = Result<(), String>;

    fn begin_tags(&mut self) -> ControlFlow<Self::Output, Self::Tags> {
        let pgn_result = self.pgn_result.get_or_insert_with(|| PGNResult::default());

        pgn_result.comments_map = OrderMap::new();
        pgn_result.suffix_map = OrderMap::new();
        pgn_result.nag_map = OrderMap::new();

        ControlFlow::Continue(())
    }

    fn tag(
        &mut self,
        _tags: &mut Self::Tags,
        name: &[u8],
        value: RawTag<'_>,
    ) -> ControlFlow<Self::Output> {
        let pgn_result = self.pgn_result.get_or_insert_with(|| PGNResult::default());

        let tag_key: String = name.iter().map(|b| *b as char).collect();
        let tag_val = str::from_utf8(value.as_bytes());

        let Ok(tag_value) = tag_val else {
            return ControlFlow::Break(Err(format!(
                "Error reading tag value\nTag Key: {}",
                tag_key
            )));
        };

        if name.to_ascii_uppercase() == b"FEN" {
            let fen = match Fen::from_ascii(tag_value.as_bytes()) {
                Ok(fen) => fen,
                Err(err) => {
                    return ControlFlow::Break(Err(format!("Error parsing fen from PGN: {}", err)));
                }
            };
            match fen.clone().into_position::<Chess>(CastlingMode::Chess960) {
                Ok(chess_pos) => {
                    pgn_result.starting_fen = fen;

                    chess_pos
                }
                Err(err) => {
                    // TODO:
                    // add recovery from too much material,
                    // and invalid castling rights ?

                    return ControlFlow::Break(Err(format!(
                        "Position error: {} for FEN: {}",
                        err, fen
                    )));
                }
            };
        };

        pgn_result
            .headers
            .insert(tag_key.clone(), tag_value.to_string());

        return ControlFlow::Continue(());
    }

    fn begin_movetext(&mut self, _tags: Self::Tags) -> ControlFlow<Self::Output, Self::Movetext> {
        let starting_fen_str = {
            let pgn_result: &mut PGNResult = self.pgn_result.get_or_insert_with(PGNResult::default);
            pgn_result.reorder_headers();

            &pgn_result.starting_fen.to_string()
        };

        match self.load_inner(
            &starting_fen_str,
            Some(PreserveHeaders {
                preserve_headers: true,
            }),
        ) {
            Ok(_) => (),
            Err(err) => {
                return ControlFlow::Break(Err(format!("{}", err)));
            }
        }

        ControlFlow::Continue(())
    }

    fn san(
        &mut self,
        _movetext: &mut Self::Movetext,
        san_plus: SanPlus,
    ) -> ControlFlow<Self::Output> {
        match self.make_move(&san_plus.san.to_string()) {
            Ok(_) => {
                return ControlFlow::Continue(());
            }
            Err(err) => ControlFlow::Break(Err(format!("{}", err))),
        }
    }

    fn nag(
        &mut self,
        _movetext: &mut Self::Movetext,
        nag: pgn_reader::Nag,
    ) -> ControlFlow<Self::Output> {
        let pgn_result = self.pgn_result.get_or_insert_with(PGNResult::default);

        let nag = nag.to_string();

        let fen_key = Fen::from_position(&self.chess, shakmaty::EnPassantMode::Legal).to_string();

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

                    pgn_result
                        .suffix_map
                        .insert(fen_key.clone(), char.to_owned());
                }

                return ControlFlow::Continue(());
            }
            _ => (),
        }

        pgn_result
            .nag_map
            .entry(fen_key)
            .or_insert(Vec::new())
            .push(nag);

        ControlFlow::Continue(())
    }

    fn comment(
        &mut self,
        _movetext: &mut Self::Movetext,
        comment: pgn_reader::RawComment<'_>,
    ) -> ControlFlow<Self::Output> {
        let pgn_result = self.pgn_result.get_or_insert_with(PGNResult::default);

        let raw_comment = comment;
        let comment = str::from_utf8(&raw_comment.as_bytes());

        if let Ok(val) = comment {
            let fen_key =
                Fen::from_position(&self.chess, shakmaty::EnPassantMode::Legal).to_string();

            pgn_result.comments_map.insert(fen_key, val.to_string());

            return ControlFlow::Continue(());
        }

        ControlFlow::Break(Err(format!(
            "Error parsing comment from PGN: {:?}",
            raw_comment
        )))
    }

    fn outcome(
        &mut self,
        _movetext: &mut Self::Movetext,
        outcome: shakmaty::Outcome,
    ) -> ControlFlow<Self::Output> {
        let pgn_result = self.pgn_result.get_or_insert_with(PGNResult::default);

        match outcome {
            shakmaty::Outcome::Known(known_outcome) => {
                pgn_result.known_outcome = Some(known_outcome);
                ControlFlow::Continue(())
            }
            shakmaty::Outcome::Unknown => {
                pgn_result.known_outcome = None;
                ControlFlow::Continue(())
            }
        }
    }

    fn end_game(&mut self, _movetext: Self::Movetext) -> Self::Output {
        return Ok(());
    }
}

impl PGNResult {
    pub fn reorder_headers(&mut self) {
        let mut ordered: OrderMap<String, Option<String>> = OrderMap::new();

        // Seven tag roster first
        for key in SEVEN_TAG_ROSTER {
            ordered.insert(key.to_string(), self.headers.get(key).cloned());
        }

        // Supplemental tags second
        for (key, _) in SUPPLEMENTAL_TAGS {
            ordered.insert(key.to_string(), self.headers.get(key).cloned());
        }

        // Remaining custom tags last
        for (key, val) in self.headers.iter() {
            if !ordered.contains_key(key) {
                ordered.insert(key.clone(), Some(val.clone()));
            }
        }

        self.headers = ordered
            .into_iter()
            .filter_map(|(k, v)| v.map(|v| (k, v)))
            .collect();
    }
}
