use crate::board::{
    Board,
    GameState::Finished,
    Turn::{P1, P2},
};
use rustc_hash::FxHashMap;
use std::cmp::{max, min};

const MAX_DEPTH: u8 = if cfg!(debug_assertions) { 17 } else { 20 };

pub struct Solver {
    best_board: Option<Board>,
    transpositions: FxHashMap<u64, i8>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            best_board: None,
            transpositions: FxHashMap::default(),
        }
    }

    pub fn solve(mut self, board: &Board) -> (Option<Board>, i8) {
        let eval = self.minimax(board, MAX_DEPTH, -1, 1);
        (self.best_board, eval)
    }

    fn minimax(&mut self, board: &Board, depth: u8, mut alpha: i8, mut beta: i8) -> i8 {
        if let Finished(eval) = board.get_state() {
            return eval * (depth.cast_signed() + 1);
        } else if depth == 0 {
            return 0;
        }

        let mut best_eval = match board.get_turn() {
            P1 => i8::MIN,
            P2 => i8::MAX,
        };
        for new_board in board.get_all_moves() {
            let eval = self.get_eval(&new_board, depth, alpha, beta);

            #[cfg(debug_assertions)]
            if depth == MAX_DEPTH {
                println!("{new_board}Eval: {eval}\n\n");
            }

            match board.get_turn() {
                P1 => self.update_best_p1(&mut best_eval, eval, new_board, &mut alpha),
                P2 => self.update_best_p2(&mut best_eval, eval, new_board, &mut beta),
            }

            if beta <= alpha {
                break;
            }
        }

        best_eval
    }

    fn get_eval(&mut self, board: &Board, depth: u8, alpha: i8, beta: i8) -> i8 {
        let board_hash = board.get_hash();
        match self.transpositions.get(&board_hash) {
            None => {
                let found_eval = self.minimax(board, depth - 1, alpha, beta);
                let _ = self.transpositions.insert(board_hash, found_eval);
                found_eval
            }
            Some(&stored_eval) => stored_eval,
        }
    }

    fn update_best_p1(&mut self, best_eval: &mut i8, eval: i8, board: Board, alpha: &mut i8) {
        if eval > *best_eval {
            self.best_board = Some(board);
            *best_eval = eval;
        }
        *alpha = max(*alpha, eval);
    }

    fn update_best_p2(&mut self, best_eval: &mut i8, eval: i8, board: Board, beta: &mut i8) {
        if eval < *best_eval {
            self.best_board = Some(board);
            *best_eval = eval;
        }
        *beta = min(*beta, eval);
    }
}

pub mod bench {
    use super::{Board, MAX_DEPTH, Solver};
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
        let _ = Solver::new().solve(&board);
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
        let _ = Solver::new().solve(&board);
        let duration = start.elapsed();
        println!(
            "{bench_num}. Bench on board with forced win at depth {MAX_DEPTH} took {duration:?}."
        );
        duration
    }
}
