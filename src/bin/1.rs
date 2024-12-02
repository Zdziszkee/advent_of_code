use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "../../resources/1.txt";

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file '{}': {}", path, e);
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

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
        right_numbers.push(right_number);
    }

    radix_sort(&mut left_numbers);
    radix_sort(&mut right_numbers);

    let mut sum: u64 = 0;
    for (left, right) in left_numbers.iter().zip(right_numbers.iter()) {
        let distance = left.abs_diff(*right);
        sum += distance as u64;
    }
    println!("{}", sum);
}

fn radix_sort(arr: &mut [u32]) {
    let mut output = vec![0; arr.len()];
    let mut count = vec![0; 256];

    for shift in (0..32).step_by(8) {
        for &num in arr.iter() {
            count[((num >> shift) & 0xFF) as usize] += 1;
        }

        for i in 1..256 {
            count[i] += count[i - 1];
        }

        for &num in arr.iter().rev() {
            let index = ((num >> shift) & 0xFF) as usize;
            count[index] -= 1;
            output[count[index]] = num;
        }

        arr.copy_from_slice(&output);
        count.fill(0);
    }
}
