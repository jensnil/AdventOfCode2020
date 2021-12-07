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
    for year in numbers.len()-1..2020-1 {
        let last_number = &numbers.last().unwrap().clone();
        let maybe_value = last_seen.get(&last_number);
        match maybe_value {
            None => {
                numbers.push(0);
            },
            Some(value)  => {
                numbers.push(year - value);
            }
        }
        last_seen.insert(*last_number, year);
    }
    println!("{}", numbers.last().unwrap());
}
