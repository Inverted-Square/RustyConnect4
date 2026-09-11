use crate::board::{Board, BoardCol, BoardMasks};

mod board;

fn main() {
    let board_masks = BoardMasks::new();
    let mut board = Board::new(board_masks.clone());
    if let Some(col) = BoardCol::new(5) {
        let _ = board.drop_counter(col);
    }
    let _ = (board.get_state(), board.get_all_moves());
}
