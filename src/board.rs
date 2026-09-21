use crate::board::BoardBuildError::{BadCols, BadDrop};
use crate::board::DropCounterError::{FullCol, GameFinished};
use crate::board::GameState::{Finished, InProgress};
use crate::board::Turn::{P1, P2};
use crate::group_info::{BIT, GROUPS_AT, GroupInfo, MAX_VIABLE_GROUPS};
use std::error::Error;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

pub const ROWS: usize = 6;
pub const COLS: usize = 7;

#[rustfmt::skip]
#[allow(clippy::unreadable_literal, reason = "They're just random numbers for hashing")]
const ZOBRIST_TABLE: [[[u64; 3]; COLS]; ROWS] = [[[14467863554011360374, 1306305715986539766, 10313488828748596837], [10823040062293105727, 9445332653814076136, 17027033093867909346], [2870399387635805161, 11021981960998498193, 1557043218729819308], [17530985080018037896, 16465974501512209552, 15579019495528923823], [5902138050431450600, 13146266801980075641, 17171532108672350900], [1420615368455098391, 5558206462434011959, 9399560847331081106], [16273344332483653419, 12006433048780278548, 5100869149839971723]], [[12902603856754900138, 3398297685039902079, 14619118835157792401], [10352013050422029603, 2731913348924119127, 4384565040983482094], [1984787515367938936, 11034021595238351854, 12633910550206477660], [17859452642366121666, 1098874429297200482, 13391712797096232892], [15369192472178982583, 5482849505560229970, 1057649968720427948], [2808791203213453456, 12266281145433231804, 195864309634654521], [2427942112948442599, 17125260241916896179, 4250788404808670315]], [[1346210601603686334, 3925923772626528100, 8408373369704071240], [11625689047224779362, 13070226940267544811, 1473233949867293815], [15495420455773790658, 6602879888701473114, 3038270026720475378], [13523342761800388088, 14568689432620802144, 3176503811488871965], [13682409642971244612, 11406003442373990080, 5713932354088894819], [16795216263640460671, 9749871826320659119, 18414520121030768393], [6613079673536264440, 10261638381650145386, 15714439616140652455]], [[16799208415941727214, 11736901474296474237, 5047697476181468745], [10671249046586962585, 2190459955336132266, 4150777787148804500], [8881105332469164746, 9093053119033184625, 7011098912710878184], [11305927192818273504, 11549224246407113107, 12961688854519507447], [3062890379498563381, 15871614864347350643, 3920375217095702857], [17091090299254761474, 2070444891820646605, 8078232590718042965], [2930025323322439921, 1066483056088604934, 11834443386007476369]], [[13541806787688097182, 17736380006571299518, 17742986889267151476], [14675622674337992741, 17974789490400220280, 8088037408765040115], [16987955088400049595, 17778700580438597716, 13431066890844402948], [3617956175701671817, 10189115425719487855, 2177129978683208552], [362877029488469745, 16230117191096584712, 1477442220235725828], [12387641826638612173, 11145192649276051850, 11507526987473029097], [812699629438988037, 6869580767186182889, 12607365011378302522]], [[10594896126131317334, 3388187030230247949, 4456410783895302538], [14382948374536409533, 14889968156195750967, 16941491645406341238], [2764133403231254815, 2338022348326815686, 14612649862369342171], [5093513187741650247, 18161670974083835557, 3267507821544480331], [8150196694838527327, 1243747126153854746, 8503298842088664921], [10056837831385776215, 1912677869684960166, 13733596380921494022], [3307984049950914339, 1659162724304211391, 10647336039792343649]]];

#[derive(Clone, Default)]
pub struct Board {
    player_bits: BitBoard,
    column_heights: [u8; COLS],
    turn: Turn,
    state: GameState,
    indifferent_info: IndifferentSetsInfo,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct BitBoard {
    p1_bits: u64,
    p2_bits: u64,
    indifferent_bits: u64,
    hash_val: u64,
}

impl PartialEq<Self> for BitBoard {
    fn eq(&self, other: &Self) -> bool {
        self.p1_bits == other.p1_bits
            && self.p2_bits == other.p2_bits
            && self.indifferent_bits == other.indifferent_bits
    }
}

impl Eq for BitBoard {}

impl Hash for BitBoard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash_val);
    }
}

