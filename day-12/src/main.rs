use eframe::egui::{self, Color32, Stroke};
use rand::Rng;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
struct LetterArea {
    letter: char,
    group_id: usize, // Added to distinguish separate groups
    color: Color32,
    coords: Vec<(i32, i32)>,
    visited: bool,
    fences: Vec<(i32, i32, Direction)>, // Add fence tracking
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TimeUnit {
    Milli,
    Micro,
    Nano,
}

#[derive(Clone, Debug)]
struct GridState {
    letter_areas: Vec<LetterArea>, // Changed from HashMap to Vec to store multiple groups
    current_area: Option<usize>,   // Now stores group_id instead of letter as letters are not unique
    step_count: usize,
    next_group_id: usize,                  // Added to generate unique group IDs
    current_fence_pos: Option<(i32, i32)>, // Track current fence position
    current_fence_dir: Option<Direction>,
    total_score: usize, // Add field for the multiplied total
}

#[derive(Clone, Debug, PartialEq)]
enum ProcessingState {
    Scanning,     // Moving through grid
    FloodFilling, // Found letter, expanding area
    Fencing,      // Drawing fences
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct MyApp {
    grid: Vec<Vec<char>>,
    state: GridState,
    current_col: i32,
    current_row: i32,
    last_update: Instant,
    animation_speed: Duration,
    processing_state: ProcessingState,
    speed_multiplier: f32,
    cursor_pos: (i32, i32),
    cursor_color: Color32,
    steps_per_frame: usize,
    time_unit: TimeUnit,
    base_duration: u64,
}

impl MyApp {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self {
            grid,
            state: GridState {
                letter_areas: Vec::new(),
                current_area: None,
                step_count: 0,
                next_group_id: 0,
                current_fence_pos: None,
                current_fence_dir: None,
                total_score: 0,
            },
            time_unit: TimeUnit::Milli,
            base_duration: 5000,
            current_col: 0,
            current_row: 0,
            last_update: Instant::now(),
            animation_speed: Duration::from_nanos(100),
            processing_state: ProcessingState::Scanning, // Start with scanning
            cursor_pos: (0, 0),
            cursor_color: Color32::from_rgb(255, 165, 0),
            speed_multiplier: 1.0,
            steps_per_frame: 1,
        }
    }

    fn update_animation_speed(&mut self) {
        let base = self.base_duration as f64 * (1.0 / self.speed_multiplier as f64);
        self.animation_speed = match self.time_unit {
            TimeUnit::Milli => Duration::from_millis(base as u64),
            TimeUnit::Micro => Duration::from_micros(base as u64),
            TimeUnit::Nano => Duration::from_nanos(base as u64),
        };
    }

    fn calculate_total_score(&self) -> usize {
        self.state
            .letter_areas
            .iter()
            .map(|area| area.coords.len() * area.fences.len())
            .sum()
    }

    fn restart(&mut self) {
        self.state = GridState {
            letter_areas: Vec::new(),
            current_area: None,
            step_count: 0,
            next_group_id: 0,
            current_fence_pos: None,
            current_fence_dir: None,
            total_score: 0,
        };
        self.current_col = 0;
        self.current_row = 0;
        self.last_update = Instant::now();
        self.processing_state = ProcessingState::Scanning;
        self.cursor_pos = (0, 0);
    }

    fn darken_color(&self, color: Color32) -> Color32 {
        let factor = 0.6; // Makes color 40% darker
        Color32::from_rgb(
            (color.r() as f32 * factor) as u8,
            (color.g() as f32 * factor) as u8,
            (color.b() as f32 * factor) as u8,
        )
    }

    fn generate_distinct_color(&self) -> Color32 {
        let mut rng = rand::thread_rng();
        // Use more saturated base values to avoid light colors
        Color32::from_rgb(
            rng.gen_range(50..200), // Lower max to avoid light colors
            rng.gen_range(50..200),
            rng.gen_range(50..200),
        )
    }

