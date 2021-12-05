use std::fs;
use std::convert::TryFrom;


fn main() {
    let sliding_window: i32 = 3;

    let file_path = "resources/day1-input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let content_value: Vec<&str> = contents.split('\n').collect();

    let int_content_value: Vec<i32> = content_value.into_iter().map(|x| { x.parse().unwrap() }).collect();

    day_1_part_1(int_content_value.clone());
    day_1_part_2(int_content_value.clone(), sliding_window);
}

fn day_1_part_1(input: Vec<i32>) {
    let (_, sum_increasing) = input.into_iter().fold((0i32, 0i32), |(curr, mut sum), val| {
        if curr == 0 { sum = 0 } else { if curr < val { sum += 1 } };
        (val, sum)
    });

    println!("{}", sum_increasing);
}

fn day_1_part_2(input: Vec<i32>, sliding_window: i32) {
    let mut sum_increasing = 0;
    let mut previous_value: i32 = 0;
    for (pos, value) in input.iter().enumerate() {
        if (input.len() - pos) >= usize::try_from(sliding_window).unwrap() {
            if (value + input[pos + 1] + input[pos + 2]) > previous_value && (previous_value != 0) {
                sum_increasing += 1;
            }
            previous_value = value + input[pos + 1] + input[pos + 2]
        }
    }

    println!("{}", sum_increasing);
}

