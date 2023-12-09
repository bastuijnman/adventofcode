use std::{fs::read_to_string, env};

///
/// Calculate the next value of a history vec of numbers
/// 
fn calculate_next_value(numbers: Vec<i32>) -> i32 {

    // Make sure we can modify our sequence in the loop later
    let mut sequence = numbers.clone();

    // Keep a list of the last numbers for all sequences we're processing
    let mut last_numbers = vec![*sequence.last().unwrap()];

    loop {

        // Breka out of the loop if all sequence numbers are the same
        if sequence.iter().filter(|n| **n != sequence[0]).count() == 0 {
            break;
        }

        // Grab the current length of the sequence
        let length = sequence.len();

        // Loop through all values - 1 in order to be able to find the
        // difference
        for i in 0..length - 1 {

            // Calculate difference and push into sequence
            let value = sequence[i + 1] - sequence[i];
            sequence.push(value);

            // If we've reached the end of the sequence (0-based index + 2 to 
            // correlate with the original length) we push it into our vector
            // containing all last numbers of sequences.
            if i + 2 == length {
                last_numbers.push(value);
            }
        }

        // Make sure the sequence is only the part we just processed
        sequence = sequence.get(length..).unwrap().to_vec();
    };

    // Sum all the last numbers for the result.
    last_numbers.iter().sum()

}

fn main() {
    
    // Grab first argument (after binary) as file name and read into string
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

    // Map the lines to a list of numbers and calculate their next value
    let answer_part_one: i32 = contents
        .lines()
        .map(|s| s.split(' ').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|numbers| calculate_next_value(numbers))
        .sum();

    // Map the lines to a list of numbers and calculate their next value, this one reverses the input line.
    let answer_part_two: i32 = contents
        .lines()
        .map(|s| s.split(' ').rev().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|numbers| calculate_next_value(numbers))
        .sum();

    println!("Answer part one: {}", answer_part_one);
    println!("Answer part two: {}", answer_part_two);
}
