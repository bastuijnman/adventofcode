use std::{fs::read_to_string, env, thread::current, collections::HashMap};

fn main() {

    // Grab first argument (after binary) as file name and read into string
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

    let line_len = contents.lines().nth(0).unwrap().trim().len();

    let map:Vec<char> = contents.replace("\n", "").chars().collect();

    let start_position = map.iter().position(|s| *s=='S').unwrap();

    // TODO: dynamically find start position
    let mut current_dir = 'N';
    let mut current_char = '_';
    let mut current_pos = start_position;
    let mut edges: HashMap<usize, [char; 2]> = HashMap::from([(current_pos, [current_char, current_dir])]);
    let mut count = 1;

    while current_char != 'S' {
        (current_char, current_dir, current_pos) = match current_dir {
            'N' => (map[current_pos - line_len], match map[current_pos - line_len] {
                '|' => 'N',
                'F' => 'E',
                '7' => 'W',
                'S' => 'A',
                _ => panic!("UNKOWN DIR")
            }, current_pos - line_len),
            'E' => (map[current_pos + 1], match map[current_pos + 1] {
                '-' => 'E',
                '7' => 'S',
                'J' => 'N',
                'S' => 'A',
                _ => panic!("UNKOWN DIR")
            }, current_pos + 1),
            'S' => (map[current_pos + line_len], match map[current_pos + line_len] {
                '|' => 'S',
                'J' => 'W',
                'L' => 'E',
                'S' => 'A',
                _ => panic!("UNKOWN DIR")
            }, current_pos + line_len),
            'W' => (map[current_pos - 1], match map[current_pos - 1] {
                '-' => 'W',
                'F' => 'S',
                'L' => 'N',
                'S' => 'A',
                _ => panic!("UNKOWN DIR")
            }, current_pos - 1),
            _ => panic!("UNKNOWN DIRECTION")
        };

        edges.insert(current_pos, [current_char, current_dir]);
        count += 1;
    }

    // for each edge in HM
    // Check type, cast ray in "normal" dir until next edge is found
    // push indexes onto vec if not contained yet
    // vec = outer elements
    // result = map - outer_elements - edges
    let mut outer_vec: HashMap<usize, usize> = HashMap::new();
    for (idx, _c) in map.iter().enumerate() {
        let t = ((idx - (idx % line_len))..idx).filter(|i| edges.contains_key(i)).collect::<Vec<usize>>().len() % 2;
        if !edges.contains_key(&idx) {
            outer_vec.insert(idx, t);
        }
    }
    
    // Debug map
    for (idx, c) in map.iter().enumerate() {
        if outer_vec.contains_key(&idx) {
            print!("{}", outer_vec.get(&idx).unwrap());
        } else {
            print!("{}", c);
        }

        if (idx + 1) % line_len == 0 {
            println!("");
        }
    }

    println!("Answer part one: {}", count / 2);
    println!("Answer part two: {}", outer_vec.clone().values().filter(|i| **i == 1).collect::<Vec<&usize>>().len())

}
