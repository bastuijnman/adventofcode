use std::{fs::read_to_string, env, collections::HashMap};
use regex::Regex;

///
/// Calculate least common multiple given a vec of numbers
///  
pub fn lcm(nums: Vec<usize>) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums.get(1..).unwrap().to_vec());
    a * b / gcd(a, b)
}

///
/// Calculate greatest common divisor given 2 numbers
/// 
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn main() {
        // Grab first argument (after binary) as file name and read into string
        let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

        let mut lines = contents.lines();

        // Grab all instructions into a vector of chars
        let instructions: Vec<char> = lines.next().unwrap().chars().collect();

        // Skip first newline in input
        lines.next();

        let node_parser = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();

        // Keep track of keys and their targets (left / right)
        let mut nodes: HashMap<String,[String; 2]> = HashMap::new();
        lines.for_each(|line| {

            // Capture numbers
            let c = node_parser.captures(line.clone()).unwrap();

            // Insert the two targets under the key
            nodes.insert(c[1].to_string(), [c[2].to_string(), c[3].to_string()]);
        });
        
        // Find a path based on the start and a given condition
        let find_path = |start: String, condition: fn(String) -> bool| -> usize { 

            // Keep track of steps
            let mut steps = 0;

            // Keep track of current location in loop
            let mut current_location = String::from(start);

            // While we don't match the required condition keep incrementing steps
            while !condition(current_location.clone()) {
                let direction: usize = (instructions[steps % instructions.len()] == 'R') as usize;
                let node = nodes.get(&current_location).unwrap();
                current_location = node[direction].clone();
                steps += 1;
            }
            steps
        };

        // Find paths for each start ending on "A" and map the number of steps into a vector
        let paths: Vec<usize> = nodes
            .iter()
            .filter(|(key, _value)| key.ends_with("A"))
            .map(|(key, _value)| find_path(key.to_string(), |l| l.ends_with("Z")))
            .collect();

        println!("Anser part one: {}", find_path("AAA".to_string(), |l| l == "ZZZ"));
        println!("Answer part two: {}", lcm(paths));

}