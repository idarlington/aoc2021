use std::fs;

#[derive(Debug, Clone)]
struct VentLine {
    first: (i32, i32),
    second: (i32, i32),
}

pub fn main() {
    let file_path = "resources/inputs/day5.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let content_value: Vec<&str> = contents.split("\n").collect();

    let vent_lines: Vec<VentLine> =
        content_value
            .clone()
            .iter()
            .fold(Vec::new(), |mut vent_lines, line| {
                let split_lines: Vec<&str> = line.split("->").collect();

                let mut first_split_line_string = split_lines[0]
                    .split(|character: char| character.is_whitespace() || character == ',')
                    .collect::<Vec<&str>>();

                let mut second_split_line_string = split_lines[1]
                    .split(|character: char| character.is_whitespace() || character == ',')
                    .collect::<Vec<&str>>();

                first_split_line_string.retain(|&position| position != "");
                second_split_line_string.retain(|&position| position != "");

                let first_positions: Vec<i32> = first_split_line_string
                    .clone()
                    .iter()
                    .map(|position| position.parse().unwrap())
                    .collect();
                let second_positions: Vec<i32> = second_split_line_string
                    .clone()
                    .iter()
                    .map(|position| position.parse().unwrap())
                    .collect();

                let vent_line = VentLine {
                    first: (first_positions[0], first_positions[1]),
                    second: (second_positions[0], second_positions[1]),
                };

                vent_lines.push(vent_line);

                vent_lines
            });

    day_5_part_1(&vent_lines);
    day_5_part_2(&vent_lines);
}

fn day_5_part_1(vent_lines: &Vec<VentLine>) {
    let travelling = calculate_travelling(&vent_lines, false);
    let overlap_count = calculate_overlap_count(&travelling);

    println!("Day 5, part 1: {}", overlap_count);
}

fn day_5_part_2(vent_lines: &Vec<VentLine>) {
    let travelling = calculate_travelling(&vent_lines, true);
    let overlap_count = calculate_overlap_count(&travelling);

    println!("Day 5, part 2: {}", overlap_count);
}

fn define_bounds(vent_lines: &Vec<VentLine>) -> (i32, i32) {
    vent_lines.iter().fold((0, 0), |(mut x, mut y), vent_line| {
        if vent_line.first.0 > x {
            x = vent_line.first.0
        }

        if vent_line.first.1 > y {
            y = vent_line.first.1
        }

        if vent_line.second.0 > x {
            x = vent_line.second.0
        }

        if vent_line.second.1 > y {
            y = vent_line.second.1
        }

        (x, y)
    })
}

fn calculate_travelling(vent_lines: &Vec<VentLine>, calculate_diagonal: bool) -> Vec<Vec<i32>> {
    let bounds = define_bounds(&vent_lines);
    let travelling: Vec<Vec<i32>> = vec![vec![0; (bounds.1 + 1) as usize]; (bounds.0 + 1) as usize];

    vent_lines
        .clone()
        .iter()
        .fold(travelling, |travelling_agg, vent_line| {
            if is_travelling_horizontal(vent_line) {
                calculate_travelling_horizontal(vent_line, &travelling_agg)
            } else if is_travelling_vertical(vent_line) {
                calculate_travelling_vertical(vent_line, &travelling_agg)
            } else if is_travelling_diagonal(vent_line) && calculate_diagonal {
                calculate_travelling_diagonal(vent_line, &travelling_agg)
            } else {
                travelling_agg
            }
        })
}

fn calculate_travelling_horizontal(
    vent_line: &VentLine,
    travelling: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let y = vent_line.first.1 as usize;
    let mut travelling = travelling.clone();

    let (start, end): (usize, usize) = {
        if vent_line.first.0 > vent_line.second.0 {
            (vent_line.second.0 as usize, vent_line.first.0 as usize)
        } else {
            (vent_line.first.0 as usize, vent_line.second.0 as usize)
        }
    };

    for index in start..=end {
        travelling[index as usize][y] += 1;
    }

    travelling
}

fn calculate_travelling_vertical(
    vent_line: &VentLine,
    travelling: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let x = vent_line.first.0 as usize;

    let mut travelling = travelling.clone();

    let (start, end): (usize, usize) = {
        if vent_line.first.1 > vent_line.second.1 {
            (vent_line.second.1 as usize, vent_line.first.1 as usize)
        } else {
            (vent_line.first.1 as usize, vent_line.second.1 as usize)
        }
    };

    for index in start..=end {
        travelling[x][index as usize] += 1;
    }

    travelling
}

fn calculate_travelling_diagonal(
    vent_line: &VentLine,
    travelling: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let mut travelling = travelling.clone();

    let both_axes_is_increasing = (vent_line.first.0 == vent_line.first.1
        && vent_line.second.0 == vent_line.second.1)
        || (vent_line.first.0 > vent_line.second.0 && vent_line.first.1 > vent_line.second.1)
        || (vent_line.first.0 < vent_line.second.0 && vent_line.first.1 < vent_line.second.1);

    if both_axes_is_increasing {
        let (start, end) = {
            if vent_line.first.0 > vent_line.second.0 {
                (vent_line.second, vent_line.first)
            } else {
                (vent_line.first, vent_line.second)
            }
        };

        for index in (start.0..=end.0).zip(start.1..=end.1) {
            let (x_index, y_index) = index;
            travelling[x_index as usize][y_index as usize] += 1
        }
    } else {
        let (increasing, reducing) = {
            if vent_line.first.0 > vent_line.second.0 {
                (
                    (vent_line.first.1, vent_line.second.1, "y"),
                    (vent_line.second.0, vent_line.first.0, "x"),
                )
            } else {
                (
                    (vent_line.first.0, vent_line.second.0, "x"),
                    (vent_line.second.1, vent_line.first.1, "y"),
                )
            }
        };

        for index in (increasing.0..=increasing.1).zip((reducing.0..=reducing.1).rev()) {
            let (x_index, y_index) = {
                if increasing.2 == "x" {
                    (index.0, index.1)
                } else {
                    (index.1, index.0)
                }
            };
            travelling[x_index as usize][y_index as usize] += 1
        }
    }

    travelling
}

fn is_travelling_horizontal(vent_line: &VentLine) -> bool {
    vent_line.first.1 == vent_line.second.1
}

fn is_travelling_vertical(vent_line: &VentLine) -> bool {
    vent_line.first.0 == vent_line.second.0
}

fn is_travelling_diagonal(vent_line: &VentLine) -> bool {
    !is_travelling_vertical(vent_line) && !is_travelling_horizontal(vent_line)
}

fn calculate_overlap_count(travelling: &Vec<Vec<i32>>) -> i32 {
    let threshold = 2;
    travelling
        .clone()
        .into_iter()
        .flatten()
        .collect::<Vec<i32>>()
        .iter()
        .fold(0, |mut agg, travel| {
            if travel >= &threshold {
                agg += 1;
            }

            agg
        })
}
