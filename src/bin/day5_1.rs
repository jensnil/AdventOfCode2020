use std::fs;

fn main() {
    let filename = "input/day05.txt";
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let boarding_numbers = contents.split("\n").collect::<Vec<_>>();

    let binaries = boarding_numbers.into_iter().map(|boarding_number| {
        let row = isize::from_str_radix(boarding_number[..7].replace("F", "0").replace("B", "1").as_str(), 2).unwrap();
        let col = isize::from_str_radix(boarding_number[7..].replace("L", "0").replace("R", "1").as_str(), 2).unwrap();
        8* row + col
    }).max().unwrap();
    println!("{:?}", binaries);
}
