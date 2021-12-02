extern crate itertools;

use std::fs;
use itertools::Itertools;

fn split_and_compute(numbers : &Vec<i32>) {
    let mut indexes = numbers.iter().enumerate().filter(|(i, &number)| {
        i + 1 < numbers.len() && number + 3 == numbers[i+1]
    }).map(|it| { it.0 +1}).collect::<Vec<_>>();
    let mut prev = 0;
    let split_by_index = indexes.into_iter().map(|it| {
        let a = numbers[prev..it].iter().collect::<Vec<_>>();
        prev = it;
        a
    }).filter(|it| { it.len() > 2}).map(|it| { (it.len(), it) })
        .collect::<Vec<(usize,Vec<&i32>)>>();
    let list_length_to_lists = split_by_index.into_iter().into_group_map();
    let list_length_to_list_count = list_length_to_lists.iter().map(|(key, value)| { (key, value.len()) }).collect::<Vec<_>>();
    let mut result = 1;
    let three = list_length_to_list_count.iter().find(|&&it| { *it.0 as i32 == 3 });
    result = result * match three {
        None => 1,
        Some(r) => 2_i64.pow(r.1 as u32)
    };
    let four = list_length_to_list_count.iter().find(|&&it| { *it.0 as i32 == 4 });
    result = result * match four {
        None => 1,
        Some(r) => 4_i64.pow(r.1 as u32)
    };
    let five = list_length_to_list_count.iter().find(|&&it| { *it.0 as i32 == 5 });
    result = result * match five {
        None => 1,
        Some(r) => 7_i64.pow(r.1 as u32)
    };
    println!("{:?}", result);
}

fn main() {
    let filename = "input/day10.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let commands = contents.lines().collect::<Vec<_>>();

    let mut numbers  = commands.into_iter().map(|it| { it.parse().unwrap() }).collect::<Vec<i32>>();
    numbers.sort();
    numbers.insert(0, 0);
    numbers.push(*numbers.last().unwrap() + 3);
    split_and_compute(&numbers);
}
