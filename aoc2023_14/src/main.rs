/*
    We need to get the input into a 2d array
    Then we iterate over each row:
    we should try and keep the same array
    'roll' each row north
        iterate over each element and ray-cast it north into the next open spaace

    o n^3

    part 2:

    what if we hash the puzzle after every cycle and see how many it takes to come back to a known position (crazy)
    then it's just modding the cycle length again 10^9 and picking the correct arrangement to 

*/
use ndarray::{Array2, ArrayView, Axis};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::{Instant, Duration};

fn main() -> Result<(), Box<dyn Error>> {
    // Record the start time
    let start_time = Instant::now();

    const DEBUG:bool = false;
    let file_path = if DEBUG {"input_debug.txt"} else {"input.txt"};
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let input_len = if DEBUG {10} else {100};
    let mut puzzle:Array2<char> = Array2::from_elem((0,input_len),'m');
    for line in reader.lines() {
        let _ = puzzle.push(Axis(0), ArrayView::from(&line?.chars().collect::<Vec<char>>())).unwrap();
    }
    
    //part 1 roll north
    let mut new_puzzle:Array2<char> = puzzle.clone();
    // iterate over rows
    for i in 1..puzzle.nrows() {
        //iterate over each character
        for (j, &chr) in puzzle.row(i).to_vec().iter().enumerate() {
            // raycast into new puzzle 
                // if it is a cube or empty, insert it as is
                // if it is a roller, ray cast it and then insert an empty space
                if chr == 'O' {
                    if roll_north(&mut new_puzzle, i, j) > 0 {
                        new_puzzle[[i,j]] = '.';
                    }
                }
                else {
                    new_puzzle[[i,j]] = chr;
                }
        }
    }

    for row in new_puzzle.rows() {
        //println!("\t{:?}", row.iter().map(|x|*x).collect::<Vec<char>>());
        for chr in row {
            print!("{}", chr);
        }
        print!("{}", '\n');
    }

    println!{"Part 1 total moment: {}", calculate_moment_north(&new_puzzle)};

    // Record the end time
    let end_time = Instant::now();
    // Calculate the elapsed time
    let elapsed_time = end_time - start_time;

    // Print the elapsed time in seconds and milliseconds
    println!("Elapsed time: {} seconds and {} milliseconds",
                elapsed_time.as_secs(),
                elapsed_time.subsec_millis());

    Ok(())
}

fn roll_north(puzzle:&mut Array2<char>, row:usize, col:usize) -> u32 {
    let mut roll_distance:u32 = 0;
    let test = puzzle.column(col).to_vec();
    for (i,&chr) in test[..row].iter().rev().enumerate() {
        if chr == '#' || chr == 'O' {
            puzzle[[row - i, col]] = 'O';
            roll_distance = i as u32;
            break;
        }
        else if row - i == 1 && chr == '.' {
            puzzle[[row - i - 1, col]] = 'O';
            roll_distance = 1;
        }
    }
    return roll_distance;
}

fn calculate_moment_north(puzzle:&Array2<char>) -> u64 {
    let mut total = 0;
    let puz_len = puzzle.nrows() as u64;
    for i in 0..puzzle.nrows() {
        for j in 0..puzzle.ncols() {
            total += if puzzle[[i,j]] == 'O' {puz_len - i as u64 } else {0} ;
        }
    }
    return total;
}