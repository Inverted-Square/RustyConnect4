use crate::board::{COLS, ROWS};
use crate::group_info::GroupInfo;
use rustc_hash::FxHashMap;
use std::array;
use std::cmp::{max, min};
use std::fmt::Write;
use std::fmt::{Display, Formatter};

pub struct BoardInfo {
    bit: [u64; 64],
    max_viable_groups: [[u8; COLS]; ROWS],
    groups_at: [[Vec<GroupInfo>; COLS]; ROWS],
    group_indices: FxHashMap<u64, usize>,
    curr_group_idx: usize,
}

impl Default for BoardInfo {
    fn default() -> Self {
        let mut board_masks = Self {
            bit: array::from_fn(|idx| 1 << idx),
            max_viable_groups: Default::default(),
            groups_at: Default::default(),
            group_indices: FxHashMap::default(),
            curr_group_idx: 0,
        };
        board_masks.set_groups_at();
        board_masks
    }
}

impl BoardInfo {
    fn set_groups_at(&mut self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                /* Note that row will be interpreted as a row on the board where 0 is the bottom.
                 * So a mask added to row 0 in groups_at will be for the bottom of the board, even
                 * though if you try to visualise groups_at as a 2D array row 0 is at the "top".
                 */
                self.add_horizontal_groups(row, col);
                self.add_diagonal1_groups(row, col);
                self.add_diagonal2_groups(row, col);
                self.add_vertical_check(row, col);
            }
        }
    }

    fn add_group(&mut self, row: usize, col: usize, positions: [[usize; 2]; 4]) {
        let mask = self.mask_from(positions);
        let idx = self.idx_from(mask);
        self.max_viable_groups[row][col] += 1;
        self.groups_at[row][col].push(GroupInfo {
            mask,
            idx,
            positions,
        });
    }

    fn mask_from(&self, positions: [[usize; 2]; 4]) -> u64 {
        positions
            .iter()
            .fold(0, |mask, &[row, col]| mask | self.bit[8 * col + row])
    }

    fn idx_from(&mut self, mask: u64) -> usize {
        match self.group_indices.get(&mask) {
            None => {
                let _ = self.group_indices.insert(mask, self.curr_group_idx);
                self.curr_group_idx += 1;
                self.curr_group_idx - 1
            }
            Some(&found_idx) => found_idx,
        }
    }

    fn add_horizontal_groups(&mut self, row: usize, col: usize) {
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
        let lower_bound = col.saturating_sub(3);
        let upper_bound = min(col, 3);
        for leftmost_col in lower_bound..=upper_bound {
            self.add_group(row, col, array::from_fn(|idx| [row, leftmost_col + idx]));
        }
    }

    fn add_diagonal1_groups(&mut self, row: usize, col: usize) {
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

        match row + col {
            3 => self.add_group(row, col, array::from_fn(|idx| [3 - idx, idx])),
            8 => self.add_group(row, col, array::from_fn(|idx| [5 - idx, 3 + idx])),
            4 => {
                if col < 4 {
                    self.add_group(row, col, array::from_fn(|idx| [4 - idx, idx]));
                }
                if col > 0 {
                    self.add_group(row, col, array::from_fn(|idx| [3 - idx, 1 + idx]));
                }
            }
            7 => {
                if col < 6 {
                    self.add_group(row, col, array::from_fn(|idx| [5 - idx, 2 + idx]));
                }
                if col > 2 {
                    self.add_group(row, col, array::from_fn(|idx| [4 - idx, 3 + idx]));
                }
            }
            5 => {
                if col < 4 {
                    self.add_group(row, col, array::from_fn(|idx| [5 - idx, idx]));
                }
                if col > 0 && col < 5 {
                    self.add_group(row, col, array::from_fn(|idx| [4 - idx, 1 + idx]));
                }
                if col > 1 {
                    self.add_group(row, col, array::from_fn(|idx| [3 - idx, 2 + idx]));
                }
            }
            6 => {
                if col < 5 {
                    self.add_group(row, col, array::from_fn(|idx| [5 - idx, 1 + idx]));
                }
                if col > 1 && col < 6 {
                    self.add_group(row, col, array::from_fn(|idx| [4 - idx, 2 + idx]));
                }
                if col > 2 {
                    self.add_group(row, col, array::from_fn(|idx| [3 - idx, 3 + idx]));
                }
            }
            _ => (),
        }
    }

    fn add_diagonal2_groups(&mut self, row: usize, col: usize) {
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

        match (ROWS - row - 1) + col {
            3 => self.add_group(row, col, array::from_fn(|idx| [2 + idx, idx])),
            8 => self.add_group(row, col, array::from_fn(|idx| [idx, 3 + idx])),
            4 => {
                if col < 4 {
                    self.add_group(row, col, array::from_fn(|idx| [1 + idx, idx]));
                }
                if col > 0 {
                    self.add_group(row, col, array::from_fn(|idx| [2 + idx, 1 + idx]));
                }
            }
            7 => {
                if col < 6 {
                    self.add_group(row, col, array::from_fn(|idx| [idx, 2 + idx]));
                }
                if col > 2 {
                    self.add_group(row, col, array::from_fn(|idx| [1 + idx, 3 + idx]));
                }
            }
            5 => {
                if col < 4 {
                    self.add_group(row, col, array::from_fn(|idx| [idx, idx]));
                }
                if col > 0 && col < 5 {
                    self.add_group(row, col, array::from_fn(|idx| [1 + idx, 1 + idx]));
                }
                if col > 1 {
                    self.add_group(row, col, array::from_fn(|idx| [2 + idx, 2 + idx]));
                }
            }
            6 => {
                if col < 5 {
                    self.add_group(row, col, array::from_fn(|idx| [idx, 1 + idx]));
                }
                if col > 1 && col < 6 {
                    self.add_group(row, col, array::from_fn(|idx| [1 + idx, 2 + idx]));
                }
                if col > 2 {
                    self.add_group(row, col, array::from_fn(|idx| [2 + idx, 3 + idx]));
                }
            }
            _ => (),
        }
    }

    fn add_vertical_check(&mut self, row: usize, col: usize) {
        /* Works under the same logic as add_horizontal_groups but instead using:
         * Row 0 -> 3
         * Row 1 -> 3, 4
         * Row 2 -> 3, 4, 5
         * Row 3 -> 3, 4, 5
         * Row 4 -> 4, 5
         * Row 5 -> 5
         */

        let lower_bound = max(row, 3);
        let upper_bound = min(row + 3, 5);
        for topmost_row in lower_bound..=upper_bound {
            self.add_group(row, col, array::from_fn(|idx| [topmost_row - idx, col]));
        }
    }
}

