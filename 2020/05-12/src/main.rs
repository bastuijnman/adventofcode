use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::Chars;
use std::ops::Range;

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
    
    // Map ID's & sort ascending
    let mut ids: Vec<i32> = list.iter().map(|row| parse_seat_id(row.to_string())).collect();
    ids.sort();

    // Highest ID (part 1) is now the last item
    println!("Highest ID: {}", ids.last().unwrap());

    /*
     * Loop through the available seat ID's, as they're incremental. Our seat should
     * be the only one not in the parsed ID's list.
     */
    let start: i32 = *ids.first().unwrap();
    let end: i32 = *ids.last().unwrap();
    for id in start..end {
        if !ids.contains(&id) {
            println!("My seat ID: {}", id);
        }
    }
    
    Ok(())
}

fn parse_seat_id (row: String) -> i32 {
    let count_row: i32 = reduce_range(0..128, 'B', row.get(0..7).unwrap().chars());
    let count_seat: i32 = reduce_range(0..8, 'R', row.get(7..10).unwrap().chars());
    count_row * 8 + count_seat
}

fn reduce_range(range: Range<i32>, upper_char: char, sequence: Chars) -> i32 {
    let mut reduced: Vec<i32> = range.collect();
    for c in sequence {
        let count = reduced.len() / 2;
        let upper = reduced.split_off(count);

        if c == upper_char {
            reduced = upper;
        }
    }
    *reduced.first().unwrap()
}