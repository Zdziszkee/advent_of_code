use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "../../resources/1-2.txt";

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file '{}': {}", path, e);
            return;
        }
    };

    let reader = BufReader::new(file);

    let mut left_numbers = Vec::new();
    let mut right_number_count: HashMap<u32, u32> = HashMap::new();

    for (line_number, line_result) in reader.lines().enumerate() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line {}: {}", line_number, e);
                return;
            }
        };

        let left_number: u32 = match line[0..5].parse::<u32>() {
            Ok(number) => number,
            Err(e) => {
                eprintln!("Error parsing left number on line {}: {}", line_number, e);
                return;
            }
        };

        let right_number: u32 = match line[8..13].parse::<u32>() {
            Ok(number) => number,
            Err(e) => {
                eprintln!("Error parsing right number on line {}: {}", line_number, e);
                return;
            }
        };

        left_numbers.push(left_number);

        right_number_count
            .entry(right_number)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut sum = 0;

    for left_number in left_numbers {
        match right_number_count.get(&left_number) {
            Some(count) => {
                sum += count * left_number;
            }
            None => {
                continue;
            }
        };
    }

    println!("{}", sum);
}
