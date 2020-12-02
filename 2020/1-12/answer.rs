use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::env;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let find_cummulative = args[2].parse::<i32>().unwrap();
    let find_by_count = args[3].parse::<usize>().unwrap();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let list: Vec<i32> = reader.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect();
    let total = list.iter().count();
    
    // Vector to hold actual cummulative values
    let mut values = Vec::new();
    values.resize(find_by_count, 0);
    
    // Cols to iterate on
    let mut cols = Vec::new();
    for _i in 0..find_by_count {
        cols.push(0);
    }

    // Count cummulative values
    let mut cummulative = 0;
    while cummulative != find_cummulative {

        // Reset for each loop
        cummulative = 0;
        
        // Find cummulative for each active col
        for col in 0..find_by_count {
            cummulative += list[cols[col]];
            values[col] = list[cols[col]];
        }

        // Increment per col backwards
        for i in (1..find_by_count).rev() {
            if i == find_by_count - 1 {
                cols[i] = cols[i] + 1;
            }

            if cols[i] > total - 1 {
                cols[i] = 0;
                cols[i - 1] = cols[i - 1] + 1;
            }
        }

    }

    println!("{}", values.iter().product::<i32>());

    Ok(())
}