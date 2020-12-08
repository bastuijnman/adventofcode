use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;

#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate lazy_static;

struct Program {
    instructions: Vec<(String, i32)>,
    current_instruction: usize,
    history_stack: Vec<usize>,
    accumulator: i32
}

impl Program {

    fn new(instructions: Vec<(String, i32)>) -> Program {
        Program { instructions, current_instruction: 0, accumulator: 0, history_stack: vec![] }
    }

    fn reset (&mut self) {
        self.current_instruction = 0;
        self.accumulator = 0;
        self.history_stack = vec![];
    }

    fn nop (&mut self) {
        self.current_instruction += 1;
    }

    fn acc(&mut self, value: i32) {
        self.current_instruction += 1;
        self.accumulator += value;
    }

    fn jmp(&mut self, value: i32) {
        let mut c = self.current_instruction as i32;
        c += value;
        self.current_instruction = c as usize;
    }

    fn fix_corruption(&mut self) {
        let instructions = &self.instructions;
        let last_code = *self.history_stack.last().unwrap();
        let (opcode, value) = &instructions[last_code];
        let mut new_opcode = "jmp";
        if opcode == "jmp" {
            new_opcode = "nop";
        }
        self.instructions[last_code] = (new_opcode.to_string(), *value);
    }

    fn run(&mut self) -> i32 {
        let instructions = self.instructions.clone();
        while self.current_instruction != self.instructions.len() - 1 {

            if self.history_stack.contains(&self.current_instruction) {
                println!("Loop detected! Current program value: {}, attempting boot correction", self.accumulator,);

                self.fix_corruption();
                self.reset();

                return self.run();
            }

            let (opcode, value) = &instructions[self.current_instruction];
            let processed_instruction = self.current_instruction;
            match opcode.as_str() {

                "nop" => self.nop(),
                "acc" => self.acc(*value),
                "jmp" => self.jmp(*value),

                _ => {
                    println!("Invalid opcode encountered: {}", opcode);
                    std::process::exit(1);
                }

            }

            self.history_stack.push(processed_instruction);
        }

        self.accumulator
    }
}

fn main() -> io::Result<()> {

    // Parse CLI args
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let list: Vec<String> = reader.lines().map(|line| line.unwrap().parse::<String>().unwrap()).collect();
    let instructions: Vec<(String, i32)> = list.iter().map(|i| parse_instruction(i.to_string())).collect();

    let mut program: Program = Program::new(instructions);
    let result = program.run();

    println!("Program result: {}", result);

    Ok(())
}

fn parse_instruction(instruction: String) -> (String, i32) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<opcode>[a-z]{3}) (?P<arg>[-|+]\d+)").unwrap();
    }
    let caps = RE.captures(&instruction).unwrap();

    (
        caps.name("opcode").unwrap().as_str().to_string(),
        caps.name("arg").unwrap().as_str().parse::<i32>().unwrap()
    )
}