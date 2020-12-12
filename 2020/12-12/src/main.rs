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

    /**
     * Generate a new ship with the standard AOC parameters
     */
    fn new() -> Ship {
        Ship {
            direction: "E".to_string(),
            position_tracker: HashMap::new(),
            waypoint_tracker: vec![1, 10, 0, 0]
        }
    }

    /**
     * Process an instruction directly
     */
    fn process(&mut self, command: &str, value: i32) {
        match command {
            "F" | "N" | "E" | "S" | "W" => {
                let direction = if command == "F" { &self.direction } else { command };
                *self.position_tracker.entry(direction.to_string()).or_insert(0) += value;
            },
            "R" | "L" => {
                let mut directions = vec!["N", "E", "S", "W"];
                if command == "L" { directions.reverse() };
                self.direction = directions[((directions.iter().position(|&x| x == self.direction).unwrap() + (value as usize / 90)) % 4)].to_string();
            },
            _ => ()
        }
    }

    /**
     * Process an instruction using waypoint mechanism
     */
    fn process_for_waypoint(&mut self, command: &str, value: i32) {
        match command {
            "F" => {
                let north_or_south = self.waypoint_tracker[0] - self.waypoint_tracker[2];
                *self.position_tracker.entry(if north_or_south > 0 { "N".to_string() } else { "S".to_string() }).or_insert(0) += value * i32::abs(north_or_south);

                let east_or_west = self.waypoint_tracker[1] - self.waypoint_tracker[3];
                *self.position_tracker.entry(if east_or_west > 0 { "E".to_string() } else { "W".to_string() }).or_insert(0) += value * i32::abs(east_or_west);
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

    /**
     * Get manhattan distance from starting position (always 0,0)
     */
    fn get_manhattan_distance(self) -> i32 {
        i32::abs(self.position_tracker.get("E").unwrap_or(&0) - self.position_tracker.get("W").unwrap_or(&0)) + i32::abs(self.position_tracker.get("N").unwrap_or(&0) - self.position_tracker.get("S").unwrap_or(&0))
    }
}

fn main() -> io::Result<()> {

    // Parse CLI args
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let list: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // Parse instructions list into tuples (COMMAND, VALUE)
    let instructions: Vec<(&str, i32)> = list.iter().map(|x| {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([A-Z])(\d+)").unwrap();
        }
        let captures = RE.captures(x).unwrap();
        (captures.get(1).unwrap().as_str(), captures.get(2).unwrap().as_str().parse::<i32>().unwrap())
    }).collect();

    // Get manhattan distance for processing regular instructions
    let mut ship = Ship::new();
    for (command, value) in instructions.iter() {
        ship.process(command, *value);
    }
    println!("Part 1: {}", ship.get_manhattan_distance());

    // Get manhattan distance for processing instructions with waypoint logic
    let mut ship = Ship::new();
    for (command, value) in instructions.iter() {
        ship.process_for_waypoint(command, *value);
    }
    println!("Part 2: {:?}", ship.get_manhattan_distance());

    Ok(())
}