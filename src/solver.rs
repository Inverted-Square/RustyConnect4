use crate::board::{
    Board,
    GameState::Finished,
    Turn::{P1, P2},
};
use rustc_hash::FxHashMap;
use std::cmp::{max, min};

const MAX_DEPTH: u8 = if cfg!(debug_assertions) { 18 } else { 20 };

pub fn computer_move(board: &Board) -> Result<Board, &str> {
    let mut alpha = -1;
    let mut beta = 1;
    let mut transpositions = FxHashMap::default();

    let mut best_eval = match board.get_turn() {
        P1 => i8::MIN,
        P2 => i8::MAX,
    };
    let mut best_board = Err("The game is over so the computer can't move.");
    for new_board in board.get_all_moves() {
        let eval = minimax(&new_board, MAX_DEPTH - 1, alpha, beta, &mut transpositions);
        println!("{new_board}Eval: {eval}\n\n");
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

fn minimax(
    board: &Board,
    depth: u8,
    mut alpha: i8,
    mut beta: i8,
    transpositions: &mut FxHashMap<u64, i8>,
) -> i8 {
    if let Finished(eval) = board.get_state() {
        return eval * (depth.cast_signed() + 1);
    }
    if depth == 0 {
        return 0;
    }

    let mut best_eval = match board.get_turn() {
        P1 => i8::MIN,
        P2 => i8::MAX,
    };
    for new_board in board.get_all_moves() {
        let eval = match transpositions.get(&new_board.get_hash()) {
            None => {
                let found_eval = minimax(&new_board, depth - 1, alpha, beta, transpositions);
                let _ = transpositions.insert(new_board.get_hash(), found_eval);
                found_eval
            }
            Some(&stored_eval) => stored_eval,
        };

        match board.get_turn() {
            P1 => {
                best_eval = max(best_eval, eval);
                alpha = max(alpha, eval);
            }
            P2 => {
                best_eval = min(best_eval, eval);
                beta = min(beta, eval);
            }
        }

        if beta <= alpha {
            break;
        }
    }

    best_eval
}

pub mod bench {
    use super::{Board, MAX_DEPTH, computer_move};
    use std::time::{Duration, Instant};

    pub fn bench_solver() {
        let num_bench_tests = 10;
        let sum1 = (1..=num_bench_tests)
            .map(bench_on_empty_board)
            .sum::<Duration>();
        let sum2 = (1..=num_bench_tests)
            .map(bench_on_board_with_forced_win)
            .sum::<Duration>();
        println!("Empty board average - {:?}", sum1 / num_bench_tests);
        println!(
            "Board with forced win average - {:?}",
            sum2 / num_bench_tests
        );
    }

    fn bench_on_empty_board(bench_num: u32) -> Duration {
        let board = Board::default();
        let start = Instant::now();
        let _ = computer_move(&board);
        let duration = start.elapsed();
        println!("{bench_num}. Bench on empty board at depth {MAX_DEPTH} took {duration:?}.");
        duration
    }

    fn bench_on_board_with_forced_win(bench_num: u32) -> Duration {
        let board = match Board::new(vec![3, 3, 4, 4]) {
            Ok(board) => board,
            Err(err) => {
                println!("Error: {err}");
                return Duration::default();
            }
        };

        let start = Instant::now();
        let _ = computer_move(&board);
        let duration = start.elapsed();
        println!(
            "{bench_num}. Bench on board with forced win at depth {MAX_DEPTH} took {duration:?}."
        );
        duration
    }
}
