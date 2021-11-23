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

    let slopes = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];

    let product = slopes.into_iter().map(|slope| {
        let mut i = 0;
        let mut counter = 0;
        let mut lineNumber = i * slope.1;
        while lineNumber < result.len() {
            let line = result.get(lineNumber).unwrap();
            if *line.get((slope.0 * i) % line.len()).unwrap() == '#' {
                counter = counter + 1;
            }
            i = i + 1;
            lineNumber = i * slope.1;
        }
        counter
    }).product::<i64>();

    println!("product: {}", product);
}
