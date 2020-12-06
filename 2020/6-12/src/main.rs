use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[macro_use]
extern crate clap;
use clap::App;

struct Group {
    chars: String,
    number: usize
}

fn main() -> io::Result<()> {

    // Parse CLI args
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let list: Vec<String> = reader.lines().map(|line| line.unwrap().parse::<String>().unwrap()).collect();
    
    let mut entries: Vec<Group> = Vec::new();
    let mut entry: Group = Group {
        chars: String::new(),
        number: 0
    };
    
    // Multiple lines into a single entry
    for line in list {
        if line == "" {
            entries.push(entry);

            entry = Group {
                chars: String::new(),
                number: 0
            }
        } else {
            entry.number += 1;
            entry.chars += &line;
        }
    }
    // push last entry as rust ignores last newline
    entries.push(entry);

    let count: usize = entries.iter().map(|entry| {
        let mut chars: Vec<char> = entry.chars.chars().collect();
        chars.sort();
        chars.dedup();
        chars.len()
    }).sum();

    // Part 1
    println!("Part 1: {}", count);

    let count: usize = entries.iter().map(|entry| {
        let chars: Vec<char> = entry.chars.chars().collect();
        let mut valid: Vec<&char> = chars.iter().filter(|c| chars.iter().filter(|cc| cc == c).count() == entry.number).collect();

        valid.sort();
        valid.dedup();
        valid.len()
    }).sum();

    println!("Part 2: {}", count);

    Ok(())
}