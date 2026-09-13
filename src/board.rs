use crate::board::BoardBuilderError::{BadCols, BadDrop};
use crate::board::DropCounterError::{FullCol, GameFinished};
use crate::board::GameState::{Finished, InProgress};
use crate::board::Turn::{P1, P2};
use std::cmp::min;
use std::error::Error;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

const ROWS: usize = 6;
const COLS: usize = 7;

pub struct BoardMasks {
    bit: Vec<u64>,
    win_checks: Vec<Vec<Vec<u64>>>,
}

impl BoardMasks {
    pub fn new() -> Rc<Self> {
        let mut board_masks = Self {
            bit: (0..64).map(|idx| 1 << idx).collect(),
            win_checks: vec![vec![vec![]; COLS]; ROWS],
        };
        board_masks.set_win_checks();
        Rc::new(board_masks)
    }

    fn set_win_checks(&mut self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                /* Note that row will be interpreted as a row on the board where 0 is the bottom.
                 * So a mask added to row 0 in win_checks will be for the bottom of the board, even
                 * though if you try to visualise win_checks as a 2D array row 0 is at the "top".
                 */
                self.add_horizontal_checks(row, col);
                self.add_diagonal1_checks(row, col);
                self.add_diagonal2_checks(row, col);
                self.add_vertical_check(row, col);
            }
        }
    }

    fn add_horizontal_checks(&mut self, row: usize, col: usize) {
        /* Below I've listed what horizontal lines to check based on the column the counter was
         * placed. The lists of numbers on the right are the starting indices for the line - with
         * the other 3 counters for that line being to the right of it (hence the name
         * leftmost_col). With these starting indices, you'll get all the lines that pass through
         * their given column. The messy RangeInclusive below produces the starting indices needed
         * for any given column.
         * Col 0 -> 0
         * Col 1 -> 0, 1
         * Col 2 -> 0, 1, 2
         * Col 3 -> 0, 1, 2, 3
         * Col 4 -> 1, 2, 3
         * Col 5 -> 2, 3
         * Col 6 -> 3
         */
        for leftmost_col in col.saturating_sub(3)..=min(col, 3) {
            self.win_checks[row][col].push(
                self.bit[leftmost_col * 8 + row]
                    | self.bit[(leftmost_col + 1) * 8 + row]
                    | self.bit[(leftmost_col + 2) * 8 + row]
                    | self.bit[(leftmost_col + 3) * 8 + row],
            );
        }
    }

    fn add_diagonal1_checks(&mut self, row: usize, col: usize) {
        /* "row + col" gives these values in the board below (A is 10 and B is 11). This lets you
         * know what bit masks to get for the given row and col.
         * 5 | 5 6 7 8 9 A B
         * 4 | 4 5 6 7 8 9 A
         * 3 | 3 4 5 6 7 8 9
         * 2 | 2 3 4 5 6 7 8
         * 1 | 1 2 3 4 5 6 7
         * 0 | O 1 2 3 4 5 6
         *   +--------------
         *     0 1 2 3 4 5 6
         */

        let checks = &mut self.win_checks[row][col];
        let bit = &self.bit;
        match row + col {
            3 => checks.push(bit[3] | bit[10] | bit[17] | bit[24]),
            8 => checks.push(bit[29] | bit[36] | bit[43] | bit[50]),
            4 => {
                if col < 4 {
                    checks.push(bit[4] | bit[11] | bit[18] | bit[25]);
                }
                if col > 0 {
                    checks.push(bit[11] | bit[18] | bit[25] | bit[32]);
                }
            }
            7 => {
                if col < 6 {
                    checks.push(bit[21] | bit[28] | bit[35] | bit[42]);
                }
                if col > 2 {
                    checks.push(bit[28] | bit[35] | bit[42] | bit[49]);
                }
            }
            5 => {
                if col < 4 {
                    checks.push(bit[5] | bit[12] | bit[19] | bit[26]);
                }
                if col > 0 && col < 5 {
                    checks.push(bit[12] | bit[19] | bit[26] | bit[33]);
                }
                if col > 1 {
                    checks.push(bit[19] | bit[26] | bit[33] | bit[40]);
                }
            }
            6 => {
                if col < 5 {
                    checks.push(bit[13] | bit[20] | bit[27] | bit[34]);
                }
                if col > 1 && col < 6 {
                    checks.push(bit[20] | bit[27] | bit[34] | bit[41]);
                }
                if col > 2 {
                    checks.push(bit[27] | bit[34] | bit[41] | bit[48]);
                }
            }
            _ => (),
        }
    }

    fn add_diagonal2_checks(&mut self, row: usize, col: usize) {
        /* "(ROWS - row - 1) + col" gives these values in the board below (A is 10 and B is 11).
         * This lets you know what bit masks to get for the given row and col. Note that for these
         * diagonals the rows have been reversed so that the sums go along the right lines.
         * 0 | O 1 2 3 4 5 6
         * 1 | 1 2 3 4 5 6 7
         * 2 | 2 3 4 5 6 7 8
         * 3 | 3 4 5 6 7 8 9
         * 4 | 4 5 6 7 8 9 A
         * 5 | 5 6 7 8 9 A B
         *   +--------------
         *     0 1 2 3 4 5 6
         */

        let checks = &mut self.win_checks[row][col];
        let bit = &self.bit;
        match (ROWS - row - 1) + col {
            3 => checks.push(bit[2] | bit[11] | bit[20] | bit[29]),
            8 => checks.push(bit[24] | bit[33] | bit[42] | bit[51]),
            4 => {
                if col < 4 {
                    checks.push(bit[1] | bit[10] | bit[19] | bit[28]);
                }
                if col > 0 {
                    checks.push(bit[10] | bit[19] | bit[28] | bit[37]);
                }
            }
            7 => {
                if col < 6 {
                    checks.push(bit[16] | bit[25] | bit[34] | bit[43]);
                }
                if col > 2 {
                    checks.push(bit[25] | bit[34] | bit[43] | bit[52]);
                }
            }
            5 => {
                if col < 4 {
                    checks.push(bit[0] | bit[9] | bit[18] | bit[27]);
                }
                if col > 0 && col < 5 {
                    checks.push(bit[9] | bit[18] | bit[27] | bit[36]);
                }
                if col > 1 {
                    checks.push(bit[18] | bit[27] | bit[36] | bit[45]);
                }
            }
            6 => {
                if col < 5 {
                    checks.push(bit[8] | bit[17] | bit[26] | bit[35]);
                }
                if col > 1 && col < 6 {
                    checks.push(bit[17] | bit[26] | bit[35] | bit[44]);
                }
                if col > 2 {
                    checks.push(bit[26] | bit[35] | bit[44] | bit[53]);
                }
            }
            _ => (),
        }
    }

    fn add_vertical_check(&mut self, row: usize, col: usize) {
        // You only need to check down so if there's no space then we can return early.
        if row < 3 {
            return;
        }

        let bit_pos = col * 8 + row;
        self.win_checks[row][col].push(
            self.bit[bit_pos]
                | self.bit[bit_pos - 1]
                | self.bit[bit_pos - 2]
                | self.bit[bit_pos - 3],
        );
    }
}

