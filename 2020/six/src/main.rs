extern crate undup;
extern crate itertools;

use std::fs::*;
use std::io::prelude::*;
use undup::*;
use itertools::Itertools;
use std::collections::HashSet;

fn solve_one(data: &String) -> i32{

    let mut total: i32 = 0;

    let groups: Vec<String> = data.split("\n\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.replace('\n', ""))
        .collect::<Vec<String>>();


    groups.into_iter()
    .map(|y| y.chars().sorted().collect::<String>())
    .map(|x| undup_chars(&x[..], vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z']))
    .collect::<Vec<String>>()
    .into_iter()
    .for_each(|x| (total = total + x.len() as i32));

    total
}

/* Redditer solution to study!
fn solve_one(data: &String) -> i32{

    data.split("\n\n")
        .map(|x| {
            x.chars()
                .filter(|c| c.is_ascii_alphabetic())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum::<usize>() as i32

}

*/


// Redditer solution to study
fn full_answer() -> HashSet<char> {
    ('a'..='z').collect()
}

fn solve_two(data: &String) -> i32 {
    data.split("\n\n")
        .map(|x| {
            x.lines()
                .map(|e| e.chars().collect::<HashSet<char>>())
                .fold(full_answer(), |acc, e| {
                    acc.intersection(&e).cloned().collect()
                })
                .len()
        })
        .sum::<usize>() as i32
}

fn main() {

    let mut file = match File::open("input.txt") {
        Err(_) => panic!("Failed to open file."),
        Ok(file) => file,
    };

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(_) => panic!("Failed to read file"),
        Ok(file) => file,
    };

    println!("Problem 1: {}", solve_one(&data));
    println!("Problem 2: {}", solve_two(&data));
}
