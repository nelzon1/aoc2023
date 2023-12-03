use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {

    // read the file line by line
    // for each line we need to:
    // create a variable for each color
    // sum to each variable and parse by ; and ,
    // regex would be a good way
    // if any of the variables is too big, add the game id to the list
    const RED_LIMIT:i32 = 12;
    const GREEN_LIMIT:i32 = 13;
    const BLUE_LIMIT:i32 = 14;
    //const DEBUG:bool = false;
    // Replace "your_file.txt" with the actual file name
    let file_path = "input.txt";
    //let debug_file_path = "debug.csv";

    let mut regex_map: HashMap<&str, Regex> = HashMap::new();

    regex_map.insert("blue", Regex::new(r"(\d+) blue").unwrap());
    regex_map.insert("green", Regex::new(r"(\d+) green").unwrap());
    regex_map.insert("red", Regex::new(r"(\d+) red").unwrap());

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    // Create a vector to store the results
    //let mut debug: Vec<String> = Vec::new();
    let mut total = 0;
    let mut total_power = 0;
    let mut line_number = 0;

    // Iterate over the lines
    for line in reader.lines() {
        line_number += 1;
        let orig_line = line?;
        let line_sums = process_line(orig_line, &regex_map);
        print!("{}", "game: ".to_string() + &line_number.to_string() + " ");
        print!("{}", "red:".to_string() + &line_sums[0].to_string() + ", ");
        print!("{}", "green:".to_string() + &line_sums[1].to_string() + ", ");
        print!("{}", "blue:".to_string() + &line_sums[2].to_string() + ", ");
        if line_sums[0] <= RED_LIMIT && line_sums[1] <= GREEN_LIMIT && line_sums[2] <= BLUE_LIMIT {
            total += line_number;
            println!("{}", "possible game: ".to_string() + &line_number.to_string());
        }
        else {
            print!("{}", "\n");
        }
        total_power += line_sums[0] * line_sums[1] * line_sums[2];
    }
    println!("{}", "possible total: ".to_string() + &total.to_string());
    println!("{}", "total power: ".to_string() + &total_power.to_string());

    Ok(())
}

fn process_line(input:String, regex_map:&HashMap<&str,Regex>) -> Vec<i32> {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    // copy the input string
    let pulls = input.split(";");
    // split it along the ";"
    for pull in pulls{
        if let Some(captures) = regex_map["red"].captures(&pull) {
            // Access captured groups using captures
            let num = captures[1].parse::<i32>().unwrap();
            if num > red {red = num;};
        }
        if let Some(captures) = regex_map["green"].captures(&pull) {
            // Access captured groups using captures
            let num = captures[1].parse::<i32>().unwrap();
            if num > green {green = num;};
        }
        if let Some(captures) = regex_map["blue"].captures(&pull) {
            // Access captured groups using captures
            let num = captures[1].parse::<i32>().unwrap();
            if num > blue {blue = num;};
        }
    }
    let mut result: Vec<i32> = Vec::new();
    result.push(red);
    result.push(green);
    result.push(blue);
    return result;
}
