use std::fs;
use std::io::{self, BufRead};

fn count_bad_levels(report: Vec<i32>) -> i32 {

    let mut is_ascending: Option<bool> = None;
    let mut bad_levels: i32 = 0;
    let mut diff: i32;
    let mut abs_diff;
    let mut prev: Option<i32> = None;

    for num in report {
        if prev.is_none() {
            prev = Some(num);
            continue;
        };

        diff = num - prev.unwrap_or(0);
        abs_diff = diff.abs();

        if !((3 >= abs_diff) && (abs_diff >= 1)) {
            bad_levels += 1;
            continue;
        };

        if is_ascending.is_none() {
            is_ascending = Some(diff > 0);
            prev = Some(num);
            continue;
        };

        if is_ascending != Some(diff > 0) {
            bad_levels += 1;
            continue;
        };

        prev = Some(num);
    }

    return bad_levels
}

fn is_report_safe_damped (report: Vec<i32>) -> bool {

    let mut new_vec: Vec<i32>;

    // just brute forcing it
    for (i, _num) in report.iter().enumerate() {

        new_vec = vec![];
        new_vec.extend(&report[..i]);
        new_vec.extend(&report[i+1..]);

        if count_bad_levels(new_vec) == 0 {
            return true;
        }
    }

    return false
}

fn main() {

    let file_path = "input_files/input_file";
    let file = fs::File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    let mut safe_report_count: i32 = 0;
    let mut safe_report_count_damped: i32 = 0;

    for line in reader.lines() {
        let report: Vec<i32> = line.unwrap().split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
        .collect();

        if count_bad_levels(report.clone()) == 0 { // future task - learn how to efficiently manage ownership issue (slices, iterators).
            safe_report_count += 1
        }

        if is_report_safe_damped(report.clone()) {
            safe_report_count_damped += 1
        }
    };

    println!("Part 1: {}", safe_report_count);
    println!("Part 2: {}", safe_report_count_damped);
}
