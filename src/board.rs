use std::fmt::Display;

/// Size of the board. The board is a square of size N x N.
pub const N: usize = 3;

/// Type of the cell in the board. It is a number between 0 and N^2 - 1.
pub type Cell = u8;

/// Represents an empty cell in the board.
pub const EMPTY_CELL: Cell = 0;

/// The board is a square of size N x N. It is represented as an array of N arrays of N cells.
///
/// ```rust
/// let board = Board::new([[1, 2, 3], [4, 5, 6], [7, 0, 8]]);
///
/// // you can access a cell of the board using the `value_at` method
/// assert_eq!(board.value_at(0, 0), 1);
///
/// // you can apply a move to the board using the `apply` method
/// // this essentially moves the empty cell in the direction
/// let new_board = board.apply(Direction::Up).unwrap();
///
/// // you can find the position of a cell in the board using the `position` method
/// assert_eq!(new_board.position(2), (0, 1));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Board {
    cells: [[Cell; N]; N],
}
impl Board {
    /// The goal state of the 8-puzzle problem.
    ///
    /// ```rust
    /// let goal: Board = Board::GOAL;
    /// assert_eq!(goal.value_at(0, 0), 1);
    /// assert_eq!(goal.value_at(2, 2), 0);
    /// ```
    pub const GOAL: Board = Board::new([[1, 2, 3], [4, 5, 6], [7, 8, 0]]);

    pub const fn new(cells: [[Cell; N]; N]) -> Board {
        Board { cells }
    }

    /// Returns the value of the cell at the given position.
    pub fn value_at(&self, line: usize, column: usize) -> Cell {
        self.cells[line][column]
    }

    /// Returns the result of applying the given action to the board.
    /// If the action is not applicable (the empty cell would move outside the board), returns `None`.
    /// Otherwise, returns the new board (wrapped in `Some(...)`).
    pub fn apply(&self, direction: Direction) -> Option<Board> {
        let (x, y) = self.position(EMPTY_CELL);
        // compute the new coordinates of the empty cell after the move
        let new_coordinates = match direction {
            Direction::Up if x > 0 => Some((x - 1, y)),
            Direction::Down if x < N - 1 => Some((x + 1, y)),
            Direction::Left if y > 0 => Some((x, y - 1)),
            Direction::Right if y < N - 1 => Some((x, y + 1)),
            _ => None, // would move out of the board
        };
        match new_coordinates {
            Some((new_x, new_y)) => {
                // empty cell can be moved to the new coordinates
                // create a new board with the empty cell moved
                let mut new_cells = self.cells.clone();
                new_cells[x][y] = new_cells[new_x][new_y];
                new_cells[new_x][new_y] = 0;
                Some(Board::new(new_cells))
            }
            None => None, // coordinates would have been out of the board, return None to indicate that the action is not applicable
        }
    }

    /// Returns the position `(line, column)` of the given cell value.
    pub fn position(&self, value: Cell) -> (usize, usize) {
        for x in 0..N {
            for y in 0..N {
                if self.cells[x][y] == value {
                    return (x, y);
                }
            }
        }
        panic!("No such cell: {value}");
    }

    /// Plays a sequence of moves on the board, printing the board at each step.
    /// Intended for displaying purpose but very slow (the thread will be put to sleep between each frame)
    pub fn play(&self, moves: &[Direction]) {
        // current board from which the play starts
        let mut current_board = self.clone();
        println!("{current_board}");
        for &direction in moves {
            if let Some(next) = current_board.apply(direction) {
                // action is applicable, update the current board
                current_board = next;

                // wait half a second before displaying
                std::thread::sleep(std::time::Duration::from_millis(500));
                // print the current board
                println!("{current_board}");
            } else {
                // non-applicable action, call `panic!()` which represents an unrecoverable error
                panic!(
                    "The action {direction:?} is not applicable on the board: {current_board:?}"
                );
            }
        }
    }

    /// Returs `true` if the given sequence of actions is a valid plan that leads to the goal state.
    pub fn is_valid_plan(&self, actions: &[Direction]) -> bool {
        let mut board = *self;

        for action in actions {
            match board.apply(*action) {
                Some(new_board) => board = new_board,
                None => (),
            }
        }

        board == Board::GOAL
    }
}

// Specifies how to display a board in a human-readable way.
// This is what is used when you use the `{}` format specifier in a `println!` macro.
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n┏━━━┳━━━┳━━━┓\n")?;
        for i in 0..N {
            write!(f, "┃")?;
            for j in 0..N {
                let value_in_cell = self.value_at(i, j);
                if value_in_cell == 0 {
                    write!(f, "   ┃")?;
                } else {
                    write!(f, " {value_in_cell} ┃")?;
                }
            }
            if i < N - 1 {
                write!(f, "\n┣━━━╋━━━╋━━━┫\n")?;
            } else {
                write!(f, "\n┗━━━┻━━━┻━━━┛\n")?;
            }
        }
        Ok(())
    }
}

/// The possible directions to move the empty cell.
///
/// A direction is *one of* `Up`, `Down`, `Left` or `Right`.
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    /// Returns the opposite direction of the current one.
    ///
    /// ```rust
    /// assert_eq!(Direction::Up.opposite(), Direction::Down);
    /// ```
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// Implements pretty printing for the `Direction` enum.
// This is what is used when you use the `{}` format specifier in a `println!` macro.
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "↑",
                Direction::Down => "↓",
                Direction::Left => "←",
                Direction::Right => "→",
            }
        )
    }
}

