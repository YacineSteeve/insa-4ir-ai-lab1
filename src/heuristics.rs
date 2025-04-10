use std::cmp::min;
use std::ops::Add;
use crate::board::*;

/// A heuristic function to estimate the cost of reaching the goal state from a given board.
///
/// ```rust
/// let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
/// let h = Heuristic::Manhattan.estimate(&board);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    /// The blind heuristic always returns 0.
    Blind,
    /// The Hamming heuristic, which counts the number of misplaced tiles.
    Hamming,
    /// The Manhattan heuristic, which computes the sum of the Manhattan distances of each tile to its goal position.
    Manhattan,
}

impl Heuristic {
    pub fn estimate(&self, board: &Board) -> u32 {
        match self {
            // blind heuristic always returns 0
            Heuristic::Blind => 0,
            Heuristic::Hamming => {
                let mut misplaced_count = 0;

                for x in 0..N {
                    for y in 0..N {
                        let cell = board.value_at(x, y);

                        if cell != EMPTY_CELL && cell != Board::GOAL.value_at(x, y) {
                            misplaced_count += 1;
                        }
                    }
                }

                misplaced_count
            }
            Heuristic::Manhattan => {
                let mut distances_sum = 0;

                for x in 0..N {
                    for y in 0..N {
                        let cell = board.value_at(x, y);

                        if (cell == EMPTY_CELL) {
                            continue;
                        }

                        let pos = board.position(cell);
                        let expected_pos = Board::GOAL.position(cell);

                        distances_sum += pos.1.abs_diff(expected_pos.1) + pos.0.abs_diff(expected_pos.0);
                    }
                }

                distances_sum as u32
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_heuristic() {
        use super::*;
        let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
        assert_eq!(Heuristic::Blind.estimate(&board), 0);
        assert_eq!(Heuristic::Hamming.estimate(&board), 7);
        assert_eq!(Heuristic::Manhattan.estimate(&board), 14);
    }
}
