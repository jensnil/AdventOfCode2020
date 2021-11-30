use std::fs;

fn main() {
    let filename = "input/day09.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let commands = contents.lines().collect::<Vec<_>>();

    let numbers  = commands.into_iter().map(|it| { it.parse().unwrap() }).collect::<Vec<i64>>();

    let range = 25;
    for i in range..numbers.len() {
        let slice = numbers[i - range .. i].iter().map(|it| { *it }).collect::<Vec<i64>>();
        let sums = slice.iter().map(|first| {
            slice.iter().filter(|second| { &first != second }).map(|second| {
                first + second
            }).collect::<Vec<i64>>()
        }).flatten().collect::<Vec<i64>>();
        if !sums.contains(&numbers[i]) {
            println!("{}", numbers[i]);
            break;
        }
    }
}
