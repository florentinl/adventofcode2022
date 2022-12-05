use std::fs;
use std::collections::HashSet;


fn value_of_char(c : char) -> u64 {
    let a = c as u64;
    if 65 <= a && a <= 90 {
        a - 64 + 26
    } else {
        a - 96
    }
}

fn common_letter(a: &str, b: &str) -> char {
    let a_chars = a.chars().collect::<HashSet<char>>();
    let b_chars = b.chars().collect::<HashSet<char>>();
    return a_chars.intersection(&b_chars).next().unwrap().clone();
}

fn common_letter_3(a: &str, b: &str, c: &str) -> char {
    let mut sets: Vec<HashSet<char>> = Vec::new();
    sets.push(a.chars().collect::<HashSet<char>>());
    sets.push(b.chars().collect::<HashSet<char>>());
    sets.push(c.chars().collect::<HashSet<char>>());
    let intersection = sets
        .iter()
        .fold(
            sets[0].clone(), |acc, x| {
                acc.intersection(x).cloned().collect()

    });
    intersection.iter().next().unwrap().clone()
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    let mut value = 0;
    for line in &lines {
        let compartments = line.split_at(line.len() / 2);
        value += value_of_char(common_letter(compartments.0, compartments.1));
    }
    println!("Value: {}", value);

    let mut value = 0;
    for i in 0..lines.len() {
        if i % 3 == 2 {
            value += value_of_char(common_letter_3(lines[i - 2], lines[i - 1], lines[i]));
        }
        i += 1;
    }
    println!("Value: {}", value);
}
