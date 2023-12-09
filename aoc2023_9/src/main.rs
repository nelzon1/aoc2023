/*
    process reach row by splitting along spaces and parsing as ints
    for each row:
        we first calculate the difference between each element
        check if the row is all equal (alt: check if element is 0)
        build up the element by going from the bottom of the stack and adding the last element to the last element of the row above

        this all screams for recursion but IDK if I want to go that route.
        // return next element
        fn (vec):
            calc diff_vector
            if first diff element is 0
                return vec[last]
            else
                return vec[last] + fn(diff_vector)

*/
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;

    let mut inputs: Vec<Vec<i32>> = Vec::new();

    let file_path = if DEBUG {"input_debug.txt"} else {"input.txt"};
    // Open the file
    let file = File::open(file_path)?;
    // Build the dataset
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let mut num_vec:Vec<i32> = Vec::new();
        let orig_line = line?;
            for chunk in orig_line.split(' '){
                let num = chunk.parse::<i32>().unwrap();
                num_vec.push(num);
            }
        inputs.push(num_vec);
    }
    let mut results:Vec<i32> = Vec::new();

    //part 1
    for row in &inputs {
        let result = calc_next_element(&row);
        println!("{}", "Input: ".to_string() + &row.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("-") + "->" + &result.to_string());
        results.push(result);
    }
    println!("{}", "Part 1: ".to_string() + &results.iter().sum::<i32>().to_string());

    //part 2
    let mut results2:Vec<i32> = Vec::new();
    for row in &inputs {
        let result = calc_prev_element(&row);
        println!("{}", "Input: ".to_string() + &result.to_string() + "->" + &row.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("-"));
        results2.push(result);
    }
    println!("{}", "Part 2: ".to_string() + &results2.iter().sum::<i32>().to_string());

    Ok(())
}

fn calc_next_element(sequence:&Vec<i32>) -> i32 {
    let mut diff_vec:Vec<i32> = Vec::new();
    if sequence.len() < 2 {return 0};
    for i in 0..sequence.len()-1 {
        diff_vec.push(sequence[i+1] - sequence[i]);
    }
    if diff_vec.iter().filter(|x| **x != 0).count() == 0 {return *sequence.iter().last().unwrap()}
    else {return *sequence.iter().last().unwrap() + calc_next_element(&diff_vec)}
}

fn calc_prev_element(sequence:&Vec<i32>) -> i32 {
    let mut diff_vec:Vec<i32> = Vec::new();
    if sequence.len() < 2 {return 0};
    for i in 0..sequence.len()-1 {
        diff_vec.push(sequence[i+1] - sequence[i]);
    }
    if diff_vec.iter().filter(|x| **x != 0).count() == 0 {return *sequence.iter().next().unwrap()}
    else {return *sequence.iter().next().unwrap() - calc_prev_element(&diff_vec)}
}

