use crate::board::BoardBuildError::{BadCols, BadDrop};
use crate::board::DropCounterError::{FullCol, GameFinished};
use crate::board::GameState::{Finished, InProgress};
use crate::board::Turn::{P1, P2};
use std::error::Error;
use std::fmt::Write;
use std::fmt::{Display, Formatter};

pub const ROWS: usize = 6;
pub const COLS: usize = 7;

#[rustfmt::skip]
#[allow(clippy::unreadable_literal, reason = "They're just constants for specific binary numbers")]
const BIT: [u64; 64] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728, 268435456, 536870912, 1073741824, 2147483648, 4294967296, 8589934592, 17179869184, 34359738368, 68719476736, 137438953472, 274877906944, 549755813888, 1099511627776, 2199023255552, 4398046511104, 8796093022208, 17592186044416, 35184372088832, 70368744177664, 140737488355328, 281474976710656, 562949953421312, 1125899906842624, 2251799813685248, 4503599627370496, 9007199254740992, 18014398509481984, 36028797018963968, 72057594037927936, 144115188075855872, 288230376151711744, 576460752303423488, 1152921504606846976, 2305843009213693952, 4611686018427387904, 9223372036854775808];

#[rustfmt::skip]
#[allow(clippy::unreadable_literal, reason = "They're just constants for specific binary numbers")]
const WIN_CHECKS: [[&[u64]; COLS]; ROWS] = [[&[16843009, 134480385], &[16843009, 4311810304, 34426978560], &[16843009, 4311810304, 1103823437824, 8813306511360], &[16843009, 4311810304, 1103823437824, 282578800082944, 16909320, 2256206466908160], &[4311810304, 1103823437824, 282578800082944, 4328785920], &[1103823437824, 282578800082944, 1108169195520], &[282578800082944, 283691314053120]], [&[33686018, 268960770], &[33686018, 8623620608, 134480385, 68853957120], &[33686018, 8623620608, 2207646875648, 16909320, 34426978560, 17626613022720], &[33686018, 8623620608, 2207646875648, 565157600165888, 33818640, 4328785920, 8813306511360, 4512412933816320], &[8623620608, 2207646875648, 565157600165888, 8657571840, 1108169195520, 2256206466908160], &[2207646875648, 565157600165888, 2216338391040, 283691314053120], &[565157600165888, 567382628106240]], [&[67372036, 537921540], &[67372036, 17247241216, 16909320, 268960770, 137707914240], &[67372036, 17247241216, 4415293751296, 33818640, 4328785920, 134480385, 68853957120, 35253226045440], &[67372036, 17247241216, 4415293751296, 1130315200331776, 67637280, 8657571840, 1108169195520, 34426978560, 17626613022720, 9024825867632640], &[17247241216, 4415293751296, 1130315200331776, 17315143680, 2216338391040, 283691314053120, 8813306511360, 4512412933816320], &[4415293751296, 1130315200331776, 4432676782080, 567382628106240, 2256206466908160], &[1130315200331776, 1134765256212480]], [&[134744072, 16909320, 15], &[134744072, 34494482432, 33818640, 4328785920, 537921540, 3840], &[134744072, 34494482432, 8830587502592, 67637280, 8657571840, 1108169195520, 268960770, 137707914240, 983040], &[134744072, 34494482432, 8830587502592, 2260630400663552, 17315143680, 2216338391040, 283691314053120, 134480385, 68853957120, 35253226045440, 251658240], &[34494482432, 8830587502592, 2260630400663552, 4432676782080, 567382628106240, 34426978560, 17626613022720, 9024825867632640, 64424509440], &[8830587502592, 2260630400663552, 1134765256212480, 8813306511360, 4512412933816320, 16492674416640], &[2260630400663552, 2256206466908160, 4222124650659840]], [&[269488144, 33818640, 30], &[269488144, 68988964864, 67637280, 8657571840, 7680], &[269488144, 68988964864, 17661175005184, 17315143680, 2216338391040, 537921540, 1966080], &[269488144, 68988964864, 17661175005184, 4521260801327104, 4432676782080, 567382628106240, 268960770, 137707914240, 503316480], &[68988964864, 17661175005184, 4521260801327104, 1134765256212480, 68853957120, 35253226045440, 128849018880], &[17661175005184, 4521260801327104, 17626613022720, 9024825867632640, 32985348833280], &[4521260801327104, 4512412933816320, 8444249301319680]], [&[538976288, 67637280, 60], &[538976288, 137977929728, 17315143680, 15360], &[538976288, 137977929728, 35322350010368, 4432676782080, 3932160], &[538976288, 137977929728, 35322350010368, 9042521602654208, 1134765256212480, 537921540, 1006632960], &[137977929728, 35322350010368, 9042521602654208, 137707914240, 257698037760], &[35322350010368, 9042521602654208, 35253226045440, 65970697666560], &[9042521602654208, 9024825867632640, 16888498602639360]]];

