use crate::board::{Board, BoardCol, BoardMasks};
use crate::solver::computer_move;

mod board;
mod solver;

fn main() {
    let board_masks = BoardMasks::new();
    let mut board = Board::new(board_masks.clone());

    [3, 3, 4, 4]
        .into_iter()
        .map(|col| BoardCol::new(col).unwrap())
        .for_each(|board_col| assert!(board.drop_counter(board_col)));

    println!("{}", computer_move(&board).unwrap());
}
