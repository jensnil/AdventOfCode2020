use std::fs;

fn main() {
    let filename = "input/day12.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input = contents.lines().collect::<Vec<&str>>();

    let commands  = input.into_iter().map(|it| {
        let split = it.split_at(1);
        (split.0.chars().next().unwrap(), split.1.parse().unwrap())
    }).collect::<Vec<(char,i32)>>();

    let mut position = (0,0);
    let mut direction = 0; // (E = 0, N = 1, W = 2, S = 3)

    for command in commands {
        match command.0 {
            'N' => {position.0 = position.0 + command.1},
            'S' => {position.0 = position.0 - command.1},
            'E' => {position.1 = position.1 + command.1},
            'W' => {position.1 = position.1 - command.1},
            'L' => {direction = (direction + command.1 / 90) % 4},
            'R' => {direction = (direction - command.1 / 90 + 4) % 4},
            'F' => {
                match direction {
                    0 => {position.1 = position.1 + command.1},
                    1 => {position.0 = position.0 + command.1},
                    2 => {position.1 = position.1 - command.1},
                    3 => {position.0 = position.0 - command.1},
                    _ => { println!("Boom!")},
                }
            },
            _ => { println!("BOOM!")}
        }
        //println!("{:?} {}", position, direction);
    }
    println!("{}", position.0.abs() + position.1.abs());
}
