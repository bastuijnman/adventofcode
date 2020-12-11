use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use regex::Captures;
use clap::{Arg, App};

struct PasswordEntry {
    value: String,
    validator: char,
    min: usize,
    max: usize
}

fn main() -> io::Result<()> {
    let matches = App::new("AOC - Day 2")
                        .arg(Arg::with_name("company")
                            .short("c")
                            .long("company")
                            .takes_value(true)
                        )
                        .arg(Arg::with_name("INPUT")
                            .required(true)
                            .index(1)
                        )
                        .get_matches();
                    

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);
    let list: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // Parse all passwords in the list
    let regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let parsed: Vec<Captures> = list.iter().map(|item| regex.captures(item).unwrap()).collect();

    // Construct password entry mappings
    let mapped: Vec<PasswordEntry> = parsed.iter().map(|item| PasswordEntry {
        value: String::from(item.get(4).unwrap().as_str()),
        validator: item.get(3).unwrap().as_str().chars().next().unwrap(),
        min: item.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        max: item.get(2).unwrap().as_str().parse::<usize>().unwrap()

    }).collect();
    
    // Check validity count
    let valid_count: usize;
    if matches.value_of("company").unwrap_or("") == "tobbogan" {
        valid_count = mapped.iter().filter(|entry| is_password_valid_toboggan_corporate(entry)).count();
    } else {
        valid_count = mapped.iter().filter(|entry| is_password_valid(entry)).count();
    }
    
    println!("{}", valid_count);

    Ok(())
}

fn is_password_valid(entry: &&PasswordEntry) -> bool {
    let counted: usize = entry.value.chars().filter(|c| c == &entry.validator).count();
    counted >= entry.min && counted <= entry.max
}

fn is_password_valid_toboggan_corporate(entry: &&PasswordEntry) -> bool {
    let value = &entry.value;
    (value.chars().nth(entry.min - 1).unwrap() == entry.validator && value.chars().nth(entry.max - 1).unwrap() != entry.validator) ||
    (value.chars().nth(entry.min - 1).unwrap() != entry.validator && value.chars().nth(entry.max - 1).unwrap() == entry.validator)
}