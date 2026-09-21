use crate::board::{COLS, ROWS};

#[derive(Debug)]
pub struct GroupInfo {
    pub mask: u64,
    pub idx: usize,
    pub positions: [[usize; 2]; 4],
}

#[rustfmt::skip]
#[allow(clippy::unreadable_literal, reason = "They're just constants for specific binary numbers")]
pub const BIT: [u64; 64] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728, 268435456, 536870912, 1073741824, 2147483648, 4294967296, 8589934592, 17179869184, 34359738368, 68719476736, 137438953472, 274877906944, 549755813888, 1099511627776, 2199023255552, 4398046511104, 8796093022208, 17592186044416, 35184372088832, 70368744177664, 140737488355328, 281474976710656, 562949953421312, 1125899906842624, 2251799813685248, 4503599627370496, 9007199254740992, 18014398509481984, 36028797018963968, 72057594037927936, 144115188075855872, 288230376151711744, 576460752303423488, 1152921504606846976, 2305843009213693952, 4611686018427387904, 9223372036854775808];

pub const MAX_VIABLE_GROUPS: [[u8; COLS]; ROWS] = [
    [3, 4, 5, 7, 5, 4, 3],
    [4, 6, 8, 10, 8, 6, 4],
    [5, 8, 11, 13, 11, 8, 5],
    [5, 8, 11, 13, 11, 8, 5],
    [4, 6, 8, 10, 8, 6, 4],
    [3, 4, 5, 7, 5, 4, 3],
];

