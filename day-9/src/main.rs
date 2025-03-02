use std::{fs, vec};

fn parse_data_from_file(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn main() {
    let data_string = parse_data_from_file("src/data.txt")[0].clone();
    // let data_string = parse_data_from_file("src/example.txt")[0].clone();
    let mut number_data: Vec<i32> = vec![];

    data_string.chars().enumerate().for_each(|(i, c)| {
        let count = c.to_digit(10).unwrap() as usize;
        if i & 1 == 0 {
            let number = (i / 2) as i32;
            number_data.extend(vec![number; count]);
        } else {
            number_data.extend(vec![-1; count]);
        }
    });

    let mut start_pointer = 0;
    let mut end_pointer = number_data.len();
    loop {
        while start_pointer < end_pointer {
            if number_data[start_pointer] == -1 {
                break;
            }
            start_pointer += 1;
        }

        while start_pointer < end_pointer {
            end_pointer -= 1;
            if number_data[end_pointer] != -1 {
                break;
            }
        }

        if start_pointer == end_pointer {
            break;
        }

        number_data.swap(start_pointer, end_pointer);
    }

    let check_sum = calculate_checksum(&number_data);

    println!("The check sum is {:?}", check_sum);

    do_part_two(data_string);
}

#[derive(Debug)]
struct FileData {
    id: isize,
    length: u32,
    start: usize,
}

#[derive(Debug)]
struct FreeSpace {
    length: u32,
    start: usize,
}

// priority queue

fn do_part_two(data_string: String) {
    let mut file_data: Vec<FileData> = Vec::new();
    let mut free_space_data: Vec<FreeSpace> = Vec::new();
    let mut current_start = 0;
    data_string.chars().enumerate().for_each(|(i, c)| {
        if i % 2 == 0 {
            let length = c.to_digit(10).unwrap();
            let id = (i / 2) as isize;
            file_data.push(FileData {
                id,
                length,
                start: current_start,
            });
            current_start += length as usize;
        } else {
            let free_space = c.to_digit(10).unwrap();
            free_space_data.push(FreeSpace {
                length: free_space,
                start: current_start,
            });
            current_start += free_space as usize;
        }
    });
    // access file data
    println!("free_space_data: {:?}", free_space_data);

    // reverse file_data

    file_data.reverse();
    file_data.iter_mut().for_each(|file| {
        let mut free_space_index = 0;
        while free_space_index < free_space_data.len() // take chris' advice
            && file.length > free_space_data[free_space_index].length
        // fail case
        {
            free_space_index += 1;
        }
        if free_space_index < free_space_data.len() {
            let free_space_start = free_space_data[free_space_index].start;
            if free_space_start > file.start {
                return;
            }
            file.start = free_space_start;
            if free_space_data[free_space_index].length == file.length {
                free_space_data.remove(free_space_index);
            } else {
                free_space_data[free_space_index].start += file.length as usize;
                free_space_data[free_space_index].length -= file.length;
            }
        }
    });
    // file_data.reverse();
    println!("file_data: {:?}", file_data);

    // the check some is the start position multiplied by the id for each
    // file in the file_data remembering to add one to the index for each file greater than 1
    let check_sum = calcualte_file_checksum(&file_data);

    // let check_sum = calculate_checksum(
    //     &file_data
    //         .iter()
    //         .map(|file| file.length as i32)
    //         .collect::<Vec<i32>>(),
    // );

    println!("The check sum is {:?}", check_sum);
}
// is it better to hash the free space as vectors for it's size
// a hash of free space size and then a mutable vector of the start index.
// if we fit a file in the free space and we remove the vector from the hash but if we have a remainder we would need to insert the remainder into the free space hash for that size
// a file needs to fit in the first avaialbe space large enough even if it's too large
// if a file is too large it is not moved
// if a file is too small it is moved to the first available space large enough

// most effiecint insert into a sorted vector

fn calculate_checksum(data: &[i32]) -> i64 {
    data.iter().enumerate().fold(0i64, |acc, (index, &s)| {
        // println!("index {:?} s {:?}, {:?}", index, s, acc);
        if s != -1 {
            acc + index as i64 * s as i64
        } else {
            acc
        }
    })
}

// iterate over the file data and for each

fn calcualte_file_checksum(data: &[FileData]) -> i64 {
    data.iter().enumerate().fold(0i64, |acc, (index, file)| {
        // println!("index {:?} s {:?}, {:?}", index, s, acc);
        let mut fileacc = 0;
        for i in 0..file.length {
            fileacc += (file.start + i as usize) as i64 * file.id as i64;
        }
        acc + fileacc
    })
}
