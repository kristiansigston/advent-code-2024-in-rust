use std::fs;

const GUARDS: &[&str] = &["^", "v", ">", "<"];

fn parse_board_from_file(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

// Only reads the board; no mutation.
fn get_guard_locations(board: &Vec<Vec<String>>) -> Vec<(char, (usize, usize))> {
    let mut locations = Vec::new();
    for (i, line) in board.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            if GUARDS.contains(&cell.as_str()) {
                let guard_char = cell.chars().next().unwrap();
                locations.push((guard_char, (i, j)));
            }
        }
    }
    locations
}

// fn print_board(board: &Vec<Vec<String>>) {
//     for row in board {
//         for cell in row {
//             print!("{}", cell);
//         }
//         println!();
//     }
// }

/// Moves each guard one step until they either leave the board, hit a "." (resetting the loop counter),
/// or repeatedly hit obstacles. If a guard hits obstacles more than 4 times, a blockage is logged at the
/// current position and the simulation immediately exits.
fn move_guard(
    play_board: &mut Vec<Vec<String>>,
    guard_data: &mut Vec<(char, (usize, usize))>,
    loop_counter: &mut u32,
    loop_creation_blockages: &mut Vec<(usize, usize)>,
) {
    let mut removals = Vec::new();

    for idx in 0..guard_data.len() {
        let (guard, pos) = &mut guard_data[idx];
        let (i, j) = *pos;
        let i_isize = i as isize;
        let j_isize = j as isize;
        let (new_i_isize, new_j_isize) = match guard {
            '^' => (i_isize - 1, j_isize),
            'v' => (i_isize + 1, j_isize),
            '>' => (i_isize, j_isize + 1),
            '<' => (i_isize, j_isize - 1),
            _ => panic!("Invalid guard character"),
        };

        // If new indices are out-of-bounds, schedule removal.
        if new_i_isize < 0
            || new_j_isize < 0
            || new_i_isize >= play_board.len() as isize
            || new_j_isize >= play_board[0].len() as isize
        {
            removals.push(idx);
            continue;
        }

        let new_i = new_i_isize as usize;
        let new_j = new_j_isize as usize;
        let next_cell = &play_board[new_i][new_j];

        match next_cell.as_str() {
            "#" => {
                // Turn guard 90° right.
                let new_guard = match guard {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => panic!("Invalid guard character"),
                };
                *loop_counter += 1;
                if *loop_counter > 3 {
                    // Log blockage at the position where the guard would have continued.
                    play_board[i][j] = "X".to_string();
                    play_board[new_i][new_j] = "X".to_string();
                    loop_creation_blockages.push((i, j));
                    return; // Exit immediately for this simulation.
                }
                *guard = new_guard;
            }
            "X" => {
                // Move the guard forward and mark their new position.
                play_board[i][j] = "X".to_string();
                play_board[new_i][new_j] = "X".to_string();
                *pos = (new_i, new_j);
            }
            "." => {
                // Reset the loop counter and move the guard.
                *loop_counter = 0;
                play_board[i][j] = "X".to_string();
                play_board[new_i][new_j] = "X".to_string();
                *pos = (new_i, new_j);
            }
            _ => panic!("Invalid cell character"),
        }
    }

    // Remove guards that have moved off-board.
    for idx in removals.into_iter().rev() {
        guard_data.remove(idx);
    }
}

/// Counts all cells marked with "X" on the board.
fn count_guard_positions(board: &Vec<Vec<String>>) -> usize {
    board
        .iter()
        .flat_map(|line| line.iter())
        .filter(|cell| *cell == "X")
        .count()
}

/// Collects all positions on the board that have been marked with "X".
fn get_all_traversed_guard_positions(board: &Vec<Vec<String>>) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for (i, line) in board.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            if cell == "X" {
                positions.push((i, j));
            }
        }
    }
    positions
}

fn main() {
    // PART 1: Simulate the primary board.
    let board_string = parse_board_from_file("src/data.txt");
    let origin_board: Vec<Vec<String>> = board_string
        .iter()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();

    let mut play_board = origin_board.clone();
    let mut guard_data = get_guard_locations(&play_board);
    let mut loop_counter = 0;
    let mut loop_creation_blockages: Vec<(usize, usize)> = Vec::new();

    while !guard_data.is_empty() {
        move_guard(
            &mut play_board,
            &mut guard_data,
            &mut loop_counter,
            &mut loop_creation_blockages,
        );
    }

    let traversed_count = count_guard_positions(&play_board);
    println!(
        "Guard positions traversed before leaving the board: {}",
        traversed_count
    );

    // PART 2: For each traversed position, see if inserting an obstruction results in an infinite loop.
    let guard_positions = get_all_traversed_guard_positions(&play_board);
    let mut total_infinite_blockages: Vec<(usize, usize)> = Vec::new();

    for (i, j) in guard_positions {
        // Clone the original board for a fresh simulation.
        let mut new_board = origin_board.clone();
        // Place an obstruction at (i, j).
        new_board[i][j] = "#".to_string();
        let mut new_guard_data = get_guard_locations(&new_board);

        // Reset local simulation state.
        loop_counter = 0;
        let mut local_blockages: Vec<(usize, usize)> = Vec::new();

        // Process the simulation until no guards remain or a blockage is recorded.
        while !new_guard_data.is_empty() {
            move_guard(
                &mut new_board,
                &mut new_guard_data,
                &mut loop_counter,
                &mut local_blockages,
            );
            if !local_blockages.is_empty() {
                break;
            }
        }
        // If a blockage was recorded, record the exact location where it was logged.
        if let Some(&_blockage_pos) = local_blockages.first() {
            total_infinite_blockages.push((i, j));
        }
    }

    println!(
        "Number of infinite loop blockages: {}",
        total_infinite_blockages.len()
    );
}
