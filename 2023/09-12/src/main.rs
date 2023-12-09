use std::{fs::read_to_string, env};

fn calculate_next_value(numbers: Vec<i32>) -> i32 {
    let mut sequence = numbers.clone();
    let mut last_numbers = vec![*sequence.last().unwrap()];

    loop {
        let mut check = sequence.clone();
        check.dedup();

        if check.len() == 1 {
            break;
        }

        let l = sequence.clone();
        sequence.clear();
        for i in 0..l.len() - 1 {
            let val = l[i + 1] - l[i];
            sequence.push(val);

            if i + 2 == l.len() {
                last_numbers.push(val);
            }
        }
    };

    last_numbers.iter().sum()

}

fn main() {
    
    // Grab first argument (after binary) as file name and read into string
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

    let answer_part_one: i32 = contents
        .lines()
        .map(|s| s.split(' ').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|numbers| calculate_next_value(numbers))
        .sum();

    let answer_part_two: i32 = contents
        .lines()
        .map(|s| s.split(' ').rev().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|numbers| calculate_next_value(numbers))
        .sum();

    println!("Answer part one: {}", answer_part_one);
    println!("Answer part two: {}", answer_part_two);
}
