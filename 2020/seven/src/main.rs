use std::fs::*;
use std::collections::HashMap;

struct Bag {
    amnt: i32,
    name: String,
}

struct Baggage {
    baggage_map: HashMap<String, Vec<Bag>>,
}

impl Bag {

    fn new(num: i32, bag: String) -> Bag{
        Bag{amnt: num, name: bag}
    }

}

impl Baggage {

    fn parse (file_data: String) -> Baggage{
        let mut bag_map: HashMap<String, Vec<Bag>> = HashMap::new();
        let mut data = file_data.replace('.', "");
        let data = data
            .split('\n')
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|x| x.to_string() != String::from(""))
            .map(|mut x| x.to_string())
            .collect::<Vec<String>>();


        for line in data {
            let line = line.replace("bags", "").replace("bag", "");
            let split_line = line.split("contain").collect::<Vec<&str>>();

            let outer_bag = split_line[0].to_string();

            if split_line[1].contains("no other") {
                bag_map.insert(outer_bag.to_owned(), vec![Bag::new(0, String::from(""))]); 
            }else if split_line[1].contains(","){
                let mut inner_bags: Vec<&str> = split_line[1].split(",").collect::<Vec<&str>>();
                let mut bag_vec: Vec<Bag> = Vec::new();
                for bag in inner_bags {
                    let bag = bag.trim();
                    let dec_bag = bag.split(" ").collect::<Vec<&str>>();
                    let new_bag_name: String = dec_bag[1].to_owned() + " " + dec_bag[2];
                    let count: i32 = dec_bag[0].parse::<i32>().unwrap();
                    let new_bag = Bag::new(count, new_bag_name);
                    bag_vec.push(new_bag);
                }
                bag_map.insert(outer_bag.to_owned(), bag_vec);
            }else {
                let bag = split_line[1].trim();
                let dec_bag = bag.split(" ").collect::<Vec<&str>>();
                let new_bag_name: String = dec_bag[1].to_owned() + " " + dec_bag[2];
                let count: i32 = dec_bag[0].parse::<i32>().unwrap();
                let new_bag = Bag::new(count, new_bag_name);
                bag_map.insert(outer_bag.to_owned(), vec![new_bag]);
            }

        }

        
        Baggage{baggage_map: bag_map}

    }
}


fn solve_one(b: &Baggage, bag_s: &String) -> bool {

    let inner_bags = b.baggage_map.get(&bag_s);

    match inner_bags {
        Some(x) => x,
        None => &Vec::new(),
    };

    for bag in inner_bags.unwrap() {
        if bag.name.contains("shiny gold") {
            return true;
        }else {
            if solve_one(&b, &bag.name) {
                return true;
            }
        }
    }

   false 
}


fn help_one(b: &Baggage) -> i32{

    let mut total = 0;
    for (k, v) in b.baggage_map {
        if solve_one(&b, &"shiny aqua".to_string()) {
            total += 1;
        }
    }

    total
}

fn main() {

    let input: String = match read_to_string("input.txt") {
        Err(_) => panic!("Failed to read in data"),
        Ok(file) => file,    
    };
    
    let my_map: Baggage = Baggage::parse(input);
    println!("Problem 1: {}", help_one(&my_map));
}

