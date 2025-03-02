use std::{collections::HashSet, env, fs};

fn parse_data_from_file(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = if args.len() > 1 { &args[1] } else { "data.txt" };

    let file_path = "src/";

    let file_path_name = format!("{}{}", file_path, file_name);

    let data = parse_data_from_file(&file_path_name);

    let rows = data.iter().map(|s| s.trim()).collect::<Vec<&str>>();

    let columns: Vec<Vec<char>> = rows.iter().map(|s| s.chars().collect()).collect();

    let mut coord_set = HashSet::new();
    let mut total_nines = 0;

    columns.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, cell)| {
            if cell == &'0' {
                find_next_number_from_current_coord(
                    &(i, j),
                    &columns,
                    &(i, j),
                    '0'.to_string().as_str(),
                    &mut coord_set,
                    &mut total_nines,
                );
            }
        });
    });
    println!(
        "The total number of 0 to end nines routes found is {:?}",
        coord_set.len()
    );
    println!(
        "The total number of unique routes to nines found is {:?}",
        total_nines
    );
}

fn find_next_number_from_current_coord(
    coords_of_zero: &(usize, usize),
    columns: &Vec<Vec<char>>,
    coords: &(usize, usize),
    current_number_in_sequence: &str,
    coord_set: &mut HashSet<String>,
    total_nines: &mut usize,
) {
    if current_number_in_sequence == "9" {
        // combine zero and ninth coords to string as te hashkey
        let combined_coords = format!(
            "{:?}{:?}{:?}{:?}",
            coords_of_zero.0, coords_of_zero.1, coords.0, coords.1
        );
        *total_nines += 1;

        coord_set.insert(combined_coords);
        return;
    }

    let next_number = (current_number_in_sequence.parse::<i32>().unwrap() + 1).to_string();
    let next_coords: Vec<(isize, isize)> = vec![
        (coords.0 as isize - 1, coords.1 as isize),
        (coords.0 as isize, coords.1 as isize + 1),
        (coords.0 as isize + 1, coords.1 as isize),
        (coords.0 as isize, coords.1 as isize - 1),
    ];
    next_coords.iter().for_each(|coord| {
        let (i, j) = coord;
        if *i < 0 || *j < 0 || *i >= columns.len() as isize || *j >= columns[0].len() as isize {
            return;
        }
        if columns[*i as usize][*j as usize] == next_number.parse::<char>().unwrap() {
            find_next_number_from_current_coord(
                coords_of_zero,
                columns,
                &((*i).try_into().unwrap(), (*j).try_into().unwrap()),
                &next_number,
                coord_set,
                total_nines,
            );
        }
    });
}
