use crate::board::{BoardBuilder, BoardMasks};
use crate::solver::computer_move;
use std::error::Error;

mod board;
mod solver;

fn main() {
    let board_masks = BoardMasks::new();
    let board_result = BoardBuilder::new(board_masks).build_with(vec![2, 2, 3, 3]);

    match board_result {
        Ok(board) => match computer_move(&board) {
            Ok(new_board) => println!("{new_board}"),
            Err(msg) => println!("{msg}"),
        },
        Err(err) => {
            println!("Error: {err}");
            if let Some(reason) = err.source() {
                println!("Caused by: {reason}");
            }
        }
    }
}
