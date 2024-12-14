use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "../input_files/input_file";

    println!("In file {file_path}");

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    let mut similarity_score: i32 = 0;

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

    let mut right_list_counts: HashMap<i32, i32> = HashMap::new();

    for num in right_list {
        *right_list_counts.entry(num).or_insert(0) +=1;
    }

    for num in left_list {
        similarity_score += num * *right_list_counts.get(&num).unwrap_or(&0);
    };

    println!("{}", similarity_score)

}
