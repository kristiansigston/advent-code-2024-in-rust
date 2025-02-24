use std::env;
// use std::io::Write;
use std::fs::{self};

// testing

// test main with the example.txt
fn parse_data_from_file(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "src/data.txt"
    };

    let data = parse_data_from_file(filename);
    let data_row = data.iter().map(|s| s.trim()).collect::<Vec<&str>>();

    let extracted_totals_and_numbers: Vec<(i64, Vec<i64>)> = data_row
        .iter()
        .map(|s| {
            let mut split = s.split(":");
            let total = split.next().unwrap().trim().parse::<i64>().unwrap();
            let data = split
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (total, data)
        })
        .collect();

    let total_sum = extracted_totals_and_numbers
        .iter()
        .fold(0, |acc, (total, data)| {
            if get_is_product_possible(&data, *total) {
                acc + total
            } else {
                acc
            }
        });
    println!("Final sum: {}", total_sum);
}

fn get_is_product_possible(data: &Vec<i64>, total: i64) -> bool {
    let n = data.len();

    if n == 0 {
        return false;
    }

    let op_count = n - 1;

    if op_count == 0 {
        return data[0] == total;
    }

    let total_combinations = 3usize.pow(op_count as u32);

    for i in 0..total_combinations {
        let mut current_total = data[0];
        for j in 0..op_count {
            let op = (i / 3usize.pow(j as u32)) % 3;
            match op {
                0 => {
                    current_total += data[j + 1];
                }
                1 => {
                    current_total *= data[j + 1];
                }
                2 => {
                    current_total = format!("{}{}", current_total, data[j + 1])
                        .parse::<i64>()
                        .unwrap();
                }
                _ => {
                    panic!("Invalid operator");
                }
            }
        }

        if current_total == total {
            println!("Found a match: with operators {:?}", current_total,);
            return true;
        }
    }

    false
}

// fn get_is_product_possible_with_concats(data: &Vec<i64>, total: i64) -> bool {
//     let n = data.len();

//     let op_count = n - 1;
//     for i in 0..op_count {
//         // remove ith and ith + one number
//         let mut cloned_data = data.clone();

//         let a = cloned_data.remove(i);
//         let b = cloned_data.remove(i);

//         // join i and the next number as an integer
//         let joined = format!("{}{}", a, b).parse::<i64>().unwrap();

//         cloned_data.insert(i, joined);
//         if get_is_product_possible(&cloned_data, total) {
//             println!("Found a match: {:?} with concats {:?}", cloned_data, total,);
//             return true;
//         }
//     }

//     false
// }
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_data_single_line() {
//         let test_file = "test_single.txt";
//         // Single line file with "1379" as the only number.
//         let data = parse_data_from_file(test_file);
//         let sum: i32 = data.iter().map(|s| s.trim().parse::<i32>().unwrap()).sum();
//         fs::remove_file(test_file).expect("Failed to remove test file");
//         assert_eq!(sum, 3749);
//     }

//     #[test]
//     fn test_parse_data_multiple_lines() {
//         let test_file = "test_multiple.txt";
//         // Multiple lines that add up to 1379: 500 + 300 + 579 = 1379
//         let data = parse_data_from_file(test_file);
//         let sum: i32 = data.iter().map(|s| s.trim().parse::<i32>().unwrap()).sum();
//         fs::remove_file(test_file).expect("Failed to remove test file");
//         assert_eq!(sum, 1379);
//     }
// }
