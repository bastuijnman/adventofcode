use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[macro_use]
extern crate clap;
use clap::App;

fn main() -> io::Result<()> {

    // Parse CLI args
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let preamble = matches.value_of("preamble").unwrap().parse::<usize>().unwrap();
    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let list: Vec<i64> = reader.lines().map(|line| line.unwrap().parse::<i64>().unwrap()).collect();

    // Preamble numbers should not be used as values.
    let values = list.clone().split_off(preamble);
    
    // Find the invalid number
    let invalid = values.iter().find(|&x| {
        let index = values.iter().position(|y| x == y).unwrap();
        let validators = &list[(0 + index)..(preamble + index)];
        let validated: Vec<bool> = validators.iter().map(|v| validators.contains(&(x - v))).collect();
        !validated.contains(&true)
    }).unwrap();

    println!("Found invalid entry {}", invalid);
    match find_contiguous_set(list, *invalid) {
        None => println!("Did not find set for invalid entry, BAD OUTPUT"),
        Some(mut set) => {

            // Sort the set high to low
            set.sort();

            // Encryption weakness = (lowest + highest)
            println!("Encryption weakness found: {}", set.first().unwrap() + set.last().unwrap())
        }
    }

    Ok(())
}

/**
 * Finds a (continuous) list of numbers in the list that together
 * sum to the needle.
 */
fn find_contiguous_set(list: Vec<i64>, needle: i64) -> Option<Vec<i64>> {
    for (i, _item) in list.iter().enumerate() {
        let mut acc: Vec<i64> = vec![];
        for num in list.iter().skip(i) {

            acc.push(*num);

            let sum: i64 = acc.iter().sum();
            if sum == needle && acc.len() > 1 {
                return Some(acc);
            } else if sum > needle {
                break;
            }
        }
    }
    None
}