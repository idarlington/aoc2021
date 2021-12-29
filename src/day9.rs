use std::collections::{HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct Basin {
    low_point: (usize, usize),
    locations: HashSet<(usize, usize)>,
}

#[derive(Debug, Clone)]
struct NextPoint {
    point: (usize, usize),
}

const RADIX: u32 = 10;

pub fn main() {
    let file_path = "resources/inputs/day9.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let heights: Vec<Vec<u32>> = contents
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

    let basins: Vec<Basin> =
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
                            line_basins.push(Basin {
                                low_point: (line_pos, pos),
                                locations: HashSet::new(),
                            });
                        }

                        line_basins
                    },
                )
            });

    day_9_part_1(&basins, &heights);
    day_9_part_2(&basins, &heights)
}

fn day_9_part_2(basins: &Vec<Basin>, heights: &Vec<Vec<u32>>) {
    let mut basins = basins.clone();
    let mut heights = heights.clone();
    for (basin_pos, basin) in basins.clone().iter().enumerate() {
        let mut locations: HashSet<(usize, usize)> = HashSet::new();
        calculate_basin(basin.low_point, &mut heights, &mut locations);
        basins[basin_pos] = Basin {
            locations,
            ..*basin
        }
    }

    basins.sort_by(|basin, other_basin| basin.locations.len().cmp(&other_basin.locations.len()));
    basins.reverse();
    println!(
        "Day 9, part 2: {}",
        basins[0].locations.len() * basins[1].locations.len() * basins[2].locations.len()
    );
}

fn day_9_part_1(basins: &Vec<Basin>, heights: &Vec<Vec<u32>>) {
    let sum: u32 = basins
        .iter()
        .map(|basin| {
            let (line_pos, pos) = basin.low_point;
            heights[line_pos][pos] + 1
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    println!("Day 9, part 1: {}", sum);
}

fn calculate_basin(
    current_position: (usize, usize),
    heights: &mut Vec<Vec<u32>>,
    locations: &mut HashSet<(usize, usize)>,
) {
    let (line_pos, pos) = current_position;
    let (data_height, data_length) = (heights.len(), heights[0].len());

    locations.insert(current_position);
    heights[current_position.0][current_position.1] = 9;

    // get next elements
    let mut next_points: Vec<NextPoint> = Vec::new();

    // check horizontal
    if pos == 0 {
        if heights[line_pos][pos + 1] != 9 {
            next_points.push(NextPoint {
                point: (line_pos, pos + 1),
            });
        }
    }

    if pos == (data_length - 1) {
        if heights[line_pos][pos - 1] != 9 {
            next_points.push(NextPoint {
                point: (line_pos, pos - 1),
            });
        }
    }

    if pos != 0 && pos != (data_length - 1) {
        if heights[line_pos][pos - 1] != 9 {
            next_points.push(NextPoint {
                point: (line_pos, pos - 1),
            });
        }
        if heights[line_pos][pos + 1] != 9 {
            next_points.push(NextPoint {
                point: (line_pos, pos + 1),
            });
        }
    }

    // check vertical
    if line_pos == 0 {
        if heights[line_pos + 1][pos] != 9 {
            next_points.push(NextPoint {
                point: (line_pos + 1, pos),
            })
        }
    }

    if line_pos == (data_height - 1) {
        if heights[line_pos - 1][pos] != 9 {
            next_points.push(NextPoint {
                point: (line_pos - 1, pos),
            })
        }
    }

    if line_pos != 0 && line_pos != (data_height - 1) {
        if heights[line_pos + 1][pos] != 9 {
            next_points.push(NextPoint {
                point: (line_pos + 1, pos),
            })
        }
        if heights[line_pos - 1][pos] != 9 {
            next_points.push(NextPoint {
                point: (line_pos - 1, pos),
            })
        }
    }

    for next_point in next_points {
        calculate_basin(next_point.point, heights, locations);
    }
}