impl Display for BoardInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut groups_at_str = '['.to_string();
        for row in &self.groups_at {
            groups_at_str.push('[');
            for col in row {
                let _ = write!(groups_at_str, "&{col:?}, ");
            }
            let _ = write!(groups_at_str, "], ");
        }
        groups_at_str.push(']');
        groups_at_str = groups_at_str.replace(", ]", "]");

        write!(
            f,
            "pub const BIT: [u64; 64] = {:?};\n\n\
            pub const GROUPS_AT: [[&[GroupInfo]; COLS]; ROWS] = {groups_at_str};\n\n\
            pub const MAX_VIABLE_GROUPS: [[u8; COLS]; ROWS] = {:?};",
            self.bit, self.max_viable_groups
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn check_group_indices_and_masks() {
        let count_masks = count_info();
        assert_eq!(count_masks.len(), 69);
        assert!(
            count_masks
                .into_iter()
                .enumerate()
                .all(|(idx, (mask_idx, count))| idx == mask_idx && count == 4)
        );
    }

    fn count_info() -> BTreeMap<usize, u8> {
        let board_info = BoardInfo::default();
        let mut count_masks = BTreeMap::default();
        for row in board_info.groups_at {
            for groups in row {
                update_count(&mut count_masks, groups);
            }
        }

        count_masks
    }

    fn update_count(count_masks: &mut BTreeMap<usize, u8>, groups: Vec<GroupInfo>) {
        for group in groups {
            match count_masks.get_mut(&group.idx) {
                None => {
                    let _ = count_masks.insert(group.idx, 1);
                }
                Some(count) => *count += 1,
            }
        }
    }
}
