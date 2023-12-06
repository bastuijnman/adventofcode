use std::{fs::read_to_string, env};

use regex::Regex;

fn main() {
   
    // Grab first argument (after binary) as file name and read into string
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

    // Regex to list all numbers in the input
    let numbers_regex: Regex = Regex::new(r"([0-9]+)").unwrap();

    // Vector of all numbers
    let numbers:Vec<usize> = numbers_regex
        .captures_iter(&contents)
        .map(|c| c.extract())
        .map(|(_, [number])| number.parse::<usize>().unwrap())
        .collect();

    // Calculate number of races we have, loop over them and then loop through
    // the time it takes the race, for each time unit subtract from the duration and
    // multiply by time that the button is pressed. Retain only entries that beat the
    // distance record.
    let number_of_races = numbers.len() / 2;
    let answer_part_one = (0..number_of_races).map(|idx| {
        (0..numbers[idx]).filter(|h| h * (numbers[idx] - h) > numbers[idx + number_of_races]).collect::<Vec<usize>>()
    }).map(|wins| wins.len()).reduce(|acc, next| acc * next).unwrap();

    // Fix input kerning by folding the values in on itself as a string and then re-parsing as usize
    let time = (0..number_of_races).fold(String::new(), |acc, next| acc + &numbers[next].to_string()).parse::<usize>().unwrap();
    let distance = (number_of_races..number_of_races * 2).fold(String::new(), |acc, next| acc + &numbers[next].to_string()).parse::<usize>().unwrap();
    
    // Do the same loop just for a single game and collect the winnings
    let answer_part_two = (0..time).filter(|h| h * (time - h) > distance).collect::<Vec<usize>>().len();

    println!("Answer part one: {}", answer_part_one);
    println!("Answer part two: {}", answer_part_two);
   
}
