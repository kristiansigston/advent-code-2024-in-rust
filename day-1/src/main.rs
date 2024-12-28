use std::collections::HashMap;

fn main() {
    // Read file contents
    let contents = std::fs::read_to_string("src/data.txt").unwrap();

    // Split data into two arrays
    let (mut array1, mut array2) = split_arrays(&contents);

    // Sort the arrays
    array1.sort();
    array2.sort();

    // Compute the sum of absolute differences between the arrays
    let sum = calculate_sum_of_differences(&array1, &array2);
    println!("Sum of differences: {:?}", sum);

    // Compute the second sum based on lookup
    let sum2 = calculate_weighted_sum(&array1, &array2);
    println!("Weighted sum: {:?}", sum2);
}

/// Splits the input string into two arrays of integers.
fn split_arrays(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let mut array1 = Vec::new();
    let mut array2 = Vec::new();

    for line in contents.lines() {
        let mut numbers = line.split_whitespace();
        let number1: i32 = numbers.next().unwrap().parse().unwrap();
        let number2: i32 = numbers.next().unwrap().parse().unwrap();
        array1.push(number1);
        array2.push(number2);
    }

    (array1, array2)
}

/// Calculates the sum of absolute differences between two sorted arrays.
fn calculate_sum_of_differences(array1: &[i32], array2: &[i32]) -> i32 {
    array1
        .iter()
        .zip(array2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

/// Calculates a weighted sum of elements in array1 based on their occurrences in array2.
fn calculate_weighted_sum(array1: &[i32], array2: &[i32]) -> i32 {
    // Create a lookup for occurrences in array2
    let mut lookup: HashMap<i32, i32> = HashMap::new();
    for number in array2 {
        let count = lookup.entry(*number).or_insert(0);
        *count += 1;
    }

    // Calculate the weighted sum
    let mut sum = 0;
    for number in array1 {
        if let Some(&count) = lookup.get(number) {
            sum += number * count;
        }
    }
    sum
}
