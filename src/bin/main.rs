#![allow(warnings)]

use std::process;
use std::fs;
use std::io::prelude::*;
use std::io;

fn main() {
    // Solve 9x9 Sudoku grids
    let grid_len = 9;
    // Initialize Sudoku grid with 0s
    let mut grid: Vec<usize> = vec![0; grid_len * grid_len];
    // Read Stdin for mode of input
    let mode = read_mode().chars().next().unwrap();
    
    // [1] : File input
    // [2] : Stdin input
    // Loading of Sudoku grid with clues and empty as 0s
    if mode == '1' {
        grid = grid_file(grid_len);
    } else if mode == '2' {
        grid = grid_input(grid_len);
    } else {
        print!("Input error");
        process::exit(0);
    }

    // Solve Sudoku grid
    let is_solved = solve(&mut grid, grid_len);
    // If cannot be solved
    if !is_solved {
        print!("Cannot be solved!");
    }
    process::exit(0);
}

// Reads mode from input
// Checks for single character input
fn read_mode() -> String {
    let mut mode = String::new();
    loop {
        println!("Enter input mode: [1]File, [2]Stdin");
        io::stdout().flush().expect("Could not flush stdout");
        io::stdin()
            .read_line(&mut mode)
            .ok()
            .expect("Read error");
        trim_newline(&mut mode);
        match mode.len() {
            0 => continue,
            1 => break,
            _ => {
                println!("Please enter a single number.");
                mode.clear();  // clear contents, otherwise read_line will append on user input
            }
        }
    }
    return mode;
}

// Trims newline character from Stdin String
fn trim_newline(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}

// Read Sudoku clues from file location
// Parses file into List of ints
fn grid_file(grid_len:usize) -> Vec<usize> {
    let mut grid: Vec<usize> = vec![];
    let mut filename = String::new();

    println!("Enter file location:");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin()
        .read_line(&mut filename)
        .ok()
        .expect("Read error");
    trim_newline(&mut filename);
    let contents = fs::read_to_string(filename)
        .expect("File read error");
    let cont_grid: Vec<Vec<usize>> = contents.lines().map(|line| {
        line.split_whitespace().filter_map(|w| w.parse().ok()).collect()
    }).collect();
    for i in cont_grid {
        grid.extend(i);
    }
    assert!(grid.len() == grid_len * grid_len);
    return grid;
}

// Read Sudoku clues from stdin
// Parses stdin into List of ints
fn grid_input(grid_len:usize) -> Vec<usize> {
    let mut grid: Vec<usize> = vec![];

    // Read stdin for Sudoku clues
    println!("Enter initial Sudoku puzzle 9x9 matrix delimited by spaces and newlines:");
    io::stdout().flush().expect("Could not flush stdout");
    for _ in 0..grid_len {
        let mut numbers = String::new();
        io::stdin()
            .read_line(&mut numbers)
            .ok()
            .expect("Read error");
	let input_grid: Vec<usize> = numbers
	    .split_whitespace()
            .filter_map(|w| w.parse().ok())
	    .collect();
        grid.extend(input_grid);
    }
    assert!(grid.len() == grid_len * grid_len);
    return grid;
}

// Solve recursive function for backtrace algorithm
fn solve(bo:&mut Vec<usize>, grid_len:usize) -> bool {
    // Initialize (x, y) as List of ints
    // where (100, 100) means the found (x, y) is filled 
    let mut coord:Vec<usize> = vec![100, 100];
    let find = find_empty(bo, grid_len);
    if find[0] == 100 || find[1] == 100 {
        return true;
    } else {
        // Need to find the number that fits (x, y)
        coord = find;
    }
    for num in 1..(grid_len+1) {
        // Check if num is valid within the Sudoku rules
        if valid(bo, num, &mut coord, grid_len) {
            // Initialize (x, y) as valid num
            bo[coord[0]*grid_len + coord[1]] = num;
            // Check if valid num in (x, y) is valid for following iterations
            if solve(bo, grid_len) {
                // Solution found where (x, y) is valid
                printboard(bo, grid_len);
                process::exit(0);
            }
            // Replace (x, y) as empty to backtrace
            bo[coord[0]*grid_len + coord[1]] = 0;
        }
    }
    return false;
}

// Checks if number is valid within the Sudoku rules at (x, y)
fn valid(bo:&mut Vec<usize>, num:usize, pos:&mut Vec<usize>, grid_len:usize) -> bool {
    // Check row
    for i in 0..grid_len {
        if bo[pos[0]*grid_len + i] == num && pos[1] != i {
            return false;
        }
    }
    // Check column
    for i in 0..grid_len {
        if bo[i*grid_len + pos[1]] == num && pos[0] != i {
            return false;
        }
    }
    // Check box
    let box_x = pos[1] / 3;
    let box_y = pos[0] / 3;
    for i in box_y*3..(box_y*3 + 3) {
        for j in box_x*3..(box_x*3 + 3) {
            if bo[i*grid_len + j] == num && pos[0] != i && pos[1] != j {
                return false;
            }
        }
    }
    return true;
}

// Finds next empty box within the Sudoku grid to be filled
fn find_empty(bo:&mut Vec<usize>, grid_len:usize) -> Vec<usize> {
    for i in 0..grid_len {
        for j in 0..grid_len {
            if bo[i*grid_len + j] == 0 {
                let coord = vec![i, j];
                return coord;   // (x, y) is empty
            }
        }
    }
    return vec![100, 100];  // (x, y) is filled
}

// Solution is found
// Prints the Sudoku grid as the solution
fn printboard(grid:&mut Vec<usize>, grid_len:usize) {
    println!("\nAnswer:");
    for i in 0..grid_len {
        for j in 0..grid_len {
            print!("{} ", grid[i*grid_len + j]);
        }
        println!(" ");
    }
    return;
}
