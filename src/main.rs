use crate::board_info_generator::BoardInfo;
use crate::group_info::{GROUPS_AT, GroupInfo};
use crate::solver::bench::bench_solver;
use std::env::args;

mod board;
mod board_info_generator;
mod group_info;
mod solver;

fn main() -> Result<(), &'static str> {
    if args().len() != 2 {
        return Err("please give *one* input argument option: [bench_solver, display_masks]");
    }

    let GroupInfo {
        mask,
        idx,
        positions,
    } = GROUPS_AT[0][0][0];
    println!("{mask}\n{idx}{positions:?}");

    #[allow(clippy::unwrap_used, reason = "Already checked len, so nth(1) exists")]
    let input_option = args().nth(1).unwrap();
    if input_option == "bench_solver" {
        bench_solver();
    } else if input_option == "display_masks" {
        println!("{}", BoardInfo::default());
    } else {
        return Err("invalid argument option, the options are: [bench_solver, display_masks]");
    }

    Ok(())
}
