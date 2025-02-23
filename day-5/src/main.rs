use std::collections::{HashMap, VecDeque};

fn create_adjacency_list_and_indegree_map(
    rules_pairs: Vec<&str>,
) -> (HashMap<i32, Vec<i32>>, HashMap<i32, usize>) {
    let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree_map: HashMap<i32, usize> = HashMap::new();

    for rule in rules_pairs {
        let pair = rule.split("|").collect::<Vec<&str>>();
        // Extract parent and child as integers
        let parent = pair[0].trim().parse::<i32>().unwrap();
        let child = pair[1].trim().parse::<i32>().unwrap();

        // Update adjacency list
        adjacency_list.entry(parent).or_default().push(child);

        // Update indegree map
        *in_degree_map.entry(child).or_default() += 1;
        in_degree_map.entry(parent).or_default(); // Ensure parent is in the map
    }

    (adjacency_list, in_degree_map)
}

fn topological_sort(
    adjacency_list: &HashMap<i32, Vec<i32>>,
    in_degree_map: &mut HashMap<i32, usize>,
) -> Vec<i32> {
    let mut zero_in_degree_queue: VecDeque<i32> = VecDeque::new();
    let mut sorted_order: Vec<i32> = Vec::new();

    for (&node, &in_degree) in in_degree_map.iter() {
        if in_degree == 0 {
            zero_in_degree_queue.push_back(node);
        }
    }

    while let Some(current_node) = zero_in_degree_queue.pop_front() {
        sorted_order.push(current_node);

        if let Some(neighbors) = adjacency_list.get(&current_node) {
            for &neighbor in neighbors {
                if let Some(in_degree) = in_degree_map.get_mut(&neighbor) {
                    *in_degree -= 1;
                    if *in_degree == 0 {
                        zero_in_degree_queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    sorted_order
}

fn get_valid_rules_pairs(all_rules_pairs: Vec<&str>, page_order: Vec<i32>) -> Vec<&str> {
    let mut valid_rules_pairs: Vec<&str> = Vec::new();

    for rule in all_rules_pairs {
        let pair = rule.split("|").collect::<Vec<&str>>();
        // Extract parent and child as integers
        let parent = pair[0].trim().parse::<i32>().unwrap();

        if page_order.contains(&parent) {
            valid_rules_pairs.push(rule);
        }
    }

    valid_rules_pairs
}

fn main() {
    let rules_string = std::fs::read_to_string("src/data.txt").unwrap();
    let all_rules_pairs = rules_string.split("\n").collect::<Vec<&str>>();
    let reports_string = std::fs::read_to_string("src/reports.txt").unwrap();
    let reports = reports_string.split("\n").collect::<Vec<&str>>();
    let mut count = 0;

    reports.iter().for_each(|report| {
        let page_order = report
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let valid_rules_pairs = get_valid_rules_pairs(all_rules_pairs.clone(), page_order.clone());
        let (adjacency_list, mut in_degree_map) =
            create_adjacency_list_and_indegree_map(valid_rules_pairs);

        let sorted_order = topological_sort(&adjacency_list, &mut in_degree_map);

        // map over the page order and ensure that all the pages are in the topological order
        count += validate_page_order(sorted_order, page_order);
    });
    println!("Middle element of all valid reports: {}", count);

    println!("Total valid reports: {}", count);
}

fn validate_page_order(sorted_order: Vec<i32>, page_order: Vec<i32>) -> i32 {
    // Track the previous page's position in the sorted order
    let mut prev_index = 0;
    // if page order is valid
    // find middle page and keep a running total of the sum

    for page in &page_order {
        // Find the position of the current page in the sorted order
        if let Some(index) = sorted_order.iter().position(|&x| x == *page) {
            // Ensure the current page's index is greater than or equal to the previous index
            if index < prev_index {
                println!("Page order is not valid! Page {} is out of order.", page);
                return 0;
            }
            prev_index = index; // Update the previous index
        } else {
            println!("Page {} is not found in the sorted order!", page);
            return 0;
        }
    }

    println!("Page order is valid!");
    // get middle element always odd o need ceiling if dvided by 2
    // math ceiling function to get the middle element

    let middle = (page_order.len() - 1) / 2;

    let middle_element = page_order[middle];
    middle_element
}
