use ndarray::{Array2, ArrayView, Axis};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;

    let input_len = if DEBUG {10} else {140};
    let mut expansion_map:HashSet<(i32, i32)> = HashSet::new();
    let mut galaxies:Vec<(usize, usize)> = Vec::new();

    let mut universe: Array2<char> = Array2::from_elem((0,input_len),'m');

    let file_path = if DEBUG {"input_debug.txt"} else {"input.txt"};
    // Open the file
    let file = File::open(file_path)?;
    // Build the dataset
    let reader = io::BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let this_line = line?;
        let found_galaxies = this_line.chars().into_iter().enumerate().filter(|(_,x)| *x == '#' ).map(|(a,_)| a).collect::<Vec<usize>>();
        for galaxy_index in found_galaxies {
            galaxies.push((i, galaxy_index) );
        }

        let _ = universe.push(Axis(0), ArrayView::from(&this_line.chars().collect::<Vec<char>>())).unwrap();
    }

    let mut i = 0;
    for row in universe.rows() {
        if row.iter().filter(|x| **x != '.').count() == 0 {
            expansion_map.insert((i,-1));
        }
        i += 1;
    }
    i = 0;
    for column in universe.columns() {
        if column.iter().filter(|x| **x != '.').count() == 0 {
            expansion_map.insert((-1,i));
        }
        i += 1;
    }

    // part 1
    let mut total_distance = 0;

    for (i, &galaxy_a) in galaxies.iter().enumerate() {
        for &galaxy_b in &galaxies[i+1..] {
            total_distance += compute_distance(&expansion_map, galaxy_a, galaxy_b, 1);
        }
    }

    // part 2
    let mut total_distance2:u64 = 0;

    for (i, &galaxy_a) in galaxies.iter().enumerate() {
        for &galaxy_b in &galaxies[i+1..] {
            total_distance2 += compute_distance(&expansion_map, galaxy_a, galaxy_b, 1000000 - 1);
        }
    }


    println!("{}", "Part 1 distance: ".to_string() + &total_distance.to_string());
    println!("{}", "Part 2 distance: ".to_string() + &total_distance2.to_string());

    Ok(())
}

fn compute_distance(expansions:&HashSet<(i32,i32)>, a: (usize,usize), b: (usize, usize), expansion_factor:u64) -> u64 {
    let mut sorted_vert = vec![a.0 as i32,b.0 as i32];
    sorted_vert.sort();
    let mut sorted_horiz = vec![a.1 as i32,b.1 as i32];
    sorted_horiz.sort();
    let vert_galaxies = expansions.iter().filter(|(x,_)| *x >= sorted_vert[0] && *x <= sorted_vert[1]).count();
    let horiz_galaxies = expansions.iter().filter(|(_, x)| *x >= sorted_horiz[0] && *x <= sorted_horiz[1]).count();
    let vertical = (sorted_vert[1] - sorted_vert[0]) as u64 + vert_galaxies as u64 * expansion_factor;
    let horizontal = (sorted_horiz[1] - sorted_horiz[0]) as u64 + horiz_galaxies as u64 * expansion_factor;
    return (vertical + horizontal) as u64;
}