use regex::Regex;

fn line(record: String, broken: Vec<usize>) -> usize {
    if broken.len() == 0 {
        if !record.contains("#") {
            return 1;
        } else {
            return 0;
        }
    } else if record.len() == 0 {
        return 0;
    }

    let sum: usize = broken.iter().sum();
    if record.len() < sum + broken.len() - 1 {
        return 0;
    }

    let c = record.chars().nth(0).unwrap();
    if c == '?' {
        return dot(record.clone(), broken.clone()) + hash(record.clone(), broken.clone());
    } else {
        if c == '.' {
            return dot(record, broken);
        } else {
            return hash(record, broken);
        }
    }
}

fn dot(record: String, broken: Vec<usize>) -> usize {
    line(record.get(1..).unwrap().to_string(), broken)
}

fn hash(record: String, broken: Vec<usize>) -> usize {
    let entry = broken[0];
    if !record.get(0..entry).unwrap_or(".").contains('.') && record.get(entry..entry).unwrap_or("#") != "#" {
        let (_left, right) = broken.split_at(1);
        return line(record.get(entry+1..).unwrap_or("").to_string(), right.to_vec());
    } else {
        return 0;
    }
}

fn main() {
    println!("Answer part one: {}", 10);
}

#[test]
fn it_calculates_correctly() {
    assert_eq!(1, line(String::from("???.###"), vec![1,1,3]));
    assert_eq!(4, line(String::from(".??..??...?##."), vec![1,1,3]));
    //assert_eq!(1, line(String::from("?#?#?#?#?#?#?#?"), vec![1,3,1,6]));
    assert_eq!(1, line(String::from("????.#...#..."), vec![4,1,1]));
    //assert_eq!(5, line(String::from("????.######..#####."), vec![1,6,5]));
    assert_eq!(10, line(String::from("?###????????"), vec![3,2,1]));
}