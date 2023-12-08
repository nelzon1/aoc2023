
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
fn main() -> Result<(), Box<dyn Error>> {
    
    const DEBUG:bool = false;

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

    Ok(())
}
