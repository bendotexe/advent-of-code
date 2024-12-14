use std::fs;

fn main() {

    let file_path = "../input_files/input_file";

    println!("In file {file_path}");

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let words = contents.split('\n');

    for word in words {
        let numbers: Vec<i32> = word.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
        .collect();

        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        } else {
            println!("Skipping invalid line: {}", word);
        }
    };

    left_list.sort();
    right_list.sort();
    let pairs: Vec<(i32, i32)> = left_list.into_iter().zip(right_list.into_iter()).collect();

    let mut total_diff = 0;

    for pair in pairs {
        total_diff += (pair.0 -pair.1).abs();
    }

    println!("{}", total_diff);

}