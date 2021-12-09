extern crate itertools;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::fs;

#[derive(Debug, Clone)]
struct Board {
    rows: Vec<Vec<i32>>,
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows
    }
}

pub fn main() {
    let file_path = "resources/inputs/day4.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let content_value: Vec<&str> = contents.split("\n\n").collect();

    let marked_numbers: Vec<i32> = content_value[0]
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|number| number.parse().unwrap())
        .collect();

    let mut boards = content_value.clone();
    boards.remove(0);

    let converted_boards: Vec<Board> =
        boards
            .into_iter()
            .fold(Vec::new(), |mut converted_boards, board| {
                let mut converted_board = Board { rows: Vec::new() };
                let split_board: Vec<&str> = board.split('\n').collect();

                for line in split_board.clone() {
                    let rows: Vec<i32> = line
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|x| x.parse().unwrap())
                        .collect();
                    converted_board.rows.push(rows)
                }

                converted_boards.push(converted_board);
                converted_boards
            });

    day_4_part_1(converted_boards.clone(), marked_numbers.clone());
    day_4_part_2(converted_boards.clone(), marked_numbers.clone());
}

fn day_4_part_1(converted_boards: Vec<Board>, marked_numbers: Vec<i32>) {
    let mut selected_marked_numbers: Vec<i32> = Vec::new();

    let winning_board: Option<Board> = marked_numbers
        .iter()
        .fold_while(None, |winning_board, marked_number| {
            selected_marked_numbers.push(marked_number.clone());

            converted_boards.clone().iter().fold_while(
                winning_board,
                |optional_winning_board, board| {
                    let win = calculate_win(board.clone(), selected_marked_numbers.clone());
                    if win {
                        Done(Some(board.clone()))
                    } else {
                        Continue(optional_winning_board)
                    }
                },
            )
        })
        .into_inner();

    let flattened_board = winning_board
        .clone()
        .unwrap()
        .rows
        .into_iter()
        .flatten()
        .collect::<Vec<i32>>();

    let mut unmarked_numbers = flattened_board.clone();
    unmarked_numbers.retain(|number| !selected_marked_numbers.contains(number));
    let sum_of_unmarked = unmarked_numbers.iter().sum::<i32>();
    let score = sum_of_unmarked * selected_marked_numbers.last().unwrap();

    println!("Day 4, part 1: {}", score);
}

fn day_4_part_2(converted_boards: Vec<Board>, marked_numbers: Vec<i32>) {
    let mut selected_marked_numbers: Vec<i32> = Vec::new();
    let mut boards_to_check = converted_boards.clone();

    let winning_boards: Vec<(Board, Vec<i32>)> = marked_numbers
        .iter()
        .fold_while(
            Vec::new(),
            |winning_boards: Vec<(Board, Vec<i32>)>, marked_number| {
                selected_marked_numbers.push(marked_number.clone());

                let current_winning_boards: Vec<Board> = winning_boards
                    .clone()
                    .iter()
                    .map(|winning_board| winning_board.0.clone())
                    .collect();

                boards_to_check.retain(|board| !current_winning_boards.contains(board));

                boards_to_check.clone().iter().fold_while(
                    winning_boards.clone(),
                    |mut mark_winning_boards, board| {
                        let win = calculate_win(board.clone(), selected_marked_numbers.clone());

                        if win {
                            mark_winning_boards
                                .push((board.clone(), selected_marked_numbers.clone()));
                        }

                        if mark_winning_boards.len() == converted_boards.len() {
                            Done(mark_winning_boards)
                        } else {
                            Continue(mark_winning_boards)
                        }
                    },
                )
            },
        )
        .into_inner();

    let (winning_board, marked_numbers_winning_board) = winning_boards.last().unwrap();

    let flattened_board = winning_board
        .rows
        .clone()
        .into_iter()
        .flatten()
        .collect::<Vec<i32>>();

    let mut unmarked_numbers = flattened_board.clone();
    unmarked_numbers.retain(|number| !marked_numbers_winning_board.contains(number));
    let sum_of_unmarked = unmarked_numbers.iter().sum::<i32>();
    let score = sum_of_unmarked * marked_numbers_winning_board.last().unwrap();

    println!("Day 4, part 2: {:?}", score);
}

fn calculate_win(board: Board, selected_marks: Vec<i32>) -> bool {
    calculate_win_horizontal(board.clone(), selected_marks.clone())
        || calculate_win_vertical(board, selected_marks.clone())
}

fn calculate_win_horizontal(board: Board, selected_marks: Vec<i32>) -> bool {
    board
        .rows
        .iter()
        .fold_while(false, |win, row| {
            if win == true {
                Done(win)
            } else {
                Continue({
                    row.clone()
                        .iter()
                        .all(|number| selected_marks.contains(number))
                })
            }
        })
        .into_inner()
}

fn calculate_win_vertical(board: Board, selected_marks: Vec<i32>) -> bool {
    let board_column_size = board.rows[0].len();

    (0..board_column_size)
        .into_iter()
        .fold_while(false, |win, column_index| {
            if win == true {
                Done(win)
            } else {
                Continue({
                    let mut column_values: Vec<i32> = Vec::new();

                    for row in board.rows.iter() {
                        column_values.push(row[column_index])
                    }

                    column_values
                        .iter()
                        .all(|number| selected_marks.contains(number))
                })
            }
        })
        .into_inner()
}
