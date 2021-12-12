use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Position {
    multiplier: i32,
    fuel: i32,
}


pub fn main() {
    let file_path = "resources/inputs/day7.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let content_value: Vec<i32> = contents
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|pos| pos.parse().unwrap())
        .collect();


    let crab_positions = content_value.iter().fold(HashMap::new(), |mut positions, pos| {
        if positions.contains_key(pos) {
            let position: &Position = positions.get(pos).unwrap();
            let updated_position = Position {
                multiplier: position.multiplier + 1,
                ..*position
            };

            positions.insert(*pos, updated_position);
        } else {
            let position = Position {
                multiplier: 1,
                fuel: 0,
            };

            positions.insert(*pos, position);
        }

        positions
    });

    let min_key = crab_positions.keys().into_iter().min().unwrap().clone();
    let max_key = crab_positions.keys().into_iter().max().unwrap().clone();

    let all_positions: HashMap<i32, Position> = (min_key.clone()..=max_key.clone())
        .fold(crab_positions.clone(), |mut positions_with_gaps_filled, pos| {
            if !positions_with_gaps_filled.contains_key(&pos) {
                let position = Position {
                    multiplier: 1,
                    fuel: 0,
                };

                positions_with_gaps_filled.insert(pos, position);
            }

            positions_with_gaps_filled.clone()
        });


    day_7_part_1(all_positions.clone(), crab_positions.clone());
    day_7_part_2(all_positions.clone(), crab_positions.clone());
}

fn day_7_part_1(all_positions: HashMap<i32, Position>, crab_positions: HashMap<i32, Position>) {
    let updated_positions = populate_positions_fuel(all_positions, crab_positions, false);
    let least_fuel_position: Option<(i32, Position)> = calculate_least_fuel_position(updated_positions);

    println!("Day 7, part 1: {}", least_fuel_position.unwrap().1.fuel);
}

fn day_7_part_2(all_positions: HashMap<i32, Position>, crab_positions: HashMap<i32, Position>) {
    let updated_positions = populate_positions_fuel(all_positions, crab_positions, true);
    let least_fuel_position: Option<(i32, Position)> = calculate_least_fuel_position(updated_positions);

    println!("Day 7, part 2: {}", least_fuel_position.unwrap().1.fuel);
}

fn binomial_coefficient(num: i32) -> f32 {
    let num = num as f32;
    num * ((num + 1.0) / 2.0)
}

fn populate_positions_fuel(all_positions: HashMap<i32, Position>, crab_positions: HashMap<i32, Position>, binomial: bool) -> HashMap<i32, Position> {
    all_positions.iter().
        fold(all_positions.clone(), |mut positions, (current_key, _)| {
            let current_key = current_key.clone();
            for (key, position) in crab_positions.clone() {
                let fuel = {
                    if binomial {
                        (binomial_coefficient((current_key.clone() - key.clone()).abs()) as i32) * position.multiplier
                    } else {
                        (current_key.clone() - key.clone()).abs() * position.multiplier
                    }
                };

                let updated_position: &Position = positions.get(&current_key).unwrap();
                let updated_position = Position {
                    fuel: updated_position.fuel + fuel,
                    ..*updated_position
                };
                positions.insert(current_key, updated_position);
            }

            positions.clone()
        })
}

fn calculate_least_fuel_position(positions: HashMap<i32, Position>) -> Option<(i32, Position)> {
    positions.iter().fold(None, |least_fuel_position, (current_key, current_position)| {
        match least_fuel_position {
            None => Some((*current_key, current_position.clone())),
            Some((key, position)) => {
                if (current_position.fuel) < (position.fuel) {
                    Some((*current_key, current_position.clone()))
                } else {
                    Some((key, position))
                }
            }
        }
    })
}