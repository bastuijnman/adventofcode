use std::{env, fs::read_to_string};

use regex::Regex;

fn main() {
    
    // Grab first argument (after binary) as file name and read into string
    let mut contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

    // Get line length first
    let line_length: i32 = contents.lines().into_iter().nth(0).unwrap().len() as i32;

    // Get rid of newlines to make char index processing easier
    contents = contents.replace("\n", "");

    // List regexes for numbers & symbols
    let numbers = Regex::new(r"([0-9]+)").unwrap();
    let symbols = Regex::new(r"[^0-9.]").unwrap();

    let sum_part_one = numbers
        .captures_iter(&contents.as_str())
        .filter_map(|c| {

            // Get match
            let m = c.get(1).unwrap();

            // Get match position (string index where the match starts)
            let pos:i32 = m.start() as i32;

            // Get actual value.
            let val = m.as_str();

            // Generate range for rows, which apparently doesn't work nicely when just putting the range in the for loop?
            let range:Vec<i32> = (-1..2).collect();

            for y in range {

                // Get the start of the row we want to process.
                let row_start = pos + (line_length * y);

                // Ignore row when trying to go under first or over last
                if row_start < 0 || row_start > contents.len() as i32 {
                    continue;
                }

                // Check offset so we don't accidentally take one from a row up, otherwise
                // offset one negative to grab char from the left of the number.
                let offset = if row_start % line_length == 0 { 0 } else { 1 };

                // Loop over row with the number length + one to grab the char from the right
                // of the number.
                for x in row_start-offset..row_start+(val.len() as i32 + 1) {

                    // Grab the character to test for symbol
                    let test = contents.chars().nth(x.try_into().unwrap()).unwrap();

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
            }
            
            None
        })
        .fold(0, |acc, next| acc + next); // Accumulate values via fold

    let gears = Regex::new(r"(\*)").unwrap();
    let sum_part_two = gears
        .captures_iter(&contents.as_str())
        .filter_map(|c| {

            // Get match
            let m = c.get(1).unwrap();

            // Get match position (string index where the match starts)
            let pos:i32 = m.start() as i32;

            // Get actual value.
            let val = m.as_str();

            // Generate range for rows, which apparently doesn't work nicely when just putting the range in the for loop?
            let range:Vec<i32> = (-1..2).collect();

            let mut results: Vec<i32> = Vec::new();
            let mut blocklist: Vec<i32> = Vec::new();

            for y in range {

                // Get the start of the row we want to process.
                let row_start = pos + (line_length * y);

                // Ignore row when trying to go under first or over last
                if row_start < 0 || row_start > contents.len() as i32 {
                    continue;
                }

                // Check offset so we don't accidentally take one from a row up, otherwise
                // offset one negative to grab char from the left of the number.
                let offset = if row_start % line_length == 0 { 0 } else { 1 };

                // Loop over row with the number length + one to grab the char from the right
                // of the number.
                for x in row_start-offset..row_start+(val.len() as i32 + 1) {

                    // If we've already processed this number skip the cell
                    if blocklist.contains(&x) {
                        continue;
                    }

                    // Grab the character to test for symbol
                    let test = contents.chars().nth(x.try_into().unwrap()).unwrap();

                    // Track offsets for grabbing the entire number
                    let mut offset_left:i32 = x;
                    let mut offset_right:i32 = x;

                    // Check if the cell is a number
                    if numbers.find(&test.to_string()).is_some() {
                        
                        // Spread out left while we find numbers
                        while offset_left > 0 {
                            let test = contents.chars().nth(offset_left as usize - 1).unwrap();

                            // No more number cells, stop loop
                            if numbers.find(&test.to_string()).is_none() {
                                break;
                            }
                            offset_left -= 1;
                        }

                        // Spread out right while we find numbers
                        while offset_right < contents.len() as i32 {
                            let test = contents.chars().nth(offset_right as usize).unwrap();

                            // No more number cells, stop loop.
                            if numbers.find(&test.to_string()).is_none() {
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
