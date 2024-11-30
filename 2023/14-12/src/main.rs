use std::{env, fs::read_to_string};

fn calculate_col(col: &[char]) -> usize {
    let col_len = col.len();
    let mut current_north_value = col_len;
    let mut total = 0;

    for (i, item) in col.iter().enumerate() {
        match item {
            'O' => {
                total += current_north_value;
                current_north_value -= 1;
            }
            '#' => {
                current_north_value = col_len - (i + 1);
            }
            _ => {}
        }
    }

    total
}

fn main() {
    // Grab first argument (after binary) as file name and read into string
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();
    let chars: Vec<char> = contents.chars().filter(|c| *c != '\n').collect();

    let lines: Vec<&str> = contents.split("\n").collect();
    let col_len = lines.len() - 1; // Account for last newline
    let row_len = lines.first().unwrap().len();

    let cols: Vec<Vec<char>> = (0..row_len)
        .map(|i| (0..col_len).map(|j| chars[(j * row_len) + i]).collect())
        .collect();

    let cols_calulated = cols.iter().fold(0, |acc, col| acc + calculate_col(col));
    println!("Answer part 1: {}", cols_calulated);
}
