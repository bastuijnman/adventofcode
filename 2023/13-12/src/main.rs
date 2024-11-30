use std::{char, cmp::min, env, fs::read_to_string, usize};

#[derive(Clone)]
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

    fn row_reflection(&self, mut exclude: usize) -> usize {
        if exclude == 0 {
            exclude = usize::MAX;
        }

        let potential_reflections: Vec<(usize, bool)> = (0..self.rows - 1)
            .map(|row| self.row(row) == self.row(row + 1) && row != exclude - 1)
            .enumerate()
            .filter(|(_, row)| *row)
            .collect();

        for (idx, _) in potential_reflections.iter() {
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

    fn col_reflection(&self, mut exclude: usize) -> usize {
        if exclude == 0 {
            exclude = usize::MAX;
        }

        let potential_reflections: Vec<(usize, bool)> = (0..self.cols - 1)
            .map(|col| self.col(col) == self.col(col + 1) && col != exclude - 1)
            .enumerate()
            .filter(|(_, col)| *col)
            .collect();

        for (idx, _) in potential_reflections.iter() {
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

    fn fix_smudges(&mut self) {
        let cc = self.col_reflection(usize::MAX);
        let cr = self.row_reflection(usize::MAX);

        // Inefficient as shit
        for i in 0..self.chars.len() {
            self.chars[i] = match self.chars[i] {
                '.' => '#',
                '#' => '.',
                _ => '_',
            };

            let c = self.col_reflection(cc);
            let r = self.row_reflection(cr);
            if (cc != c && c != 0) || (r != cr && r != 0) {
                return;
            }

            // Revert if not found
            self.chars[i] = match self.chars[i] {
                '.' => '#',
                '#' => '.',
                _ => '_',
            };
        }
        println!("NO SMUDGES FOUND");
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

    let reflections: Vec<[usize; 2]> = patterns
        .iter()
        .map(|pattern| {
            [
                pattern.col_reflection(usize::MAX),
                pattern.row_reflection(usize::MAX),
            ]
        })
        .collect();

    let sums = reflections
        .iter()
        .fold((0, 0), |(col_acc, row_acc), reflection| {
            (col_acc + reflection[0], row_acc + reflection[1])
        });

    println!("Answer part 1: {}", sums.0 + (sums.1 * 100));

    let mut fixed_patterns = patterns.clone();
    fixed_patterns.iter_mut().for_each(|p| p.fix_smudges());

    let reflections: (usize, usize) =
        fixed_patterns
            .iter()
            .enumerate()
            .fold((0, 0), |(col_acc, row_acc), (i, pattern)| {
                let cr = pattern.col_reflection(reflections[i][0]);
                let rr = pattern.row_reflection(reflections[i][1]);

                (col_acc + cr, row_acc + rr)
            });

    println!("Answer part 2: {}", reflections.0 + (reflections.1 * 100));
}
