use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct Basin {
    low_point: (usize, usize),
    locations: HashSet<(usize, usize)>,
}

#[derive(Debug, Clone)]
struct NextPoint {
    depth: (i32, i32),
    point: (usize, usize),
}

const RADIX: u32 = 10;

pub fn main() {
    let file_path = "resources/inputs/day9.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");


    let heights = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .iter()
                .map(|height| height.to_digit(RADIX).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut basins: Vec<Basin> =
        heights
            .clone()
            .iter()
            .enumerate()
            .fold(Vec::new(), |basins, (line_pos, line_heights)| {
                let size = line_heights.len();
                line_heights.clone().iter().enumerate().fold(
                    basins,
                    |mut line_basins, (pos, height)| {
                        let horizontal = {
                            if pos == 0 {
                                height < &line_heights[pos + 1]
                            } else if pos == (size - 1) {
                                height < &line_heights[pos - 1]
                            } else {
                                height < &line_heights[pos - 1] && height < &line_heights[pos + 1]
                            }
                        };

                        let vertical = {
                            if line_pos == 0 {
                                height < &heights[line_pos + 1][pos]
                            } else if line_pos == (heights.len() - 1) {
                                height < &heights[line_pos - 1][pos]
                            } else {
                                height < &heights[line_pos + 1][pos]
                                    && height < &heights[line_pos - 1][pos]
                            }
                        };

                        if vertical && horizontal {
                            let low_point = (line_pos, pos);
                            let mut locations: HashSet<(usize, usize)> = HashSet::new();
                            calculate_basin(low_point, &heights, (0, 0), &mut locations);

                            line_basins.push(Basin {
                                low_point,
                                locations,
                            });
                        }

                        line_basins
                    },
                )
            });


    basins.sort_by(|basin, other_basin| basin.locations.len().cmp(&other_basin.locations.len()));
    basins.reverse();


    println!(
        "{} {} {}",
        basins[0].locations.len(),
        basins[1].locations.len(),
        basins[2].locations.len()
    );

    println!(
        "{:?}",
        basins[0].locations.len() * basins[1].locations.len() * basins[2].locations.len()
    );

    println!("{:?}", basins);
}

fn day_9_part_1() {
    let file_path = "resources/inputs/day9.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let heights = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .iter()
                .map(|height| height.to_digit(RADIX).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let low_points: Vec<u32> = heights.clone().iter().enumerate().fold(
        Vec::new(),
        |low_points, (line_pos, line_heights)| {
            let size = line_heights.len();
            line_heights.clone().iter().enumerate().fold(
                low_points,
                |mut line_low_points, (pos, height)| {
                    let horizontal = {
                        if pos == 0 {
                            height < &line_heights[pos + 1]
                        } else if pos == (size - 1) {
                            height < &line_heights[pos - 1]
                        } else {
                            height < &line_heights[pos - 1] && height < &line_heights[pos + 1]
                        }
                    };

                    let vertical = {
                        if line_pos == 0 {
                            height < &heights[line_pos + 1][pos]
                        } else if line_pos == (heights.len() - 1) {
                            height < &heights[line_pos - 1][pos]
                        } else {
                            height < &heights[line_pos + 1][pos]
                                && height < &heights[line_pos - 1][pos]
                        }
                    };

                    if vertical && horizontal {
                        line_low_points.push(*height)
                    }

                    line_low_points.clone()
                },
            )
        },
    );

    let sum: u32 = low_points
        .iter()
        .map(|low_point| low_point + 1)
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    println!("{:?}", sum);
}

fn calculate_basin(
    current_position: (usize, usize),
    heights: &Vec<Vec<u32>>,
    calculation_depth: (i32, i32),
    basins: &mut HashSet<(usize, usize)>,
) {
    let (line_depth, depth) = calculation_depth;
    let (line_pos, pos) = current_position;
    let (data_height, data_length) = (heights.len(), heights[0].len());
    let current_value = heights[line_pos][pos];


    basins.insert(current_position);
    // get next elements
    // for each of them, check they are higher than current, not edge, not 9
    // get next elements for each item
    // continue

    // get next elements
    let mut next_points: Vec<NextPoint> = Vec::new();

    // check horizontal
    if pos == 0 && depth == 0 {
        if heights[line_pos][pos + 1] != 9 /*&& heights[line_pos][pos + 1] > current_value */ {
            next_points.push(NextPoint {
                depth: (line_depth, depth + 1),
                point: (line_pos, pos + 1),
            });
        }
    } else if pos == (data_length - 1) && depth == 0 {
        if heights[line_pos][pos - 1] != 9 /*&& heights[line_pos][pos - 1] > current_value*/ {
            next_points.push(NextPoint {
                depth: (line_depth, depth - 1),
                point: (line_pos, pos - 1),
            });
        }
    } else if pos != 0 && pos != (data_length - 1) {
        if (depth.is_negative() || depth == 0)
            && heights[line_pos][pos - 1] != 9
        /*&& heights[line_pos][pos - 1] > current_value*/
        {
            next_points.push(NextPoint {
                depth: (line_depth, depth - 1),
                point: (line_pos, pos - 1),
            });
        }
        if (depth.is_positive() || depth == 0)
            && heights[line_pos][pos + 1] != 9
        /*&& heights[line_pos][pos + 1] > current_value*/
        {
            next_points.push(NextPoint {
                depth: (line_depth, depth + 1),
                point: (line_pos, pos + 1),
            });
        }
    }

    // check vertical
    if line_pos == 0 && line_depth == 0 {
        if heights[line_pos + 1][pos] != 9 /*&& heights[line_pos + 1][pos] > current_value*/ {
            next_points.push(NextPoint {
                depth: (line_depth + 1, depth),
                point: (line_pos + 1, pos),
            })
        }
    } else if line_pos == (data_height - 1) && line_depth == 0 {
        if heights[line_pos - 1][pos] != 9 /*&& heights[line_pos - 1][pos] > current_value*/ {
            next_points.push(NextPoint {
                depth: (line_depth - 1, depth),
                point: (line_pos - 1, pos),
            })
        }
    } else if line_pos != 0 && line_pos != (data_height - 1) {
        if (line_depth.is_positive() || line_depth == 0)
            && heights[line_pos + 1][pos] != 9
        /*&& heights[line_pos + 1][pos] > current_value*/
        {
            next_points.push(NextPoint {
                depth: (line_depth + 1, depth),
                point: (line_pos + 1, pos),
            })
        }
        if (line_depth.is_negative() || line_depth == 0)
            && heights[line_pos - 1][pos] != 9
        /*&& heights[line_pos - 1][pos] > current_value*/
        {
            next_points.push(NextPoint {
                depth: (line_depth - 1, depth),
                point: (line_pos - 1, pos),
            })
        }
    }


    for next_point in next_points {
        calculate_basin(
            next_point.point,
            heights,
            next_point.clone().depth,
            basins,
        );
    }
}
