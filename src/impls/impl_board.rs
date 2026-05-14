use super::*;

#[wasm_bindgen]
impl WasmChess {
    pub(crate) fn put(&mut self, piece_obj: PieceObj, square: SquareStr) -> bool {
        let piece = piece_obj.to_shakmaty_piece();
        let square = square.to_shakmaty_sq();

        // TODO: rules:
        // no more than TWO kings, but can be less

        // TODO: change to behavior below
        // right not we create editable setup even on only half-valid put() call
        // but we should only do that if the put action was valid all the way through OR
        // if the setup already exist

        // TODO: we can do logic by passing board chain of methods on board, but without
        // knowing which one of the boards it is ?
        // OR we just do  `if _ {} else {}`

        let editable = match self.editable.as_mut() {
            Some(val) => val,
            None => &mut EditablePosition {
                setup: Chess::to_setup(&self.chess, EnPassantMode::Legal),
                validated: None,
            },
        };

        let setup = &mut editable.setup;
        let _ = &setup.board.set_piece_at(square, piece);

        let pos = setup
            .clone()
            .position::<Chess>(shakmaty::CastlingMode::Chess960);

        // TODO:
        // update castling rights
        // update setup
        // update ep square

        // TODO:
        // immediatelly try to replace current pos with new one

        let pos = match pos {
            Ok(val) => Some(val),
            Err(err) => {
                let result = err.ignore_too_much_material();

                result.ok()
            }
        };

        if let Some(validated) = pos {
            editable.validated = Some(validated.clone());
            // TODO:
            // why even bother with this if we immediately replace
            self.chess = validated;
            return true;
        }

        editable.validated = None;

        return false;
    }

    fn remove(&mut self, square: SquareStr) -> Option<PieceObj> {
        let square = square.to_shakmaty_sq();

        let editable = match self.editable.as_mut() {
            Some(val) => val,
            None => &mut EditablePosition {
                setup: Chess::to_setup(&self.chess, EnPassantMode::Legal),
                validated: None,
            },
        };

        let setup = &mut editable.setup;
        let piece = setup.board.remove_piece_at(square);

        let pos = setup
            .clone()
            .position::<Chess>(shakmaty::CastlingMode::Chess960);

        editable.validated = pos.ok();

        if let Some(p) = piece {
            return Some(PieceObj::from_shakmaty_piece(&p));
        }

        // TODO:
        // update castling rights
        // update setup
        // update ep square

        return None;
    }

    fn clear(&mut self, preserve_headers: Option<PreserveHeaders>) -> () {
        let editable = match self.editable.as_mut() {
            Some(val) => val,
            None => &mut EditablePosition {
                setup: Chess::to_setup(&self.chess, EnPassantMode::Legal),
                validated: None,
            },
        };

        let empty_setup = Setup::empty();

        let chess = Chess::from_setup(empty_setup, shakmaty::CastlingMode::Chess960);

        match chess {
            Ok(val) => val,
            Err(err) => {
                todo!()
            }
        };

        todo!()
    }

    fn set_turn(&mut self, color: ColorChar) -> bool {
        todo!()
    }

    #[wasm_bindgen(js_name = "setCastlingRights")]
    pub fn set_castling_rights(&mut self, color: ColorChar, castling_obj: CastlingObj) -> bool {
        let editable = match self.editable.as_mut() {
            Some(val) => val,
            None => &mut EditablePosition {
                setup: Chess::to_setup(&self.chess, EnPassantMode::Legal),
                validated: None,
            },
        };

        let setup = &mut editable.setup;

        let (kingside_sq, queenside_sq) = match color {
            ColorChar::W => (Square::H1, Square::A1),
            ColorChar::B => (Square::H8, Square::A8),
        };

        if castling_obj.king == Some(true) {
            setup.castling_rights.add(kingside_sq);
        } else if castling_obj.king == Some(false) {
            let _ = setup.castling_rights.remove(kingside_sq);
        }

        if castling_obj.queen == Some(true) {
            setup.castling_rights.add(queenside_sq);
        } else if castling_obj.queen == Some(false) {
            let _ = setup.castling_rights.remove(queenside_sq);
        }

        let chess_temp = Chess::from_setup(setup.clone(), shakmaty::CastlingMode::Chess960);

        let pos: Option<Chess> = match chess_temp {
            Ok(val) => Some(val),
            Err(err) => {
                let result = err.ignore_invalid_castling_rights();

                result.ok()
            }
        };

        if let Some(validated) = pos {
            editable.validated = Some(validated.clone());
            // TODO:
            // why even bother with this if we immediately replace
            self.chess = validated;
        };

        let rights_final = self.get_castling_rights(color);

        let successfully_set = (castling_obj.king.is_none()
            || rights_final.king == castling_obj.king)
            && (castling_obj.queen.is_none() || rights_final.queen == castling_obj.queen);

        return successfully_set;
    }

    fn set_en_passant_square() {
        //
    }
}
