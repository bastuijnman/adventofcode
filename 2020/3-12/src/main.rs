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

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let list: Vec<String> = reader.lines().map(|line| line.unwrap().parse::<String>().unwrap()).collect();
    
    println!("Part one: {}", traverse(list.clone(), 3, 1));
    
    println!("part two: {}", vec![
        traverse(list.clone(), 1, 1),
        traverse(list.clone(), 3, 1),
        traverse(list.clone(), 5, 1),
        traverse(list.clone(), 7, 1),
        traverse(list.clone(), 1, 2)
    ].iter().product::<usize>());

    Ok(())
}

fn traverse<'a>(list: Vec<String>, horizontal_increment: usize, vertical_increment: usize) -> usize {

    // Grab with of the first row to know when to wrap around
    let width = list.iter().peekable().peek().unwrap().len();

    // Keep track of trees & position
    let mut trees_counted = 0;
    let mut horizontal_position = 0;
    let mut vertical_position = 0;

    // While we've not reached the bottom of the list
    while vertical_position < list.len() {
        
        let row = &list[vertical_position];

        // In case we're further along than the max horizontal input we wrap around
        if horizontal_position > width - 1 {
            horizontal_position = horizontal_position % width;
        }

        // Check for tree
        if row.chars().nth(horizontal_position).unwrap() == '#' {
            trees_counted += 1;
        }

        // Move right & down
        horizontal_position += horizontal_increment;
        vertical_position += vertical_increment;
    }

    trees_counted
}