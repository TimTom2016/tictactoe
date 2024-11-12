use crate::grid::{FieldStates, Grid};
#[derive(Clone)]
pub struct MiniMax {
    grid: Grid,
}

impl MiniMax {
    pub fn new(grid: &Grid) -> Self {
        Self { grid: grid.clone() }
    }
    pub fn calculate(&mut self) -> Grid {
        let size = self.grid.size();
        let (best_move, score) =
            self.minimax((size.1 * size.0) as i8, true, std::i8::MIN, std::i8::MAX);
        self.grid.set_elem(best_move as usize, FieldStates::Player2);
        self.grid.clone()
    }

    fn minimax(&self, depth: i8, maximize_win: bool, mut alpha: i8, mut beta: i8) -> (i8, i8) {
        if self.grid.check_win(FieldStates::Player2) {
            return (-1, 10 + depth); // AI wins
        }
        if self.grid.check_win(FieldStates::Player1) {
            return (-1, -10 - depth); // Human wins
        }
        if depth == 0 || self.grid.is_full() {
            return (-1, 0); // Tie or max depth reached
        }

        let mut best_move = -1;
        let mut best_score = if maximize_win {
            std::i8::MIN
        } else {
            std::i8::MAX
        };

        for (i, cell) in self.grid.clone().into_iter().enumerate() {
            if cell != FieldStates::Empty {
                continue;
            }
            let mut next_move = self.clone();
            next_move.grid.set_elem(
                i,
                if maximize_win {
                    FieldStates::Player2
                } else {
                    FieldStates::Player1
                },
            );
            let (_, score) = next_move.minimax(depth - 1, !maximize_win, alpha, beta);

            if maximize_win {
                if score > best_score {
                    best_score = score;
                    best_move = i as i8;
                }
                alpha = alpha.max(best_score);
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = i as i8;
                }
                beta = beta.min(best_score);
            }

            if beta <= alpha {
                break;
            }
        }

        (best_move, best_score)
    }
    pub fn calculate_without_pruning(&mut self) -> Grid {
        let size = self.grid.size();
        let (best_move, score) = self.minimax_simple((size.1 * size.0) as i8, true);
        self.grid.set_elem(best_move as usize, FieldStates::Player2);
        self.grid.clone()
    }

    fn minimax_simple(&self, depth: i8, maximize_win: bool) -> (i8, i8) {
        if self.grid.check_win(FieldStates::Player2) {
            return (-1, 10 + depth); // AI wins
        }
        if self.grid.check_win(FieldStates::Player1) {
            return (-1, -10 - depth); // Human wins
        }
        if depth == 0 || self.grid.is_full() {
            return (-1, 0); // Tie or max depth reached
        }

        let mut best_move = -1;
        let mut best_score = if maximize_win {
            std::i8::MIN
        } else {
            std::i8::MAX
        };

        for (i, cell) in self.grid.clone().into_iter().enumerate() {
            if cell != FieldStates::Empty {
                continue;
            }
            let mut next_move = self.clone();
            next_move.grid.set_elem(
                i,
                if maximize_win {
                    FieldStates::Player2
                } else {
                    FieldStates::Player1
                },
            );
            let (_, score) = next_move.minimax_simple(depth - 1, !maximize_win);

            if maximize_win {
                if score > best_score {
                    best_score = score;
                    best_move = i as i8;
                }
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = i as i8;
                }
            }
        }

        (best_move, best_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{FieldStates, Grid};

    #[test]
    fn test_immediate_win() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, FieldStates::Player2);
        grid.set(1, 1, FieldStates::Player2);
        grid.set(0, 1, FieldStates::Player1);
        grid.set(0, 2, FieldStates::Player1);

        let mut minimax = MiniMax::new(&grid);
        let result = minimax.calculate();
        assert_eq!(result.get(2, 2), Some(&FieldStates::Player2));
    }

    #[test]
    fn test_block_opponent_win() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, FieldStates::Player1);
        grid.set(1, 1, FieldStates::Player1);

        let mut minimax = MiniMax::new(&grid);
        let result = minimax.calculate();

        assert_eq!(result.get(2, 2), Some(&FieldStates::Player2));
    }

    #[test]
    fn test_empty_board() {
        let grid = Grid::new(3, 3);
        let mut minimax = MiniMax::new(&grid);
        let result = minimax.calculate();

        // The first move should be in a corner or center for optimal play
        let corner_or_center = vec![(0, 0), (0, 2), (2, 0), (2, 2), (1, 1)];
        let first_move = corner_or_center
            .iter()
            .any(|&(x, y)| result.get(x, y) == Some(&FieldStates::Player2));
        assert!(first_move);
    }

    #[test]
    fn test_prefer_win_over_block() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, FieldStates::Player2);
        grid.set(1, 1, FieldStates::Player2);
        grid.set(0, 1, FieldStates::Player1);
        grid.set(0, 2, FieldStates::Player1);
        grid.set(1, 0, FieldStates::Player1);

        let mut minimax = MiniMax::new(&grid);
        let result = minimax.calculate();

        // Player2 should choose to win rather than block
        assert_eq!(result.get(2, 2), Some(&FieldStates::Player2));
    }

    #[test]
    fn test_tie_game() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, FieldStates::Player1);
        grid.set(0, 1, FieldStates::Player2);
        grid.set(0, 2, FieldStates::Player1);
        grid.set(1, 0, FieldStates::Player2);
        grid.set(1, 1, FieldStates::Player1);
        grid.set(2, 0, FieldStates::Player2);
        grid.set(2, 1, FieldStates::Player1);

        let mut minimax = MiniMax::new(&grid);
        let result = minimax.calculate();

        // The only move left should be (1, 2) or (2, 2)
        assert!(
            result.get(1, 2) == Some(&FieldStates::Player2)
                || result.get(2, 2) == Some(&FieldStates::Player2)
        );
    }

    #[test]
    fn test_performance() {
        let grid = Grid::new(3, 3);
        let mut minimax = MiniMax::new(&grid);

        use std::time::Instant;
        let start = Instant::now();
        minimax.calculate();
        let duration = start.elapsed();
        assert!(duration.as_secs() < 1, "Minimax took too long to calculate");
    }
}
