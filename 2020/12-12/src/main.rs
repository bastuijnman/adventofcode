use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate lazy_static;

struct Ship {
    direction: String,
    position_tracker: HashMap<String, i32>,
    waypoint_tracker: Vec<i32>
}

impl Ship {

    fn new() -> Ship {
        Ship {
            direction: "E".to_string(),
            position_tracker: HashMap::new(),
            waypoint_tracker: vec![1, 10, 0, 0]
        }
    }

    fn process(&mut self, command: &str, value: i32) {
        match command {
            "F" | "N" | "E" | "S" | "W" => {
                let direction = if command == "F" { &self.direction } else { command };
                let count = self.position_tracker.entry(direction.to_string()).or_insert(0);
                *count += value;
            },
            "R" | "L" => {
                let mut directions = vec!["N", "E", "S", "W"];
                if command == "L" { directions.reverse() };
                self.direction = directions[((directions.iter().position(|&x| x == self.direction).unwrap() + (value as usize / 90)) % 4)].to_string();
            },
            _ => ()
        }
    }

    fn process_for_waypoint(&mut self, command: &str, value: i32) {
        match command {
            "F" => {
                let north = self.waypoint_tracker[0];
                let east = self.waypoint_tracker[1];
                let south = self.waypoint_tracker[2];
                let west = self.waypoint_tracker[3];

                // Can move north
                if north - south > 0 {
                    let count = self.position_tracker.entry("N".to_string()).or_insert(0);
                    *count += value * (north - south);
                }

                // Can move east
                if east - west > 0 {
                    let count = self.position_tracker.entry("E".to_string()).or_insert(0);
                    *count += value * (east - west);
                }

                // Can move south
                if south - north > 0 {
                    let count = self.position_tracker.entry("S".to_string()).or_insert(0);
                    *count += value * (south - north);
                }

                // Can move west
                if west - east > 0 {
                    let count = self.position_tracker.entry("W".to_string()).or_insert(0);
                    *count += value * (west - east);
                }
            },
            "N" | "E" | "S" | "W" => {
                let directions = vec!["N", "E", "S", "W"];
                let position = directions.iter().position(|&x| x == command).unwrap();
                self.waypoint_tracker[position] += value;
            },
            "R" | "L" => {
                if command == "R" {
                    self.waypoint_tracker.rotate_right((value / 90) as usize);
                } else {
                    self.waypoint_tracker.rotate_left((value / 90) as usize)
                }
            },
            _ => ()
        }
    }

    fn get_manhattan_distance(self) -> i32 {
        let north = self.position_tracker.get("N").unwrap_or(&0);
        let south = self.position_tracker.get("S").unwrap_or(&0);
        let east = self.position_tracker.get("E").unwrap_or(&0);
        let west = self.position_tracker.get("W").unwrap_or(&0);
        i32::abs(east - west) + i32::abs(north - south)
    }

}

fn main() -> io::Result<()> {

    // Parse CLI args
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let list: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let instructions: Vec<(&str, i32)> = list.iter().map(|x| {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([A-Z])(\d+)").unwrap();
        }
        let captures = RE.captures(x).unwrap();
        (captures.get(1).unwrap().as_str(), captures.get(2).unwrap().as_str().parse::<i32>().unwrap())
    }).collect();

    let mut ship = Ship::new();
    for (command, value) in instructions.iter() {
        ship.process(command, *value);
    }
    println!("Part 1: {}", ship.get_manhattan_distance());

    let mut ship = Ship::new();
    for (command, value) in instructions.iter() {
        ship.process_for_waypoint(command, *value);
    }
    println!("Part 2: {:?}", ship.get_manhattan_distance());

    Ok(())
}