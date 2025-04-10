use crate::board::*;
use crate::heuristics::*;
use crate::min_heap::*;
use std::collections::*;
use std::ptr::null;
use std::time::Duration;

/// Statistics of the search, used to evaluate the performance of the search algorithms.
/// Feel free to add more fields to this struct if you need them.
pub struct Stats {
    /// Numbers of states expanded during search
    pub expanded: usize,
    /// Total runtime spend in the search.
    ///
    /// ```rust
    /// let start_time: Instant = std::time::Instant::now();
    /// // do something
    /// let runtime: Duration = start_time.elapsed();
    /// ```
    pub runtime: Duration,
}

impl Stats {
    /// Creates a new `Stats` instance with the given expanded states count and runtime.
    pub fn new(expanded: usize, runtime: Duration) -> Stats {
        Stats { expanded, runtime }
    }
}

pub fn search(init_state: Board, heuristic: Heuristic) -> (Option<Vec<Direction>>, Stats) {
    let start = std::time::Instant::now();
    // MinHeap provide allows to store the states to explore, with associated priority
    let mut heap: MinHeap<Board> = MinHeap::new();
    // the standard library provides a HashMap, that can be used to store the cost or other things
    let mut costs: HashMap<Board, u32> = HashMap::new();
    // ...
    let mut parent_actions: HashMap<Board, Option<(Board, Direction)>> = HashMap::new();

    let mut visited_board: HashSet<Board> = HashSet::new();

    let mut path: Vec<Direction> = Vec::new();

    heap.insert(init_state, 0); //heuristic.estimate(&init_state));
    costs.insert(init_state, 0);
    parent_actions.insert(init_state, None);

    loop {
        if heap.is_empty() {
            break;
        }

        match heap.pop() {
            Some (board) => {
                if visited_board.contains(&board) {
                    continue;
                }

                if board == Board::GOAL {
                    let mut current_board = board;

                    loop {
                        match parent_actions.get(&current_board) {
                            Some(Some((parent_board, direction))) => {
                                path.push(*direction);
                                current_board = *parent_board;
                            },
                            _ => break
                        }
                    }

                    break
                }

                for direction in DIRECTIONS {
                    match board.apply(direction) {
                        Some(new_board) => {
                            let new_cost = match (costs.get(&board)) {
                                Some(cost) => cost + 1,
                                None => panic!("No cost found"),
                            };

                            let better_path = match costs.get(&new_board) {
                                Some(previous_cost) => new_cost < *previous_cost,
                                None => true
                            };

                            if better_path {
                                costs.insert(new_board, new_cost);
                                parent_actions.insert(new_board, Some((board, direction)));
                                heap.insert(new_board, new_cost); //heuristic.estimate(&new_board));
                            }
                        },
                        None => continue
                    }
                }

                visited_board.insert(board);
            },
            None => continue,
        }
    }

    path.reverse();
    // here is an example to measure the runtime and returns the statistics
    let runtime = start.elapsed();
    // example to construct a Stats instance
    let stats = Stats::new(0, runtime);
    // return the results and associated stats
    (Some(path), stats)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_search() {
        use super::*;

        // validates that search oes return the optimal plan on the first 20 isntances
        for (expected_cost, init) in &INSTANCES[0..20] {
            let (path, stats) = search(*init, Heuristic::Hamming);
            let path = path.expect("no plan");
            assert!(init.is_valid_plan(&path));
            assert_eq!(path.len(), *expected_cost as usize);
        }
    }
}
