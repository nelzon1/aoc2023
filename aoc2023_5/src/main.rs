use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
/*
    read in seed numbers
    begin a new map on a colon :
    end a map on an empty line

    for each entry:
    start of range in source
    difference
    length

    lookup function:
    takes a map and an ID
    returns the lookup ID
    checks all items in the map (are we in source + range?)
    if so, get id from map
    otherwise return same ID

    iterate over our seeds, look up for each map:
        iterate over the maps, call the lookup function for each one, feeding the output to input
*/

fn main() -> Result<(), Box<dyn Error>> {
    
    const DEBUG:bool = false;
    let re_number:Regex = Regex::new(r"\d+").unwrap();

    let mut original: Vec<String> = Vec::new();
    let mut transforms:Vec<Vec<(i64,i64,i64)>> = Vec::new();
    let mut seed_vec: Vec<i64> = Vec::new();

    let file_path = if DEBUG {"input_debug.txt"} else {"input.txt"};
    // Open the file
    let file = File::open(file_path)?;
    let mut map_index = 0;

    // Build the dataset
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let orig_line = line?;
            original.push(orig_line);
    }

    for line in &original {
        // read seeds
        if let Some(seed_index) = line.find("seeds:"){
            let seed_matches = re_number.captures_iter(&line[seed_index..]);
            // Access captured groups using captures
            for capture in seed_matches{
                for group in capture.iter(){
                    if let Some(matched) = group {
                        seed_vec.push(matched.as_str().parse::<i64>().unwrap())
                    }
                }
            }
        }
        else if line.len() == 0 {continue;}
        // read maps
        else if line.contains("map:"){
            transforms.push(Vec::new());
            map_index += 1;
        }
        else{
            let matches = re_number.captures_iter(&line);
            let mut map_vec: Vec<i64> = Vec::new();
            // Access captured groups using captures
            for capture in matches{
                for group in capture.iter(){
                    if let Some(matched) = group {
                        map_vec.push(matched.as_str().parse::<i64>().unwrap())
                    }
                }
            }
            // destination source length
            transforms[map_index-1].push((map_vec[0],map_vec[1],map_vec[2]));
        }
    }

    let mut new_ids:Vec<i64> = Vec::new();

    println!("{}", &seed_vec.iter().map(|i| i.to_string() + ", ").collect::<String>());
    let seeds_pt_2 = seed_vec.clone();
    
    for seed in seed_vec {
        let mut cur_id = seed;
        let mut next_id = cur_id;
        for transform in &transforms{
            cur_id = next_id;
            next_id = lookup_id(transform, &cur_id);
        }
        new_ids.push(next_id);
    }

    let mut min_location:i64 = 99999999999;

    for seed_range in seeds_pt_2.chunks(2){
        for i in 0..seed_range[1] {
            let mut cur_id = seed_range[0] + i;
            let mut next_id = cur_id;
            for transform in &transforms{
                cur_id = next_id;
                next_id = lookup_id(transform, &cur_id);
                
            }
            min_location = if next_id < min_location {next_id} else {min_location};
        }
        println!("{}", "done pair !".to_string());
    }

    println!("{}", "done : ".to_string());
    println!("{}", &new_ids.iter().map(|i| i.to_string() + ", ").collect::<String>());
    println!("{}", "part1: ".to_string() + &new_ids.iter().min().unwrap().to_string());
    println!("{}", "part2: ".to_string() + &min_location.to_string() );

    Ok(())
}

fn lookup_id(map:&Vec<(i64,i64,i64)>, id:&i64) -> i64 {
    let input:i64 = *id;
    for range in map {
        if input >= range.1 && input <= range.1 + range.2 {
            return input + (range.0 - range.1);
        }
    }
    return input;
}