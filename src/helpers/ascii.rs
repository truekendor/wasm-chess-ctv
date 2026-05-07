use shakmaty::{Board, Square};

pub fn from_board(board: &Board) -> String {
    let border: &str = "   +------------------------+\n";
    let letters: &str = "     a  b  c  d  e  f  g  h\n";
    let end_of_board_str = "|\n";
    let mut ascii_str = String::with_capacity(328);

    ascii_str.push_str(border);

    for rank in (0..8).rev() {
        ascii_str.push_str(&format!(" {} |", rank + 1));

        for file in 0..8 {
            let sq = Square::from_coords(
                shakmaty::File::new(file as u32),
                shakmaty::Rank::new(rank as u32),
            );

            let piece = board.piece_at(sq);

            match piece {
                Some(p) => {
                    let symbol = p.char();
                    ascii_str.push(' ');
                    ascii_str.push(symbol);
                    ascii_str.push(' ');
                }
                None => {
                    ascii_str.push_str(" . ");
                }
            }
        }

        ascii_str.push_str(&end_of_board_str);
    }

    ascii_str.push_str(border);
    ascii_str.push_str(letters.trim_end());

    ascii_str
}
