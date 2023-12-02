use std::{env, fs::read_to_string};
use regex::Regex;

const GAME_MAX_RED_VALUE: i32 = 12;
const GAME_MAX_GREEN_VALUE: i32 = 13;
const GAME_MAX_BLUE_VALUE: i32 = 14;

/// Game object, contains it's ID and all of the colors that a game
/// can support each round. They are stored as a Vector with a number
/// assigned for each round.
#[derive(Default, Debug)]
struct Game {
    id: i32,
    red: Vec<i32>, // Holds all red cube counts
    green: Vec<i32>, // Holds all green cube counts
    blue: Vec<i32> // Holds all blue cube counts
}

/// Contains a set of colors and their corresponding count, makes
/// the code a little bit easier to read rather than straight up 
/// returning a tuple.
struct ColorSet {
    red: i32,
    green: i32,
    blue: i32
}

/// Finds the max number of cubes used per color for a game
/// Returns a ColorSet for better readability when needing to 
/// use the values.
fn find_max(game: &Game) -> ColorSet {
    ColorSet {
        red: *game.red.iter().max().unwrap(),
        green: *game.green.iter().max().unwrap(),
        blue: *game.blue.iter().max().unwrap(),
    }
}

/// Checks if a game is valid according to the rules of part one of the
/// puzzle. 
fn is_valid_game(game: &&Game) -> bool {
    let values: ColorSet = find_max(game);
    values.red <= GAME_MAX_RED_VALUE && values.green <= GAME_MAX_GREEN_VALUE && values.blue <= GAME_MAX_BLUE_VALUE
}

/// Calculate the "power" of the games max color values according to the
/// rules of part two of the puzzle.
fn calculate_game_power(game: &Game) -> i32 {
    let values: ColorSet = find_max(game);
    values.red * values.green * values.blue
}

fn main() {

    // Grab first argument (after binary) as file name and read into string
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

    // Create regexes to use on each line, this can probably be combined in some 
    // way, but I'm not a good enough of a regex expert to match that, so KISS
    let game_regex = Regex::new(r"Game ([0-9]+)").unwrap();
    let color_regex = Regex::new(r"(([0-9]+) (blue|red|green))+").unwrap();

    // Map out all lines into Game instances
    let games: Vec<Game> = contents
        .lines()
        .map(|line| {

            // Create a Game instance and just assign it's defaults
            let mut game = Game{ ..Default::default() };

            // Parse the game ID and assign if captured
            if let Some(game_id_caps) = game_regex.captures(line) {
                game.id = game_id_caps[1].parse().unwrap();
            }
            
            // Capture all color patterns and map out into their color and count
            for (_, [_full, count, color]) in color_regex.captures_iter(line).map(|c| c.extract()) {
                
                // Parse count as an integer
                let count: i32 = count.parse().unwrap_or_default();
                
                // Match supported colors and push their count onto their respective vectors.
                // Ignore anything that's not supported
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

        // Check which games are valid and add all the game ID's together
        let answer_part_one = games
            .iter()
            .filter(is_valid_game)
            .fold(0, |a,n|a+n.id);

        // Calculate the power of all cubes together
        let answer_part_two = games
            .iter()
            .map(calculate_game_power)
            .fold(0, |a,n| a+n);

        // Display puzzle answers
        println!("Answer part one: {:?}", answer_part_one);
        println!("Answer part two: {:?}", answer_part_two);

}
