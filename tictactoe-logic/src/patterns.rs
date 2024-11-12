use crate::grid::{FieldStates, Grid};

pub fn patterns(grid: &Grid, state: FieldStates) -> Vec<Grid> {
    let (rows, cols) = grid.size();
    // tic tac toe patterns
    // Diagonals
    let mut patterns = Vec::new();

    // Diagonal from top-left to bottom-right
    let mut diagonal1 = Grid::new(rows, cols).populate();
    for i in 0..std::cmp::min(rows, cols) {
        diagonal1.set(i, i, state.clone());
    }
    patterns.push(diagonal1);

    // Diagonal from top-right to bottom-left
    let mut diagonal2 = Grid::new(rows, cols);
    for i in 0..std::cmp::min(rows, cols) {
        diagonal2.set(i, cols - 1 - i, state.clone());
    }
    patterns.push(diagonal2);

    // Rows
    for i in 0..rows {
        let mut row = Grid::new(rows, cols);
        for j in 0..cols {
            row.set(i, j, state.clone());
        }
        patterns.push(row);
    }

    // Columns
    for j in 0..cols {
        let mut col = Grid::new(rows, cols);
        for i in 0..rows {
            col.set(i, j, state.clone());
        }
        patterns.push(col);
    }
    patterns
}

#[cfg(test)]
mod test {
    use crate::grid::{FieldStates, Grid};

    use super::patterns;

    #[test]
    pub fn correct_patterns() {
        let initial = Grid::new(3, 3).populate();
        let patterns = patterns(&initial, FieldStates::Player1);

        let expected_patterns = vec![
            // Diagonal from top-left to bottom-right
            "X - -\n- X -\n- - X\n",
            // Diagonal from top-right to bottom-left
            "- - X\n- X -\nX - -\n",
            // Rows
            "X X X\n- - -\n- - -\n",
            "- - -\nX X X\n- - -\n",
            "- - -\n- - -\nX X X\n",
            // Columns
            "X - -\nX - -\nX - -\n",
            "- X -\n- X -\n- X -\n",
            "- - X\n- - X\n- - X\n",
        ]
        .iter()
        .map(|string| {
            string
                .replace("X", &FieldStates::Player1.to_string())
                .replace("-", &FieldStates::Empty.to_string())
        })
        .collect::<Vec<_>>();

        assert_eq!(
            patterns.len(),
            expected_patterns.len(),
            "Number of patterns doesn't match"
        );

        for (index, pattern) in patterns.iter().enumerate() {
            let pattern_string = pattern.to_string();
            assert_eq!(
                pattern_string, expected_patterns[index],
                "Pattern {} doesn't match expected pattern:\nExpected:\n{}\nGot:\n{}",
                index, expected_patterns[index], pattern_string
            );
        }
    }
}
