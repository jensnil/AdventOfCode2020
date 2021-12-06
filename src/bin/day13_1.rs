use std::fs;

fn main() {
    let filename = "input/day13.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input = contents.lines().collect::<Vec<&str>>();

    let start_time : i32 = input[0].parse().unwrap();
    let buses = input[1].split(",").filter(|&it| { it != "x" }).map(|it| { it.parse().unwrap() }).collect::<Vec<i32>>();

    let closest_bus = buses.into_iter().map(|it| {
        (it, (start_time/it + 1) * it)
    }).min_by_key(|&it| it.1).unwrap();

    println!("{:?}", closest_bus);
    println!("{:?}", closest_bus.0 * (closest_bus.1 - start_time));
}
