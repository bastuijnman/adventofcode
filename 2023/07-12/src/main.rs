use std::{fs::read_to_string, env, collections::HashMap, cmp::Ordering};

///
/// Get the "value" of a hand. Return it as a string
/// so we can easily concatenate the "high value" on
/// it for easy sorting.
/// 
fn get_hand_value(map: HashMap<char, usize>) -> String {

    // Grab all values from the map we use for counts
    let mut vals: Vec<&usize> = map.values().collect();

    // Sort on cards with the most values
    vals.sort();
    
    match vals.as_slice() {
        [1, 1, 1, 1, 1] => String::from("1"), // High card
        [1, 1, 1, 2] => String::from("2"), // One pair
        [1, 2, 2] => String::from("3"), // Two pairs
        [1, 1, 3] => String::from("4"), // Three of a kind
        [2, 3] => String::from("5"), // Full House
        [1, 4] => String::from("6"), // Four of a kind
        [5] => String::from("7"), // Five of a kind
        _ => panic!("Unknown card combination!")
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<char>,
    bid: usize
}

impl Hand {

    fn get_hand_score(&self, process_jokers: bool) -> usize {

        // Define the value of the cards in strings, this is helpful later 
        // on when comparing equal hand values and checking the highest 
        // card values.
        //
        // TODO: Theoretically this could be defined as a static value using an external crate
        let card_values: HashMap<char,&str> = if process_jokers {
            HashMap::from([('J',"01"),('2',"02"),('3',"03"),('4',"04"),('5',"05"),('6',"06"),('7',"07"),('8',"08"),('9',"09"),('T',"10"),('Q',"11"),('K',"12"),('A',"13")])
        } else {
            HashMap::from([('2',"01"),('3',"02"),('4',"03"),('5',"04"),('6',"05"),('7',"06"),('8',"07"),('9',"08"),('T',"09"),('J',"10"),('Q',"11"),('K',"12"),('A',"13")])
        };

        // Setup a hashmap to keep track of number of cards in a hand 
        let mut cards_map: HashMap<char, usize> = HashMap::new();

        // Keep track of the "highest" value in case the hands are equal
        let mut high_value = String::new();

        // Keep track of the number of jokers.
        let jokers = self.cards.iter().filter(|c| **c == 'J').count();

        // Loop over each card in the hand
        for card in self.cards.iter() {

            // Track the cards and how often they occur. Don't track jokers when
            // we're in joker processing mode.
            if process_jokers == false || *card != 'J' {

                // Check if we've already counted the card somewhere else in the 
                // hand, if not set to one otherwise increment.
                let count = cards_map.get(&card);
                let insert = match count {
                    Some(value) => value + 1,
                    None => 1
                };
                cards_map.insert(*card, insert);
            }

            // Add the card value to the "high" value (still as a string.)
            high_value += card_values.get(card).unwrap();
        }

        if process_jokers {

            // If we process jokers we want to clone the cards map and figure
            // out the card we have the most at the moment.
            let cards_map_clone = cards_map.clone();
            let cards_max_value = cards_map_clone.iter().max_by_key(|entry | entry.1);
            
            // If we have a max value add the jokers to it. If we do not then
            // we don't have any entries and likely just 5 jokers. So in that
            // case we just insert 5 aces.
            if cards_max_value.is_some() {
                let (card, value) = cards_max_value.unwrap();
                cards_map.insert(*card, value + jokers);
            } else {
                cards_map.insert('A', 5);
            }
        }

        let hand_value = get_hand_value(cards_map);
        (hand_value.to_owned() + &high_value).parse::<usize>().unwrap()
    }
}

///
/// Folds a hand according to the puzzle rules
/// 
fn fold_hand_values(acc: usize, (idx, next): (usize, &Hand)) -> usize {
    acc + (next.bid * (idx + 1))
}

///
/// Sorting function for hand sorting without joker processing
/// 
fn sort_hands_by_score(a: &Hand, b: &Hand) -> Ordering {
    a.get_hand_score(false).partial_cmp(&b.get_hand_score(false)).unwrap()
}

///
/// Sorting function for hand sorting with joker processing
/// 
fn sort_hands_by_score_with_processed_jokers(a: &Hand, b: &Hand) -> Ordering {
    a.get_hand_score(true).partial_cmp(&b.get_hand_score(true)).unwrap()
}

fn main() {

    // Grab first argument (after binary) as file name and read into string
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

    // Map lines into hands
    let hands: Vec<Hand> = contents.lines().map(|line| {
        let contents: Vec<&str> = line.split(' ').collect();
        Hand {

            // Collect all cards as a char vector
            cards: contents[0].chars().collect(),

            // Parse the bid value
            bid: contents[1].parse().unwrap()
        }
    }).collect();

    // Clone hand vector & sort by their hand scores
    let mut hands_part_one = hands.clone();
    hands_part_one.sort_by(sort_hands_by_score);
    let answer_part_one = hands_part_one.iter().enumerate().fold(0, fold_hand_values);

    // Clone hand vector & sort by their hand scores
    let mut hands_part_two = hands.clone();
    hands_part_two.sort_by(sort_hands_by_score_with_processed_jokers);
    let answer_part_two = hands_part_two.iter().enumerate().fold(0, fold_hand_values);

    println!("Answer part one: {}", answer_part_one);
    println!("Answer part two: {}", answer_part_two);
}