#[allow(
    clippy::unreadable_literal,
    reason = "They're just constants for specific binary numbers"
)]
pub const GROUPS_AT: [[&[GroupInfo]; COLS]; ROWS] = [
    [
        &[
            GroupInfo {
                mask: 16843009,
                idx: 0,
                positions: [[0, 0], [0, 1], [0, 2], [0, 3]],
            },
            GroupInfo {
                mask: 134480385,
                idx: 1,
                positions: [[0, 0], [1, 1], [2, 2], [3, 3]],
            },
            GroupInfo {
                mask: 15,
                idx: 2,
                positions: [[3, 0], [2, 0], [1, 0], [0, 0]],
            },
        ],
        &[
            GroupInfo {
                mask: 16843009,
                idx: 0,
                positions: [[0, 0], [0, 1], [0, 2], [0, 3]],
            },
            GroupInfo {
                mask: 4311810304,
                idx: 3,
                positions: [[0, 1], [0, 2], [0, 3], [0, 4]],
            },
            GroupInfo {
                mask: 34426978560,
                idx: 4,
                positions: [[0, 1], [1, 2], [2, 3], [3, 4]],
            },
            GroupInfo {
                mask: 3840,
                idx: 5,
                positions: [[3, 1], [2, 1], [1, 1], [0, 1]],
            },
        ],
        &[
            GroupInfo {
                mask: 16843009,
                idx: 0,
                positions: [[0, 0], [0, 1], [0, 2], [0, 3]],
            },
            GroupInfo {
                mask: 4311810304,
                idx: 3,
                positions: [[0, 1], [0, 2], [0, 3], [0, 4]],
            },
            GroupInfo {
                mask: 1103823437824,
                idx: 6,
                positions: [[0, 2], [0, 3], [0, 4], [0, 5]],
            },
            GroupInfo {
                mask: 8813306511360,
                idx: 7,
                positions: [[0, 2], [1, 3], [2, 4], [3, 5]],
            },
            GroupInfo {
                mask: 983040,
                idx: 8,
                positions: [[3, 2], [2, 2], [1, 2], [0, 2]],
            },
        ],
        &[
            GroupInfo {
                mask: 16843009,
                idx: 0,
                positions: [[0, 0], [0, 1], [0, 2], [0, 3]],
            },
            GroupInfo {
                mask: 4311810304,
                idx: 3,
                positions: [[0, 1], [0, 2], [0, 3], [0, 4]],
            },
            GroupInfo {
                mask: 1103823437824,
                idx: 6,
                positions: [[0, 2], [0, 3], [0, 4], [0, 5]],
            },
            GroupInfo {
                mask: 282578800082944,
                idx: 9,
                positions: [[0, 3], [0, 4], [0, 5], [0, 6]],
            },
            GroupInfo {
                mask: 16909320,
                idx: 10,
                positions: [[3, 0], [2, 1], [1, 2], [0, 3]],
            },
            GroupInfo {
                mask: 2256206466908160,
                idx: 11,
                positions: [[0, 3], [1, 4], [2, 5], [3, 6]],
            },
            GroupInfo {
                mask: 251658240,
                idx: 12,
                positions: [[3, 3], [2, 3], [1, 3], [0, 3]],
            },
        ],
        &[
            GroupInfo {
                mask: 4311810304,
                idx: 3,
                positions: [[0, 1], [0, 2], [0, 3], [0, 4]],
            },
            GroupInfo {
                mask: 1103823437824,
                idx: 6,
                positions: [[0, 2], [0, 3], [0, 4], [0, 5]],
            },
            GroupInfo {
                mask: 282578800082944,
                idx: 9,
                positions: [[0, 3], [0, 4], [0, 5], [0, 6]],
            },
            GroupInfo {
                mask: 4328785920,
                idx: 13,
                positions: [[3, 1], [2, 2], [1, 3], [0, 4]],
            },
            GroupInfo {
                mask: 64424509440,
                idx: 14,
                positions: [[3, 4], [2, 4], [1, 4], [0, 4]],
            },
        ],
        &[
            GroupInfo {
                mask: 1103823437824,
                idx: 6,
                positions: [[0, 2], [0, 3], [0, 4], [0, 5]],
            },
            GroupInfo {
                mask: 282578800082944,
                idx: 9,
                positions: [[0, 3], [0, 4], [0, 5], [0, 6]],
            },
            GroupInfo {
                mask: 1108169195520,
                idx: 15,
                positions: [[3, 2], [2, 3], [1, 4], [0, 5]],
            },
            GroupInfo {
                mask: 16492674416640,
                idx: 16,
                positions: [[3, 5], [2, 5], [1, 5], [0, 5]],
            },
        ],
        &[
            GroupInfo {
                mask: 282578800082944,
                idx: 9,
                positions: [[0, 3], [0, 4], [0, 5], [0, 6]],
            },
            GroupInfo {
                mask: 283691314053120,
                idx: 17,
                positions: [[3, 3], [2, 4], [1, 5], [0, 6]],
            },
            GroupInfo {
                mask: 4222124650659840,
                idx: 18,
                positions: [[3, 6], [2, 6], [1, 6], [0, 6]],
            },
        ],
    ],
    [
        &[
            GroupInfo {
                mask: 33686018,
                idx: 19,
                positions: [[1, 0], [1, 1], [1, 2], [1, 3]],
            },
            GroupInfo {
                mask: 268960770,
                idx: 20,
                positions: [[1, 0], [2, 1], [3, 2], [4, 3]],
            },
            GroupInfo {
                mask: 15,
                idx: 2,
                positions: [[3, 0], [2, 0], [1, 0], [0, 0]],
            },
            GroupInfo {
                mask: 30,
                idx: 21,
                positions: [[4, 0], [3, 0], [2, 0], [1, 0]],
            },
        ],
        &[
            GroupInfo {
                mask: 33686018,
                idx: 19,
                positions: [[1, 0], [1, 1], [1, 2], [1, 3]],
            },
            GroupInfo {
                mask: 8623620608,
                idx: 22,
                positions: [[1, 1], [1, 2], [1, 3], [1, 4]],
            },
            GroupInfo {
                mask: 134480385,
                idx: 1,
                positions: [[0, 0], [1, 1], [2, 2], [3, 3]],
            },
            GroupInfo {
                mask: 68853957120,
                idx: 23,
                positions: [[1, 1], [2, 2], [3, 3], [4, 4]],
            },
            GroupInfo {
                mask: 3840,
                idx: 5,
                positions: [[3, 1], [2, 1], [1, 1], [0, 1]],
            },
            GroupInfo {
                mask: 7680,
                idx: 24,
                positions: [[4, 1], [3, 1], [2, 1], [1, 1]],
            },
        ],
        &[
            GroupInfo {
                mask: 33686018,
                idx: 19,
                positions: [[1, 0], [1, 1], [1, 2], [1, 3]],
            },
            GroupInfo {
                mask: 8623620608,
                idx: 22,
                positions: [[1, 1], [1, 2], [1, 3], [1, 4]],
            },
            GroupInfo {
                mask: 2207646875648,
                idx: 25,
                positions: [[1, 2], [1, 3], [1, 4], [1, 5]],
            },
            GroupInfo {
                mask: 16909320,
                idx: 10,
                positions: [[3, 0], [2, 1], [1, 2], [0, 3]],
            },
            GroupInfo {
                mask: 34426978560,
                idx: 4,
                positions: [[0, 1], [1, 2], [2, 3], [3, 4]],
            },
            GroupInfo {
                mask: 17626613022720,
                idx: 26,
                positions: [[1, 2], [2, 3], [3, 4], [4, 5]],
            },
            GroupInfo {
                mask: 983040,
                idx: 8,
                positions: [[3, 2], [2, 2], [1, 2], [0, 2]],
            },
            GroupInfo {
                mask: 1966080,
                idx: 27,
                positions: [[4, 2], [3, 2], [2, 2], [1, 2]],
            },
        ],
        &[
            GroupInfo {
                mask: 33686018,
                idx: 19,
                positions: [[1, 0], [1, 1], [1, 2], [1, 3]],
            },
            GroupInfo {
                mask: 8623620608,
                idx: 22,
                positions: [[1, 1], [1, 2], [1, 3], [1, 4]],
            },
            GroupInfo {
                mask: 2207646875648,
                idx: 25,
                positions: [[1, 2], [1, 3], [1, 4], [1, 5]],
            },
            GroupInfo {
                mask: 565157600165888,
                idx: 28,
                positions: [[1, 3], [1, 4], [1, 5], [1, 6]],
            },
            GroupInfo {
                mask: 33818640,
                idx: 29,
                positions: [[4, 0], [3, 1], [2, 2], [1, 3]],
            },
            GroupInfo {
                mask: 4328785920,
                idx: 13,
                positions: [[3, 1], [2, 2], [1, 3], [0, 4]],
            },
            GroupInfo {
                mask: 8813306511360,
                idx: 7,
                positions: [[0, 2], [1, 3], [2, 4], [3, 5]],
            },
            GroupInfo {
                mask: 4512412933816320,
                idx: 30,
                positions: [[1, 3], [2, 4], [3, 5], [4, 6]],
            },
            GroupInfo {
                mask: 251658240,
                idx: 12,
                positions: [[3, 3], [2, 3], [1, 3], [0, 3]],
            },
            GroupInfo {
                mask: 503316480,
                idx: 31,
                positions: [[4, 3], [3, 3], [2, 3], [1, 3]],
            },
        ],
        &[
            GroupInfo {
                mask: 8623620608,
                idx: 22,
                positions: [[1, 1], [1, 2], [1, 3], [1, 4]],
            },
            GroupInfo {
                mask: 2207646875648,
                idx: 25,
                positions: [[1, 2], [1, 3], [1, 4], [1, 5]],
            },
            GroupInfo {
                mask: 565157600165888,
                idx: 28,
                positions: [[1, 3], [1, 4], [1, 5], [1, 6]],
            },
            GroupInfo {
                mask: 8657571840,
                idx: 32,
                positions: [[4, 1], [3, 2], [2, 3], [1, 4]],
            },
            GroupInfo {
                mask: 1108169195520,
                idx: 15,
                positions: [[3, 2], [2, 3], [1, 4], [0, 5]],
            },
            GroupInfo {
                mask: 2256206466908160,
                idx: 11,
                positions: [[0, 3], [1, 4], [2, 5], [3, 6]],
            },
            GroupInfo {
                mask: 64424509440,
                idx: 14,
                positions: [[3, 4], [2, 4], [1, 4], [0, 4]],
            },
            GroupInfo {
                mask: 128849018880,
                idx: 33,
                positions: [[4, 4], [3, 4], [2, 4], [1, 4]],
            },
        ],
        &[
            GroupInfo {
                mask: 2207646875648,
                idx: 25,
                positions: [[1, 2], [1, 3], [1, 4], [1, 5]],
            },
            GroupInfo {
                mask: 565157600165888,
                idx: 28,
                positions: [[1, 3], [1, 4], [1, 5], [1, 6]],
            },
            GroupInfo {
                mask: 2216338391040,
                idx: 34,
                positions: [[4, 2], [3, 3], [2, 4], [1, 5]],
            },
            GroupInfo {
                mask: 283691314053120,
                idx: 17,
                positions: [[3, 3], [2, 4], [1, 5], [0, 6]],
            },
            GroupInfo {
                mask: 16492674416640,
                idx: 16,
                positions: [[3, 5], [2, 5], [1, 5], [0, 5]],
            },
            GroupInfo {
                mask: 32985348833280,
                idx: 35,
                positions: [[4, 5], [3, 5], [2, 5], [1, 5]],
            },
        ],
        &[
            GroupInfo {
                mask: 565157600165888,
                idx: 28,
                positions: [[1, 3], [1, 4], [1, 5], [1, 6]],
            },
            GroupInfo {
                mask: 567382628106240,
                idx: 36,
                positions: [[4, 3], [3, 4], [2, 5], [1, 6]],
            },
            GroupInfo {
                mask: 4222124650659840,
                idx: 18,
                positions: [[3, 6], [2, 6], [1, 6], [0, 6]],
            },
            GroupInfo {
                mask: 8444249301319680,
                idx: 37,
                positions: [[4, 6], [3, 6], [2, 6], [1, 6]],
            },
        ],
    ],
    [
        &[
            GroupInfo {
                mask: 67372036,
                idx: 38,
                positions: [[2, 0], [2, 1], [2, 2], [2, 3]],
            },
            GroupInfo {
                mask: 537921540,
                idx: 39,
                positions: [[2, 0], [3, 1], [4, 2], [5, 3]],
            },
            GroupInfo {
                mask: 15,
                idx: 2,
                positions: [[3, 0], [2, 0], [1, 0], [0, 0]],
            },
            GroupInfo {
                mask: 30,
                idx: 21,
                positions: [[4, 0], [3, 0], [2, 0], [1, 0]],
            },
            GroupInfo {
                mask: 60,
                idx: 40,
                positions: [[5, 0], [4, 0], [3, 0], [2, 0]],
            },
        ],
        &[
            GroupInfo {
                mask: 67372036,
                idx: 38,
                positions: [[2, 0], [2, 1], [2, 2], [2, 3]],
            },
            GroupInfo {
                mask: 17247241216,
                idx: 41,
                positions: [[2, 1], [2, 2], [2, 3], [2, 4]],
            },
            GroupInfo {
                mask: 16909320,
                idx: 10,
                positions: [[3, 0], [2, 1], [1, 2], [0, 3]],
            },
            GroupInfo {
                mask: 268960770,
                idx: 20,
                positions: [[1, 0], [2, 1], [3, 2], [4, 3]],
            },
            GroupInfo {
                mask: 137707914240,
                idx: 42,
                positions: [[2, 1], [3, 2], [4, 3], [5, 4]],
            },
            GroupInfo {
                mask: 3840,
                idx: 5,
                positions: [[3, 1], [2, 1], [1, 1], [0, 1]],
            },
            GroupInfo {
                mask: 7680,
                idx: 24,
                positions: [[4, 1], [3, 1], [2, 1], [1, 1]],
            },
            GroupInfo {
                mask: 15360,
                idx: 43,
                positions: [[5, 1], [4, 1], [3, 1], [2, 1]],
            },
        ],
        &[
            GroupInfo {
                mask: 67372036,
                idx: 38,
                positions: [[2, 0], [2, 1], [2, 2], [2, 3]],
            },
            GroupInfo {
                mask: 17247241216,
                idx: 41,
                positions: [[2, 1], [2, 2], [2, 3], [2, 4]],
            },
            GroupInfo {
                mask: 4415293751296,
                idx: 44,
                positions: [[2, 2], [2, 3], [2, 4], [2, 5]],
            },
            GroupInfo {
                mask: 33818640,
                idx: 29,
                positions: [[4, 0], [3, 1], [2, 2], [1, 3]],
            },
            GroupInfo {
                mask: 4328785920,
                idx: 13,
                positions: [[3, 1], [2, 2], [1, 3], [0, 4]],
            },
            GroupInfo {
                mask: 134480385,
                idx: 1,
                positions: [[0, 0], [1, 1], [2, 2], [3, 3]],
            },
            GroupInfo {
                mask: 68853957120,
                idx: 23,
                positions: [[1, 1], [2, 2], [3, 3], [4, 4]],
            },
            GroupInfo {
                mask: 35253226045440,
                idx: 45,
                positions: [[2, 2], [3, 3], [4, 4], [5, 5]],
            },
            GroupInfo {
                mask: 983040,
                idx: 8,
                positions: [[3, 2], [2, 2], [1, 2], [0, 2]],
            },
            GroupInfo {
                mask: 1966080,
                idx: 27,
                positions: [[4, 2], [3, 2], [2, 2], [1, 2]],
            },
            GroupInfo {
                mask: 3932160,
                idx: 46,
                positions: [[5, 2], [4, 2], [3, 2], [2, 2]],
            },
        ],
        &[
            GroupInfo {
                mask: 67372036,
                idx: 38,
                positions: [[2, 0], [2, 1], [2, 2], [2, 3]],
            },
            GroupInfo {
                mask: 17247241216,
                idx: 41,
                positions: [[2, 1], [2, 2], [2, 3], [2, 4]],
            },
            GroupInfo {
                mask: 4415293751296,
                idx: 44,
                positions: [[2, 2], [2, 3], [2, 4], [2, 5]],
            },
            GroupInfo {
                mask: 1130315200331776,
                idx: 47,
                positions: [[2, 3], [2, 4], [2, 5], [2, 6]],
            },
            GroupInfo {
                mask: 67637280,
                idx: 48,
                positions: [[5, 0], [4, 1], [3, 2], [2, 3]],
            },
            GroupInfo {
                mask: 8657571840,
                idx: 32,
                positions: [[4, 1], [3, 2], [2, 3], [1, 4]],
            },
            GroupInfo {
                mask: 1108169195520,
                idx: 15,
                positions: [[3, 2], [2, 3], [1, 4], [0, 5]],
            },
            GroupInfo {
                mask: 34426978560,
                idx: 4,
                positions: [[0, 1], [1, 2], [2, 3], [3, 4]],
            },
            GroupInfo {
                mask: 17626613022720,
                idx: 26,
                positions: [[1, 2], [2, 3], [3, 4], [4, 5]],
            },
            GroupInfo {
                mask: 9024825867632640,
                idx: 49,
                positions: [[2, 3], [3, 4], [4, 5], [5, 6]],
            },
            GroupInfo {
                mask: 251658240,
                idx: 12,
                positions: [[3, 3], [2, 3], [1, 3], [0, 3]],
            },
            GroupInfo {
                mask: 503316480,
                idx: 31,
                positions: [[4, 3], [3, 3], [2, 3], [1, 3]],
            },
            GroupInfo {
                mask: 1006632960,
                idx: 50,
                positions: [[5, 3], [4, 3], [3, 3], [2, 3]],
            },
        ],
        &[
            GroupInfo {
                mask: 17247241216,
                idx: 41,
                positions: [[2, 1], [2, 2], [2, 3], [2, 4]],
            },
            GroupInfo {
                mask: 4415293751296,
                idx: 44,
                positions: [[2, 2], [2, 3], [2, 4], [2, 5]],
            },
            GroupInfo {
                mask: 1130315200331776,
                idx: 47,
                positions: [[2, 3], [2, 4], [2, 5], [2, 6]],
            },
            GroupInfo {
                mask: 17315143680,
                idx: 51,
                positions: [[5, 1], [4, 2], [3, 3], [2, 4]],
            },
            GroupInfo {
                mask: 2216338391040,
                idx: 34,
                positions: [[4, 2], [3, 3], [2, 4], [1, 5]],
            },
            GroupInfo {
                mask: 283691314053120,
                idx: 17,
                positions: [[3, 3], [2, 4], [1, 5], [0, 6]],
            },
            GroupInfo {
                mask: 8813306511360,
                idx: 7,
                positions: [[0, 2], [1, 3], [2, 4], [3, 5]],
            },
            GroupInfo {
                mask: 4512412933816320,
                idx: 30,
                positions: [[1, 3], [2, 4], [3, 5], [4, 6]],
            },
            GroupInfo {
                mask: 64424509440,
                idx: 14,
                positions: [[3, 4], [2, 4], [1, 4], [0, 4]],
            },
            GroupInfo {
                mask: 128849018880,
                idx: 33,
                positions: [[4, 4], [3, 4], [2, 4], [1, 4]],
            },
            GroupInfo {
                mask: 257698037760,
                idx: 52,
                positions: [[5, 4], [4, 4], [3, 4], [2, 4]],
            },
        ],
        &[
            GroupInfo {
                mask: 4415293751296,
                idx: 44,
                positions: [[2, 2], [2, 3], [2, 4], [2, 5]],
            },
            GroupInfo {
                mask: 1130315200331776,
                idx: 47,
                positions: [[2, 3], [2, 4], [2, 5], [2, 6]],
            },
            GroupInfo {
                mask: 4432676782080,
                idx: 53,
                positions: [[5, 2], [4, 3], [3, 4], [2, 5]],
            },
            GroupInfo {
                mask: 567382628106240,
                idx: 36,
                positions: [[4, 3], [3, 4], [2, 5], [1, 6]],
            },
            GroupInfo {
                mask: 2256206466908160,
                idx: 11,
                positions: [[0, 3], [1, 4], [2, 5], [3, 6]],
            },
            GroupInfo {
                mask: 16492674416640,
                idx: 16,
                positions: [[3, 5], [2, 5], [1, 5], [0, 5]],
            },
            GroupInfo {
                mask: 32985348833280,
                idx: 35,
                positions: [[4, 5], [3, 5], [2, 5], [1, 5]],
            },
            GroupInfo {
                mask: 65970697666560,
                idx: 54,
                positions: [[5, 5], [4, 5], [3, 5], [2, 5]],
            },
        ],
        &[
            GroupInfo {
                mask: 1130315200331776,
                idx: 47,
                positions: [[2, 3], [2, 4], [2, 5], [2, 6]],
            },
            GroupInfo {
                mask: 1134765256212480,
                idx: 55,
                positions: [[5, 3], [4, 4], [3, 5], [2, 6]],
            },
            GroupInfo {
                mask: 4222124650659840,
                idx: 18,
                positions: [[3, 6], [2, 6], [1, 6], [0, 6]],
            },
            GroupInfo {
                mask: 8444249301319680,
                idx: 37,
                positions: [[4, 6], [3, 6], [2, 6], [1, 6]],
            },
            GroupInfo {
                mask: 16888498602639360,
                idx: 56,
                positions: [[5, 6], [4, 6], [3, 6], [2, 6]],
            },
        ],
    ],
    [
        &[
            GroupInfo {
                mask: 134744072,
                idx: 57,
                positions: [[3, 0], [3, 1], [3, 2], [3, 3]],
            },
            GroupInfo {
                mask: 16909320,
                idx: 10,
                positions: [[3, 0], [2, 1], [1, 2], [0, 3]],
            },
            GroupInfo {
                mask: 15,
                idx: 2,
                positions: [[3, 0], [2, 0], [1, 0], [0, 0]],
            },
            GroupInfo {
                mask: 30,
                idx: 21,
                positions: [[4, 0], [3, 0], [2, 0], [1, 0]],
            },
            GroupInfo {
                mask: 60,
                idx: 40,
                positions: [[5, 0], [4, 0], [3, 0], [2, 0]],
            },
        ],
        &[
            GroupInfo {
                mask: 134744072,
                idx: 57,
                positions: [[3, 0], [3, 1], [3, 2], [3, 3]],
            },
            GroupInfo {
                mask: 34494482432,
                idx: 58,
                positions: [[3, 1], [3, 2], [3, 3], [3, 4]],
            },
            GroupInfo {
                mask: 33818640,
                idx: 29,
                positions: [[4, 0], [3, 1], [2, 2], [1, 3]],
            },
            GroupInfo {
                mask: 4328785920,
                idx: 13,
                positions: [[3, 1], [2, 2], [1, 3], [0, 4]],
            },
            GroupInfo {
                mask: 537921540,
                idx: 39,
                positions: [[2, 0], [3, 1], [4, 2], [5, 3]],
            },
            GroupInfo {
                mask: 3840,
                idx: 5,
                positions: [[3, 1], [2, 1], [1, 1], [0, 1]],
            },
            GroupInfo {
                mask: 7680,
                idx: 24,
                positions: [[4, 1], [3, 1], [2, 1], [1, 1]],
            },
            GroupInfo {
                mask: 15360,
                idx: 43,
                positions: [[5, 1], [4, 1], [3, 1], [2, 1]],
            },
        ],
        &[
            GroupInfo {
                mask: 134744072,
                idx: 57,
                positions: [[3, 0], [3, 1], [3, 2], [3, 3]],
            },
            GroupInfo {
                mask: 34494482432,
                idx: 58,
                positions: [[3, 1], [3, 2], [3, 3], [3, 4]],
            },
            GroupInfo {
                mask: 8830587502592,
                idx: 59,
                positions: [[3, 2], [3, 3], [3, 4], [3, 5]],
            },
            GroupInfo {
                mask: 67637280,
                idx: 48,
                positions: [[5, 0], [4, 1], [3, 2], [2, 3]],
            },
            GroupInfo {
                mask: 8657571840,
                idx: 32,
                positions: [[4, 1], [3, 2], [2, 3], [1, 4]],
            },
            GroupInfo {
                mask: 1108169195520,
                idx: 15,
                positions: [[3, 2], [2, 3], [1, 4], [0, 5]],
            },
            GroupInfo {
                mask: 268960770,
                idx: 20,
                positions: [[1, 0], [2, 1], [3, 2], [4, 3]],
            },
            GroupInfo {
                mask: 137707914240,
                idx: 42,
                positions: [[2, 1], [3, 2], [4, 3], [5, 4]],
            },
            GroupInfo {
                mask: 983040,
                idx: 8,
                positions: [[3, 2], [2, 2], [1, 2], [0, 2]],
            },
            GroupInfo {
                mask: 1966080,
                idx: 27,
                positions: [[4, 2], [3, 2], [2, 2], [1, 2]],
            },
            GroupInfo {
                mask: 3932160,
                idx: 46,
                positions: [[5, 2], [4, 2], [3, 2], [2, 2]],
            },
        ],
        &[
            GroupInfo {
                mask: 134744072,
                idx: 57,
                positions: [[3, 0], [3, 1], [3, 2], [3, 3]],
            },
            GroupInfo {
                mask: 34494482432,
                idx: 58,
                positions: [[3, 1], [3, 2], [3, 3], [3, 4]],
            },
            GroupInfo {
                mask: 8830587502592,
                idx: 59,
                positions: [[3, 2], [3, 3], [3, 4], [3, 5]],
            },
            GroupInfo {
                mask: 2260630400663552,
                idx: 60,
                positions: [[3, 3], [3, 4], [3, 5], [3, 6]],
            },
            GroupInfo {
                mask: 17315143680,
                idx: 51,
                positions: [[5, 1], [4, 2], [3, 3], [2, 4]],
            },
            GroupInfo {
                mask: 2216338391040,
                idx: 34,
                positions: [[4, 2], [3, 3], [2, 4], [1, 5]],
            },
            GroupInfo {
                mask: 283691314053120,
                idx: 17,
                positions: [[3, 3], [2, 4], [1, 5], [0, 6]],
            },
            GroupInfo {
                mask: 134480385,
                idx: 1,
                positions: [[0, 0], [1, 1], [2, 2], [3, 3]],
            },
            GroupInfo {
                mask: 68853957120,
                idx: 23,
                positions: [[1, 1], [2, 2], [3, 3], [4, 4]],
            },
            GroupInfo {
                mask: 35253226045440,
                idx: 45,
                positions: [[2, 2], [3, 3], [4, 4], [5, 5]],
            },
            GroupInfo {
                mask: 251658240,
                idx: 12,
                positions: [[3, 3], [2, 3], [1, 3], [0, 3]],
            },
            GroupInfo {
                mask: 503316480,
                idx: 31,
                positions: [[4, 3], [3, 3], [2, 3], [1, 3]],
            },
            GroupInfo {
                mask: 1006632960,
                idx: 50,
                positions: [[5, 3], [4, 3], [3, 3], [2, 3]],
            },
        ],
        &[
            GroupInfo {
                mask: 34494482432,
                idx: 58,
                positions: [[3, 1], [3, 2], [3, 3], [3, 4]],
            },
            GroupInfo {
                mask: 8830587502592,
                idx: 59,
                positions: [[3, 2], [3, 3], [3, 4], [3, 5]],
            },
            GroupInfo {
                mask: 2260630400663552,
                idx: 60,
                positions: [[3, 3], [3, 4], [3, 5], [3, 6]],
            },
            GroupInfo {
                mask: 4432676782080,
                idx: 53,
                positions: [[5, 2], [4, 3], [3, 4], [2, 5]],
            },
            GroupInfo {
                mask: 567382628106240,
                idx: 36,
                positions: [[4, 3], [3, 4], [2, 5], [1, 6]],
            },
            GroupInfo {
                mask: 34426978560,
                idx: 4,
                positions: [[0, 1], [1, 2], [2, 3], [3, 4]],
            },
            GroupInfo {
                mask: 17626613022720,
                idx: 26,
                positions: [[1, 2], [2, 3], [3, 4], [4, 5]],
            },
            GroupInfo {
                mask: 9024825867632640,
                idx: 49,
                positions: [[2, 3], [3, 4], [4, 5], [5, 6]],
            },
            GroupInfo {
                mask: 64424509440,
                idx: 14,
                positions: [[3, 4], [2, 4], [1, 4], [0, 4]],
            },
            GroupInfo {
                mask: 128849018880,
                idx: 33,
                positions: [[4, 4], [3, 4], [2, 4], [1, 4]],
            },
            GroupInfo {
                mask: 257698037760,
                idx: 52,
                positions: [[5, 4], [4, 4], [3, 4], [2, 4]],
            },
        ],
        &[
            GroupInfo {
                mask: 8830587502592,
                idx: 59,
                positions: [[3, 2], [3, 3], [3, 4], [3, 5]],
            },
            GroupInfo {
                mask: 2260630400663552,
                idx: 60,
                positions: [[3, 3], [3, 4], [3, 5], [3, 6]],
            },
            GroupInfo {
                mask: 1134765256212480,
                idx: 55,
                positions: [[5, 3], [4, 4], [3, 5], [2, 6]],
            },
            GroupInfo {
                mask: 8813306511360,
                idx: 7,
                positions: [[0, 2], [1, 3], [2, 4], [3, 5]],
            },
            GroupInfo {
                mask: 4512412933816320,
                idx: 30,
                positions: [[1, 3], [2, 4], [3, 5], [4, 6]],
            },
            GroupInfo {
                mask: 16492674416640,
                idx: 16,
                positions: [[3, 5], [2, 5], [1, 5], [0, 5]],
            },
            GroupInfo {
                mask: 32985348833280,
                idx: 35,
                positions: [[4, 5], [3, 5], [2, 5], [1, 5]],
            },
            GroupInfo {
                mask: 65970697666560,
                idx: 54,
                positions: [[5, 5], [4, 5], [3, 5], [2, 5]],
            },
        ],
        &[
            GroupInfo {
                mask: 2260630400663552,
                idx: 60,
                positions: [[3, 3], [3, 4], [3, 5], [3, 6]],
            },
            GroupInfo {
                mask: 2256206466908160,
                idx: 11,
                positions: [[0, 3], [1, 4], [2, 5], [3, 6]],
            },
            GroupInfo {
                mask: 4222124650659840,
                idx: 18,
                positions: [[3, 6], [2, 6], [1, 6], [0, 6]],
            },
            GroupInfo {
                mask: 8444249301319680,
                idx: 37,
                positions: [[4, 6], [3, 6], [2, 6], [1, 6]],
            },
            GroupInfo {
                mask: 16888498602639360,
                idx: 56,
                positions: [[5, 6], [4, 6], [3, 6], [2, 6]],
            },
        ],
    ],
    [
        &[
            GroupInfo {
                mask: 269488144,
                idx: 61,
                positions: [[4, 0], [4, 1], [4, 2], [4, 3]],
            },
            GroupInfo {
                mask: 33818640,
                idx: 29,
                positions: [[4, 0], [3, 1], [2, 2], [1, 3]],
            },
            GroupInfo {
                mask: 30,
                idx: 21,
                positions: [[4, 0], [3, 0], [2, 0], [1, 0]],
            },
            GroupInfo {
                mask: 60,
                idx: 40,
                positions: [[5, 0], [4, 0], [3, 0], [2, 0]],
            },
        ],
        &[
            GroupInfo {
                mask: 269488144,
                idx: 61,
                positions: [[4, 0], [4, 1], [4, 2], [4, 3]],
            },
            GroupInfo {
                mask: 68988964864,
                idx: 62,
                positions: [[4, 1], [4, 2], [4, 3], [4, 4]],
            },
            GroupInfo {
                mask: 67637280,
                idx: 48,
                positions: [[5, 0], [4, 1], [3, 2], [2, 3]],
            },
            GroupInfo {
                mask: 8657571840,
                idx: 32,
                positions: [[4, 1], [3, 2], [2, 3], [1, 4]],
            },
            GroupInfo {
                mask: 7680,
                idx: 24,
                positions: [[4, 1], [3, 1], [2, 1], [1, 1]],
            },
            GroupInfo {
                mask: 15360,
                idx: 43,
                positions: [[5, 1], [4, 1], [3, 1], [2, 1]],
            },
        ],
        &[
            GroupInfo {
                mask: 269488144,
                idx: 61,
                positions: [[4, 0], [4, 1], [4, 2], [4, 3]],
            },
            GroupInfo {
                mask: 68988964864,
                idx: 62,
                positions: [[4, 1], [4, 2], [4, 3], [4, 4]],
            },
            GroupInfo {
                mask: 17661175005184,
                idx: 63,
                positions: [[4, 2], [4, 3], [4, 4], [4, 5]],
            },
            GroupInfo {
                mask: 17315143680,
                idx: 51,
                positions: [[5, 1], [4, 2], [3, 3], [2, 4]],
            },
            GroupInfo {
                mask: 2216338391040,
                idx: 34,
                positions: [[4, 2], [3, 3], [2, 4], [1, 5]],
            },
            GroupInfo {
                mask: 537921540,
                idx: 39,
                positions: [[2, 0], [3, 1], [4, 2], [5, 3]],
            },
            GroupInfo {
                mask: 1966080,
                idx: 27,
                positions: [[4, 2], [3, 2], [2, 2], [1, 2]],
            },
            GroupInfo {
                mask: 3932160,
                idx: 46,
                positions: [[5, 2], [4, 2], [3, 2], [2, 2]],
            },
        ],
        &[
            GroupInfo {
                mask: 269488144,
                idx: 61,
                positions: [[4, 0], [4, 1], [4, 2], [4, 3]],
            },
            GroupInfo {
                mask: 68988964864,
                idx: 62,
                positions: [[4, 1], [4, 2], [4, 3], [4, 4]],
            },
            GroupInfo {
                mask: 17661175005184,
                idx: 63,
                positions: [[4, 2], [4, 3], [4, 4], [4, 5]],
            },
            GroupInfo {
                mask: 4521260801327104,
                idx: 64,
                positions: [[4, 3], [4, 4], [4, 5], [4, 6]],
            },
            GroupInfo {
                mask: 4432676782080,
                idx: 53,
                positions: [[5, 2], [4, 3], [3, 4], [2, 5]],
            },
            GroupInfo {
                mask: 567382628106240,
                idx: 36,
                positions: [[4, 3], [3, 4], [2, 5], [1, 6]],
            },
            GroupInfo {
                mask: 268960770,
                idx: 20,
                positions: [[1, 0], [2, 1], [3, 2], [4, 3]],
            },
            GroupInfo {
                mask: 137707914240,
                idx: 42,
                positions: [[2, 1], [3, 2], [4, 3], [5, 4]],
            },
            GroupInfo {
                mask: 503316480,
                idx: 31,
                positions: [[4, 3], [3, 3], [2, 3], [1, 3]],
            },
            GroupInfo {
                mask: 1006632960,
                idx: 50,
                positions: [[5, 3], [4, 3], [3, 3], [2, 3]],
            },
        ],
        &[
            GroupInfo {
                mask: 68988964864,
                idx: 62,
                positions: [[4, 1], [4, 2], [4, 3], [4, 4]],
            },
            GroupInfo {
                mask: 17661175005184,
                idx: 63,
                positions: [[4, 2], [4, 3], [4, 4], [4, 5]],
            },
            GroupInfo {
                mask: 4521260801327104,
                idx: 64,
                positions: [[4, 3], [4, 4], [4, 5], [4, 6]],
            },
            GroupInfo {
                mask: 1134765256212480,
                idx: 55,
                positions: [[5, 3], [4, 4], [3, 5], [2, 6]],
            },
            GroupInfo {
                mask: 68853957120,
                idx: 23,
                positions: [[1, 1], [2, 2], [3, 3], [4, 4]],
            },
            GroupInfo {
                mask: 35253226045440,
                idx: 45,
                positions: [[2, 2], [3, 3], [4, 4], [5, 5]],
            },
            GroupInfo {
                mask: 128849018880,
                idx: 33,
                positions: [[4, 4], [3, 4], [2, 4], [1, 4]],
            },
            GroupInfo {
                mask: 257698037760,
                idx: 52,
                positions: [[5, 4], [4, 4], [3, 4], [2, 4]],
            },
        ],
        &[
            GroupInfo {
                mask: 17661175005184,
                idx: 63,
                positions: [[4, 2], [4, 3], [4, 4], [4, 5]],
            },
            GroupInfo {
                mask: 4521260801327104,
                idx: 64,
                positions: [[4, 3], [4, 4], [4, 5], [4, 6]],
            },
            GroupInfo {
                mask: 17626613022720,
                idx: 26,
                positions: [[1, 2], [2, 3], [3, 4], [4, 5]],
            },
            GroupInfo {
                mask: 9024825867632640,
                idx: 49,
                positions: [[2, 3], [3, 4], [4, 5], [5, 6]],
            },
            GroupInfo {
                mask: 32985348833280,
                idx: 35,
                positions: [[4, 5], [3, 5], [2, 5], [1, 5]],
            },
            GroupInfo {
                mask: 65970697666560,
                idx: 54,
                positions: [[5, 5], [4, 5], [3, 5], [2, 5]],
            },
        ],
        &[
            GroupInfo {
                mask: 4521260801327104,
                idx: 64,
                positions: [[4, 3], [4, 4], [4, 5], [4, 6]],
            },
            GroupInfo {
                mask: 4512412933816320,
                idx: 30,
                positions: [[1, 3], [2, 4], [3, 5], [4, 6]],
            },
            GroupInfo {
                mask: 8444249301319680,
                idx: 37,
                positions: [[4, 6], [3, 6], [2, 6], [1, 6]],
            },
            GroupInfo {
                mask: 16888498602639360,
                idx: 56,
                positions: [[5, 6], [4, 6], [3, 6], [2, 6]],
            },
        ],
    ],
    [
        &[
            GroupInfo {
                mask: 538976288,
                idx: 65,
                positions: [[5, 0], [5, 1], [5, 2], [5, 3]],
            },
            GroupInfo {
                mask: 67637280,
                idx: 48,
                positions: [[5, 0], [4, 1], [3, 2], [2, 3]],
            },
            GroupInfo {
                mask: 60,
                idx: 40,
                positions: [[5, 0], [4, 0], [3, 0], [2, 0]],
            },
        ],
        &[
            GroupInfo {
                mask: 538976288,
                idx: 65,
                positions: [[5, 0], [5, 1], [5, 2], [5, 3]],
            },
            GroupInfo {
                mask: 137977929728,
                idx: 66,
                positions: [[5, 1], [5, 2], [5, 3], [5, 4]],
            },
            GroupInfo {
                mask: 17315143680,
                idx: 51,
                positions: [[5, 1], [4, 2], [3, 3], [2, 4]],
            },
            GroupInfo {
                mask: 15360,
                idx: 43,
                positions: [[5, 1], [4, 1], [3, 1], [2, 1]],
            },
        ],
        &[
            GroupInfo {
                mask: 538976288,
                idx: 65,
                positions: [[5, 0], [5, 1], [5, 2], [5, 3]],
            },
            GroupInfo {
                mask: 137977929728,
                idx: 66,
                positions: [[5, 1], [5, 2], [5, 3], [5, 4]],
            },
            GroupInfo {
                mask: 35322350010368,
                idx: 67,
                positions: [[5, 2], [5, 3], [5, 4], [5, 5]],
            },
            GroupInfo {
                mask: 4432676782080,
                idx: 53,
                positions: [[5, 2], [4, 3], [3, 4], [2, 5]],
            },
            GroupInfo {
                mask: 3932160,
                idx: 46,
                positions: [[5, 2], [4, 2], [3, 2], [2, 2]],
            },
        ],
        &[
            GroupInfo {
                mask: 538976288,
                idx: 65,
                positions: [[5, 0], [5, 1], [5, 2], [5, 3]],
            },
            GroupInfo {
                mask: 137977929728,
                idx: 66,
                positions: [[5, 1], [5, 2], [5, 3], [5, 4]],
            },
            GroupInfo {
                mask: 35322350010368,
                idx: 67,
                positions: [[5, 2], [5, 3], [5, 4], [5, 5]],
            },
            GroupInfo {
                mask: 9042521602654208,
                idx: 68,
                positions: [[5, 3], [5, 4], [5, 5], [5, 6]],
            },
            GroupInfo {
                mask: 1134765256212480,
                idx: 55,
                positions: [[5, 3], [4, 4], [3, 5], [2, 6]],
            },
            GroupInfo {
                mask: 537921540,
                idx: 39,
                positions: [[2, 0], [3, 1], [4, 2], [5, 3]],
            },
            GroupInfo {
                mask: 1006632960,
                idx: 50,
                positions: [[5, 3], [4, 3], [3, 3], [2, 3]],
            },
        ],
        &[
            GroupInfo {
                mask: 137977929728,
                idx: 66,
                positions: [[5, 1], [5, 2], [5, 3], [5, 4]],
            },
            GroupInfo {
                mask: 35322350010368,
                idx: 67,
                positions: [[5, 2], [5, 3], [5, 4], [5, 5]],
            },
            GroupInfo {
                mask: 9042521602654208,
                idx: 68,
                positions: [[5, 3], [5, 4], [5, 5], [5, 6]],
            },
            GroupInfo {
                mask: 137707914240,
                idx: 42,
                positions: [[2, 1], [3, 2], [4, 3], [5, 4]],
            },
            GroupInfo {
                mask: 257698037760,
                idx: 52,
                positions: [[5, 4], [4, 4], [3, 4], [2, 4]],
            },
        ],
        &[
            GroupInfo {
                mask: 35322350010368,
                idx: 67,
                positions: [[5, 2], [5, 3], [5, 4], [5, 5]],
            },
            GroupInfo {
                mask: 9042521602654208,
                idx: 68,
                positions: [[5, 3], [5, 4], [5, 5], [5, 6]],
            },
            GroupInfo {
                mask: 35253226045440,
                idx: 45,
                positions: [[2, 2], [3, 3], [4, 4], [5, 5]],
            },
            GroupInfo {
                mask: 65970697666560,
                idx: 54,
                positions: [[5, 5], [4, 5], [3, 5], [2, 5]],
            },
        ],
        &[
            GroupInfo {
                mask: 9042521602654208,
                idx: 68,
                positions: [[5, 3], [5, 4], [5, 5], [5, 6]],
            },
            GroupInfo {
                mask: 9024825867632640,
                idx: 49,
                positions: [[2, 3], [3, 4], [4, 5], [5, 6]],
            },
            GroupInfo {
                mask: 16888498602639360,
                idx: 56,
                positions: [[5, 6], [4, 6], [3, 6], [2, 6]],
            },
        ],
    ],
];
