use std::{env, fs::read_to_string, ops::Range};

use regex::Regex;

fn main() {
    
    // Grab first argument (after binary) as file name and read into string
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

    let numbers = Regex::new(r"([0-9]+)").unwrap();

    let sections: Vec<Vec<i64>> = contents
        .split("\n\n")
        .map(|s| {
            numbers
                .captures_iter(s)
                .map(|c| c.extract())
                .map(|(_, [n])| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let seeds = sections[0].clone();
    let mut res = seeds.clone();

    let categories: Vec<Vec<(Range<i64>, i64)>> = sections.iter().skip(1).map(|category| {
        category.chunks(3).map(|chunk| (chunk[1]..chunk[1]+chunk[2], chunk[0]-chunk[1])).collect()
    }).collect();

    for (idx, _seed) in seeds.iter().enumerate() {
        for category in &categories {
            for chunk in category {
                if chunk.0.contains(&res[idx]) {
                    res[idx] = res[idx] + chunk.1;
                    break;
                }
           }
        }
    }

    let seed_ranges = sections[0].clone();
    let seed_ranges_flat: Vec<i64> = seed_ranges.chunks(2).map(|r|r[0]..r[0]+r[1]).flatten().collect();
    let mut seed_ranges_res = seed_ranges_flat.clone();
    for (idx, _seed) in seed_ranges_flat.iter().enumerate() {
        for category in &categories {
            for chunk in category {
                if chunk.0.contains(&seed_ranges_res[idx]) {
                    seed_ranges_res[idx] = seed_ranges_res[idx] + chunk.1;
                    break;
                }
           }
        }
    }
    
    seed_ranges_res.sort();
    println!("{:?}", res.first());
    println!("{:?}", seed_ranges_res.first());
}
