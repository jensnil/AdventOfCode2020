use std::collections::HashMap;
use std::fs;
use itertools;
use itertools::Itertools;

fn main() {
    let filename = "input/day15.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input = contents.lines().collect::<Vec<&str>>();

    let mut numbers = input[0].split(",").map(|it| { it.parse().unwrap() }).collect::<Vec<usize>>();
    let mut last_seen: HashMap<usize,usize> = numbers[0..numbers.len()-1].iter().enumerate().map(|(index,value)| { (*value,index) }).collect::<HashMap<usize,usize>>();
    let mut current = numbers.last().unwrap().clone();
    for year in numbers.len()-1..30000000-1 {
        let maybe_value = last_seen.get(&current);
        let next = match maybe_value {
            None => 0,
            Some(value) => year - value
        };
        last_seen.insert(current, year);
        current = next;
    }
    println!("{}", current);
}
