use std::{env, fs::read_to_string};
use regex::Regex;

#[derive(Default, Debug)]
struct Game {
    id: i32,
    red: Vec<i32>,
    green: Vec<i32>,
    blue: Vec<i32>
}

fn find_max(game: &Game) -> (i32, i32, i32) {
    (
        *game.red.iter().max().unwrap(),
        *game.green.iter().max().unwrap(),
        *game.blue.iter().max().unwrap(),
    )
}

fn is_valid_game(game: &&Game) -> bool {
    let values = find_max(game);
    values.0 <= 12 && values.1 <= 13 && values.2 <= 14
}

fn calculate_game_power(game: &Game) -> i32 {
    let values = find_max(game);
    values.0 * values.1 * values.2
}

fn main() {

    // Grab first argument (after binary) as file name and read into string
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();
    let games: Vec<Game> = contents
        .lines()
        .map(|line| {
            let mut game = Game{ ..Default::default() };

            // TODO: These regexes should be combined into one, but I'm a lazy boi
            let game_regex = Regex::new(r"Game ([0-9]+)").unwrap();
            let color_regex = Regex::new(r"(([0-9]+) (blue|red|green))+").unwrap();

            let game_id_caps = game_regex.captures(line).unwrap();
            game.id = game_id_caps.get(1).unwrap().as_str().parse().unwrap(); // My eyes 0.0
            
            for (_, [_full, count, color]) in color_regex.captures_iter(line).map(|c| c.extract()) {
                let count: i32 = count.parse().unwrap();
                match color {
                    "red" => game.red.push(count),
                    "green" => game.green.push(count),
                    "blue" => game.blue.push(count),
                    _ => ()
                };
            }

            game
        })
        .collect();

        let answer_part_one = games
            .iter()
            .filter(is_valid_game)
            .fold(0, |a,n|a+n.id);

        let answer_part_two = games
            .iter()
            .map(calculate_game_power)
            .fold(0, |a,n| a+n);

        println!("Answer part one: {:?}", answer_part_one);
        println!("Answer part two: {:?}", answer_part_two);

}
