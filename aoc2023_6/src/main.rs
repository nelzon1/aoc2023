use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    
    const DEBUG:bool = false;
    let re_number:Regex = Regex::new(r"\d+").unwrap();

    let mut original: Vec<String> = Vec::new();

    let file_path = if DEBUG {"input_debug.txt"} else {"input.txt"};
    // Open the file
    let file = File::open(file_path)?;

    // Build the dataset
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let orig_line = line?;
            original.push(orig_line);
    }
    let mut times:Vec<i64> = Vec::new();
    let mut records:Vec<i64> = Vec::new();
    let mut race_wins:Vec<i64> = Vec::new();

    for capture in re_number.captures_iter(&original[0]){
        for group in capture.iter(){
            if let Some(matched) = group {
                times.push(matched.as_str().parse::<i64>().unwrap())
            }
        }
    }
    for capture in re_number.captures_iter(&original[1]){
        for group in capture.iter(){
            if let Some(matched) = group {
                records.push(matched.as_str().parse::<i64>().unwrap())
            }
        }
    }

    for race in times.iter().zip(&records){
        let mut win_count = 0;
        for i in 1..*race.0 {
            if i * (race.0 - i) > *race.1 {
                win_count += 1;
            }
        }
        race_wins.push(win_count);
    }

    println!("{}", "part1: ".to_string() + &race_wins.iter().fold(1, |acc, &x| acc * x).to_string());


    Ok(())
}
