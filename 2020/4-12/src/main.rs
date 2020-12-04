use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
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

    let list: Vec<String> = reader.lines().map(|line| line.unwrap().parse::<String>().unwrap()).collect();
    
    let mut entries: Vec<String> = Vec::new();
    let mut entry: String = String::new();
    
    // Multiple lines into a single entry
    for line in list {
        if line == "" {
            entries.push(entry);
            entry = String::new();
        } else {
            entry += &line;
            entry += " ";
        }
    }

    let valid_count = entries.iter().filter(|i| validate(i.to_string(), matches.is_present("validators"))).count();
    println!("Valid entries: {}", valid_count);

    Ok(())
}

fn validate(entry: String, validation_rules: bool) -> bool {

    /*
     * This SUCKS for performance due to compliation of regex in every
     * item of the loop.
     */
    let regex = Regex::new(r"((?P<key>[a-z]+):(?P<value>\S+))").unwrap();

    // Combine values extracted via regex into map
    let mut values: HashMap<String, String> = HashMap::new();
    for caps in regex.captures_iter(&entry) {
        values.insert(caps["key"].to_string(), caps["value"].to_string());
    }

    let mut required: HashMap<String, &dyn Fn(String) -> bool> = HashMap::new();
    required.insert(String::from("byr"), &|value| rule_minmax(value, 1920, 2002));
    required.insert(String::from("iyr"), &|value| rule_minmax(value, 2010, 2020));
    required.insert(String::from("eyr"), &|value| rule_minmax(value, 2020, 2030));
    required.insert(String::from("hgt"), &|value| rule_length(value));
    required.insert(String::from("hcl"), &|value| rule_hair_color(value));
    required.insert(String::from("ecl"), &|value| rule_eye_color(value));
    required.insert(String::from("pid"), &|value| rule_password_id(value));

    for (key, validator) in required {
        if !values.contains_key(&key) {
            return false;
        }

        if validation_rules && !validator(values.get(&key).unwrap().to_string()) {
            return false;
        }
    }

    return true;
}

fn rule_minmax(value: String, min: i32, max: i32) -> bool {
    let validating = value.parse::<i32>().unwrap();
    validating >= min && validating <= max
}

fn rule_length(value: String) -> bool {
    let regex = Regex::new(r"(?P<num>\d+)(?P<unit>in|cm)").unwrap();
    if regex.is_match(&value) {
        let captures = regex.captures(&value).unwrap();

        let unit: String = captures.name("unit").unwrap().as_str().to_string();
        let number: i32 = captures.name("num").unwrap().as_str().parse().unwrap();

        if unit == "in" && number >= 59 && number <= 76 { return true; }
        if unit == "cm" && number >= 150 && number <= 193 { return true; }
    }

    false
}

fn rule_hair_color(value: String) -> bool {
    Regex::new(r"#(\d|\w){6}").unwrap().is_match(&value)
}

fn rule_eye_color(value: String) -> bool {
    let allowed = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    allowed.contains(&value.as_str())
}

fn rule_password_id(value: String) -> bool {
    Regex::new(r"^[0-9]{9}$").unwrap().is_match(&value)
}