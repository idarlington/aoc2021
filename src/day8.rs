extern crate itertools;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct Line<'a> {
    patterns: Vec<&'a str>,
    output: Vec<&'a str>,
}

pub(crate) fn main() {
    let file_path = "resources/inputs/day8.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let lines = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .clone()
        .iter()
        .map(|line| {
            let line_contents = line.clone().split("|").collect::<Vec<&str>>();

            let output = line_contents[1]
                .clone()
                .split_whitespace()
                .clone()
                .collect::<Vec<&str>>();
            let patterns = line_contents[0]
                .clone()
                .split_whitespace()
                .clone()
                .collect::<Vec<&str>>();

            Line { patterns, output }
        })
        .collect::<Vec<Line>>();

    day_8_part_1(&lines);
    day_8_part_2(&lines);
}

fn day_8_part_1(lines: &Vec<Line>) {
    let flattened_output = lines
        .clone()
        .iter()
        .map(|line| line.clone().output)
        .flatten()
        .collect::<Vec<&str>>();

    let sum = flattened_output.iter().fold(0, |mut sum, item| {
        if vec![2, 3, 4, 7].contains(&(item).len()) {
            sum += 1
        }
        sum
    });

    println!("Day 8, part 1: {}", sum);
}

fn day_8_part_2(lines: &Vec<Line>) {
    let sum = lines.clone().iter().fold(0, |mut sum, line| {
        let filled_digit_patterns = deduct_digit_patterns(line.clone().patterns, HashMap::new());
        let output_digits =
            line.output
                .iter()
                .fold(String::from(""), |mut output_digits, output| {
                    let output_characters = output.chars().collect::<HashSet<char>>();
                    let key: usize = filled_digit_patterns
                        .iter()
                        .fold_while(None, |_, (character_key, characters)| {
                            if characters.eq(&output_characters) {
                                Done(Some(character_key.clone()))
                            } else {
                                Continue(None)
                            }
                        })
                        .into_inner()
                        .unwrap();
                    output_digits.push_str(&(key.to_string()));
                    output_digits
                });

        sum += output_digits.parse::<i32>().unwrap();

        sum
    });

    println!("Day 8, part 1: {}", sum);
}

fn deduct_digit_patterns(
    line: Vec<&str>,
    digit_patterns: HashMap<usize, HashSet<char>>,
) -> HashMap<usize, HashSet<char>> {
    let mut digit_patterns: HashMap<usize, HashSet<char>> = digit_patterns.clone();
    let mut remainder = line.clone();

    for pattern in line.clone() {
        let characters: HashSet<char> = pattern.chars().collect();
        if pattern.len() == 2 {
            digit_patterns.insert(1, characters.clone());
        }

        if pattern.len() == 4 {
            digit_patterns.insert(4, characters.clone());
        }

        if pattern.len() == 3 {
            digit_patterns.insert(7, characters.clone());
        }

        if pattern.len() == 7 {
            digit_patterns.insert(8, characters.clone());
        }

        if !digit_patterns.contains_key(&9)
            && pattern.len() == 6
            && digit_patterns.contains_key(&4)
            && digit_patterns
                .get(&4)
                .unwrap()
                .is_subset(&characters.clone())
        {
            digit_patterns.insert(9, pattern.chars().collect());
        }

        if !digit_patterns.contains_key(&3)
            && pattern.len() == 5
            && digit_patterns.contains_key(&1)
            && characters
                .clone()
                .is_superset(digit_patterns.get(&1).unwrap())
        {
            digit_patterns.insert(3, characters.clone());
        }

        if !digit_patterns.contains_key(&0)
            && pattern.len() == 6
            && digit_patterns.contains_key(&9)
            && digit_patterns.contains_key(&1)
            && digit_patterns.get(&1).unwrap().is_subset(&characters)
            && digit_patterns.get(&9).unwrap() != &characters
        {
            digit_patterns.insert(0, characters.clone());
        }

        if !digit_patterns.contains_key(&6)
            && pattern.len() == 6
            && digit_patterns.contains_key(&9)
            && digit_patterns.contains_key(&0)
            && digit_patterns.get(&0).unwrap() != &characters
            && digit_patterns.get(&9).unwrap() != &characters
        {
            digit_patterns.insert(6, characters.clone());
        }

        if !digit_patterns.contains_key(&5)
            && pattern.len() == 5
            && digit_patterns.contains_key(&6)
            && digit_patterns
                .get(&6)
                .unwrap()
                .is_superset(&pattern.chars().collect())
            && digit_patterns.get(&6).unwrap() != &characters
        {
            digit_patterns.insert(5, pattern.chars().collect());
        }

        if !digit_patterns.contains_key(&2)
            && pattern.len() == 5
            && digit_patterns.contains_key(&5)
            && digit_patterns.contains_key(&3)
            && digit_patterns.get(&5).unwrap() != &characters
            && digit_patterns.get(&3).unwrap() != &characters
        {
            digit_patterns.insert(2, pattern.chars().collect());
        }
    }

    for (_, characters) in digit_patterns.clone() {
        remainder.retain(|line| line.clone().chars().collect::<HashSet<char>>() != characters)
    }

    if digit_patterns.len() != 10 {
        deduct_digit_patterns(remainder.clone(), digit_patterns.clone())
    } else {
        digit_patterns
    }
}
