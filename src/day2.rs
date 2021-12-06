use std::fs;

#[derive(Debug)]
#[derive(Clone)]
struct Sub {
    depth: i32,
    length: i32,
}


pub fn day2() {
    let file_path = "resources/inputs/day2.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");


    let content_value: Vec<&str> = contents.split('\n').collect();
    let mut directions: Vec<Sub> = Vec::new();

    for line in content_value {
        let mut depth = 0;
        let mut length = 0;
        let split_line = line.split(" ").collect::<Vec<&str>>();

        if split_line[0] == "down" {
            depth = split_line[1].parse::<i32>().unwrap();
        } else if split_line[0] == "up" {
            depth = -split_line[1].parse::<i32>().unwrap();
        } else if split_line[0] == "forward" {
            length = split_line[1].parse::<i32>().unwrap();
        }

        directions.push(Sub { depth, length });
    };

    day_2_part_1(directions.clone());
    day_2_part_2(directions.clone());
}

fn day_2_part_1(directions: Vec<Sub>) {
    let sum_sub_directions: Sub = directions.into_iter().fold(
        Sub { depth: 0, length: 0 }, |sum_direction, direction| {
            Sub {
                depth: sum_direction.depth + direction.depth,
                length: sum_direction.length + direction.length,
            }
        });

    println!("Day 2, part 1: {}", sum_sub_directions.depth * sum_sub_directions.length)
}

fn day_2_part_2(directions: Vec<Sub>) {
    let (_, sum_sub_directions): (i32, Sub) = directions.into_iter().fold(
        (0i32, Sub { depth: 0, length: 0 }), |(aim, sum_direction), direction| {
            let updated_aim = aim + direction.depth;
            let updated_depth: i32 = (aim * direction.length) + sum_direction.depth;
            let updated_length = direction.length + sum_direction.length;

            (updated_aim, Sub {
                depth: updated_depth,
                length: updated_length,
            })
        });

    println!("Day 2, part 2: {}", sum_sub_directions.depth * sum_sub_directions.length)
}