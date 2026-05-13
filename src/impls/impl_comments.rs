use super::*;

#[wasm_bindgen]
impl WasmChess {
    #[wasm_bindgen(js_name = "removeComment")]
    pub fn remove_comment(&mut self) -> Option<String> {
        let current_fen = self.fen(None);

        let pgn_result = self.pgn_result.as_mut()?;
        let comment = pgn_result.comments_map.get(&current_fen).cloned()?;
        pgn_result.comments_map.remove(&current_fen);
        Some(comment)
    }

    #[wasm_bindgen(js_name = "removeComments")]
    pub fn remove_comments(&mut self) -> Vec<PrunedCommentsObj> {
        let Some(pgn_result) = self.pgn_result.as_mut() else {
            return vec![];
        };

        pgn_result
            .comments_map
            .drain(..)
            .map(|(key, value)| PrunedCommentsObj {
                fen: key,
                comment: value,
            })
            .collect()
    }

    #[wasm_bindgen(js_name = "getComment")]
    pub fn get_comment(&self) -> Option<String> {
        let Some(pgn_result) = self.pgn_result.as_ref() else {
            return None;
        };

        let comments = pgn_result.comments_map.get(&self.fen(None));

        comments.cloned()
    }

    // TODO: suffix annotations not working for now (deprecated ?j)
    #[wasm_bindgen(js_name = "getComments")]
    pub fn get_comments(&mut self) -> Vec<CommentsObj> {
        let mut comments_vec: Vec<CommentsObj> = vec![];

        let Some(pgn_result) = self.pgn_result.as_ref() else {
            return comments_vec;
        };

        let initial_fen_dev = match self.history.len() {
            0 => self.fen(None),
            _ => self.history[0].fen_before.to_string(),
        };

        let comment_obj = self.get_comment_object(initial_fen_dev, pgn_result);

        if let Some(comment_obj) = comment_obj {
            comments_vec.push(comment_obj);
        }

        if self.history.len() == 0 {
            return comments_vec;
        }

        self.history.iter().for_each(|history_entry| {
            let fen_str = history_entry.fen_after.to_string();

            let comment_obj = self.get_comment_object(fen_str, pgn_result);

            if let Some(comment_obj) = comment_obj {
                comments_vec.push(comment_obj);
            }
        });

        comments_vec
    }

    #[wasm_bindgen(js_name = "setComment")]
    pub fn set_comment(&mut self, comment: &str) {
        let fen = self.fen(None);
        let pgn_result = self.pgn_result.get_or_insert_with(PGNResult::default);

        pgn_result
            .comments_map
            .insert(fen, comment.replace('{', "[").replace('}', "]"));
    }

    fn get_comment_object(
        &self,
        fen_str: FenString,
        pgn_result: &PGNResult,
    ) -> Option<CommentsObj> {
        let comment_str = pgn_result.comments_map.get(&fen_str);
        let suffix: Option<String> = pgn_result.suffix_map.get(&fen_str).cloned();
        let nags = match pgn_result.nag_map.get(&fen_str) {
            Some(val) => val.clone(),
            None => vec![],
        };

        if comment_str.is_some() || suffix.is_some() || nags.len() > 0 {
            return Some(CommentsObj {
                fen: fen_str,
                comment: comment_str.cloned(),
                suffix_annotation: suffix,
                nags,
            });
        }

        return None;
    }
}
