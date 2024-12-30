use std::time::Instant;

const WORD_TO_SEARCH_FOR: [char; 4] = ['X', 'M', 'A', 'S'];

const WORD_TO_SEARCH_FOR_LENGTH: usize = WORD_TO_SEARCH_FOR.len();

fn main() {
    let file_data = std::fs::read_to_string("src/data.txt").unwrap();
    let mut word_search: Vec<Vec<char>> = Vec::new();

    file_data.lines().for_each(|line| {
        let row: Vec<char> = line.chars().collect();
        word_search.push(row);
    });

    let start_optimized = Instant::now();
    let optimized_count: i32 = check_all_directions_optimised(&word_search);
    let duration_optimized = start_optimized.elapsed();

    let start_unoptimized = Instant::now();
    let unoptimized_count: i32 = check_all_directions(&word_search);
    let duration_unoptimized = start_unoptimized.elapsed();

    println!(
        "Unoptimized Count: {}, Time: {:?}",
        unoptimized_count, duration_unoptimized
    );
    println!(
        "Optimized Count: {}, Time: {:?}",
        optimized_count, duration_optimized
    );
}

fn check_all_directions_optimised(word_search: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (-1, -1), // Up-Left Diagonal
        (-1, 1),  // Up-Right Diagonal
        (1, -1),  // Down-Left Diagonal
        (1, 1),   // Down-Right Diagonal
    ];

    let rows = word_search.len();
    let cols = word_search[0].len();

    for x in 0..rows {
        for y in 0..cols {
            for &(x_dir, y_dir) in &directions {
                if can_fit_word(x, y, x_dir, y_dir, rows, cols) {
                    if check_next_char(word_search, 0, x as isize, y as isize, x_dir, y_dir) {
                        println!(
                            "Found word at x: {}, y: {} in direction ({}, {})",
                            x, y, x_dir, y_dir
                        );
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn check_all_directions(word_search: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (-1, -1), // Up-Left Diagonal
        (-1, 1),  // Up-Right Diagonal
        (1, -1),  // Down-Left Diagonal
        (1, 1),   // Down-Right Diagonal
    ];

    for x in 0..word_search.len() {
        for y in 0..word_search[x].len() {
            for &(x_dir, y_dir) in &directions {
                if check_next_char(word_search, 0, x as isize, y as isize, x_dir, y_dir) {
                    println!(
                        "Found word at x: {}, y: {} in direction ({}, {})",
                        x, y, x_dir, y_dir
                    );
                    count += 1;
                }
            }
        }
    }
    count
}

fn check_next_char(
    word_search: &Vec<Vec<char>>,
    char_position: usize,
    x: isize,
    y: isize,
    x_direction: isize,
    y_direction: isize,
) -> bool {
    if char_position >= WORD_TO_SEARCH_FOR_LENGTH {
        return true; // Successfully matched the whole word
    }

    // Check bounds
    if x < 0 || y < 0 || x as usize >= word_search.len() || y as usize >= word_search[0].len() {
        return false;
    }

    // Check character match
    if word_search[x as usize][y as usize] == WORD_TO_SEARCH_FOR[char_position] {
        return check_next_char(
            word_search,
            char_position + 1,
            x + x_direction,
            y + y_direction,
            x_direction,
            y_direction,
        );
    }

    false
}

fn can_fit_word(
    x: usize,
    y: usize,
    x_direction: isize,
    y_direction: isize,
    rows: usize,
    cols: usize,
) -> bool {
    let word_length = WORD_TO_SEARCH_FOR_LENGTH as isize;

    // Calculate the ending position of the word
    let end_x = x as isize + (word_length - 1) * x_direction;
    let end_y = y as isize + (word_length - 1) * y_direction;

    // Check if the ending position is within bounds
    end_x >= 0 && end_x < rows as isize && end_y >= 0 && end_y < cols as isize
}

fn get_cross_mas() -> i32 {

}
