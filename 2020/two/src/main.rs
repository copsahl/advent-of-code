//TODO: Refactor plz

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn solve_one(pass: &String) -> bool {
    
    let mut t: i32 = 0;
    let items:Vec<&str> = pass.split(" ").collect();
    
    let rule = items[0];
    let x: Vec<&str> = rule.split("-").collect();
    let min: i32 = x[0].parse().expect("failed");
    let max: i32 = x[1].parse().expect("failed2");

    let c: &str = items[1];
    let c_type = c.chars().nth(0).unwrap();

    let passwd = items[2];

    for p in passwd.chars() {
        if p == c_type {
            t = t + 1;
        }
    }
    if t >= min && t <= max {
        return true;
    }
    return false;
}

fn solve_two(pass: &String) -> bool {
    let items:Vec<&str> = pass.split(" ").collect();
    
    let rule = items[0];
    let x: Vec<&str> = rule.split("-").collect();
    let c: &str = items[1];
    let c_type = c.chars().nth(0).unwrap();
    let pos1: i32 = x[0].parse().expect("failed");
    let pos2: i32 = x[1].parse().expect("failed2");

    let passwd = items[2];

    if passwd.chars().nth((pos1 - 1) as usize).unwrap() ==  passwd.chars().nth((pos2 - 1) as usize).unwrap() {
        return false;
    }
    if passwd.chars().nth((pos1 - 1) as usize).unwrap() == c_type || passwd.chars().nth((pos2 - 1) as usize).unwrap() == c_type {
        return true;
    }

    return false;

}

fn main() {


    if let Ok(lines) = read_lines("input.txt") {
        let mut total = 0;
        let mut total2 = 0;
        for line in lines {
            if let Ok(x) = line {
                if solve_one(&x) {
                    total = total + 1;
                }
                if solve_two(&x) {
                    total2 = total2 + 1;
                }
            }
        }
        println!("Total: {}", total);
        println!("Total: {}", total2);
    }



}

fn read_lines<P>(filename:P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
