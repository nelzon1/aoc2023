use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

// create a vec for each card's winning numbers
// create a vec for the game numbers
// see how many there are by iterating over winning numbers and seeing if game numbers contain them
// calculate score
fn main() -> Result<(), Box<dyn Error>>{

    const DEBUG:bool = false;
    let re_num:Regex = Regex::new( r"\d+").unwrap();
    let mut master: Vec<Vec<char>>= vec![];
    let mut original: Vec<String> = Vec::new();
    let mut card_total = 0;
    let mut card_counts:Vec<u64> = Vec::new();
    let mut game_index = 0;

    let file_path = if DEBUG {"input_debug.txt"} else {"input.txt"};
    // Open the file
    let file = File::open(file_path)?;

    // Build the dataset
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let orig_line = line?;
            master.push(orig_line.chars().collect());
            original.push(orig_line);
            card_counts.push(1 as u64);
    }

    for line in &original{
        //let captures = re_number.captures_iter(&line);
        let pipe_index = line.find('|').unwrap();
        let start_index: usize = if DEBUG {8} else {10};
        let winners = re_num.captures_iter(&line[start_index..pipe_index]);
        let game = re_num.captures_iter(&line[pipe_index..]);
        let mut winners_vec: Vec<i32> = Vec::new();
        let mut game_vec: Vec<i32> = Vec::new();
        // Access captured groups using captures
        for capture in winners{
            for group in capture.iter(){
                if let Some(matched) = group {
                    winners_vec.push(matched.as_str().parse::<i32>().unwrap())
                }
            }
        }
        for capture in game{
            for group in capture.iter(){
                if let Some(matched) = group {
                    game_vec.push(matched.as_str().parse::<i32>().unwrap())
                }
            }
        }
        let mut win_count:i32 = 0;
        for winning_number in winners_vec {
            if game_vec.contains(&winning_number) {
                win_count += 1;
            }
        }
        let score = if win_count <= 1 {win_count} else {2_i32.pow(win_count as u32 - 1)};
        card_total += score;
        for i in 0..win_count {
            card_counts[(game_index + i + 1) as usize] += card_counts[game_index as usize] as u64;
        }

        game_index += 1;
    }
    let card_count_total:u64 = card_counts.iter().sum();
    println!("{}", "card total : ".to_string() + &card_total.to_string());
    println!("{}", "card count total : ".to_string() + &card_count_total.to_string());
    Ok(())
}