    // Remove scan_step as it's no longer needed
    fn process_next_step(&mut self) {
        match self.processing_state {
            ProcessingState::Scanning => {
                let pos = (self.current_row, self.current_col);
                let current_char = self.grid[pos.0 as usize][pos.1 as usize];

                if current_char.is_alphabetic() {
                    // Check if position is connected to existing group
                    let connected_group = self
                        .state
                        .letter_areas
                        .iter()
                        .find(|area| {
                            area.letter == current_char
                                && self.is_adjacent_to_area(&area.coords, pos)
                        })
                        .map(|area| area.group_id);

                    if let Some(group_id) = connected_group {
                        // Add to existing group
                        if let Some(area) = self
                            .state
                            .letter_areas
                            .iter_mut()
                            .find(|a| a.group_id == group_id)
                        {
                            if !area.coords.contains(&pos) {
                                area.coords.push(pos);
                            }
                        }
                    } else {
                        // Create new group
                        let color = self.generate_distinct_color();
                        let group_id = self.state.next_group_id;
                        self.state.next_group_id += 1;

                        self.state.letter_areas.push(LetterArea {
                            letter: current_char,
                            group_id,
                            color,
                            coords: vec![pos],
                            visited: false,
                            fences: vec![],
                        });
                        self.state.current_area = Some(group_id);
                        self.processing_state = ProcessingState::FloodFilling;
                    }
                }
                self.advance_position();
            }
            ProcessingState::FloodFilling => {
                if let Some(current_group) = self.state.current_area {
                    let mut found_new = false;
                    // First get the area's information
                    if let Some(area) = self
                        .state
                        .letter_areas
                        .iter()
                        .find(|a| a.group_id == current_group)
                    {
                        let coords = area.coords.clone();
                        let current_char = area.letter;
                        'outer: for (row, col) in coords {
                            for (dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                                let nr = row + dr;
                                let nc = col + dc;

                                if nr >= 0
                                    && nr < self.grid.len() as i32
                                    && nc >= 0
                                    && nc < self.grid[0].len() as i32
                                    && self.grid[nr as usize][nc as usize] == current_char
                                {
                                    let new_pos = (nr, nc);
                                    // Then modify it
                                    if let Some(area) = self
                                        .state
                                        .letter_areas
                                        .iter_mut()
                                        .find(|a| a.group_id == current_group)
                                    {
                                        if !area.coords.contains(&new_pos) {
                                            area.coords.push(new_pos);
                                            found_new = true;
                                            self.cursor_pos = new_pos;
                                            break 'outer;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if !found_new {
                        self.processing_state = ProcessingState::Scanning;
                        self.state.current_area = None;
                    }
                }
            }
            ProcessingState::Fencing => self.fence_step(),
            ProcessingState::Complete => return,
        }

        // Check if scanning is complete
        if self.processing_state == ProcessingState::Scanning
            && self.current_row == 0
            && self.current_col == 0
        {
            self.processing_state = ProcessingState::Fencing;
        }
    }

    fn fence_step(&mut self) {
        if let Some(current_group) = self.state.current_area {
            if let Some(area) = self
                .state
                .letter_areas
                .iter()
                .find(|a| a.group_id == current_group)
            {
                if !area.visited {
                    let coords = area.coords.clone();

                    // Get current position or start from first coordinate
                    let (row, col) = self.state.current_fence_pos.unwrap_or_else(|| coords[0]);
                    let current_dir = self.state.current_fence_dir.unwrap_or(Direction::North);

                    if self.needs_fence((row, col), current_dir, &coords, current_group) {
                        // Add fence and increment count
                        if let Some(area) = self
                            .state
                            .letter_areas
                            .iter_mut()
                            .find(|a| a.group_id == current_group)
                        {
                            if !area.fences.contains(&(row, col, current_dir)) {
                                area.fences.push((row, col, current_dir));
                                self.state.step_count += 1;

                                let area_size = area.coords.len();
                                let fence_count = area.fences.len();
                                let area_score = area_size * fence_count;

                                // Update total score considering all areas
                                self.state.total_score = self.calculate_total_score();
                            }
                        }
                    }

                    // Move to next direction or position
                    match current_dir {
                        Direction::North => self.state.current_fence_dir = Some(Direction::East),
                        Direction::East => self.state.current_fence_dir = Some(Direction::South),
                        Direction::South => self.state.current_fence_dir = Some(Direction::West),
                        Direction::West => {
                            self.state.current_fence_dir = Some(Direction::North);
                            let pos_index =
                                coords.iter().position(|&p| p == (row, col)).unwrap_or(0);
                            if pos_index + 1 < coords.len() {
                                self.state.current_fence_pos = Some(coords[pos_index + 1]);
                            } else {
                                // Finished this area
                                if let Some(area) = self
                                    .state
                                    .letter_areas
                                    .iter_mut()
                                    .find(|a| a.group_id == current_group)
                                {
                                    area.visited = true;
                                }
                                self.state.current_area = None;
                                self.state.current_fence_pos = None;
                                self.state.current_fence_dir = None;
                            }
                        }
                    }
                }
            }
        } else {
            // Find next unvisited group
            let next_group = self
                .state
                .letter_areas
                .iter()
                .find(|area| !area.visited)
                .map(|area| area.group_id);

            self.state.current_area = next_group;
            if self.state.current_area.is_none() {
                self.processing_state = ProcessingState::Complete;
            }
        }
    }

    fn needs_fence(
        &self,
        pos: (i32, i32),
        dir: Direction,
        coords: &[(i32, i32)],
        current_group: usize,
    ) -> bool {
        let (row, col) = pos;
        let (dr, dc) = match dir {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        };
        let nr = row + dr;
        let nc = col + dc;
        let next_pos = (nr, nc);

        // Must be part of our area
        if !coords.contains(&pos) {
            return false;
        }

        // Next position is outside grid
        if nr < 0 || nr >= self.grid.len() as i32 || nc < 0 || nc >= self.grid[0].len() as i32 {
            return true;
        }

        // Next position is empty space
        if !coords.contains(&next_pos)
            && !self
                .state
                .letter_areas
                .iter()
                .any(|area| area.coords.contains(&next_pos))
        {
            return true;
        }

        // Next position is part of different letter group
        if !coords.contains(&next_pos) && self.is_adjacent_to_other_group(next_pos, current_group) {
            return true;
        }

        false
    }

    fn advance_position(&mut self) {
        self.current_col += 1;
        if self.current_col >= self.grid[0].len() as i32 {
            self.current_col = 0;
            self.current_row += 1;
            if self.current_row >= self.grid.len() as i32 {
                self.current_row = 0;
                if self.processing_state == ProcessingState::Scanning {
                    self.processing_state = ProcessingState::Fencing;
                }
            }
        }
        self.cursor_pos = (self.current_row, self.current_col);
    }

    fn get_cell_color(&self, row: i32, col: i32) -> (Color32, Vec<Direction>) {
        let pos = (row, col);

        // Only show cursor if not complete
        if pos == self.cursor_pos && self.processing_state != ProcessingState::Complete {
            return (self.cursor_color, vec![]);
        }

        for area in &self.state.letter_areas {
            if area.coords.contains(&pos) {
                let fences = area
                    .fences
                    .iter()
                    .filter(|&&(r, c, _)| r == row && c == col)
                    .map(|&(_, _, dir)| dir)
                    .collect();
                return (area.color, fences);
            }
        }

        (Color32::from_gray(240), vec![])
    }

    fn calculate_grid_size(&self, available_size: egui::Vec2) -> f32 {
        let grid_rows = self.grid.len();
        let grid_cols = self.grid[0].len();

        // FIXED: Add minimum cell size and more padding
        let min_cell_size = 10.0;
        let padding = 60.0; // Increased from 40.0

        // Calculate sizes to fit either dimension, with padding
        let height_constrained =
            ((available_size.y - padding) / grid_rows as f32).max(min_cell_size);
        let width_constrained =
            ((available_size.x - padding) / grid_cols as f32).max(min_cell_size);

        // Use the smaller of the two to ensure grid fits
        height_constrained.min(width_constrained)
    }

    fn is_adjacent_to_area(&self, coords: &[(i32, i32)], pos: (i32, i32)) -> bool {
        coords.iter().any(|&(row, col)| {
            let dr = (row - pos.0).abs();
            let dc = (col - pos.1).abs();
            (dr == 1 && dc == 0) || (dr == 0 && dc == 1)
        })
    }
    fn is_adjacent_to_other_group(&self, pos: (i32, i32), current_group: usize) -> bool {
        self.state
            .letter_areas
            .iter()
            .any(|area| area.group_id != current_group && area.coords.contains(&pos))
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.animation_speed {
            for _ in 0..self.steps_per_frame {
                self.process_next_step();
                if self.processing_state == ProcessingState::Complete {
                    break;
                }
            }
            self.last_update = now;
        }
        egui::SidePanel::right("stats_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Controls");
                ui.add_space(8.0);
                if ui.button("⟳ Restart").clicked() {
                    self.restart();
                }
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.label("Base Duration:");
                    if ui
                        .add(egui::Slider::new(&mut self.base_duration, 1..=5000))
                        .changed()
                    {
                        self.update_animation_speed();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Time Unit:");
                    if ui
                        .radio_value(&mut self.time_unit, TimeUnit::Milli, "Milliseconds")
                        .clicked()
                        || ui
                            .radio_value(&mut self.time_unit, TimeUnit::Micro, "Microseconds")
                            .clicked()
                        || ui
                            .radio_value(&mut self.time_unit, TimeUnit::Nano, "Nanoseconds")
                            .clicked()
                    {
                        self.update_animation_speed();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Speed Multiplier:");
                    if ui
                        .add(egui::Slider::new(&mut self.speed_multiplier, 0.1..=100.0).text("x"))
                        .changed()
                    {
                        self.animation_speed =
                            Duration::from_nanos((1.0 / self.speed_multiplier) as u64);
                    }
                });

                // Steps per frame control
                ui.horizontal(|ui| {
                    ui.label("Steps:");
                    ui.add(egui::Slider::new(&mut self.steps_per_frame, 1..=1000).text("/frame"));
                });
                ui.separator();
                ui.heading("Statistics");
                ui.add_space(8.0);

                ui.label(format!("State: {:?}", self.processing_state));
                ui.add_space(4.0);

                ui.label(format!("Total Fences: {}", self.state.step_count));
                ui.add_space(4.0);

                // Current Area Stats
                if let Some(current_group) = self.state.current_area {
                    if let Some(area) = self
                        .state
                        .letter_areas
                        .iter()
                        .find(|a| a.group_id == current_group)
                    {
                        ui.separator();
                        ui.label("Current Area:");
                        ui.label(format!("Letter: '{}'", area.letter));
                        ui.label(format!("Group: {}", current_group));
                        ui.label(format!("Size: {}", area.coords.len()));
                        ui.label(format!("Fences: {}", area.fences.len()));
                        ui.label(format!("Score: {}", area.coords.len() * area.fences.len()));
                    }
                }

                ui.separator();
                ui.heading("Total Score");
                ui.label(format!("{}", self.state.total_score));
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let cell_size = self.calculate_grid_size(available_size);
            let padding = 2.0;
            ui.vertical(|ui| {
                ui.heading("Grid View");

                egui::Grid::new("my_grid")
                    .spacing([padding, padding]) // Same spacing for both dimensions
                    .min_col_width(cell_size)
                    .min_row_height(cell_size) // Add this to ensure row height matches column width
                    .show(ui, |ui| {
                        for (row_idx, row) in self.grid.iter().enumerate() {
                            for (col_idx, &cell) in row.iter().enumerate() {
                                let (rect, _response) = ui.allocate_exact_size(
                                    egui::vec2(cell_size, cell_size),
                                    egui::Sense::click(),
                                );

                                if ui.is_rect_visible(rect) {
                                    // Get both color and fence information
                                    let (bg_color, fences) =
                                        self.get_cell_color(row_idx as i32, col_idx as i32);

                                    // Draw cell background
                                    ui.painter().rect_filled(rect, 4.0, bg_color);

                                    // Draw cell border
                                    ui.painter().rect_stroke(
                                        rect,
                                        4.0,
                                        Stroke::new(1.0, Color32::from_gray(200)),
                                        egui::StrokeKind::Inside,
                                    );

                                    // Draw fences as thick black lines
                                    let fence_color = self.darken_color(bg_color);
                                    let fence_stroke = Stroke::new(2.0, fence_color);

                                    if fences.contains(&Direction::North) {
                                        ui.painter().line_segment(
                                            [rect.left_top(), rect.right_top()],
                                            fence_stroke,
                                        );
                                    }
                                    if fences.contains(&Direction::East) {
                                        ui.painter().line_segment(
                                            [rect.right_top(), rect.right_bottom()],
                                            fence_stroke,
                                        );
                                    }
                                    if fences.contains(&Direction::South) {
                                        ui.painter().line_segment(
                                            [rect.left_bottom(), rect.right_bottom()],
                                            fence_stroke,
                                        );
                                    }
                                    if fences.contains(&Direction::West) {
                                        ui.painter().line_segment(
                                            [rect.left_top(), rect.left_bottom()],
                                            fence_stroke,
                                        );
                                    }

                                    // Draw cell text
                                    let text_size = (cell_size * 0.6).min(16.0); // Scale text with cell, max 16px
                                    ui.painter().text(
                                        rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        cell.to_string(),
                                        egui::FontId::proportional(text_size),
                                        if bg_color == self.cursor_color {
                                            Color32::WHITE
                                        } else {
                                            Color32::BLACK
                                        },
                                    );
                                }
                            }
                            ui.end_row();
                        }
                    });
            });
        });
    }
}
fn main() -> eframe::Result {
    std::env::set_var("RUST_LOG", "error");
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    env_logger::init();

    let grid = load_grid_from_file("src/data.txt");
    println!("Loaded grid: {:?}", grid); // Debug print
    let app = MyApp::new(grid.clone());

    let min_cell_size = 40.0; // Try a larger minimum cell size
    let padding = 100.0; // Add more padding

    let width = (grid[0].len() as f32 * min_cell_size) + 240.0;
    let height = (grid.len() as f32 * min_cell_size) + 80.0;

    let options = eframe::NativeOptions {
        vsync: false,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([width.max(800.0), height.max(600.0)]) // Ensure a larger initial size
            .with_min_inner_size([width.max(800.0), height.max(600.0)]) // Set minimum size to fit content
            .with_transparent(false)
            .with_decorations(true),
        ..Default::default()
    };

    eframe::run_native(
        "Grid Display",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            cc.egui_ctx.set_pixels_per_point(1.5);
            Ok(Box::new(app))
        }),
    )
}

fn load_grid_from_file(path: &str) -> Vec<Vec<char>> {
    use std::fs;
    match fs::read_to_string(path) {
        Ok(content) => {
            let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
            println!("Loaded grid: {:?}", grid); // Debug print
            if grid.is_empty() || grid[0].is_empty() {
                // Return default grid if file was empty
                return vec![
                    vec!['A', 'B', 'C'],
                    vec!['D', 'E', 'F'],
                    vec!['G', 'H', 'I'],
                ];
            }
            grid
        }
        Err(_) => vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ],
    }
}
