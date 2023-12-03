use std::error::Error;
use std::fs::File;
use std::fmt::Display;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>>{

    // read each line, regex (\d+) group, iterate over groups?
    // we need index of each number
    // we calculate row and start/end (inclusive)
    // function will check the master array and see if there are any special chars in the surrounding cells
    let re_number:Regex = Regex::new(r"(\d+)").unwrap();

    let mut master: Vec<Vec<char>>= vec![vec![]];
    let mut original: Vec<String> = Vec::new();


    let file_path = "input_debug.txt";
    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let orig_line = line?;
        if let captures = re_number.captures_iter(&orig_line) {
            // Access captured groups using captures
            for number in captures{
                //println!("{}", number.extract().to_string());
            }
        }
        master.push(orig_line.chars().collect());
        original.push(orig_line);

    }

    println!("{}", "total power: ".to_string());
    Ok(())
}


fn is_part_number(master:Vec<Vec<char>>, row:i32, start:i32, len: i32) -> bool {

    return true;
}