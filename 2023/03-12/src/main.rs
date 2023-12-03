use std::{env, fs::read_to_string};

use regex::{Regex, Captures};

fn map_position_and_value(captures: Captures<'_>) -> (i32, &str) {

    // Get match
    let m = captures.get(1).unwrap();

    // Get match position (string index where the match starts)
    let pos:i32 = m.start() as i32;

    // Get actual value.
    let val = m.as_str();

    (pos, val)
}

fn main() {
    
    // Grab first argument (after binary) as file name and read into string
    let mut contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

    // Get line length first
    let line_length: i32 = contents.lines().into_iter().nth(0).unwrap().len() as i32;

    // Get rid of newlines to make char index processing easier
    contents = contents.replace("\n", "");

    // List regexes for part numbers, symbols and gears
    let part_numbers = Regex::new(r"([0-9]+)").unwrap();
    let symbols = Regex::new(r"[^0-9.]").unwrap();
    let gears = Regex::new(r"(\*)").unwrap();

    let sum_part_one = part_numbers
        .captures_iter(&contents.as_str())
        .map(map_position_and_value)
        .filter_map(|(pos, val)| {

            let not_beginning_of_line: bool = pos % line_length != 0;
            let cell_positions: Vec<i32> = [

                // Cells in row above position
                (((pos - line_length) - not_beginning_of_line as i32)..((pos - line_length) + (val.len() as i32 + 1))).collect::<Vec<i32>>(),
                
                // Cells in same row as position
                ((pos - not_beginning_of_line as i32)..(pos + (val.len() as i32 + 1))).collect::<Vec<i32>>(),
                
                // Cells in row below position
                (((pos + line_length) - not_beginning_of_line as i32)..((pos + line_length) + (val.len() as i32 + 1))).collect::<Vec<i32>>()
            ].concat().into_iter().filter(|i| *i >= 0 && *i <= contents.len() as i32-1).collect(); // Filter out non-valid positions

            for cell in cell_positions {
                // Grab the character to test for symbol
                let test = contents.chars().nth(cell.try_into().unwrap()).unwrap();

                // Check wether the char is a symbol.
                let found = match symbols.find(&test.to_string()) {
                    Some(_val) => true,
                    None => false
                };
                
                // Found a symbol so we can return it in the map
                if found {
                    return Some(val.parse::<i32>().unwrap());
                }
            }
            
            None
        })
        .fold(0, |acc, next| acc + next); // Accumulate values via fold

    let sum_part_two = gears
        .captures_iter(&contents.as_str())
        .map(map_position_and_value)
        .filter_map(|(pos, val)| {
            let mut results: Vec<i32> = Vec::new();
            let mut blocklist: Vec<i32> = Vec::new();

            let not_beginning_of_line: bool = pos % line_length != 0;
            let cell_positions: Vec<i32> = [

                // Cells in row above position
                (((pos - line_length) - not_beginning_of_line as i32)..((pos - line_length) + (val.len() as i32 + 1))).collect::<Vec<i32>>(),
                
                // Cells in same row as position
                ((pos - not_beginning_of_line as i32)..(pos + (val.len() as i32 + 1))).collect::<Vec<i32>>(),
                
                // Cells in row below position
                (((pos + line_length) - not_beginning_of_line as i32)..((pos + line_length) + (val.len() as i32 + 1))).collect::<Vec<i32>>()
            ].concat().into_iter().filter(|i| *i >= 0 && *i <= contents.len() as i32-1).collect(); // Filter out non-valid positions

            for cell in cell_positions {

                // If we've already processed this number skip the cell
                if blocklist.contains(&cell) {
                    continue;
                }

                // Grab the character to test for symbol
                let test = contents.chars().nth(cell.try_into().unwrap()).unwrap();

                // If cell is not a number we can ignore it.
                if part_numbers.find(&test.to_string()).is_none() {
                    continue;
                }

                // Track offsets for grabbing the entire number
                let mut offset_left:i32 = cell;
                let mut offset_right:i32 = cell;
                    
                // Spread out left while we find numbers
                while offset_left > 0 {
                    let test = contents.chars().nth(offset_left as usize - 1).unwrap();

                    // No more number cells, stop loop
                    if part_numbers.find(&test.to_string()).is_none() {
                        break;
                    }
                    offset_left -= 1;
                }

                // Spread out right while we find numbers
                while offset_right < contents.len() as i32 {
                    let test = contents.chars().nth(offset_right as usize).unwrap();

                    // No more number cells, stop loop.
                    if part_numbers.find(&test.to_string()).is_none() {
                        break;
                    }
                    offset_right += 1;
                }

                // Grab substring based on offsets 
                let result = contents.get(offset_left as usize..offset_right as usize);
                if result.is_some() {

                    // Add processed cells to blocklist
                    let offsets: Vec<i32> = (offset_left..offset_right).collect();
                    blocklist = [blocklist, offsets].concat();

                    // Push result into vec
                    results.push(result.unwrap().parse::<i32>().unwrap());
                }
            }

            // Exactly two adjecent numbers, so it's a gear and can be processed
            if results.len() == 2 {
                Some(results[0] * results[1])
            } else {
                None
            }
        })
        .fold(0, |acc, next| acc + next);
    
    println!("Answer part one: {}", sum_part_one);
    println!("Answer part one: {}", sum_part_two);
}
