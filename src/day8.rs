use std::fs;

fn main() {
    let file_path = "resources/inputs/day8.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");


    let content_value = contents.split("\n").collect::<Vec<&str>>().iter().map(|line| {
        line.clone().split("|").collect::<Vec<&str>>()[1].clone().split_whitespace().clone().collect::<Vec<&str>>()
    }).collect::<Vec<Vec<&str>>>();

    let flattened = content_value.iter().flatten().collect::<Vec<&&str>>();
    let special_characters: Vec<usize> = vec![2, 3, 4, 7];

    let sum = flattened.iter().fold(0, |mut sum, item| {
        if special_characters.contains(&(item).len()) {
            sum += 1
        }
        sum
    });

    println!("{:?}", sum);
}