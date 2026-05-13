use shakmaty::{Chess, Move, Position, Role, Square, san::San, uci::UciMove};

use crate::{
    helpers::parsing::{san_to_san_plus, verbose_move_from_raw_move},
    tsify_structs::{MoveVerbose, others::LegalMovesFilterOptions},
};

pub fn uci(chess: &Chess, filter_options: Option<LegalMovesFilterOptions>) -> Vec<String> {
    let filter_options = unwrap_filter_options(&filter_options);

    let legal_moves: Vec<String> = chess
        .legal_moves()
        .iter()
        .filter_map(|mov| {
            let move_filtered = filter_move(mov, &filter_options);
            if move_filtered {
                return None;
            }

            let uci_move = UciMove::from_move(*mov, shakmaty::CastlingMode::Chess960);
            return Some(uci_move.to_string());
        })
        .collect();

    legal_moves
}

pub fn san(chess: &Chess, filter_options: Option<LegalMovesFilterOptions>) -> Vec<String> {
    let filter_options = unwrap_filter_options(&filter_options);

    let legal_moves: Vec<String> = chess
        .legal_moves()
        .iter()
        .filter_map(|mov| {
            let mut chess_clone = chess.clone();

            let move_filtered = filter_move(mov, &filter_options);
            if move_filtered {
                return None;
            }

            let san_move = San::from_move(chess, *mov);
            chess_clone.play_unchecked(*mov);

            let san_str = san_to_san_plus(&san_move, &chess_clone);

            return Some(san_str);
        })
        .collect();

    legal_moves
}

pub fn verbose(chess: &Chess, filter_options: Option<LegalMovesFilterOptions>) -> Vec<MoveVerbose> {
    let filter_options = unwrap_filter_options(&filter_options);

    let legal_moves: Vec<MoveVerbose> = chess
        .legal_moves()
        .iter()
        .filter_map(|raw_move| {
            let move_filtered = filter_move(raw_move, &filter_options);
            if move_filtered {
                return None;
            }

            let move_verbose = verbose_move_from_raw_move(*raw_move, chess);

            Some(move_verbose)
        })
        .collect();

    legal_moves
}

struct FilterOptions {
    pub square: Option<Square>,
    pub piece: Option<Role>,
}

fn unwrap_filter_options(options: &Option<LegalMovesFilterOptions>) -> FilterOptions {
    let filter_square_option: Option<Square> = match options.as_ref() {
        Some(val) => {
            if let Some(square) = &val.from_square {
                Some(square.to_shakmaty_sq())
            } else {
                None
            }
        }
        None => None,
    };

    let filter_piece_option: Option<Role> = match options.as_ref() {
        Some(val) => {
            let p = val.piece.as_ref();

            if let Some(oo) = p {
                Some(oo.to_shakmaty_piece_role())
            } else {
                None
            }
        }
        None => None,
    };

    FilterOptions {
        square: filter_square_option,
        piece: filter_piece_option,
    }
}

fn filter_move(mov: &Move, options: &FilterOptions) -> bool {
    if let Some(filter_square) = options.square {
        let from = mov.from();

        let Some(from) = from else {
            return true;
        };

        if filter_square != from {
            return true;
        }
    }

    if let Some(filter_piece) = options.piece {
        if mov.role() != filter_piece {
            return true;
        }
    }

    false
}
