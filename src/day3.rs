use std::fs;

#[derive(Debug, Clone)]
struct BitCount {
    one: i32,
    zero: i32,
}

pub fn day3() {
    let file_path = "resources/inputs/day3.txt";
    let input = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let input_lines: Vec<&str> = input.split('\n').collect();

    day_3_part_1(input_lines.clone());
    day_3_part_2(input_lines.clone());
}

fn day_3_part_1(input_lines: Vec<&str>) {
    let column_bit_counts: Vec<BitCount> = get_column_bit_counts(input_lines);

    let (gamma_rate_string, epsilon_rate_string): (String, String) =
        column_bit_counts.clone().into_iter().fold(
            (String::from(""), String::from("")),
            |(mut gamma_rate, mut epsilon_rate), column_count| {
                if column_count.zero > column_count.one {
                    gamma_rate.push_str("0");
                    epsilon_rate.push_str("1");
                } else if column_count.zero < column_count.one {
                    gamma_rate.push_str("1");
                    epsilon_rate.push_str("0");
                }
                (gamma_rate, epsilon_rate)
            },
        );

    let gamma_rate_decimal = isize::from_str_radix(&gamma_rate_string, 2).unwrap();
    let epsilon_rate_decimal = isize::from_str_radix(&epsilon_rate_string, 2).unwrap();

    println!(
        "Day 3, part 1: {}",
        gamma_rate_decimal * epsilon_rate_decimal
    );
}

fn day_3_part_2(input_lines: Vec<&str>) {
    let number_of_bits = input_lines.clone()[0].len();

    let mut oxygen_generator_lines: Vec<&str> = input_lines.clone();
    let mut co2_generator_lines: Vec<&str> = input_lines.clone();

    for index in 0..number_of_bits {
        let oxygen_generator_lines_column_count =
            get_column_bit_counts(oxygen_generator_lines.clone());
        let co2_generator_lines_column_count = get_column_bit_counts(co2_generator_lines.clone());

        let oxygen_generator_line_bit_count = &oxygen_generator_lines_column_count[index];
        let co2_generator_lines_bit_count = &co2_generator_lines_column_count[index];

        if oxygen_generator_lines.len() == 1 && co2_generator_lines.len() == 1 {
            break;
        }

        for line in oxygen_generator_lines.clone() {
            let line_columns = &line.chars().collect::<Vec<char>>();

            if oxygen_generator_line_bit_count.zero > oxygen_generator_line_bit_count.one
                && line_columns[index] == '1'
                && oxygen_generator_lines.len() > 1
            {
                oxygen_generator_lines.retain(|current_line| current_line != &line)
            } else if (oxygen_generator_line_bit_count.one > oxygen_generator_line_bit_count.zero
                || oxygen_generator_line_bit_count.zero == oxygen_generator_line_bit_count.one)
                && line_columns[index] == '0'
                && oxygen_generator_lines.len() > 1
            {
                oxygen_generator_lines.retain(|current_line| current_line != &line)
            }
        }

        for line in co2_generator_lines.clone() {
            let line_columns = &line.chars().collect::<Vec<char>>();

            if co2_generator_lines_bit_count.zero > co2_generator_lines_bit_count.one
                && line_columns[index] == '0'
                && co2_generator_lines.len() > 1
            {
                co2_generator_lines.retain(|current_line| current_line != &line)
            } else if (co2_generator_lines_bit_count.one > co2_generator_lines_bit_count.zero
                || co2_generator_lines_bit_count.zero == co2_generator_lines_bit_count.one)
                && line_columns[index] == '1'
                && co2_generator_lines.len() > 1
            {
                co2_generator_lines.retain(|current_line| current_line != &line)
            }
        }
    }

    let oxygen_generator_decimal = isize::from_str_radix(&oxygen_generator_lines[0], 2).unwrap();
    let co2_generator_decimal = isize::from_str_radix(&co2_generator_lines[0], 2).unwrap();

    println!(
        "Day 3, part 2: {}",
        oxygen_generator_decimal * co2_generator_decimal
    );
}

fn get_column_bit_counts(input_lines: Vec<&str>) -> Vec<BitCount> {
    let number_of_bits = input_lines.clone()[0].len();
    let mut column_bit_counts: Vec<BitCount> = Vec::new();

    for _ in 0..number_of_bits {
        column_bit_counts.push(BitCount { one: 0, zero: 0 })
    }

    for line in input_lines {
        let line_columns = line.chars().collect::<Vec<char>>();
        for index in 0..number_of_bits {
            if line_columns[index] == '1' {
                column_bit_counts[index].one += 1
            } else if line_columns[index] == '0' {
                column_bit_counts[index].zero += 1
            }
        }
    }

    column_bit_counts
}
