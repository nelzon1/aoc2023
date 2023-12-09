
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use num::integer::lcm;

/*
    hash map 

*/
fn main() -> Result<(), Box<dyn Error>> {
    
    const DEBUG:bool = false;
    let re_triple:Regex = Regex::new(r"[A-Z]{3}").unwrap();

    let mut original: Vec<String> = Vec::new();
    let mut map:HashMap<String, (String,String)> = HashMap::new();

    let file_path = if DEBUG {"input_debug.txt"} else {"input.txt"};
    // Open the file
    let file = File::open(file_path)?;

    // Build the dataset
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let orig_line = line?;
            original.push(orig_line);
    }
    let sequence = original[0].to_string().chars().collect::<Vec<char>>();

    for line in original[2..].iter() {
        let mut line_data:Vec<&str> = Vec::new();
        for capture in  re_triple.captures_iter(&line){
            for group in capture.iter(){
                if let Some(matched) = group {
                    line_data.push(matched.as_str())
                }
            }
        }
            
        map.entry(line_data[0].to_string()).or_insert((line_data[1].to_string(),line_data[2].to_string()));
    }
    println!("{}", "map built");
    let mut steps_taken:u32 = 0;

    // part 1
    let mut cur_node:String = "AAA".to_string();
    let mut direction_loop = sequence.iter().cycle();
    while cur_node != "ZZZ" || steps_taken == 0{
        steps_taken += 1;
        if let Some(dir) = direction_loop.next(){
            if *dir == 'L' {
                let old_node = &cur_node as &str;
                cur_node = map[old_node].0.clone();
            }
            else {
                let old_node = &cur_node as &str;
                cur_node = map[old_node].1.clone();
            }
        }
    }
    println!("{}", "Part1: steps taken to ZZZ: ".to_string() + &steps_taken.to_string());
    // reset for part 2
    direction_loop = sequence.iter().cycle();

    let mut part2 = vec!["AAA","VCA", "FRA", "RPA","SNA", "HNA"];
    if DEBUG {
        part2 = vec!["FRA","DRA"];
    }

    let mut cycles:Vec<u32> = Vec::new();
    for start_pos in part2 {
        steps_taken = 0;
        cur_node = start_pos.to_string();
        while !cur_node.ends_with('Z'){
            steps_taken += 1;
            if let Some(dir) = direction_loop.next(){
                if *dir == 'L' {
                    let old_node = &cur_node as &str;
                    cur_node = map[old_node].0.clone();
                }
                else {
                    let old_node = &cur_node as &str;
                    cur_node = map[old_node].1.clone();
                }
            }
        }
        cycles.push(steps_taken);
    }
    let lcm = find_lcm_of_numbers(&cycles);
    println!("{}", "Part 2 cycles: ".to_string() + &cycles.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("-"));
    println!("{}", "Part2: lcm of various cycles: ".to_string() + &lcm.to_string());
    Ok(())
}

fn find_lcm_of_numbers(numbers: &[u32]) -> u64 {
    if numbers.is_empty() {
        return 0; // LCM is undefined for an empty list
    }
    let mut result:u64 = numbers[0].into();
    for &num in numbers.iter().skip(1) {
        result = lcm(result, num as u64);
    }
    result
}