/*
    part 1
    we look at the number of springs 3,2,1
    we have an re for the springs 
    (\.)*(#){3}(\.)+(#){2}(\.)+(#){1}(\.)*
    build an iteration of the string

    test it
    if it matches, add to count

*/
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {

    const DEBUG:bool = false;

    let mut inputs:Vec<String> = Vec::new();
    let mut springs:Vec<Vec<usize>> = Vec::new();
    let mut total_count:u64 = 0;

    let file_path = if DEBUG {"input_debug.txt"} else {"input.txt"};
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let next_line = line?;
        let mut chunks = next_line.split(' ');
        if let Some(input) = chunks.next() {
            inputs.push(input.to_string());
        }
        if let Some(spring_input) = chunks.next() {
            let mut input_vec:Vec<usize> = Vec::new();
            for digit in spring_input.split(',') {
                input_vec.push(digit.parse::<usize>().unwrap());
            }
            springs.push(input_vec);
        }
    }



    // if DEBUG {
    //     for (i, input) in inputs.iter().enumerate() {
    //         print!("{}", input);
    //         print!("\t{}\t", springs[i].iter().map(|x|x.to_string()).collect::<String>());
    //         println!(" - {}", build_regex(&springs[i]));
    //         let re_test = Regex::new(&build_regex(&springs[i])).unwrap();
    //         for test in generate_test_strings(input) {
    //             print!("\t\t{}", test );
    //             println!("\t{}", re_test.is_match(&test) );
    //         }
    //         println!("\tMatches: {}", generate_test_strings(input).iter().filter(|x| re_test.is_match(&**x)).count().to_string());
    //     }
    // }

    // for (i, input) in inputs.iter().enumerate() {
    //     let tests = generate_test_strings(input);
    //     let re_test = Regex::new(&build_regex(&springs[i])).unwrap();
    //     total_count += tests.iter().filter(|x| re_test.is_match(&**x)).count() as u64;
    // }
    //part 1

    // println!("Part 1 - {}", total_count );

    // part 1 memoization
    let mut part_1_total:u64 = 0;
    

    for (i, input) in inputs.iter().enumerate() {
        let mut memo:HashMap<(i32,i32,i32),u64> = HashMap::new();
        part_1_total += count_possible_matches(&mut memo, &input, &springs[i], 0, 0, 0) as u64;
    }

    println!("Part 2 - {}", part_1_total );

    // part 2 memoization
    let mut part_2_total:u64 = 0;

    for (i, input) in inputs.iter().enumerate() {
        let mut memo:HashMap<(i32,i32,i32),u64> = HashMap::new();
        let new_input = input.to_owned() + &"?" + &input.to_string() + &"?" + &input.to_string() + &"?" + &input.to_string() + &"?" + &input.to_string();
        let mut new_springs:Vec<usize> = springs[i].clone();
        for _ in 0..4{
            new_springs.append( &mut springs[i].clone() );
        }
        part_2_total += count_possible_matches(&mut memo, &new_input, &new_springs, 0, 0, 0) as u64;
    }

    println!("Part 2 - {}", part_2_total );


    Ok(())
}

fn build_regex(springs:&Vec<usize>) -> String {
    let re_start = r"^(\.)*(#)";
    let re_repeat = r"(\.)+(#)";
    let re_end = r"(\.)*$";
    if springs.len() < 1 {
        return "".to_string();
    }
    let mut regex_str = re_start.to_owned() + "{" + &springs[0].to_string() + "}";
    for i in 1..springs.len() {
        regex_str = regex_str + &re_repeat.to_string() + "{" + &springs[i].to_string() + "}"
    }
    return regex_str + re_end;
    
}
    /*
        Count ? in input
        2 ^ count
        iterate up to this number and cast as binary
        iterate over the count
        Bit shift the binary number and mask to get bit
        use bit to either put a . or a #
        return str
     */
fn generate_test_strings(input:&str) -> Vec<String> {
    let chars = ['.','#'];
    let mut tests:Vec<String> = Vec::new();
    let count = input.chars().filter(|x| *x == '?').count();
    for i in 0..2_u32.pow(count as u32) {
        let mut mask = i;
        let mut cur_string = input.to_owned();
        for _ in 0..count {
            let cur_bit = mask & 0x1;
            mask = mask >> 1;
            let character = chars[cur_bit as usize];
            let cur_index = cur_string.find('?').unwrap();
            cur_string.replace_range(cur_index..cur_index+1, &character.to_string());
        }
        tests.push(cur_string);
    }

    return tests;
}

fn count_possible_matches(memo:&mut HashMap<(i32,i32,i32),u64>, puzzle:&str, springs:&Vec<usize>, index:i32, spring_index:i32, spring_pos:i32) -> u64 {

    if memo.contains_key(&(index,spring_index,spring_pos)) {
        return memo[&(index,spring_index,spring_pos)];
    }

    let mut count = 0;
    //base cases (end of string, end of spring)
    if index == puzzle.len() as i32 {
        if spring_index == springs.len() as i32 && spring_pos == 0 {
            return 1;
        }
        else if spring_index == springs.len() as i32 - 1 && springs[spring_index as usize] == spring_pos as usize{
            return 1;
        }
        else {return 0;}
    }

    for &char in vec!['.','#'].iter(){
        if puzzle.chars().nth(index as usize).unwrap() == char || puzzle.chars().nth(index as usize).unwrap() == '?' {
            if char=='.' && spring_pos == 0 {
                count += count_possible_matches(memo, puzzle, springs, index+1, spring_index, 0);
            }
            else if char=='.' && spring_pos > 0 && spring_index < (springs.len() as i32) && springs[spring_index as usize] == (spring_pos as usize){
                count += count_possible_matches(memo, puzzle, springs, index + 1, spring_index + 1, 0)
            }
            else if char=='#' {
                count += count_possible_matches(memo, puzzle, springs, index+1, spring_index, spring_pos+1)
            }
        }
    }

    memo.entry((index,spring_index, spring_pos)).or_insert(count);
    return count;
    //iteration

}