/// An iterable sequence of all possible directions.
///
/// ```rust
/// for direction in &DIRECTIONS {
///    println!("{direction:?}");
/// }
/// ```
pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

/// A list of instances of the 8-puzzle problem, with know distance to the goal state.
pub const INSTANCES: [(u32, Board); 32] = [
    (0, Board::new([[1, 2, 3], [4, 5, 6], [7, 8, 0]])), // goal state (reachable in 0 actions)
    (1, Board::new([[1, 2, 3], [4, 5, 6], [7, 0, 8]])), // state reachable in 1 action)
    (2, Board::new([[1, 2, 3], [4, 5, 6], [0, 7, 8]])), // state reachable in 2 actions)
    (3, Board::new([[1, 2, 3], [4, 8, 5], [7, 0, 6]])), // ...
    (4, Board::new([[1, 5, 2], [4, 0, 3], [7, 8, 6]])),
    (5, Board::new([[4, 1, 3], [0, 2, 6], [7, 5, 8]])),
    (6, Board::new([[4, 1, 3], [7, 2, 6], [0, 5, 8]])),
    (7, Board::new([[5, 1, 3], [0, 2, 6], [4, 7, 8]])),
    (8, Board::new([[5, 4, 2], [1, 0, 3], [7, 8, 6]])),
    (9, Board::new([[8, 1, 3], [0, 2, 5], [4, 7, 6]])),
    (10, Board::new([[8, 1, 3], [4, 2, 5], [0, 7, 6]])),
    (11, Board::new([[8, 1, 3], [4, 2, 5], [7, 0, 6]])),
    (12, Board::new([[8, 4, 2], [1, 0, 3], [7, 6, 5]])),
    (13, Board::new([[8, 4, 3], [0, 1, 5], [2, 7, 6]])),
    (14, Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]])),
    (15, Board::new([[8, 7, 3], [2, 5, 0], [1, 4, 6]])),
    (16, Board::new([[8, 7, 3], [4, 1, 5], [0, 2, 6]])),
    (17, Board::new([[8, 7, 3], [4, 1, 5], [2, 0, 6]])),
    (18, Board::new([[8, 7, 5], [3, 0, 2], [1, 4, 6]])),
    (19, Board::new([[8, 7, 5], [3, 4, 2], [1, 0, 6]])),
    (20, Board::new([[8, 7, 6], [2, 0, 3], [1, 4, 5]])),
    (21, Board::new([[8, 7, 6], [2, 4, 3], [1, 0, 5]])),
    (22, Board::new([[8, 7, 6], [4, 1, 2], [0, 5, 3]])),
    (23, Board::new([[8, 7, 6], [4, 1, 2], [5, 0, 3]])),
    (24, Board::new([[8, 7, 6], [5, 3, 1], [0, 2, 4]])),
    (25, Board::new([[8, 7, 6], [5, 4, 2], [1, 0, 3]])),
    (26, Board::new([[8, 7, 6], [5, 4, 2], [1, 3, 0]])),
    (27, Board::new([[8, 7, 6], [5, 4, 1], [3, 0, 2]])),
    (28, Board::new([[8, 7, 6], [5, 4, 3], [0, 2, 1]])),
    (29, Board::new([[8, 7, 6], [5, 4, 3], [2, 0, 1]])),
    (30, Board::new([[8, 7, 6], [5, 4, 3], [2, 1, 0]])),
    (31, Board::new([[8, 6, 7], [2, 5, 4], [3, 0, 1]])),
];

/// A module that will be compiled only when running tests.
#[cfg(test)]
mod tests {
    // import everything from the containing module (Board, Direction, ...)
    use super::*;

    // A unit test that succeeds if the code does not panic.
    // This one is meant to test the indexing of the board
    #[test]
    fn test_position() {
        // create a new board
        let board = Board::new([[1, 2, 3], [4, 5, 6], [0, 7, 8]]);
        // check that the empty cell is at the expected position
        // the code will panic if this is not the case, making the test fail
        assert_eq!(board.position(EMPTY_CELL), (2, 0));
        assert_eq!(board.value_at(2, 0), EMPTY_CELL);

        assert_eq!(board.value_at(1, 1), 5);
        assert_eq!(board.value_at(2, 2), 8);
        assert_eq!(board.position(3), (0, 2));
        assert_eq!(board.position(5), (1, 1));
    }

    #[test]
    fn test_apply() {
        let board = Board::new([[1, 2, 3], [4, 5, 6], [0, 7, 8]]);
        assert_eq!(
            board.apply(Direction::Up),
            Some(Board::new([[1, 2, 3], [0, 5, 6], [4, 7, 8]]))
        );
        // what is the result of moving the empty cell right? was the `board` binding modified by the apply method?
        assert_eq!(
            board.apply(Direction::Right),
            Some(Board::new([[1, 2, 3], [4, 5, 6], [7, 0, 8]]))
        );
        // what is the result of moving the empty cell left?
        assert_eq!(
            board.apply(Direction::Left),
            None
        );
    }

    #[test]
    fn test_plan() {
        use Direction::*; // import to avoid repeating `Direction::`
        let board = Board::new([[1, 2, 3], [4, 5, 6], [0, 7, 8]]);

        // a valid optimal plan
        assert!(board.is_valid_plan(&[Right, Right]));

        // a valid suboptimal plan
        assert!(board.is_valid_plan(&[Right, Up, Down, Right]));

        // invalid plan (does not end in the goal state)
        assert!(!board.is_valid_plan(&[Right, Up, Down, Right, Up]));
        // invalid plan (moves the empty cell out of the board)
        assert!(!board.is_valid_plan(&[Left]));
    }
}