pub struct BoardBuilder {
    board_masks: Rc<BoardMasks>,
}

#[derive(Debug)]
pub enum BoardBuilderError {
    BadCols,
    BadDrop(DropCounterError),
}

#[derive(Debug)]
pub enum DropCounterError {
    GameFinished,
    FullCol,
}

impl Display for BoardBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BadCols => write!(f, "build failed as 1 or more columns were bigger than 6"),
            BadDrop(_) => write!(f, "build failed as drop_counter failed"),
        }
    }
}

impl Error for BoardBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BadCols => None,
            BadDrop(err) => Some(err),
        }
    }
}

impl Display for DropCounterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameFinished => write!(f, "drop_counter failed as game is over"),
            FullCol => write!(f, "drop_counter failed as column was full"),
        }
    }
}

impl Error for DropCounterError {}

impl BoardBuilder {
    pub fn new(board_masks: Rc<BoardMasks>) -> Self {
        Self { board_masks }
    }

    pub fn build(self) -> Board {
        Board::new(self.board_masks)
    }

    pub fn build_with(self, moves: Vec<usize>) -> Result<Board, BoardBuilderError> {
        let mut board = self.build();

        for col in moves {
            let board_col = BoardCol::new(col).ok_or(BadCols)?;
            board.drop_counter(board_col).map_err(BadDrop)?;
        }

        Ok(board)
    }
}

