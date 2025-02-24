use std::fs;

fn parse_data_from_file(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn main() {
    let data = parse_data_from_file("src/data.txt");
    // let data = parse_data_from_file("src/example.txt");
    let rows = data.iter().map(|s| s.trim()).collect::<Vec<&str>>();
    // split rows into array of chars
    let columns: Vec<Vec<char>> = rows.iter().map(|s| s.chars().collect()).collect();
    // create a vector of vectors the same rows and column size as the rows and columns above
    // print each row item on its own line
    for row in columns.iter() {
        println!("{:?}", row);
    }
    let mut anti_pode_container = vec![vec!['.'; columns.len()]; rows.len()];

    // tracked differences should be the same size as the columns and rows
    // let mut tracked_differences = std::collections::HashMap::new();
    // craete an object with a key value of a char and an array of coords
    // append the extra vector where the char key already exists
    let mut char_coords = std::collections::HashMap::new();
    for (i, row) in columns.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c != '.' {
                char_coords.entry(c).or_insert(vec![]).push((i, j));
            }
        }
    }
    println!("{:?}", char_coords);
    // reduce over the hashmap and get the differences in x-y space between all the coords
    // + the differences and minus the differences.
    // for each difference that is within the map boundary add 1 to the reduce total

    let mut seen = std::collections::HashSet::new();
    for (_, coords) in char_coords.iter() {
        if coords.len() == 1 {
            continue;
        }

        coords.iter().for_each(|c| {
            seen.insert((c.0 as isize, c.1 as isize));
        });

        let op_count = coords.len();
        for i in 0..op_count {
            for j in 0..op_count {
                if i == j {
                    continue;
                }

                let (xi, yi) = (coords[i].0 as isize, coords[i].1 as isize);
                let (xj, yj) = (coords[j].0 as isize, coords[j].1 as isize);

                let mut anti_node_x: isize = xj;
                let mut anti_node_y: isize = yj;

                loop {
                    anti_node_x = anti_node_x + xj - xi;
                    anti_node_y = anti_node_y + yj - yi;
                    if anti_node_x < 0
                        || anti_node_x >= rows.len().try_into().unwrap()
                        || anti_node_y < 0
                        || anti_node_y >= columns.len().try_into().unwrap()
                    {
                        break;
                    }
                    // add a hash character to the map to keep track of the differences
                    anti_pode_container[anti_node_x as usize][anti_node_y as usize] = '#';
                    seen.insert((anti_node_x, anti_node_y));
                    // print each array item on its own line
                }
            }
        }
    }
    println!("Total diff: {}", seen.len());
    // print each array item on its own line
    // for row in anti_pode_container.iter() {
    //     println!("{:?}", row);
    // }
    // for row in seen.iter() {
    //     println!("{:?}", row);
    // }
    // println!("total diff: {}", total_diff);
}