#[derive(Copy, Clone, Default)]
pub enum Turn {
    #[default]
    P1,
    P2,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum GameState {
    #[default]
    InProgress,
    Finished(i8),
}

#[derive(Clone)]
struct IndifferentSetsInfo {
    group_counts: [[u8; COLS]; ROWS],
    removed_groups: (u8, u64),
}

impl Default for IndifferentSetsInfo {
    fn default() -> Self {
        Self {
            group_counts: MAX_VIABLE_GROUPS,
            removed_groups: (0, 0),
        }
    }
}

impl Board {
    pub fn new(moves: Vec<usize>) -> Result<Self, BoardBuildError> {
        let mut board = Board::default();
        for col in moves {
            let board_col = BoardCol::new(col).ok_or(BadCols)?;
            board.drop_counter(board_col).map_err(BadDrop)?;
        }

        Ok(board)
    }

    pub fn get_player_bits(&self) -> BitBoard {
        self.player_bits
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

        let row = self.column_heights[col] as usize;
        if row == ROWS {
            Err(FullCol)
        } else {
            match self.turn {
                P1 => self.player_bits.p1_bits |= BIT[col * 8 + row],
                P2 => self.player_bits.p2_bits |= BIT[col * 8 + row],
            }
            self.column_heights[col] += 1;
            self.player_bits.hash_val ^= ZOBRIST_TABLE[row][col][self.turn as usize];
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
        } else if (self.player_bits.p1_bits
            | self.player_bits.p2_bits
            | self.player_bits.indifferent_bits)
            == 0x003F_3F3F_3F3F_3F3F
        {
            self.state = Finished(0);
        } else {
            self.flip_turn();
        }
    }

    fn has_completed_line(&mut self, row: usize, col: usize) -> bool {
        let (bit_board, other_board) = match self.turn {
            P1 => (self.player_bits.p1_bits, self.player_bits.p2_bits),
            P2 => (self.player_bits.p2_bits, self.player_bits.p1_bits),
        };

        if self.indifferent_info.group_counts[row][col] == 0 {
            match self.turn {
                P1 => {
                    self.player_bits.p1_bits ^= BIT[col * 8 + row];
                    self.player_bits.hash_val ^= ZOBRIST_TABLE[row][col][0];
                }
                P2 => {
                    self.player_bits.p2_bits ^= BIT[col * 8 + row];
                    self.player_bits.hash_val ^= ZOBRIST_TABLE[row][col][1];
                }
            }
            self.player_bits.indifferent_bits |= BIT[col * 8 + row];
            self.player_bits.hash_val ^= ZOBRIST_TABLE[row][col][2];
            return false;
        }

        let mut found = false;
        for group in GROUPS_AT[row][col] {
            let &GroupInfo {
                mask,
                idx,
                positions,
            } = group;

            if (bit_board & mask) == mask {
                found = true;
            } else if (other_board & mask) != 0 {
                self.update_groups(idx, positions);
            }
        }

        found
    }

    fn update_groups(&mut self, idx: usize, positions: [[usize; 2]; 4]) {
        let (upper_bits, lower_bits) = &mut self.indifferent_info.removed_groups;
        #[allow(clippy::cast_possible_truncation, reason = "idx - 64 < 5 so fits u8")]
        if idx < 64 {
            if (*lower_bits & BIT[idx]) != 0 {
                return;
            }
            *lower_bits |= BIT[idx];
        } else {
            if (u64::from(*upper_bits) & BIT[idx - 64]) != 0 {
                return;
            }
            *upper_bits |= BIT[idx - 64] as u8;
        }

        for [row, col] in positions {
            let group_count = &mut self.indifferent_info.group_counts[row][col];
            *group_count -= 1;

            let bit = BIT[col * 8 + row];
            if *group_count == 0 {
                if (self.player_bits.p1_bits & bit) == bit {
                    self.player_bits.p1_bits ^= bit;
                    self.player_bits.hash_val ^= ZOBRIST_TABLE[row][col][0];
                    self.player_bits.indifferent_bits |= bit;
                    self.player_bits.hash_val ^= ZOBRIST_TABLE[row][col][2];
                } else if (self.player_bits.p2_bits & bit) == bit {
                    self.player_bits.p2_bits ^= bit;
                    self.player_bits.hash_val ^= ZOBRIST_TABLE[row][col][1];
                    self.player_bits.indifferent_bits |= bit;
                    self.player_bits.hash_val ^= ZOBRIST_TABLE[row][col][2];
                }
            }
        }
    }

    fn flip_turn(&mut self) {
        self.turn = match self.turn {
            P1 => P2,
            P2 => P1,
        }
    }

