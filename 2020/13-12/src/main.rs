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
    let mut list = reader.lines().map(|line| line.unwrap());

    let earliest_estimate: usize = list
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let busses: Vec<usize> = list
        .next()
        .unwrap()
        .split(',')
        .map(|b| b.parse::<usize>().unwrap_or(0))
        .collect();

    match find_closest_bus(busses.clone(), earliest_estimate) {
        Some((time, bus)) => println!("Part 1: {}", (time - earliest_estimate) * bus),
        None => ()
    }

    match find_time_subsequent_departures(busses.clone()) {
        Some(time) => println!("Part 2: {}", time),
        None => ()
    }

    Ok(())
}

fn find_closest_bus(busses: Vec<usize>, time: usize) -> Option<(usize, usize)> {
    let mut current_time = time;
    loop {
        for bus in &busses {
            if *bus == 0 {
                continue;
            }

            if current_time % bus == 0 {
                return Some((current_time, *bus));
            }
        }
        current_time += 1;
    }
}

fn find_time_subsequent_departures(busses: Vec<usize>) -> Option<usize> {

    let mut time = 0;
    
    let size = busses.len();
    let first = busses.first().unwrap();
    let last = busses.last().unwrap();

    let mut biggest_busses = busses.clone();
    biggest_busses.sort();
    let biggest = biggest_busses.last().unwrap();
    let biggest_index = busses.iter().position(|x| x == biggest).unwrap();

    'outer: loop {
        time += biggest;

        if (time + ((size - 1) - biggest_index)) % last == 0  && (time - biggest_index) % first == 0 {

            for i in 1..size {

                if busses[i] == 0 {
                    continue;
                }

                let check = time as i64 + (i as i64 - biggest_index as i64);
                if check % busses[i] as i64 != 0 {
                    continue 'outer;
                }
            }

            return Some(time);
        }
    }
}