use std::{fs::read_to_string, env, cmp};

///
/// Calculate the sum of shortest paths given the contentents of the input
/// and the dark energy constant.
/// 
fn calculate (contents: String, dark_energy: usize) -> i64 {

    // Grab line length and map out characters into vec
    let line_len = contents.lines().nth(0).unwrap().trim().len();
    let map:Vec<char> = contents.replace("\n", "").chars().collect();

    // Find the rows that should be expanding due to empty space
    let expanding_rows: Vec<usize> = (0..(map.len() / line_len)).filter(|row| {
        let mut is_empty = true;
        for col in 0..line_len {
            if map[col + (row * line_len)] == '#' {
                is_empty = false;
                break;
            }
        }
        is_empty
    }).collect();

    // Find the cols that should be expanding due to empty space
    let expanding_cols: Vec<usize> = (0..line_len).filter(|col| {
        let mut is_empty = true;
        for row in 0..(map.len() / line_len) {
            if map[col + (row * line_len)] == '#' {
                is_empty = false;
                break;
            }
        }
        is_empty
    }).collect();

    // Find all galacies and map them out in X/Y coordinates
    let galaxies: Vec<(i64, i64)> = map
        .iter()
        .enumerate()
        .map(|(idx, c)| (idx % line_len, idx / line_len, *c))
        .filter(|(_x, _y, c)| *c == '#')
        .map(|(x, y, _c)| (x as i64, y as i64))
        .collect();
    
    // Hold a reference to the sum of the shorted paths
    let mut paths_sum = 0;

    // Loop through all possible galaxy pairs
    for i in 0..(galaxies.len() - 1) {
        for j in (i+1)..galaxies.len() {

            // Find shortest horizontal between galaxy pair
            let horizontal_distance = (galaxies[j].0 - galaxies[i].0).abs();

            // Calculate the additional space due to expansion
            let horizontal_expansion_impacted = expanding_cols
                .iter()
                .filter(|n| {
                    let min = cmp::min(galaxies[i].0, galaxies[j].0);
                    let max = cmp::max(galaxies[i].0, galaxies[j].0);
                    
                    (min..max).contains(&(**n as i64))
                })
                .count() as i64 * dark_energy as i64;

            // Find the shortest vertical path between galacy pair
            let vertical_distance = (galaxies[j].1 - galaxies[i].1).abs();

            // Calculate the additional space due to expansion
            let vertical_expansion_impacted = expanding_rows
                .iter()
                .filter(|n| {
                    let min = cmp::min(galaxies[i].1, galaxies[j].1);
                    let max = cmp::max(galaxies[i].1, galaxies[j].1);
                    
                    (min..max).contains(&(**n as i64))
                })
                .count() as i64 * dark_energy as i64;
            
            // Add all path components & add to sum
            paths_sum += horizontal_distance + horizontal_expansion_impacted + vertical_distance + vertical_expansion_impacted;
        }
    }

    // Return sum
    paths_sum
}

fn main() {

    // Grab first argument (after binary) as file name and read into string
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();
    
    // Calculate the sum of shortest distances between all galaxies, instead of 
    // replacing the space that is expanding we add onto it, so we subtract one
    // from the expansion ratio as defined in the puzzle.
    println!("Answer part one: {}", calculate(contents.clone(), 1));
    println!("Answer part two: {}", calculate(contents.clone(), 999999));
}
