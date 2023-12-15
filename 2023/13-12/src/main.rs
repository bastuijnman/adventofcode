use std::{fs::read_to_string, env, cmp};

fn diff_strs(first: &str, second: &str) -> usize {
    let second_chars: Vec<char> = second.chars().collect();
    return first.chars().into_iter().enumerate().filter(|(idx,c)| second_chars[*idx] != *c).count();
}

fn main() {
    let contents: String = read_to_string(env::args().nth(1).unwrap()).unwrap();

    let patterns: Vec<&str> = contents.split("\n\n").collect();

    let mut count = 0;
    'pattern: for (pi, pattern) in patterns.iter().enumerate() {

        let line_len = pattern.find("\n").unwrap();
        let mut s = pattern.replace("\n", "");
        let rows = s.len() / line_len;

        // Fix smudges
        let mut processed_hor = false;
        'outer: for i in 0..rows-1 {
            for j in i+1..rows {
                let first = s.get((i * line_len)..((i + 1) * line_len)).unwrap();
                let second = s.get((j * line_len)..((j + 1) * line_len)).unwrap();

                if diff_strs(first, second) == 1 {
                    s = s.replace(first, second);
                    processed_hor = true;
                    break 'outer;
                }
            }
        }

        if !processed_hor {
            let mut vs: Vec<char> = s.chars().collect();
            'outer: for i in 0..line_len - 1 {
                for j in i+1..line_len {
                    let first: String = (0..rows).map(|r| (r * line_len) + i).map(|i| vs[i]).collect();
                    let second: String = (0..rows).map(|r| (r * line_len) + j).map(|i| vs[i]).collect();
                    if diff_strs(first.as_str(), second.as_str()) == 1 {
                        let sv: Vec<char> = second.chars().collect();
                        (0..rows).enumerate().for_each(|(idx, r)| vs[(r * line_len) + i] = sv[idx]);
                        s = vs.iter().collect();
                        break 'outer;
                    }
                }
            }
        }

        // Horizontal line reflection
        'outer: for i in 0..rows-1 {
            let first = s.get((i * line_len)..((i + 1) * line_len));
            let second = s.get(((i + 1) * line_len)..((i + 2) * line_len));
            if first == second {
                
                let min = cmp::min(i + 1, rows - (i + 1));
                for j in 1..min {
                    let f: Vec<char> = s.get(((i - j) * line_len)..(((i - j) + 1) * line_len)).unwrap().chars().collect();
                    let l: Vec<char> = s.get((((i + j) + 1) * line_len)..(((i + j) + 2) * line_len)).unwrap().chars().collect();

                    let m: Vec<(usize, &char)> = f.iter().enumerate().filter(|(idx, c)| l[*idx] != **c).collect();
                    if m.len() > 0 {
                        continue 'outer;
                    }
                }
                count += (i+1) * 100;
                continue 'pattern;
            }
        }

        let vs: Vec<char> = s.chars().collect();
        'outer: for i in 0..line_len - 1 {
            let first: String = (0..rows).map(|r| (r * line_len) + i).map(|i| vs[i]).collect();
            let second: String = (0..rows).map(|r| (r * line_len) + i + 1 ).map(|i| vs[i]).collect();
            if pi == 1 {
                let f: Vec<char> = (0..rows).map(|r| (r * line_len) + i).map(|i| vs[i]).collect();
                let s: Vec<char> = (0..rows).map(|r| (r * line_len) + i + 1).map(|i| vs[i]).collect();
            }
            if first == second {
                let min = cmp::min(i + 1, line_len - (i + 1));
                for j in 1..min {
                    let f: Vec<char> = (0..rows).map(|r| (r * line_len) + i - j).map(|i| vs[i]).collect();
                    let s: Vec<char> = (0..rows).map(|r| (r * line_len) + i + 1 + j ).map(|i| vs[i]).collect();

                    if f.iter().enumerate().filter(|(idx, c)| s[*idx] != **c).count() > 0 {
                        continue 'outer;
                    }
                }
                count += i+1;
                continue 'pattern;
            }
        }
    }

    println!("Answer part one: {}", count);
}
