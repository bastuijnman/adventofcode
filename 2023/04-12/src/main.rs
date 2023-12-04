use std::{env, fs::read_to_string, collections::HashMap, cmp};

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

    // Store total number of cards available
    let length = cards.len();

    // Keep a cache of the number of winnings per card
    let winnings_cache: HashMap<usize, usize> = cards.iter().map(|c| (c.id, c.get_won_cards().len())).collect();

    // Keep an incrementing index
    let mut index = 0;

    // Keep track of how many cards we won per card
    let mut incrementer:Vec<usize> = vec![0; cards.len()];

    // Keep track of the cards we've finished processing
    let mut finished_total = 0;

    loop {

        // Check if we still have cards to process at index, use modulo with length
        // to make sure we loop over the total number of cards in the input over and
        // over again 
        if incrementer[index % length] >= index / length {
            finished_total = 0;

            // Get card at current index
            let card = &cards[index % length];
            
            // Grab the number of winnings from cache
            let num_won = winnings_cache[&card.id];

            // Loop through the cards we've won and add them to the respective index in the incrementor
            // we can treat the card id here as the start since it's already 1-based as opposed to
            // the index we keep track of.
            for i in (card.id)..cmp::min(card.id + num_won, length) {
                incrementer[i] += 1;
            }
        } else {

            // No more card to process at index, so mark one as finished.
            finished_total += 1;

            // If we've marked all cards as finished we can break the loop
            if finished_total == length {
                break;
            }
        }

        // Increment index
        index += 1;
    };
    let sum_part_two = incrementer.iter().fold(0, |acc, next| acc + next) + cards.len(); // combine counted cards + initial amount

    println!("Answer part one: {}", sum_part_one);
    println!("Answer part two: {}", sum_part_two);
}
