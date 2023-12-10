/*
    read input into 2d array
    find the S on the way in

    create rules for getting next element based on element found

    go aloung until we find S again and count steps

    . - None
    S - Start

    next pipe -> (x,y)
    takes in a position and direction (from)
    looks up character
    gets next position
*/
use ndarray::{Array2, ArrayView, Axis};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;

    let input_len = if DEBUG {20} else {140};
    let mut start_position = (0,0);
    let mut pipe_map:HashMap<char, [char; 2]> = HashMap::new();
    pipe_map.entry('|').or_insert(['U','D']);
    pipe_map.entry('-').or_insert(['L','R']);
    pipe_map.entry('L').or_insert(['U','R']);
    pipe_map.entry('J').or_insert(['U','L']);
    pipe_map.entry('7').or_insert(['D','L']);
    pipe_map.entry('F').or_insert(['D','R']);


    let mut inputs: Array2<char> = Array2::from_elem((0,input_len),'m');

    let file_path = if DEBUG {"input_debug2.txt"} else {"input.txt"};
    // Open the file
    let file = File::open(file_path)?;
    // Build the dataset
    let reader = io::BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let this_line = line?;
        if let Some(_) = this_line.find('S') {
            start_position = (index,this_line.chars().enumerate().filter(|(_, x)| *x == 'S' ).map(|(x,_)| x).next().unwrap());
        }
        let _ = inputs.push(Axis(0), ArrayView::from(&this_line.chars().collect::<Vec<char>>())).unwrap();
    }

    // part 2 variables

    let mut history:HashSet<(i32,i32)> = HashSet::new();
    let mut loop_vec:Vec<(usize,usize,char, char)> = Vec::new();

    // Printing the 2D array
    for row in inputs.rows() {
        for &c in row {
            print!("{} ", c);
        }
        println!();
    }
    println!("{}", "Part 1 start: ".to_string() + "(" + &start_position.0.to_string() + "," + &start_position.1.to_string() + ")" );

    let mut current_char = if DEBUG {'7'} else {'7'};
    let mut current_dir = 'R';
    let mut current_step = 1;
    let (mut row, mut column) = (start_position.0,start_position.1 + 1);
    history.insert((row as i32,column as i32));
    loop_vec.push((row,column, current_dir, current_char));

    while current_char != 'S' || current_step == 0 {
        current_step += 1;
        (row, column, current_dir) = next_pipe(&pipe_map, &inputs, row, column, current_dir);
        current_char = inputs[[row,column]];
        history.insert((row as i32,column as i32));
        loop_vec.push((row,column, current_dir, current_char));
        if current_char == 'S' {
            break;
        }
    }

    println!("{}", "Part 1 end: ".to_string() + "(" + &row.to_string() + "," + &column.to_string() + ")" );
    println!("{}", "Part 1 cycle len: ".to_string() + &current_step.to_string() );
    println!("{}", "Part 1 half-cycle len: ".to_string() + &(current_step / 2).to_string() );

    //part 2
    // walk along vector
    for step in loop_vec{
        raycast(&history, &mut inputs, step.0, step.1, step.2, step.3);
    }

    let interior_tiles = inputs.indexed_iter().filter(|(_, x)| **x == 'I').count();
    

    println!("{}", "Part 2 interior tiles: ".to_string() + &interior_tiles.to_string() );

    Ok(())
}

// takes in row, column, direction
// returns row, column, direction
fn next_pipe(pipe_map:&HashMap<char,[char; 2]>, map:&Array2<char>, row:usize, column:usize, in_dir:char) -> (usize, usize, char) {
    let current_char = map[[row,column]];
    let new_dir = if in_dir == 'U' {'D'} else if in_dir == 'D' {'U'} else if in_dir == 'L' {'R'} else {'L'};
    let out_dir = *pipe_map[&current_char].iter().filter(|x|**x != new_dir).next().unwrap();
    if out_dir == 'U' {
        return (row - 1, column, out_dir);
    }
    else if out_dir == 'D' {
        return (row + 1, column, out_dir);
    }
    else if out_dir == 'L' {
        return (row, column - 1, out_dir);
    }
    else {
        return (row, column + 1, out_dir);
    }
}

/*
    Raycasting
    We need a hashmap with all the position tuples of the whole loop
    we also need a list to walk it in order
    We walk along the loop
    we cast rays on the "right" side until we hit another part of the loop
    straights and left turns only
    combine piece + direction to get casting direction
    e.g. | + U = Right
   | + D = Left
    - + L = Down
    - + R = Up
    F + R = Up + Left
    J + L = Down + Right
    L + U = Down + Left
    7 + D = Up + Right
*/

fn raycast(pipe_loop:&HashSet<(i32,i32)>, map: &mut Array2<char>, row:usize, column:usize, in_dir:char, cur_char:char) {
    
    let free_ground = true;
    let mut cur_index = 0;
    let new_dir = if in_dir == 'U' {'D'} else if in_dir == 'D' {'U'} else if in_dir == 'L' {'R'} else {'L'};
    let (cast_r, cast_l, cast_u, cast_d);
    cast_r = (cur_char == 'J' && new_dir == 'L') || (cur_char == '7' && new_dir == 'D') || (cur_char == '|' && new_dir == 'D');
    cast_l = (cur_char == 'F' && new_dir == 'R') || (cur_char == 'L' && new_dir == 'U') || (cur_char == '|' && new_dir == 'U');
    cast_u = (cur_char == 'F' && new_dir == 'R') || (cur_char == '7' && new_dir == 'D') || (cur_char == '-' && new_dir == 'R');
    cast_d = (cur_char == 'J' && new_dir == 'L') || (cur_char == 'L' && new_dir == 'U') || (cur_char == '-' && new_dir == 'L');

    if cast_r {
        while free_ground {
            cur_index += 1;
            if pipe_loop.contains(&(row as i32, (column+cur_index) as i32)) {
                break;
            }
            map[[row,column + cur_index]] = 'I';
        }
    }

    //left
    cur_index = 0;
    if cast_l {
        while free_ground {
            cur_index += 1;
            if pipe_loop.contains(&(row as i32, (column-cur_index) as i32)) {
                break;
            }
            map[[row,column - cur_index]] = 'I';
        }
    }

    //up
    cur_index = 0;
    if cast_u {
        while free_ground {
            cur_index += 1;
            if pipe_loop.contains(&((row - cur_index) as i32, column as i32)) {
                break;
            }
            map[[row - cur_index, column]] = 'I';
        }
    }

    //down
    cur_index = 0;
    if cast_d {
        while free_ground {
            cur_index += 1;
            if pipe_loop.contains(&((row + cur_index) as i32, column as i32)) {
                break;
            }
            map[[row + cur_index, column]] = 'I';
        }
    }

}