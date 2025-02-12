#![allow(unused)] // suppress warnings for unused code (there is plenty when you start)

// declare other modules that are in other files and must be compiled
mod board;
mod heuristics;
mod min_heap;
mod search;

// import the content of the modules
use board::*;
use heuristics::*;
use search::*;
use Direction::*;

fn main() {
    let board = Board::new([[1, 2, 3], [4, 8, 5], [0, 7, 6]]);

    let plan = [Right, Up, Right, Down];

    if board.is_valid_plan(&plan) {
        board.play(&plan);
    }
}
