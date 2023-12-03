use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>>{

    // read each line, regex (\d+) group, iterate over groups?
    // we need index of each number
    // we calculate row and start/end (inclusive)
    // function will check the master array and see if there are any special chars in the surrounding cells
    let re_number:Regex = Regex::new(r"\d+").unwrap();
    let re_gear:Regex = Regex::new( r"\*").unwrap();

    let mut master: Vec<Vec<char>>= vec![];
    let mut original: Vec<String> = Vec::new();
    let mut part_total = 0;
    let mut part_numbers:Vec<(i32,i32,i32,i32)> = Vec::new();
    let mut gear_total: i64 = 0;


    let file_path = "input.txt";
    // Open the file
    let file = File::open(file_path)?;
    let mut row_num = 0;

    // Build the dataset
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let orig_line = line?;
            master.push(orig_line.chars().collect());
            original.push(orig_line);
    }

    // Part 1
    for line in &original{
        let captures = re_number.captures_iter(&line);
        // Access captured groups using captures
        for capture in captures{
            for group in capture.iter(){
                if let Some(matched) = group {
                    let start = matched.start() as i32;
                    let end = matched.end() as i32;
                    if is_part_number(&master, row_num, start, end) {
                        let result = matched.as_str().parse::<i32>().unwrap();
                        part_total += result;
                        part_numbers.push((start, end - 1, row_num, result));
                        println!("{}", "Part ".to_string() + matched.as_str() + " valid" );
                    }
                    else {
                        println!("{}", "Part ".to_string() + matched.as_str() + " INVALID" );
                    }
                }
            }
        }
        row_num += 1;
    }

    // Part 2
    row_num = 0; // reset
    for line in &original{
        let captures = re_gear.captures_iter(&line);
        // Access captured groups using captures
        for capture in captures{
            for group in capture.iter(){
                if let Some(matched) = group {
                    let start = matched.start() as i32;
                    gear_total += get_gear_ratio(row_num, start, &part_numbers) as i64;
                }
            }
        }
        row_num += 1;
    }

    println!("{}", "part total : ".to_string() + &part_total.to_string());
    println!("{}", "gear total : ".to_string() + &gear_total.to_string());
    Ok(())
}


fn is_part_number(master:&Vec<Vec<char>>, row:i32, start:i32, end: i32) -> bool {
    let special_chars:Vec<char> = vec!['@','#','$','%','&','*','-','=','+','/'];
    let start_min = if start > 0 {start - 1} else {start};
    let end_max = if end < master[row as usize].len() as i32 - 1 {end + 1} else {end};
    // get row above (check if index is 0)
    if row > 0 {
        let test_chars = &master[(row - 1) as usize][start_min as usize..end_max as usize];
        if test_chars.iter().any(|&element| special_chars.contains(&element)) {
            return true;
        }
    }
    // get row below (check if index = len-1)
    if row < master.len() as i32 - 1 {
        let test_chars = &master[(row + 1) as usize][start_min as usize..end_max as usize];
        if test_chars.iter().any(|&element| special_chars.contains(&element)) {
            return true;
        }
    }
    // get left char (check start)
    if start > 0 {
        let test_char = &master[row as usize][(start - 1) as usize];
        if special_chars.contains(test_char) {
            return true;
        }
    }
    // get right char (check end)
    if end < master[row as usize].len() as i32 {
        let test_char = &master[row as usize][(end) as usize];
        if special_chars.contains(test_char) {
            return true;
        }
    }
    return false;
}

fn get_gear_ratio(gear_row:i32, gear_index:i32, numbers:&Vec<(i32,i32,i32,i32)>) -> i32 {
    // check their surroundings for numbers
    // if they have 2 numbers they are a gear ratio
    // we loop through our list of gears and check for one that has the right position
    // iterate over vector and collect the numbers

    // check the list of numbers for how many match
    // if the number is two, return gear ratio
    let mut gear_ratio = 0;
    let matching_numbers:Vec<(i32,i32,i32,i32)> = numbers.iter().filter(|&number| test_number_for_gear(gear_row, gear_index, number)).cloned().collect();
    if matching_numbers.len() == 2 {
        gear_ratio = matching_numbers[0].3 * matching_numbers[1].3;
    }
    return gear_ratio;
}

fn test_number_for_gear(gear_row:i32, gear_index:i32, part_number:&(i32,i32,i32,i32)) -> bool {
    // Part_number = (start, end, row, value)
        return  (part_number.2 - 1 <= gear_row &&  part_number.2 + 1 >= gear_row)
                && (part_number.0 - 1 <= gear_index && part_number.1 + 1 >= gear_index);
}