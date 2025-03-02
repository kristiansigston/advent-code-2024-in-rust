use std::collections::HashMap;

fn parse_data_from_file(file_name: &str) -> String {
  return std::fs::read_to_string(file_name).unwrap();
}

fn main() {
  let file_data = parse_data_from_file("src/data.txt");
  let stone_data: Vec<usize> = file_data
    .split(" ")
    .filter(|s| !s.is_empty())
    .map(|s| s.trim().parse::<usize>().unwrap())
    .collect();

  let stones_map = create_stones_map(stone_data);
  let mut new_stones_map = stones_map.clone();

  new_stones_map = iterate_stones_map(&stones_map, 0);
  println!("{:?}", new_stones_map);
  println!(
    "{:?}",
    new_stones_map.iter().fold(0, |acc, (_, value)| acc + value)
  );
}

fn create_stones_map(stone_data: Vec<usize>) -> HashMap<usize, usize> {
  let mut stones_map: HashMap<usize, usize> = HashMap::new();
  stone_data.iter().for_each(|stone_number| {
    insert_key_or_increment_value(&mut stones_map, *stone_number, 1);
  });
  return stones_map;
}

fn insert_key_or_increment_value(
  updated_stone_data: &mut HashMap<usize, usize>,
  key: usize,
  value: usize,
) {
  *updated_stone_data.entry(key).or_insert(0) += value;
}

fn iterate_stones_map(
  stones_map: &HashMap<usize, usize>,
  counter: usize,
) -> HashMap<usize, usize> {
  if counter > 74 {
    return stones_map.clone();
  }
  let mut new_map = HashMap::new();

  for (key, value) in stones_map.iter() {
    if *key == 0 {
      insert_key_or_increment_value(&mut new_map, 1, *value);
    } else if key.to_string().len() % 2 == 0 {
      let stone_number_string = key.to_string();
      let half = stone_number_string.len() / 2;
      let left_stone = &stone_number_string[..half];
      let right_stone = &stone_number_string[half..];

      insert_key_or_increment_value(
        &mut new_map,
        left_stone.parse::<usize>().unwrap(),
        *value,
      );
      insert_key_or_increment_value(
        &mut new_map,
        right_stone.parse::<usize>().unwrap(),
        *value,
      );
    } else {
      insert_key_or_increment_value(&mut new_map, key * 2024, *value);
    }
  }
  println!("counter {:?}", counter);
  return iterate_stones_map(&new_map, counter + 1);
}
