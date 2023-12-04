use std::{env, fs::read_to_string};

use regex::Regex;

///
/// Split a number sequence into a vec
/// 
fn split_numbers_string(numbers: &str) -> Vec<usize> {
    numbers

        // Split number sequence on space
        .split(' ')

        // We can have leading or trailing spaces both on the string itself as 
        // well as between numbers, make sure that that's filtered out.
        .filter(|s| !s.is_empty())

        // Parse into integer
        .map(|n| n.parse::<usize>().unwrap_or(0))

        // Collect as vec
        .collect()
}

#[derive(Clone)]
struct Card {

    /// ID (number) of the card
    id: usize,

    /// Numbers that would classify a win
    winning_numbers: Vec<usize>,

    /// Numbers scratched off the ticket
    ticket_numbers: Vec<usize>
}

impl Card {

    ///
    /// Instantiate a Card based on a given input line
    /// 
    fn new_from_line(line: &str) -> Card {

        // Capture ID
        let id_regex = Regex::new(r"Card\s+([0-9]+):").unwrap();
        let captures = id_regex.captures(line);

        // Grab ID and parse as int
        let id: usize = captures.unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();

        // Get rid of ID from the input line and split into 2 number sequences
        let numbers_string = id_regex.replace(line, "");
        let numbers: Vec<&str> = numbers_string.split("|").collect();

        Card {
            id: id,

            // Grab winning numbers from first split section
            winning_numbers: split_numbers_string(numbers[0]),

            // Grab scratched numbers from second split section
            ticket_numbers: split_numbers_string(numbers[1])
        }
    }

    ///
    /// Get the point value of a card
    /// 
    fn get_point_value(&self) -> usize {
        let valid: Vec<usize> = self.get_won_cards();

        if valid.len() == 0 {
            return 0;
        }

        valid
            .iter()
            .skip(1) // Ignore the first card, since it's accounted for in the init
            .fold(1, |acc, _| acc * 2)
    }

    ///
    /// Get a list of the card ID's you've won from this card
    /// 
    fn get_won_cards(&self) -> Vec<usize> {
        self.ticket_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .map(|n| *n)
            .collect()
    }
}

fn main() {
    
    // Grab first argument (after binary) as file name and read into string
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

    // Map out all Cards from the input lines
    let cards: Vec<Card> = contents
        .lines()
        .into_iter()
        .map(|s| Card::new_from_line(s))
        .collect();
    
    // Accumulate point values for cards
    let sum_part_one = cards
        .iter()
        .fold(0, |acc, card| acc + card.get_point_value());

    // Clone cards and make sure it's mutable
    let mut second_cards = cards.clone();

    // Retain total number or cards
    let mut total: u32 = 0;
    let mut index = 0;

    // Loop until we have no more cards to check
    // This is very sub-optimal, it's better to count this on the original card instances
    // but can't be arsed right now.
    let sum_part_two = loop {

        // No more value from the cards vec, can return the total value
        if second_cards.get(index).is_none() {
            break total;
        }

        // Get card at current index
        let game = &second_cards[index];

        // Check which cards we have won
        let num_won = game.get_won_cards().len();

        // Loop through the cards we've won and add them to the vec we're using for the loop
        for i in (game.id + 1)..(game.id + 1 + num_won) {
            if second_cards.get(i - 1).is_some() {
                second_cards.append(&mut vec![second_cards[i - 1].clone()]);

                // Count the card we processed/added
                total += 1;
            }
        }
        index += 1;
    } + cards.len() as u32; // combine counted cards + initial amount


    println!("Answer part one: {}", sum_part_one);
    println!("Answer part two: {}", sum_part_two);
}
