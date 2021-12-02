use std::collections::{HashSet};
use std::fs;

fn main() {
    let filename = "input/day10.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let commands = contents.lines().collect::<Vec<_>>();

    let mut numbers  = commands.into_iter().map(|it| { it.parse().unwrap() }).collect::<HashSet<i32>>();

    let mut current_jolt : usize = 0;
    let mut jolts_diffs: [i32; 3] = Default::default();
    while !numbers.is_empty() {
        let min = *numbers.iter().min().unwrap();
        jolts_diffs[min as usize - &current_jolt - 1] = jolts_diffs[min as usize - &current_jolt - 1]  + 1;
        current_jolt = min as usize;
        let _ = numbers.remove(&min);
    }
    jolts_diffs[3-1] = jolts_diffs[3-1] + 1;
    println!("{:?} -> {}", jolts_diffs, jolts_diffs[0]* jolts_diffs[2]);
}
