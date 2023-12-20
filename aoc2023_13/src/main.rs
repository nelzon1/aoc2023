
/*
    Iterate over input, building a 2d array for each puzzle (separated by empty lines)
    We need a vector of puzzles


*/
use ndarray::{Array2, ArrayView, Axis, ViewRepr, ArrayBase, Dim};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {

    const DEBUG:bool = false;

    let mut puzzles:Vec<Array2<char>> = Vec::new();

    let file_path = if DEBUG {"input_debug.txt"} else {"input.txt"};
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut puzzle_input:Array2<char> = Array2::from_elem((0,4),'m');
    let mut new_puzzle = true;
    for line in reader.lines() {
        let next_line = line?;
        if new_puzzle {
            puzzle_input = Array2::from_elem((0,next_line.len()),'m');
        }
        if next_line.len() != 0 {
            new_puzzle =  false;
            let _ = puzzle_input.push(Axis(0), ArrayView::from(&next_line.chars().collect::<Vec<char>>())).unwrap();

        }
        else {
            puzzles.push(puzzle_input.clone());
            new_puzzle = true;
        }
    }

    //part 1
    let mut p1_score = 0;
    for puzzle in &puzzles {
        let mirror_score = find_mirror_score(&puzzle);
        p1_score += mirror_score;
        println!("\tMirror score: {}", mirror_score);
        for row in puzzle.rows() {
            println!("\t\t{}", row.iter().collect::<String>())
        }
    }

    println!("Part 1 score: {}", p1_score);

    //part 2
    let mut p2_score = 0;
    for puzzle in &puzzles {
        let mirror_score = find_mirror_score2(&puzzle);
        p2_score += mirror_score;
        println!("\tMirror score: {}", mirror_score);
        for row in puzzle.rows() {
            println!("\t\t{}", row.iter().collect::<String>())
        }
    }

    println!("Part 2 score: {}", p2_score);

    Ok(())
}

fn find_mirror_score(puzzle:&Array2<char>) -> u64 {
/*
    For each puzzle:
        Iterate over the columns forwards and look at next for match
            Once match it found, iterate backwards and validate
        Repeat for the rows
        Return the score according to the rules
        1 for each column left of the mirror
        100 for each row above the mirror
*/
    //compare columns for vert reflection
    let (mut x, mut y) = (0, 0);
    for i in 0..puzzle.ncols()-1 {
        if compare_2(puzzle.column(i).to_vec(), puzzle.column(i+1).to_vec()) {
            let mut valid_reflection = true;
            for j in 1..i+1 {
                if i-j < 0 || i+j+2 > puzzle.ncols() {break};
                if compare_2(puzzle.column(i-j).to_vec(), puzzle.column(i+1+j).to_vec()) {

                }
                else {
                    valid_reflection = false;
                    break;
                }
            }
            // valid reflection found
            if valid_reflection {
                x = (i as u64) + 1;
            }
            
        }
    };

    //compare rows for horizontal reflection
    for i in 0..puzzle.nrows()-1 {
        if compare_2(puzzle.row(i).to_vec(), puzzle.row(i+1).to_vec()) {
            let mut valid_reflection = true;
            for j in 1..i+1 {
                if i-j < 0 || i+j+2 > puzzle.nrows() {break};
                if compare_2(puzzle.row(i-j).to_vec(), puzzle.row(i+1+j).to_vec()) {

                }
                else {
                    valid_reflection = false;
                    break;
                }
            }
            // valid reflection found
            if valid_reflection {
                y = (i as u64) + 1;
            }
            
        }
    };

    return x + (100 * y);
}

fn compare_2(a:Vec<char>, b:Vec<char>) -> bool {
    for (i, chr_a) in a.iter().enumerate() {
        if *chr_a != b[i] {return false;}
    }
    return true;
}

// we now instead calculate the total mismatches and if it is exactly 1, then it's valid
fn find_mirror_score2(puzzle:&Array2<char>) -> u64 {
    //compare columns for vert reflection
    let (mut x, mut y) = (0, 0);
    for i in 0..puzzle.ncols()-1 { 
        let mut mismatches = compare_4(puzzle.column(i).to_vec(), puzzle.column(i+1).to_vec());
        if mismatches <= 1 {
            let mut valid_reflection = true;
            for j in 1..i+1 {
                if i-j < 0 || i+j+2 > puzzle.ncols() {break};
                mismatches += compare_4(puzzle.column(i-j).to_vec(), puzzle.column(i+1+j).to_vec());
                if mismatches > 1 {
                    valid_reflection = false;
                    break;
                }
            }
            // valid reflection found
            if valid_reflection && mismatches == 1 {
                x = (i as u64) + 1;
            }
            
        }
    };

    //compare rows for horizontal reflection
    for i in 0..puzzle.nrows()-1 {
        let mut mismatches = compare_4(puzzle.row(i).to_vec(), puzzle.row(i+1).to_vec());
        
        if mismatches <= 1 {
            let mut valid_reflection = true;
            for j in 1..i+1 {
                if i-j < 0 || i+j+2 > puzzle.nrows() {break};
                mismatches += compare_4(puzzle.row(i-j).to_vec(), puzzle.row(i+1+j).to_vec());
                if mismatches > 1 {
                    valid_reflection = false;
                    break;
                }
            }
            // valid reflection found
            if valid_reflection && mismatches == 1 {
                y = (i as u64) + 1;
            }
            
        }
    };

    return x + (100 * y);
}

fn compare_4(a:Vec<char>, b:Vec<char>) -> u32 {
    let mut sum = 0;
    for (i, chr_a) in a.iter().enumerate() {
        if *chr_a != b[i] {sum += 1}
    }
    return sum;
}