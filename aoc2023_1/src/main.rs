use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::Write;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {

    const DEBUG:bool = false;
    // Replace "your_file.txt" with the actual file name
    let file_path = "input.txt";
    let debug_file_path = "debug_csv.csv";

    let mut regex_list: Vec<Regex> = Vec::new();
    regex_list.push(Regex::new(r"zero").unwrap());
    regex_list.push(Regex::new(r"one").unwrap());
    regex_list.push(Regex::new(r"two").unwrap());
    regex_list.push(Regex::new(r"three").unwrap());
    regex_list.push(Regex::new(r"four").unwrap());
    regex_list.push(Regex::new(r"five").unwrap());
    regex_list.push(Regex::new(r"six").unwrap());
    regex_list.push(Regex::new(r"seven").unwrap());
    regex_list.push(Regex::new(r"eight").unwrap());
    regex_list.push(Regex::new(r"nine").unwrap());

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    // Define your regular expression pattern
    let regex_pattern = r"\d";
    let regex = Regex::new(regex_pattern)?;

    // Create a vector to store the results
    let mut results: Vec<String> = Vec::new();
    let mut debug: Vec<String> = Vec::new();
    let mut total = 0;

    // Iterate over the lines
    for line in reader.lines() {
        let orig_line = line?;
        let mut line = orig_line.clone();
        line = process_line(line, &regex_list);
        //let line = line?;
        let mut line_digits =  String::new();
        
        // Iterate over all matches on the line
        for captures in regex.captures_iter(&line) {
            // Concatenate all capture groups into a single string
            let result = captures
                .iter()
                //.skip(1) // Skip the full match (group 0)
                .flat_map(|capture| capture.map(|m| m.as_str()))
                .collect::<String>();
            line_digits = line_digits + &result;
        }
        debug.push(orig_line + " - " + &line + " - " + &line_digits);
        results.push(line_digits);
    }
    let mut answer_pairs:Vec<String> = Vec::new();

    // process the results as needed
    for result in &results {
        let pair = result.chars().next().unwrap().to_string() + &(result.chars().last().unwrap().to_string());
        total += pair.parse::<i32>().unwrap();
        answer_pairs.push(pair);
    }
    if DEBUG {
     // Print or process the results as needed
        for i in 0..debug.len() {
            debug[i] = debug[i].to_string() + " - " + &answer_pairs[i];
            println!("{}", debug[i]);
        }
        save_to_csv(debug, debug_file_path)?;
        println!("debug.csv saved.");
    }

    println!("{}", total.to_string());

    Ok(())
}


fn process_line(line:String, regex_list:&Vec<Regex>) -> String {
        let mut result = line;
        //special cases first
        result = Regex::new(r"twone").unwrap().replace_all(&result, "21").to_string();
        result = Regex::new(r"eightwo").unwrap().replace_all(&result, "82").to_string();
        result = Regex::new(r"oneight").unwrap().replace_all(&result, "18").to_string();
        for i in 0..regex_list.len() {
            result = regex_list[i].replace_all(&result, i.to_string()).to_string();
        }
        return result;
}

fn save_to_csv(data: Vec<String>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;

    for record in data {
        writeln!(file, "{}", record)?;
    }

    Ok(())
}