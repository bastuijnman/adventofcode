use std::{char, cmp::min, env, fs::read_to_string};

struct Pattern {
    chars: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Pattern {
    fn row(&self, idx: usize) -> Vec<char> {
        (0..self.cols)
            .map(|i| self.chars[(idx * self.cols) + i])
            .collect()
    }

    fn col(&self, idx: usize) -> Vec<char> {
        (0..self.rows)
            .map(|i| self.chars[(i * self.cols) + idx])
            .collect()
    }

    fn row_reflection(&self) -> usize {
        let potential_reflections: Vec<(usize, bool)> = (0..self.rows - 1)
            .map(|row| self.row(row) == self.row(row + 1))
            .enumerate()
            .filter(|(_, row)| *row)
            .collect();

        for (idx, _) in potential_reflections.iter().rev() {
            let offset = min(*idx, self.rows - (idx + 2));

            let a: Vec<Vec<char>> = (idx - offset..=*idx).map(|i| self.row(i)).collect();
            let b: Vec<Vec<char>> = (idx + 1..=idx + 1 + offset)
                .rev()
                .map(|i| self.row(i))
                .collect();

            if a == b {
                // Convert from index to row number
                return *idx + 1;
            }
        }

        0
    }

    fn col_reflection(&self) -> usize {
        let potential_reflections: Vec<(usize, bool)> = (0..self.cols - 1)
            .map(|col| self.col(col) == self.col(col + 1))
            .enumerate()
            .filter(|(_, col)| *col)
            .collect();

        for (idx, _) in potential_reflections.iter().rev() {
            let offset = min(*idx, self.cols - (idx + 2));

            let a: Vec<Vec<char>> = (idx - offset..=*idx).map(|i| self.col(i)).collect();
            let b: Vec<Vec<char>> = (idx + 1..=idx + 1 + offset)
                .rev()
                .map(|i| self.col(i))
                .collect();

            if a == b {
                // Convert from index to row number
                return *idx + 1;
            }
        }
        0
    }
}

fn parse_pattern(input: &str) -> Pattern {
    let lines: Vec<&str> = input.split("\n").filter(|line| !line.is_empty()).collect();
    let rows = lines.len();
    let cols = lines[0].len();

    Pattern {
        chars: lines.join("").chars().collect(),
        rows,
        cols,
    }
}

fn main() {
    // Grab first argument (after binary) as file name and read into string
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

    let patterns: Vec<Pattern> = contents.split("\n\n").map(parse_pattern).collect();
    let reflections: (usize, usize) =
        patterns.iter().fold((0, 0), |(col_acc, row_acc), pattern| {
            (
                col_acc + pattern.col_reflection(),
                row_acc + pattern.row_reflection(),
            )
        });

    println!("Answer part 1: {}", reflections.0 + (reflections.1 * 100));
}
