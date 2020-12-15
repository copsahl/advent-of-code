use std::fs::*;
use std::io::prelude::*;
use std::collections::HashMap;

fn solve_one(passports: &Vec<&str>) -> i32 {

    let mut valid_passports = 0;

    for p in passports.iter() {
        p.to_string();
        let p = p.replace('\n', " ");
        let p = &p[..];
        if p.contains("ecl") && p.contains("byr") && p.contains("iyr") && p.contains("eyr") && p.contains("hgt") && p.contains("hcl") && p.contains("pid") {
            valid_passports += 1;
        }
    }
    valid_passports
}

fn solve_two(passports: &Vec<&str>) -> i32 {

    let mut valid_passports = 0;


    for passport in passports.iter(){
        let mut pass = 0;
        passport.to_string();
        let passport = passport.replace('\n', " ");
        let passport = &passport[..];
        let data = format_passport_data(passport);
        if data.get("byr").unwrap().to_string().parse::<i32>().unwrap() >= 1920 && data.get("byr").unwrap().to_string().parse::<i32>().unwrap() <= 2002 {pass += 1;}

    

    }
    valid_passports
}

fn format_passport_data(passport: &str) -> HashMap<&str, &str> {

    let mut data = HashMap::new();
    let passport: Vec<&str> = passport.split(" ").collect();

    for x in passport.iter() {
        let y: Vec<&str> = x.split(":").collect();
        data.insert(y[0], y[1]);
    }

    println!("{:?}", data);
    return data

}

fn main() {

    let mut file = match File::open("input.txt") {
        Err(_) => panic!("Failed to open file."),
        Ok(file) => file,
    };

    let mut passports = String::new();
    match file.read_to_string(&mut passports) {
        Err(_) => panic!("Failed to read file."),
        Ok(_) => file,
    };

    // Split data on blank lines
    let passports: Vec<&str> = passports.split("\n\n").collect();
    println!("Problem 1: {:?}", solve_one(&passports));
    solve_two(&passports);
}
