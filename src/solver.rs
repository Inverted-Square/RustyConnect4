use crate::board::{Board, GameState::*, Turn::*};
use std::cmp::{max, min};

const MAX_DEPTH: u8 = 19;

pub fn computer_move(board: &Board) -> Result<Board, &str> {
    let mut alpha = -1;
    let mut beta = 1;
    let mut best_eval = match board.get_turn() {
        P1 => i8::MIN,
        P2 => i8::MAX,
    };
    let mut best_board = Err("The game is over so the computer can't move.");
    for new_board in board.get_all_moves() {
        let eval = minimax(&new_board, MAX_DEPTH - 1, alpha, beta);
        println!("{new_board}Eval: {}\n\n", eval);
        match board.get_turn() {
            P1 => {
                if eval > best_eval {
                    best_board = Ok(new_board);
                    best_eval = eval;
                }
                alpha = max(alpha, eval);
            }
            P2 => {
                if eval < best_eval {
                    best_board = Ok(new_board);
                    best_eval = eval;
                }
                beta = min(beta, eval);
            }
        }

        if beta <= alpha {
            break;
        }
    }

    best_board
}

fn minimax(position: &Board, depth: u8, mut alpha: i8, mut beta: i8) -> i8 {
    if let Finished(eval) = position.get_state() {
        return eval * (depth as i8 + 1);
    }
    if depth == 0 {
        return 0;
    }

    let mut best_eval = match position.get_turn() {
        P1 => i8::MIN,
        P2 => i8::MAX,
    };
    for child in position.get_all_moves() {
        let eval = minimax(&child, depth - 1, alpha, beta);
        match position.get_turn() {
            P1 => {
                best_eval = max(best_eval, eval);
                alpha = max(alpha, eval);
            }
            P2 => {
                best_eval = min(best_eval, eval);
                beta = min(beta, eval)
            }
        }

        if beta <= alpha {
            break;
        }
    }

    best_eval
}
