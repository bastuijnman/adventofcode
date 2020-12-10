use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::cmp::min;
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

    let mut list: Vec<usize> = reader.lines().map(|line| line.unwrap().parse::<usize>().unwrap()).collect();
    list.sort();

    // Add charging port & device
    let last = list.last().unwrap() + 3;
    list = [vec![0], list].concat();
    list.push(last);
    
    let mut jumps: Vec<usize> = vec![];
    for (index, joltage) in list.iter().enumerate() {
        match list.get(index + 1) {
            Some(value) => jumps.push(value - joltage),
            None => println!("Last reached")
        }
    }

    println!("Part 1: {}", jumps.iter().filter(|x| **x == 1).count() * jumps.iter().filter(|x| **x == 3).count());
    println!("Part 2: {}", calc(&list, 0, last, &mut HashMap::new()));

    Ok(())
}


fn calc(list: &Vec<usize>, from: usize, needle: usize, cache: &mut HashMap<String, i64>) -> i64 {
    let mut paths: i64 = 0;

    let value = list[from];
    let next = list[from + 1..min(from + 4, list.len())].iter().filter(|x| *x - value <= 3);

    for item in next {

        if *item == needle {
            return 1;
        }
        
        match cache.get(&format!("{}-{}", from, item)) {
            Some(result) =>  paths += result,
            None => {
                let result = calc(list, list.iter().position(|v| v == item).unwrap(), needle, cache);
                paths += result;
                cache.insert(format!("{}-{}", from, item), result);
            }
        }
    }

    paths
}