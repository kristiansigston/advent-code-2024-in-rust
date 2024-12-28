fn main() {
    let contents = std::fs::read_to_string("src/data.txt").unwrap();

    // split data by lines
    let lines: Vec<&str> = contents.lines().collect();

    // split each line by whitespace
    // then make sure that the difference between each intger is between 1 and 3 inclusive.
    // if so then increment the safeReport counter

    let mut direct_safe_report = 0;
    let mut direct_and_one_removed_safe_report = 0;
    let mut total = 0;

    for line in lines {
        let numbers: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        if is_safe_report(&numbers) {
            direct_safe_report += 1;
            total += 1;
        } else if is_safe_by_removing_one_element(&numbers) {
            direct_and_one_removed_safe_report += 1;
            total += 1;
        }
    }
    // add new line to make the output more readable
    print!("Safe reports: {:?}", direct_safe_report);
    print!("\n");
    print!(
        "adjusted safe reports {:?}",
        direct_and_one_removed_safe_report
    );
    print!("\n");
    let total_safe_reports = direct_safe_report + direct_and_one_removed_safe_report;
    print!(" total safe {:?}", total_safe_reports);
    print!(" out of {:?}", total);
    println!();
}

fn is_safe_report(numbers: &[i32]) -> bool {
    let mut is_increasing = false;
    let mut is_decreasing = false;

    for pair in numbers.windows(2) {
        let diff = pair[1] - pair[0];

        // Check if the sequence is inconsistent or out of range
        if diff.abs() > 3 || diff.abs() < 1 {
            return false;
        }

        // Track whether the sequence is increasing or decreasing
        if diff > 0 {
            is_increasing = true;
        } else if diff < 0 {
            is_decreasing = true;
        }

        // If both increasing and decreasing trends are detected, it's unsafe
        if is_increasing && is_decreasing {
            return false;
        }
    }

    true
}

fn is_safe_by_removing_one_element(numbers: &[i32]) -> bool {
    for (i, _) in numbers.iter().enumerate() {
        let numbers_copy = [&numbers[..i], &numbers[i + 1..]].concat();

        if is_safe_report(&numbers_copy) {
            return true;
        }
    }

    false
}
