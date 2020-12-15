use std::fs::*;
use std::io::prelude::*;

fn solve_one(slope: &Vec<&str>, y: usize, x: usize) -> u32 {

    let mut total_trees = 0;
    let mut index_y = 0;
    let mut index_x = 0;

    loop {
        index_y = index_y + y;
        index_x = index_x + x;

        if index_y >= slope.len() {
            break;
        }

        if index_x > 30 {
            index_x = (index_x - 30) - 1;
        }
/*
        Original Soultion:         

        let c = slope[index_y].chars().nth(index_x);
        if c != None {
                if c.unwrap() == '#' {
                total_trees = total_trees + 1;
            }
        }
*/

    // If let code provided by redditer
    if let Some('#') = slope[index_y].chars().nth(index_x){total_trees += 1;}

    }

    total_trees
}

fn solve_two(slope: &Vec<&str>) -> u32 {
    
    solve_one(&slope, 1, 1) * solve_one(&slope, 1, 3) * solve_one(&slope, 1, 5) * solve_one(&slope, 1, 7) * solve_one(&slope, 2, 1)

}

fn main() {

    let mut file = match File::open("input.txt") {
        Err(why) => panic!("Failed to open file.{}.", why),
        Ok(file) => file,
    };

    let mut slope = String::new();
    match file.read_to_string(&mut slope) {
        Err(why) => panic!("Failed to read in data. {}.", why),
        Ok(_) => file,
    };

    let slope: Vec<&str> = slope.split("\n").collect();
    println!("Problem 1: {}", solve_one(&slope, 1, 3));
    println!("Problem 2: {}", solve_two(&slope));

}