#[derive(Clone)]
pub struct Board {
    p1_bits: u64,
    p2_bits: u64,
    column_heights: [usize; COLS],
    turn: Turn,
    state: GameState,
    masks: Rc<BoardMasks>,
}

#[derive(Copy, Clone)]
pub enum Turn {
    P1,
    P2,
}

#[derive(Debug, Copy, Clone)]
pub enum GameState {
    InProgress,
    Finished(i8),
}

#[derive(Copy, Clone)]
pub struct BoardCol(usize);

impl BoardCol {
    pub fn new(col: usize) -> Option<BoardCol> {
        if col < COLS {
            Some(BoardCol(col))
        } else {
            None
        }
    }
}

impl Board {
    fn new(board_masks: Rc<BoardMasks>) -> Board {
        Board {
            p1_bits: 0,
            p2_bits: 0,
            column_heights: [0; COLS],
            turn: P1,
            state: InProgress,
            masks: board_masks,
        }
    }

    pub fn get_turn(&self) -> Turn {
        self.turn
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn get_all_moves(&self) -> Vec<Self> {
        let mut moves = Vec::with_capacity(7);
        for col in 0..COLS {
            let mut new_board = self.clone();
            if new_board.drop_counter_helper(col).is_ok() {
                Board::insert_board(&mut moves, new_board);
            }
        }

        moves
    }

    pub fn drop_counter(&mut self, board_col: BoardCol) -> Result<(), DropCounterError> {
        self.drop_counter_helper(board_col.0)
    }

    fn drop_counter_helper(&mut self, col: usize) -> Result<(), DropCounterError> {
        let InProgress = self.state else {
            return Err(GameFinished);
        };

        let row = self.column_heights[col];
        if row == ROWS {
            Err(FullCol)
        } else {
            match self.turn {
                P1 => self.p1_bits |= self.masks.bit[col * 8 + row],
                P2 => self.p2_bits |= self.masks.bit[col * 8 + row],
            }
            self.column_heights[col] += 1;
            self.update_state(row, col);

            Ok(())
        }
    }

    fn update_state(&mut self, row: usize, col: usize) {
        if self.has_completed_line(row, col) {
            match self.turn {
                P1 => self.state = Finished(1),
                P2 => self.state = Finished(-1),
            }
        } else if (self.p1_bits | self.p2_bits) == 0x003F_3F3F_3F3F_3F3F {
            self.state = Finished(0);
        } else {
            self.flip_turn();
        }
    }

    fn has_completed_line(&self, row: usize, col: usize) -> bool {
        let checks = &self.masks.win_checks[row][col];
        let bit_board = match self.turn {
            P1 => self.p1_bits,
            P2 => self.p2_bits,
        };

        #[allow(clippy::manual_contains)]
        checks.iter().any(|&check| (bit_board & check) == check)
    }

    fn flip_turn(&mut self) {
        self.turn = match self.turn {
            P1 => P2,
            P2 => P1,
        }
    }

    fn insert_board(moves: &mut Vec<Board>, new_board: Board) {
        if let InProgress = new_board.state {
            moves.push(new_board);
            return;
        }

        moves.push(new_board);
        for idx in (1..moves.len()).rev() {
            match moves[idx - 1].state {
                InProgress => moves.swap(idx, idx - 1),
                Finished(_) => return,
            }
        }
    }

    fn get_tile_as_char(&self, row: usize, col: usize) -> char {
        let bit = self.masks.bit[col * 8 + row];
        if (self.p1_bits & bit) == bit {
            'X'
        } else if (self.p2_bits & bit) == bit {
            'O'
        } else {
            ' '
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board_str = "\n".to_string();
        for row in (0..ROWS).rev() {
            let _ = write!(board_str, "{} |", row + 1);
            for col in 0..COLS {
                let _ = write!(board_str, " {} ", self.get_tile_as_char(row, col));
            }

            board_str += "\n";
        }

        board_str += "  +---------------------\n    A  B  C  D  E  F  G\n";
        write!(f, "{board_str}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;

    fn set_up_board(moves: Vec<usize>) -> Board {
        let board_masks = BoardMasks::new();
        #[allow(clippy::unwrap_used)]
        BoardBuilder::new(board_masks).build_with(moves).unwrap()
    }

    #[test]
    fn get_all_moves_on_empty_board() {
        let board = BoardBuilder::new(BoardMasks::new()).build();
        let moves = board.get_all_moves();
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn get_all_moves_on_non_empty_board_with_no_full_columns() {
        let board = set_up_board((0..COLS).collect());
        let moves = board.get_all_moves();
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn get_all_moves_with_a_full_column() {
        let board = set_up_board(vec![0; ROWS]);
        let moves = board.get_all_moves();
        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn get_all_moves_with_multiple_full_columns() {
        let board = set_up_board([[0; ROWS], [2; ROWS], [6; ROWS]].concat());
        let moves = board.get_all_moves();
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn drop_counter_on_full_column() {
        let mut board = set_up_board(vec![0; ROWS]);
        assert_matches!(board.drop_counter_helper(0), Err(FullCol));
    }

    #[test]
    fn drop_counter_leading_to_p1_win() {
        let board = set_up_board(vec![0, 0, 1, 1, 2, 2, 3]);
        assert_matches!(board.state, Finished(1));
    }

    #[test]
    fn drop_counter_leading_to_p2_win() {
        let board = set_up_board(vec![0, 2, 2, 3, 3, 4, 4, 5]);
        assert_matches!(board.state, Finished(-1));
    }

    #[test]
    fn drop_counter_leading_to_draw() {
        let board = set_up_board(
            [
                vec![0; ROWS],
                vec![1; ROWS],
                vec![2; ROWS],
                vec![4],
                vec![3; ROWS],
                vec![4; ROWS - 1],
                vec![5; ROWS],
                vec![6; ROWS],
            ]
            .concat(),
        );
        assert_matches!(board.state, Finished(0));
    }

    #[test]
    fn drop_counter_when_game_already_finished() {
        let mut board = set_up_board(vec![0, 2, 2, 3, 3, 4, 4, 5]);
        assert_matches!(board.drop_counter_helper(0), Err(GameFinished));
    }

    #[test]
    fn visually_confirm_board_masks_win_checks() {
        let board_masks = BoardMasks::new();
        for (row_idx, row) in board_masks.win_checks.iter().enumerate() {
            for (col_idx, checks) in row.iter().enumerate() {
                display_check(checks, &board_masks, row_idx, col_idx);
            }
        }
    }

    fn display_check(checks: &Vec<u64>, board_masks: &Rc<BoardMasks>, row: usize, col: usize) {
        println!("================================================================");
        for check in checks {
            let mut board = Board::new(board_masks.clone());
            board.p1_bits |= check;
            board.p1_bits ^= board.masks.bit[col * 8 + row];
            board.p2_bits |= board.masks.bit[col * 8 + row];
            println!("{board}");
        }
    }
}
