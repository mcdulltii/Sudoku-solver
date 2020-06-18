use std::process;
use std::fs;
use std::io::prelude::*;
use std::io;

fn main() {
    // Solve 9x9 Sudoku grids
    let grid_len = 9;
    // Initialize Sudoku grid with 0s
    let mut grid: Vec<Vec<u32>> = vec![vec![0; 9]; 9];
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

    // Prevent reimplementations of Copy
    let grid_solve = grid.clone();
    // Solve Sudoku grid
    let is_solved = solve(grid_solve, grid_len);
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
// Parses file into List of List of ints
fn grid_file(grid_len:usize) -> Vec<Vec<u32>> {
    let grid: Vec<Vec<u32>> = vec![];
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
    let grid = contents.lines().map(|line| {
        line.split_whitespace().filter_map(|w| w.parse().ok()).collect()
    }).collect();
    return grid;
}

// Read Sudoku clues from stdin
// Parses stdin into List of List of ints
fn grid_input(grid_len:usize) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = vec![];

    // Read stdin for Sudoku clues
    println!("Enter initial Sudoku puzzle 9x9 matrix delimited by spaces and newlines:");
    io::stdout().flush().expect("Could not flush stdout");
    for i in 0..grid_len {
        let mut numbers = String::new();
        io::stdin()
            .read_line(&mut numbers)
            .ok()
            .expect("Read error");
	grid.push(numbers
	    .split_whitespace()
            .filter_map(|w| w.parse().ok())
	    .collect());
        assert!(grid[i].len() == 9);
    }
    return grid;
}

// Solve recursive function for backtrace algorithm
fn solve(bo:Vec<Vec<u32>>, grid_len:usize) -> bool {
    // Initialize (x, y) as List of ints
    // where (100, 100) means the found (x, y) is filled 
    let mut coord:Vec<u32> = vec![100, 100];
    let find = find_empty(bo.clone());
    let mut bo2 = bo.clone();
    if find[0] == 100 || find[1] == 100 {
        return true;
    } else {
        // Need to find the number that fits (x, y)
        coord = find;
    }
    for num in 1..10 {
        // Check if num is valid within the Sudoku rules
        if valid(bo2.clone(), num, coord.clone()) {
            // Initialize (x, y) as valid num
            bo2[coord[0] as usize][coord[1] as usize] = num;
            // Check if valid num in (x, y) is valid for following iterations
            if solve(bo2.clone(), grid_len) {
                // Solution found where (x, y) is valid
                printboard(bo2.clone(), grid_len);
                process::exit(0);
            }
            // Replace (x, y) as empty to backtrace
            bo2[coord[0] as usize][coord[1] as usize] = 0;
        }
    }
    return false;
}

// Checks if number is valid with the Sudoku rules at (x, y)
fn valid(bo:Vec<Vec<u32>>, num:u32, pos:Vec<u32>) -> bool {
    // Check row
    for i in 0..bo[0].len() {
        if bo[pos[0] as usize][i] == num && pos[1] != i as u32 {
            return false;
        }
    }
    // Check column
    for i in 0..bo.len() {
        if bo[i][pos[1] as usize] == num && pos[0] != i as u32 {
            return false;
        }
    }
    // Check box
    let box_x = pos[1] / 3;
    let box_y = pos[0] / 3;
    for i in box_y*3..(box_y*3 + 3) {
        for j in box_x*3..(box_x*3 + 3) {
            if bo[i as usize][j as usize] == num && pos != vec![i, j] {
                return false;
            }
        }
    }
    return true;
}

// Finds next empty box within the Sudoku grid to be filled
fn find_empty(bo:Vec<Vec<u32>>) -> Vec<u32> {
    for i in 0..bo.len() {
        for j in 0..bo[0].len() {
            if bo[i][j] == 0 {
                let coord = vec![i as u32, j as u32];
                return coord;   // (x, y) is empty
            }
        }
    }
    return vec![100, 100];  // (x, y) is filled
}

// Solution is found
// Prints the Sudoku grid as the solution
fn printboard(grid:Vec<Vec<u32>>, grid_len:usize) {
    println!("\nAnswer:");
    for i in 0..grid_len {
        for j in 0..grid_len {
            print!("{} ", grid[i][j]);
        }
        println!(" ");
    }
    return;
}