#[rustfmt::skip]
#[allow(clippy::unreadable_literal, reason = "They're just random numbers for hashing")]
const ZOBRIST_TABLE: [[[u64; 2]; COLS]; ROWS] = [[[9351543000069148727, 5691933524360276676], [9147536183333472440, 17549656232030087907], [5562979162874276111, 4215143189366588270], [17207351728752092811, 9231081037056901349], [2477476474377656062, 16866343659304088472], [10893974287695793347, 9228792675638351133], [4657704497154974639, 18210602477669909631]], [[7061702949332388589, 14276056753460234363], [8348351524084627068, 478462755017893193], [7444918653097023780, 5176717050937306791], [1986487973583215326, 18136546716676371530], [17929250101224492385, 5589671207470731658], [13260828001632126366, 12967730261195639562], [9063522003766710743, 2374717761508042399]], [[3026974079049156385, 9955688699042696796], [9014700098095053840, 2243906978976509320], [5404087178935388157, 12154374279952473092], [8122716050300030240, 6044413828283743443], [2926490043870920578, 1844273341041146771], [18061812195486304151, 17408569648803377895], [11034092129662992875, 9463455164115100627]], [[14220167580117014545, 13138781109727440810], [13521671292216768147, 1246666947883262128], [15136837964180520243, 5404390819736584762], [4618858759546036375, 13782952762990999500], [2026914633184229991, 4795605775300495685], [16154646453133813365, 17547545170322516086], [11093730159042650627, 18049464735842824199]], [[7177263294104962059, 2372937991963160167], [11747799407082374897, 2593846908880165321], [9177549805582938141, 1410249117128530222], [15124124645558580629, 3359900549929608144], [5471878864843508202, 2380576746614701906], [14600151983340132708, 5571980405572412002], [13724864714577765393, 16041329946984188225]], [[12327895548274155578, 6054039446160875415], [13205435451647096321, 8502271719022006447], [13501478777981532461, 15941893788333859783], [6698994516329878211, 13103681521163299285], [14265291345454588705, 568955121651295797], [306806888309149821, 14114546947230079571], [14170255324682069141, 7097331192898184724]]];

#[derive(Clone, Default)]
pub struct Board {
    player_bits: BitBoard,
    column_heights: [u8; COLS],
    turn: Turn,
    state: GameState,
}

#[derive(Clone, Default)]
pub struct BitBoard {
    p1_bits: u64,
    p2_bits: u64,
    hash_val: u64,
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

impl Board {
    pub fn new(moves: Vec<usize>) -> Result<Self, BoardBuildError> {
        let mut board = Board::default();
        for col in moves {
            let board_col = BoardCol::new(col).ok_or(BadCols)?;
            board.drop_counter(board_col).map_err(BadDrop)?;
        }

        Ok(board)
    }

    pub fn get_hash(&self) -> u64 {
        self.player_bits.hash_val
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
            self.update_state(row, col);
            self.player_bits.hash_val ^= ZOBRIST_TABLE[row][col][self.turn as usize];

            Ok(())
        }
    }

    fn update_state(&mut self, row: usize, col: usize) {
        if self.has_completed_line(row, col) {
            match self.turn {
                P1 => self.state = Finished(1),
                P2 => self.state = Finished(-1),
            }
        } else if (self.player_bits.p1_bits | self.player_bits.p2_bits) == 0x003F_3F3F_3F3F_3F3F {
            self.state = Finished(0);
        } else {
            self.flip_turn();
        }
    }

    fn has_completed_line(&self, row: usize, col: usize) -> bool {
        let checks = WIN_CHECKS[row][col];
        let bit_board = match self.turn {
            P1 => self.player_bits.p1_bits,
            P2 => self.player_bits.p2_bits,
        };

        #[allow(clippy::manual_contains, reason = "The closure can't be simplified")]
        checks.iter().any(|&check| (bit_board & check) == check)
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

impl Display for DropCounterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameFinished => write!(f, "drop_counter failed as game is over"),
            FullCol => write!(f, "drop_counter failed as column was full"),
        }
    }
}

impl Error for DropCounterError {}

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
    }

    #[test]
    fn drop_counter_when_game_already_finished() {
        let mut board = Board::new(vec![0, 2, 2, 3, 3, 4, 4, 5]).unwrap();
        assert_matches!(board.drop_counter_helper(0), Err(GameFinished));
    }

    #[test]
    fn visually_confirm_board_masks_win_checks() {
        for (row_idx, row) in WIN_CHECKS.iter().enumerate() {
            for (col_idx, checks) in row.iter().enumerate() {
                display_check(checks, row_idx, col_idx);
            }
        }
    }

    fn display_check(checks: &&[u64], row: usize, col: usize) {
        println!("================================================================");
        for check in *checks {
            let mut board = Board::default();
            board.player_bits.p1_bits |= check;
            board.player_bits.p1_bits ^= BIT[col * 8 + row];
            board.player_bits.p2_bits |= BIT[col * 8 + row];
            println!("{board}");
        }
    }
}
