use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;

#[macro_use]
extern crate clap;
use clap::App;

fn main() -> io::Result<()> {

    // Parse CLI args
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let list: Vec<String> = reader.lines().map(|line| line.unwrap().parse::<String>().unwrap()).collect();
 
    let regex = Regex::new(r"(?P<key>\D+) bags contain (?P<rest>.+)").unwrap();
    let contains_regex = Regex::new(r"(?P<count>\d+) (?P<key>\w+ \w+) bag(?s)").unwrap();

    /*
     * Construct hash map with bag type as key & another map as value:
     * 
     * "bag type": {
     *     "bag type": 3,
     *     "bag type": 8
     * }
     */
    let map: HashMap<&str, HashMap<&str, i32>> = list.iter().map(|item| {
        let matches = regex.captures(item).unwrap();
        let base = matches.name("key").unwrap().as_str();
        let rest = matches.name("rest").unwrap().as_str();
        (base, contains_regex.captures_iter(rest).map(|matches| {

            // Capture bag name & the number of bags into tuple
            (matches.name("key").unwrap().as_str(), matches.name("count").unwrap().as_str().parse::<i32>().unwrap())
        }).collect())
    }).collect();
    
    println!("Part 1: {}", get_unique_bags_inside_out(map.clone(), "shiny gold").len());
    println!("Part 2: {}", get_count_outside_in(map.clone(), "shiny gold"));

    Ok(())
}

/*
 * Gets list of unique bags that can eventually contain at least one
 * bag that you search for.
 */
fn get_unique_bags_inside_out(map: HashMap<&str, HashMap<&str, i32>>, search: &str) -> Vec<String> {
    let mut count: Vec<String> = Vec::new();
    for (key, value) in map.iter() {
        if value.contains_key(search) {
            count = [vec![String::from(*key)], count, get_unique_bags_inside_out(map.clone(), key)].concat();
        }
    }
    count.sort();
    count.dedup();
    count
}

/*
 * Gets the number of bags that will end up in the one you search for.
 */
fn get_count_outside_in(map: HashMap<&str, HashMap<&str, i32>>, search: &str) -> i32 {
    let mut count = 0;
    for (key, value) in map.get(search).unwrap().iter() {
        count += value + (value * get_count_outside_in(map.clone(), key));
    }
    count
}