use std::fs;

fn main() {
    let filename = "input/day03.txt";
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let result : Vec<Vec<char>> = lines.into_iter()
        .map(|line | {
            line.chars().collect()
        })
        .collect::<Vec<Vec<char>>>();

    let mut counter = 0;
    for i in 1..result.len() {
        let line = result.get(i).unwrap();
        if *line.get((3 * i) % line.len()).unwrap() == '#' {
            counter = counter + 1;
        }
    }

    println!("Count: {}", counter);

}
