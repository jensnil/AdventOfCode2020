use std::fs;

fn rotate(waypoint : (i32, i32), rotation: i32) -> (i32,i32) {
    return match rotation {
        1 => {(-waypoint.1,waypoint.0)},
        2 => {(-waypoint.0, -waypoint.1)},
        3 => {(waypoint.1, -waypoint.0)},
        _ => {
            println!("Boom!");
            waypoint
        }
    };
}

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

    let mut position :(i32, i32) = (0,0);
    let mut waypoint :(i32, i32) = (10,1);

    for command in commands {
        match command.0 {
            'N' => {waypoint.1 = waypoint.1 + command.1},
            'S' => {waypoint.1 = waypoint.1 - command.1},
            'E' => {waypoint.0 = waypoint.0 + command.1},
            'W' => {waypoint.0 = waypoint.0 - command.1},
            'L' => {waypoint = rotate(waypoint, (command.1 / 90) % 4)},
            'R' => {waypoint = rotate(waypoint, (-command.1 / 90 + 4) % 4)},
            'F' => {
                position.0 = position.0 + waypoint.0 * command.1;
                position.1 = position.1 + waypoint.1 * command.1;
            },
            _ => { println!("BOOM!")}
        }
        //println!("{:?} {:?}", position, waypoint);
    }
    println!("{}", position.0.abs() + position.1.abs());
}
