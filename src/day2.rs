use std::fs;

#[derive(Debug)]
#[derive(Clone)]
struct SubDirection {
    depth: i32,
    length: i32,
}


pub fn day2() {
    let file_path = "resources/inputs/day2.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");


    let content_value: Vec<&str> = contents.split('\n').collect();
    let mut directions: Vec<SubDirection> = Vec::new();

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

        directions.push(SubDirection { depth, length });
    };

    day_2_part_1(directions.clone());
    day_2_part_2(directions.clone());
}

fn day_2_part_1(directions: Vec<SubDirection>) {
    let sum_sub_directions: SubDirection = directions.into_iter().fold(
        SubDirection { depth: 0, length: 0 }, |sum_direction, direction| {
            SubDirection {
                depth: sum_direction.depth + direction.depth,
                length: sum_direction.length + direction.length,
            }
        });

    println!("Day 2, part 1: {}", sum_sub_directions.depth * sum_sub_directions.length)
}

fn day_2_part_2(directions: Vec<SubDirection>) {
    let (_, sum_sub_directions): (i32, SubDirection) = directions.into_iter().fold(
        (0i32, SubDirection { depth: 0, length: 0 }), |(aim, sum_direction), direction| {
            let updated_aim = aim + direction.depth;
            let updated_depth: i32 = (aim * direction.length) + sum_direction.depth;
            let updated_length = direction.length + sum_direction.length;

            (updated_aim, SubDirection {
                depth: updated_depth,
                length: updated_length,
            })
        });

    println!("Day 2, part 2: {}", sum_sub_directions.depth * sum_sub_directions.length)
}