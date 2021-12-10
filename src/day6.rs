use std::fs;

pub fn main() {
    let file_path = "resources/inputs/day6.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let content_value: Vec<usize> = contents
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|timer| timer.parse().unwrap())
        .collect();

    let init_growth_timers: Vec<i64> =
        content_value.iter().fold(vec![0; 9], |mut timers, timer| {
            timers[timer.clone()] += 1;
            timers
        });

    day_6_part_1(init_growth_timers.clone(), 80);
    day_6_part_2(init_growth_timers.clone(), 256);
}

fn day_6_part_1(init_growth_timers: Vec<i64>, days_after: i32) {
    let growth_timers = calculate_growth_timers(init_growth_timers, days_after);

    println!(
        "Day 6, part 1: {}",
        growth_timers.clone().iter().sum::<i64>()
    );
}

fn day_6_part_2(init_growth_timers: Vec<i64>, days_after: i32) {
    let growth_timers = calculate_growth_timers(init_growth_timers, days_after);

    println!(
        "Day 6, part 2: {}",
        growth_timers.clone().iter().sum::<i64>()
    );
}

fn calculate_growth_timers(init_growth_timers: Vec<i64>, days_after: i32) -> Vec<i64> {
    (0..days_after).fold(init_growth_timers.clone(), |growth_timers, _| {
        let mut updated_growth_timers = growth_timers.clone();

        for index in (0..growth_timers.len()).rev() {
            if index != 0 {
                updated_growth_timers[index - 1] = growth_timers[index];
                if index == 8 && growth_timers[0] == 0 {
                    updated_growth_timers[index] = 0
                }
            } else if index == 0 && growth_timers[index] != 0 {
                updated_growth_timers[8] = growth_timers[index];
                updated_growth_timers[6] += growth_timers[index];
            }
        }

        updated_growth_timers
    })
}
