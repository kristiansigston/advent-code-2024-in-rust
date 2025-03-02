use std::{collections::HashSet, env, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

struct Grid {
    cells: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(rows: Vec<String>) -> Self {
        let cells: Vec<Vec<char>> = rows.iter().map(|s| s.chars().collect()).collect();
        let height = cells.len();
        let width = if height > 0 { cells[0].len() } else { 0 };

        Self {
            cells,
            height,
            width,
        }
    }

    fn is_in_bounds(&self, row: isize, col: isize) -> bool {
        row >= 0 && col >= 0 && row < self.height as isize && col < self.width as isize
    }

    fn get(&self, row: usize, col: usize) -> char {
        self.cells[row][col]
    }

    fn find_digit(&self, digit: char) -> Vec<Coord> {
        let mut result = Vec::new();
        for row in 0..self.height {
            for col in 0..self.width {
                if self.cells[row][col] == digit {
                    result.push(Coord { row, col });
                }
            }
        }
        result
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Path {
    start: Coord,
    end: Coord,
}

impl Path {
    fn new(start: Coord, end: Coord) -> Self {
        Self { start, end }
    }

    fn to_string(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.start.row, self.start.col, self.end.row, self.end.col
        )
    }
}

struct NumberPathFinder<'a> {
    grid: &'a Grid,
    unique_paths: HashSet<String>,
    total_paths: usize,
}

impl<'a> NumberPathFinder<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            unique_paths: HashSet::new(),
            total_paths: 0,
        }
    }

    fn find_all_paths(&mut self) {
        // Find all '0' positions
        let zeros = self.grid.find_digit('0');

        // Start DFS from each zero
        for zero_pos in zeros {
            self.dfs(zero_pos, zero_pos, '0');
        }
    }

    fn dfs(&mut self, start: Coord, current: Coord, current_digit: char) {
        // If we've reached '9', we've found a complete path
        if current_digit == '9' {
            let path_key = Path::new(start, current).to_string();
            self.unique_paths.insert(path_key);
            self.total_paths += 1;
            return;
        }

        // Calculate next digit
        let next_digit = char::from_digit((current_digit as u8 - b'0' + 1) as u32, 10).unwrap();

        // Check all adjacent cells for next digit
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // right, down, left, up

        for (dr, dc) in directions {
            let next_row = current.row as isize + dr;
            let next_col = current.col as isize + dc;

            if self.grid.is_in_bounds(next_row, next_col) {
                let next_row = next_row as usize;
                let next_col = next_col as usize;

                if self.grid.get(next_row, next_col) == next_digit {
                    self.dfs(
                        start,
                        Coord {
                            row: next_row,
                            col: next_col,
                        },
                        next_digit,
                    );
                }
            }
        }
    }

    fn unique_path_count(&self) -> usize {
        self.unique_paths.len()
    }

    fn total_path_count(&self) -> usize {
        self.total_paths
    }
}

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
    let file_path = format!("src/{}", file_name);

    let data = parse_data_from_file(&file_path);
    let grid = Grid::new(data);

    let mut finder = NumberPathFinder::new(&grid);
    finder.find_all_paths();

    println!(
        "The total number of 0 to end nines routes found is {}",
        finder.unique_path_count()
    );
    println!(
        "The total number of unique routes to nines found is {}",
        finder.total_path_count()
    );
}
