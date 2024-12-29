fn main() {
    let file_data = std::fs::read_to_string("src/data.txt").unwrap();
    let contents = format!("do(){}don't()", file_data.trim());
    get_all_muls(contents.clone());
    get_all_muls_with_enablers(contents.clone());
}

fn get_all_muls(contents: String) {
    extract_sum_of_muls(contents.clone());
}

fn get_all_muls_with_enablers(contents: String) {
    // let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let enabled_muls = regex::Regex::new(r"(?s)do\(\)(.*?)don't\(\)").unwrap();
    let result: String = enabled_muls
        .captures_iter(&contents)
        .map(|cap| cap[1].to_string()) // Extract and convert capture group to String
        .collect::<Vec<String>>() // Collect matches into a Vec<String>
        .join(" ");

    extract_sum_of_muls(result);
    // let result: String = re
    //     .captures_iter(enabledMuls.clone())
    //     .map(|cap| cap[1].to_string()) // Extract and convert capture group to String
    //     .collect::<Vec<String>>() // Collect matches into a Vec<String>
    //     .join(" ");
    // extractSumOfMuls(result);
}

fn extract_sum_of_muls(contents: String) {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let total: i32 = re
        .captures_iter(&contents)
        .map(|cap| {
            let num1 = cap[1].parse::<i32>().unwrap();
            let num2 = cap[2].parse::<i32>().unwrap();
            num1 * num2
        })
        .sum();

    println!("Total sum of products: {}", total);
}
