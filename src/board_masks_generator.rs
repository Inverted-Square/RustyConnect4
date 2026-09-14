use crate::board::{COLS, ROWS};
use std::cmp::min;
use std::fmt::Write;
use std::fmt::{Display, Formatter};

pub struct BoardMasks {
    bit: Vec<u64>,
    win_checks: Vec<Vec<Vec<u64>>>,
}

impl BoardMasks {
    pub fn new() -> Self {
        let mut board_masks = Self {
            bit: (0..64).map(|idx| 1 << idx).collect(),
            win_checks: vec![vec![vec![]; COLS]; ROWS],
        };
        board_masks.set_win_checks();
        board_masks
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

impl Display for BoardMasks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut win_checks_str = '['.to_string();
        for row in &self.win_checks {
            win_checks_str.push('[');
            for col in row {
                let _ = write!(win_checks_str, "&{col:?}, ");
            }
            let _ = write!(win_checks_str, "], ");
        }
        win_checks_str.push(']');

        write!(f, "BIT: {:?}\n\nWIN_CHECKS: {:?}", self.bit, win_checks_str)
    }
}