    fn insert_board(moves: &mut Vec<Board>, new_board: Board) {
        moves.push(new_board);
        let last_idx = moves.len() - 1;
        let InProgress = moves[last_idx].state else {
            moves.swap(0, last_idx);
            return;
        };
    }

    fn get_tile_as_char(&self, row: usize, col: usize) -> char {
        let bit = BIT[col * 8 + row];
        if (self.player_bits.p1_bits & bit) == bit {
            'X'
        } else if (self.player_bits.p2_bits & bit) == bit {
            'O'
        } else if (self.player_bits.indifferent_bits & bit) == bit {
            '-'
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

#[derive(Debug)]
pub enum BoardBuildError {
    BadCols,
    BadDrop(DropCounterError),
}

#[derive(Debug)]
pub enum DropCounterError {
    GameFinished,
    FullCol,
}

impl Display for BoardBuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BadCols => write!(f, "build failed as 1 or more columns were bigger than 6"),
            BadDrop(_) => write!(f, "build failed as drop_counter failed"),
        }
    }
}

impl Error for BoardBuildError {
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

#[cfg(test)]
#[allow(clippy::unwrap_used, reason = "It's for tests where a panic is wanted")]
mod tests {
    use super::*;
    use std::assert_matches;

    #[test]
    fn get_all_moves_on_empty_board() {
        let board = Board::default();
        let moves = board.get_all_moves();
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn get_all_moves_on_non_empty_board_with_no_full_columns() {
        let board = Board::new((0..COLS).collect()).unwrap();
        let moves = board.get_all_moves();
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn get_all_moves_with_a_full_column() {
        let board = Board::new(vec![0; ROWS]).unwrap();
        let moves = board.get_all_moves();
        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn get_all_moves_with_multiple_full_columns() {
        let board = Board::new([[0; ROWS], [2; ROWS], [6; ROWS]].concat()).unwrap();
        let moves = board.get_all_moves();
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn drop_counter_on_full_column() {
        let mut board = Board::new(vec![0; ROWS]).unwrap();
        assert_matches!(board.drop_counter_helper(0), Err(FullCol));
    }

    #[test]
    fn drop_counter_leading_to_p1_win() {
        let board = Board::new(vec![0, 0, 1, 1, 2, 2, 3]).unwrap();
        assert_matches!(board.state, Finished(1));
    }

    #[test]
    fn drop_counter_leading_to_p2_win() {
        let board = Board::new(vec![0, 2, 2, 3, 3, 4, 4, 5]).unwrap();
        assert_matches!(board.state, Finished(-1));
    }

    #[test]
    fn drop_counter_leading_to_draw() {
        let board = Board::new(
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
        )
        .unwrap();
        assert_matches!(board.state, Finished(0));

        println!("drop_counter_leading_to_draw:{board}");
    }

    #[test]
    fn drop_counter_when_game_already_finished() {
        let mut board = Board::new(vec![0, 2, 2, 3, 3, 4, 4, 5]).unwrap();
        assert_matches!(board.drop_counter_helper(0), Err(GameFinished));
    }

    #[test]
    fn set_up_board_leading_to_an_indifferent_bit() {
        let board = Board::new(vec![0, 0, 6, 1, 6, 1]).unwrap();
        println!("set_up_board_leading_to_an_indifferent_bit:{board}");
    }

    #[test]
    fn check_different_boards_are_same_with_indifferent_sets() {
        let board1 = Board::new(vec![
            0, 0, 0, 0, 0, 0, 3, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
        ])
        .unwrap();
        let board2 = Board::new(vec![
            3, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
        ])
        .unwrap();

        println!(
            "check_different_boards_are_same_with_indifferent_sets - Board1:{board1}\n\
        check_different_boards_are_same_with_indifferent_sets - Board2:{board2}"
        );

        assert_eq!(board1.player_bits, board2.player_bits);
    }

    #[test]
    fn visually_confirm_board_masks_win_checks() {
        for (row_idx, row) in GROUPS_AT.iter().enumerate() {
            for (col_idx, checks) in row.iter().enumerate() {
                display_check(checks, row_idx, col_idx);
            }
        }
    }

    fn display_check(checks: &&[GroupInfo], row: usize, col: usize) {
        println!("================================================================");
        for check in *checks {
            let mut board = Board::default();
            board.player_bits.p1_bits |= check.mask;
            board.player_bits.p1_bits ^= BIT[col * 8 + row];
            board.player_bits.p2_bits |= BIT[col * 8 + row];
            println!("{board}");
        }
    }
}
