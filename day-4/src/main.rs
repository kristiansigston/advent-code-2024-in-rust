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

    let mas_count = get_cross_mas(&word_search);

    println!(
        "Unoptimized Count: {}, Time: {:?}",
        unoptimized_count, duration_unoptimized
    );
    println!(
        "Optimized Count: {}, Time: {:?}",
        optimized_count, duration_optimized
    );
    println!("MAS Count:{:?}", mas_count);
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

pub fn get_cross_mas(word_search: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for x in 0..word_search.len() {
        for y in 0..word_search[x].len() {
            if word_search[x][y] != 'M' {
                continue;
            }
            let mas_count: i32 = check_cross_mas(word_search, x, y);
            count += mas_count
        }
    }
    println!("MAS Count:{} ", count);
    count
}

fn check_cross_mas(word_search: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut count = 0;
    // should this check be in the previous function?
    let is_up = check_x_up(word_search, x, y);
    if is_up {
        count += 1;
    }

    let is_down = check_x_down(word_search, x, y);
    if is_down {
        count += 1;
    }
    let is_right = check_y_right(word_search, x, y);
    if is_right {
        count += 1;
    }

    let is_left = check_y_left(word_search, x, y);
    if is_left {
        count += 1;
    }

    count
}

pub fn check_y_left(word_search: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let rows = word_search.len() as isize;

    // guard top and bottom edge
    if x as isize + 2 >= rows || y as isize - 2 < 0 {
        return false;
    }

    if word_search[x + 2][y] == 'M'
        && word_search[x][y - 2] == 'S'
        && word_search[x + 2][y - 2] == 'S'
        && word_search[x + 1][y - 1] == 'A'
    {
        return true;
    }

    return false;
}

pub fn check_y_right(word_search: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let rows = word_search.len() as isize;
    let cols = word_search[x].len() as isize;

    // guard top and bottom edge
    if x as isize + 2 >= rows || y as isize + 2 >= cols {
        return false;
    }

    if word_search[x + 2][y] == 'M'
        && word_search[x][y + 2] == 'S'
        && word_search[x + 2][y + 2] == 'S'
        && word_search[x + 1][y + 1] == 'A'
    {
        return true;
    }

    return false;
}

pub fn check_x_up(word_search: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let cols = word_search[x].len() as isize;

    // guard top and bottom edge
    if x as isize - 2 < 0 || y as isize + 2 >= cols {
        return false;
    }

    if word_search[x][y + 2] == 'M'
        && word_search[x - 2][y] == 'S'
        && word_search[x - 2][y + 2] == 'S'
        && word_search[x - 1][y + 1] == 'A'
    {
        return true;
    }

    return false;
}

pub fn check_x_down(word_search: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let rows = word_search.len() as isize;
    let cols = word_search[x].len() as isize;

    // guard top and bottom edge
    if x as isize + 2 >= rows || y as isize + 2 >= cols {
        return false;
    }

    if word_search[x][y + 2] == 'M'
        && word_search[x + 2][y] == 'S'
        && word_search[x + 2][y + 2] == 'S'
        && word_search[x + 1][y + 1] == 'A'
    {
        return true;
    }

    return false;
}

// test check cross mas

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_cross_mas_nothing() {
        let word_search = vec![
            vec!['M', 'A', 'S'],
            vec!['A', 'A', 'A'],
            vec!['S', 'A', 'S'],
        ];
        assert_eq!(check_cross_mas(&word_search, 1, 0), 0);
    }

    #[test]
    fn test_check_cross_mas_up() {
        let word_search = vec![
            vec!['S', 'A', 'S'],
            vec!['A', 'A', 'A'],
            vec!['M', 'A', 'M'],
        ];
        assert_eq!(check_cross_mas(&word_search, 2, 0), 1);
    }

    #[test]
    fn test_get_cross_mas() {
        let word_search: Vec<Vec<char>> = vec![
            vec!['M', 'A', 'M'],
            vec!['A', 'A', 'A'],
            vec!['S', 'A', 'S'],
        ];

        let answer = get_cross_mas(&word_search);
        assert_eq!(answer, 1); // Adjust based on the expected count
    }

    #[test]
    fn test_get_cross_mas_2() {
        let word_search: Vec<Vec<char>> = vec![
            vec!['M', 'M', 'M', 'M'],
            vec!['.', 'A', 'A', '.'],
            vec!['S', 'S', 'S', 'S'],
        ];

        let answer = get_cross_mas(&word_search);
        assert_eq!(answer, 2); // Adjust based on the expected count
    }

    #[test]
    fn test_get_cross_mas_4() {
        let word_search: Vec<Vec<char>> = vec![
            vec!['M', 'M', 'M', 'M'],
            vec!['.', 'A', 'A', '.'],
            vec!['S', 'S', 'S', 'S'],
            vec!['.', 'A', 'A', '.'],
            vec!['M', 'M', 'M', 'M'],
        ];

        let answer = get_cross_mas(&word_search);
        assert_eq!(answer, 4); // Adjust based on the expected count
    }

    #[test]
    fn test_vertical_down() {
        let word_search: Vec<Vec<char>> = vec![
            vec!['M', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'S'],
        ];

        assert_eq!(check_x_down(&word_search, 0, 0), true);
    }
    #[test]
    fn test_vertical_down_no() {
        let word_search: Vec<Vec<char>> = vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'S'],
        ];

        assert_eq!(check_x_down(&word_search, 0, 0), false);
    }

    #[test]
    fn test_vertical_up() {
        let word_search: Vec<Vec<char>> = vec![
            vec!['S', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'M'],
        ];

        assert_eq!(check_x_up(&word_search, 2, 0), true);
    }
    
    #[test]
    fn test_vertical_up_no() {
        let word_search: Vec<Vec<char>> = vec![
            vec!['S', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'M'],
        ];

        assert_eq!(check_x_up(&word_search, 2, 0), false);
    }
    #[test]
    fn test_check_y_left() {
        let word_search: Vec<Vec<char>> = vec![
            vec!['S', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'M'],
        ];

        assert_eq!(check_y_left(&word_search, 0, 2), true);
    }
    #[test]
    fn test_check_y_left_no() {
        let word_search: Vec<Vec<char>> = vec![
            vec!['S', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'S'],
        ];

        assert_eq!(check_y_left(&word_search, 0, 2), false);
    }
    #[test]
    fn test_check_y_right() {
        let word_search: Vec<Vec<char>> = vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'S'],
        ];

        assert_eq!(check_y_right(&word_search, 0, 0), true);
    }
    #[test]
    fn test_check_y_right_no() {
        let word_search: Vec<Vec<char>> = vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'S'],
        ];

        assert_eq!(check_y_right(&word_search, 0, 0), false);
    }
}
