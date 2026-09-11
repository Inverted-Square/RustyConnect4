use crate::board::GameState::*;
use crate::board::Turn::*;
use std::cmp::min;
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

#[derive(Clone)]
pub struct Board {
    p1_board: u64,
    p2_board: u64,
    column_heights: [usize; COLS],
    turn: Turn,
    state: GameState,
    board_masks: Rc<BoardMasks>,
}

#[derive(Clone)]
enum Turn {
    P1,
    P2,
}

#[derive(Debug, Copy, Clone)]
pub enum GameState {
    InProgress,
    Draw,
    P1Win,
    P2Win,
}

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
    pub fn new(board_masks: Rc<BoardMasks>) -> Board {
        Board {
            p1_board: 0,
            p2_board: 0,
            column_heights: [0; COLS],
            turn: P1,
            state: InProgress,
            board_masks,
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn get_all_moves(&self) -> Vec<Self> {
        let mut moves = Vec::with_capacity(7);
        for col in 0..COLS {
            let mut new_board = self.clone();
            if new_board.drop_counter_helper(col) {
                moves.push(new_board);
            }
        }

        moves
    }

    pub fn drop_counter(&mut self, board_col: BoardCol) -> bool {
        self.drop_counter_helper(board_col.0)
    }

    fn drop_counter_helper(&mut self, col: usize) -> bool {
        let InProgress = self.state else { return false };

        let row = self.column_heights[col];
        if row == ROWS {
            false
        } else {
            match self.turn {
                P1 => self.p1_board |= self.board_masks.bit[col * 8 + row],
                P2 => self.p2_board |= self.board_masks.bit[col * 8 + row],
            }
            self.column_heights[col] += 1;
            self.update_state(row, col);

            true
        }
    }

    fn update_state(&mut self, row: usize, col: usize) {
        if self.has_completed_line(row, col) {
            match self.turn {
                P1 => self.state = P1Win,
                P2 => self.state = P2Win,
            }
        } else if (self.p1_board | self.p2_board) == 0x003F3F3F3F3F3F3F {
            self.state = Draw;
        } else {
            self.flip_turn();
        }
    }

    fn has_completed_line(&self, row: usize, col: usize) -> bool {
        let checks = &self.board_masks.win_checks[row][col];
        let bit_board = match self.turn {
            P1 => self.p1_board,
            P2 => self.p2_board,
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

    fn get_tile_as_char(&self, row: usize, col: usize) -> char {
        let bit = self.board_masks.bit[col * 8 + row];
        if (self.p1_board & bit) == bit {
            'X'
        } else if (self.p2_board & bit) == bit {
            'O'
        } else {
            ' '
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board_str = String::from("\n");
        for row in (0..ROWS).rev() {
            board_str += &format!("{} |", row + 1);
            for col in 0..COLS {
                board_str += &format!(" {} ", self.get_tile_as_char(row, col));
            }

            board_str += "\n";
        }

        board_str += "  +---------------------\n    A  B  C  D  E  F  G\n";
        write!(f, "{}", board_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;

    fn set_up_board(moves: Vec<usize>) -> Board {
        let board_masks = BoardMasks::new();
        let mut board = Board::new(board_masks);

        moves
            .into_iter()
            .map(|col| BoardCol::new(col).unwrap())
            .for_each(|board_col| assert!(board.drop_counter(board_col)));

        board
    }

    #[test]
    fn get_all_moves_on_empty_board() {
        let board = set_up_board(vec![]);
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
        assert!(!board.drop_counter_helper(0));
    }

    #[test]
    fn drop_counter_leading_to_p1_win() {
        let board = set_up_board(vec![0, 0, 1, 1, 2, 2, 3]);
        assert_matches!(board.state, P1Win);
    }

    #[test]
    fn drop_counter_leading_to_p2_win() {
        let board = set_up_board(vec![0, 2, 2, 3, 3, 4, 4, 5]);
        assert_matches!(board.state, P2Win);
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
        assert_matches!(board.state, Draw);
    }

    #[test]
    fn drop_counter_when_game_already_finished() {
        let mut board = set_up_board(vec![0, 2, 2, 3, 3, 4, 4, 5]);
        assert!(!board.drop_counter_helper(0));
    }

    #[test]
    fn visually_confirm_board_masks_win_checks() {
        let board_masks = BoardMasks::new();
        for (row_idx, row) in board_masks.win_checks.iter().enumerate() {
            for (col_idx, checks) in row.iter().enumerate() {
                println!("================================================================");
                for check in checks {
                    let mut board = Board::new(board_masks.clone());
                    board.p1_board |= check;
                    board.p1_board ^= board_masks.bit[col_idx * 8 + row_idx];
                    board.p2_board |= board_masks.bit[col_idx * 8 + row_idx];
                    println!("{board}");
                }
            }
        }
    }
}
