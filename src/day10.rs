extern crate itertools;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

pub fn main() {
    let file_path = "resources/inputs/day10.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let chunks: Vec<Vec<char>> = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|chunk_line| chunk_line.chars().collect())
        .collect();

    let legal_chunks = HashMap::from([('{', '}'), ('[', ']'), ('(', ')'), ('<', '>')]);

    day_10_part_1(&chunks, &legal_chunks);
    day_10_part_2(&chunks, &legal_chunks);
}

fn day_10_part_2(chunks: &Vec<Vec<char>>, legal_chunks: &HashMap<char, char>) {
    let chunk_scores = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let mut completion_scores =
        chunks
            .clone()
            .iter()
            .fold(Vec::new(), |mut completion_scores, chunk_lines| {
                let mut chunk_stack: Vec<char> = Vec::new();
                let discard: bool = chunk_lines
                    .clone()
                    .iter()
                    .fold_while(false, |discard, chunk| {
                        if legal_chunks.contains_key(chunk) {
                            chunk_stack.push(chunk.clone());
                            Continue(discard)
                        } else {
                            let last_chunk = chunk_stack.last();
                            if last_chunk.is_some()
                                && legal_chunks.get(last_chunk.unwrap()).is_some()
                                && legal_chunks.get(last_chunk.unwrap()).unwrap() == chunk
                            {
                                chunk_stack.pop();
                                Continue(discard)
                            } else {
                                Done(true)
                            }
                        }
                    })
                    .into_inner();

                if !discard {
                    chunk_stack.reverse();
                    let chunk_score: i64 = chunk_stack.clone().iter().fold(0, |sum, chunk| {
                        let total = sum * 5;
                        let score = chunk_scores.get(chunk).unwrap();

                        total + score
                    });
                    completion_scores.push(chunk_score);
                }

                completion_scores
            });

    completion_scores.sort();
    println!(
        "Day 10, part 2: {}",
        completion_scores[completion_scores.len() / 2]
    );
}

fn day_10_part_1(chunks: &Vec<Vec<char>>, legal_chunks: &HashMap<char, char>) {
    let illegal_scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let total_score = chunks.clone().iter().fold(0, |sum, chunk_lines| {
        let mut chunk_stack: Vec<char> = Vec::new();
        let syntax_error_score = chunk_lines
            .clone()
            .iter()
            .fold_while(0, |sum, chunk| {
                if legal_chunks.contains_key(chunk) {
                    chunk_stack.push(chunk.clone());
                    Continue(sum)
                } else {
                    let last_chunk = chunk_stack.last();
                    if last_chunk.is_some()
                        && legal_chunks.get(last_chunk.unwrap()).is_some()
                        && legal_chunks.get(last_chunk.unwrap()).unwrap() == chunk
                    {
                        chunk_stack.pop();
                        Continue(sum)
                    } else {
                        let score = illegal_scores.get(chunk).unwrap();
                        Done(sum + score)
                    }
                }
            })
            .into_inner();

        sum + syntax_error_score
    });

    println!("Day 10, part 1: {}", total_score);
}
