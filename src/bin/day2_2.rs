use std::fs;
use regex::Regex;

fn main() {
    let filename = "input/day02.txt";
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();

    let result = lines.into_iter()
        .map(|line | {
            let result = re.captures(line).unwrap();
            Record {
                min: result.get(1).unwrap().as_str().parse().unwrap(),
                max : result.get(2).unwrap().as_str().parse().unwrap(),
                ch: result.get(3).unwrap().as_str().chars().nth(0).unwrap(),
                password: result.get(4).unwrap().as_str()
            }
        })
        .filter(| line| {
            let first_char = line.password.chars().nth(line.min - 1).unwrap();
            let second_char = line.password.chars().nth(line.max - 1).unwrap();
            (first_char == line.ch || second_char == line.ch) && first_char != second_char
        })
        .collect::<Vec<Record>>();

    println!("Count: {}", result.len());

}

struct Record<'a> {
    min : usize,
    max : usize,
    ch : char,
    password: &'a str
}