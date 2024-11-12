use std::{fmt::Display, str::FromStr};

use crate::patterns::patterns;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FieldStates {
    Empty,
    Player1,
    Player2,
}

impl Display for FieldStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldStates::Empty => write!(f, "0"),
            FieldStates::Player1 => write!(f, "1"),
            FieldStates::Player2 => write!(f, "2"),
        }
    }
}

impl FromStr for FieldStates {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::Empty,
            "1" => Self::Player1,
            "2" => Self::Player2,
            _ => return Err(()),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    rows: usize,
    cols: usize,
    fields: Vec<FieldStates>,
}
impl Grid {
    pub fn new(rows: usize, cols: usize) -> Grid {
        Self {
            rows,
            cols,
            fields: Vec::new(),
        }
        .populate()
    }
    pub fn populate(mut self) -> Self {
        self.fields.clear();
        self.fields
            .resize(self.rows * self.cols, FieldStates::Empty);
        self
    }
    #[cfg(feature = "random")]
    pub fn random<R: rand::Rng + ?Sized>(mut self, rng: &mut R) -> Self {
        use rand::seq::*;
        self = self.populate();
        self.fields.iter_mut().for_each(move |value| {
            *value = [
                FieldStates::Empty,
                FieldStates::Player1,
                FieldStates::Player2,
            ]
            .choose(rng)
            .unwrap()
            .clone()
        });
        self
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&FieldStates> {
        self.fields.get(col + (self.cols * row))
    }

    pub fn set(&mut self, row: usize, col: usize, new_state: FieldStates) -> Option<FieldStates> {
        let field = self.fields.get_mut(col + (self.cols * row))?;
        if matches!(field, FieldStates::Empty) {
            let old = field.clone();
            *field = new_state;
            Some(old)
        } else {
            None
        }
    }

    pub fn set_elem(&mut self, element: usize, new_state: FieldStates) -> Option<FieldStates> {
        let field = self.fields.get_mut(element)?;
        if matches!(field, FieldStates::Empty) {
            let old = field.clone();
            *field = new_state;
            Some(old)
        } else {
            None
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn check_win(&self, player: FieldStates) -> bool {
        let possible_wins = patterns(self, player);
        for pattern in possible_wins {
            if self.matches(&pattern, player) {
                return true;
            }
        }
        false
    }

    fn matches(&self, pattern: &Grid, player: FieldStates) -> bool {
        for (index, &field) in pattern.fields.iter().enumerate() {
            if field == player && self.fields[index] != player {
                return false;
            }
        }
        true
    }
    pub fn is_full(&self) -> bool {
        for field in self.clone().into_iter() {
            if matches!(field, FieldStates::Empty) {
                return false;
            }
        }
        true
    }
    pub fn from_vec(rows: usize, cols: usize, fields: Vec<&str>) -> Self {
        Self {
            rows,
            cols,
            fields: fields
                .iter()
                .map(|str| FieldStates::from_str(str).unwrap())
                .collect::<Vec<FieldStates>>(),
        }
    }
}

impl FromStr for Grid {
    type Err = String; // Using String as the error type for simplicity

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<&str> = s.trim().split('\n').collect();
        if rows.is_empty() {
            return Err("Empty input".to_string());
        }

        let cols = rows[0].split_whitespace().count();
        if cols == 0 {
            return Err("Invalid row format".to_string());
        }

        let mut grid = Grid::new(rows.len(), cols);

        for (row_idx, row) in rows.iter().enumerate() {
            let fields: Vec<&str> = row.split_whitespace().collect();
            if fields.len() != cols {
                return Err(format!(
                    "Inconsistent number of columns in row {}",
                    row_idx + 1
                ));
            }

            for (col_idx, field) in fields.iter().enumerate() {
                let state = field.parse::<FieldStates>().map_err(move |_| {
                    format!(
                        "Invalid field value '{}' at row {}, column {}",
                        field,
                        row_idx + 1,
                        col_idx + 1
                    )
                })?;
                grid.set(row_idx, col_idx, state);
            }
        }

        Ok(grid)
    }
}

impl IntoIterator for Grid {
    type Item = FieldStates;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if let Some(field) = self.get(row, col) {
                    if col == self.cols - 1 {
                        writeln!(f, "{}", field)?;
                    } else {
                        write!(f, "{} ", field)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::grid::{FieldStates, Grid};

    use super::patterns;
    use std::{fmt::Display, str::FromStr};

    #[test]
    fn test_grid_from_str() {
        // Test valid inputs
        let test_cases = vec![
            ("1 2 0\n2 1 2\n2 2 1", 3, 3),
            ("1 2 0\n0 1 2\n2 0 1", 3, 3),
            ("1 2 0 2\n2 1 2 0\n0 2 1 2\n2 0 2 1", 4, 4),
            ("1", 1, 1),
            ("1 2\n2 1", 2, 2),         // Added 2x2 grid
            ("1 2 0 1\n2 1 0 2", 2, 4), // Added 2x4 grid
        ];

        for (input, expected_rows, expected_cols) in test_cases {
            let grid = Grid::from_str(input).expect("Failed to parse valid grid");
            assert_eq!(
                grid.size(),
                (expected_rows, expected_cols),
                "Incorrect grid size for input: {}",
                input
            );

            // Verify grid contents
            let lines: Vec<&str> = input.lines().collect();
            for (row, line) in lines.iter().enumerate() {
                let fields: Vec<&str> = line.split_whitespace().collect();
                for (col, &field) in fields.iter().enumerate() {
                    let expected_state = match field {
                        "1" => FieldStates::Player1,
                        "2" => FieldStates::Player2,
                        "0" => FieldStates::Empty,
                        _ => panic!("Unexpected field value in test case"),
                    };
                    assert_eq!(
                        grid.get(row, col),
                        Some(&expected_state),
                        "Mismatch at row {}, col {} for input: {}",
                        row,
                        col,
                        input
                    );
                }
            }
        }

        // Test invalid inputs
        let invalid_cases = vec![
            "",             // Empty input
            "0 2\n2 1 1",   // Inconsistent number of columns
            "3 4 5\n1 2 0", // Invalid characters
            "1 2\n2 1\n0",  // Extra row
        ];

        for invalid_input in invalid_cases {
            assert!(
                Grid::from_str(invalid_input).is_err(),
                "Expected error for invalid input: {}",
                invalid_input
            );
        }
    }

    #[test]
    pub fn check_win() {
        // Horizontal win for Player1
        let grid = Grid::from_str("1 1 1\n0 2 0\n2 0 2").unwrap();
        assert!(grid.check_win(FieldStates::Player1));
        assert!(!grid.check_win(FieldStates::Player2));

        // Vertical win for Player2
        let grid = Grid::from_str("2 1 0\n2 1 0\n2 0 1").unwrap();
        assert!(grid.check_win(FieldStates::Player2));
        assert!(!grid.check_win(FieldStates::Player1));

        // Diagonal win for Player1
        let grid = Grid::from_str("1 2 0\n0 1 2\n2 0 1").unwrap();
        assert!(grid.check_win(FieldStates::Player1));
        assert!(!grid.check_win(FieldStates::Player2));

        // No win
        let grid = Grid::from_str("1 2 0\n0 1 2\n2 0 0").unwrap();
        assert!(!grid.check_win(FieldStates::Player1));
        assert!(!grid.check_win(FieldStates::Player2));
    }
}
