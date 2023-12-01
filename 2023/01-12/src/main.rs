use std::{env, fs::read_to_string, collections::HashMap};

fn list_numbers(line: &String) -> Vec<char> {
    line
        .chars()
        .filter(|c: &char| c.is_digit(10))
        .collect()
}

fn list_numbers_including_numbers_as_words(line: &String) -> Vec<char> {

    // List out a map of the number words and their corresponding integer as a char
    let map: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ]);

    line
        .chars()
        .enumerate()
        .map(|(idx, c)| -> char {

            // Digit chars can just be returned immediately
            if c.is_digit(10) {
                c
            } else {

                // Loop over all hashmap keys and try to find one of them starting from the current char.
                let found = map
                    .keys()
                    .find(|key| {

                        // If we get the substring (not the case when going over max string length) then check if 
                        // it equals the key we're trying to find. If it equals it mark the key as found. 
                        match line.get(idx..idx+key.len()) {
                            Some(_key) => _key == **key,
                            None => false
                        }
                    });

                // Check if we found a key, if we did we can return the value from the hash map, otherwise
                // we can just return a nonsensical char
                match found {
                    Some(key) => *map.get(key).unwrap(),
                    None => 'N'
                }

            }
        })
        .filter(|c| c.is_digit(10)) // Filter out any non digit chars
        .collect() // Collect into vector
}

fn format_line_number(line: Vec<char>) -> i32 {
    // Concat chars and parse into integer
    format!("{}{}", line.first().unwrap(), line.last().unwrap()).parse::<i32>().unwrap()
}

fn main() {

    // Grab first argument (after binary) as file name and read into string
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

    // List out all lines into a string vector
    let lines: Vec<String> = contents.lines().map(|l| l.to_owned()).collect::<Vec<String>>();

    // Map out all known integer numbers into a result vec, then fold into the answer
    let total_part_one: i32 = lines
        .iter()
        .map(list_numbers)
        .map(format_line_number)
        .fold(0, |acc, next| acc + next);

    // Map out all known integer numbers & word numbers into a result vec, then fold into the answer
    let total_part_two: i32 = lines
        .iter()
        .map(list_numbers_including_numbers_as_words)
        .map(format_line_number)
        .fold(0, |acc, next| acc + next);

    println!("Answer part one: {}", total_part_one);
    println!("Answer part two: {}", total_part_two);
}
