use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::cmp::min;

#[macro_use]
extern crate clap;
use clap::App;

fn main() -> io::Result<()> {

    // Parse CLI args
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let list: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let mut map: Vec<Vec<char>> = list.iter().map(|row| row.chars().collect()).collect();

    loop {

        let mut new_map: Vec<Vec<char>> = map.clone();

        for row in 0..map.len() {
            for col in 0..map[row].len() {

                let seat = map[row][col];
                let adjacent = get_surrounding_seats(&map, row, col);

                if seat == 'L' && adjacent.matches('#').count() == 0 {
                    new_map[row][col] = '#';
                } else if seat == '#' && adjacent.matches('#').count() >= 4 {
                    new_map[row][col] = 'L';
                } else {
                    new_map[row][col] = seat;
                }
            }
        }


        if new_map == map {
            map = new_map;
            break;
        }
        map = new_map;
    }

    println!("Occupied count: {:?}", map.iter().flatten().filter(|c| **c == '#').count());
    
    Ok(())
}

fn get_surrounding_seats(map:&Vec<Vec<char>>, row: usize, col: usize) -> String {
    
    // Need to deal with usize restrictions, negative operations will overflow
    let top = if row == 0 { 0 } else { row - 1 };
    let left = if col == 0 { 0 } else { col - 1 };

    let adjacent: &String = &map[top..=min(map.len() - 1, row + 1)]
        .iter()
        .map(|x| &x[left..=min(x.len() - 1, col + 1)])
        .flatten()
        .collect();


    // Remove occurance of actual wanted seat
    adjacent.replacen(map[row][col], "", 1)
}