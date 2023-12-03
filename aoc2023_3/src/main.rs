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

    let mut master: Vec<Vec<char>>= vec![];
    let mut original: Vec<String> = Vec::new();
    let mut part_total = 0;
    let mut part_numbers:Vec<i32> = Vec::new();


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

    // Iterate over the line
    for line in original{
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
                        part_numbers.push(result);
                        println!("{}", "Part ".to_string() + matched.as_str() + " valid" );
                    }
                    else {
                        println!("{}", "Part ".to_string() + matched.as_str() + " INVALID" );
                    }
                }
                //println!("{}", "number.extract().to_string()");
                // if let (match, match_info) = number.unwrap(); 
                // println!("{}", number.extract());
            }
        }

        
        row_num += 1;

    }

    println!("{}", "part total : ".to_string() + &part_total.to_string());
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