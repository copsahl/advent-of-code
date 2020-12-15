use std::fs::*;
use std::io::*;

fn main() {

    let mut file = match File::open("input.txt") {
        Err(_) => panic!("failed to open file"),
        Ok(file) => file,
    };

    let mut data = String::new();

    match file.read_to_string(&mut data) {
        Err(_) => panic!("failed to read data"),
        Ok(_) => file,
    };

    let data: Vec<&str> = data.trim().split('\n').collect();
    let mut all_seats: Vec<i32> = Vec::new();
    for line in data.iter(){
        all_seats.push(get_seat_id(line.to_string()));
    }

    println!("Problem 1: {:?}", all_seats.iter().max().unwrap());
    println!("Problem 2: {:?}", part_two());
}

fn get_seat_id(orig: String) -> i32 {
        let mut u_bound = 127;
        let mut l_bound = 0;
        let mut modifier = 64;
        let row: &str = &orig[..7];
        let col: &str = &orig[7..];

        for c in row.chars() {
            match c {
                'F' => { u_bound = u_bound - modifier; modifier = modifier / 2},
                'B' => { l_bound = l_bound + modifier; modifier = modifier / 2},
                _ => { u_bound = u_bound},
            };
        }

        let ans = u_bound;


        u_bound = 7;
        l_bound = 0;
        modifier = 4;

        for c in col.chars() {
            match c {
                'R' => {l_bound = l_bound + modifier; modifier = modifier / 2},
                'L' => {u_bound = u_bound - modifier; modifier = modifier / 2},
                _ => {u_bound = u_bound},
            };
        }
        ans * 8 + u_bound
}    

/*
fn find_seat(all_seats: &mut Vec<i32>) -> i32{
    all_seats.sort();
    println!("{:?}", all_seats);

    let min = all_seats[0];
    let max = all_seats.last().unwrap(); 

    for seat in min..*max {
        if all_seats.iter().any(|&i| i != seat + 1){
            return seat + 1;
        }
    }
    0
}
*/

fn part_two() -> i32 {

    let mut seats = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|r| r.unwrap())
        .map(get_seat_id)
        .collect::<Vec<i32>>();


    seats.sort();
    seats.iter()
        .zip(seats.iter().skip(1))
        .filter(|(&a, &b)| a+1 != b)
        .take(1)
        .next()
        .unwrap().0 + 1

}
