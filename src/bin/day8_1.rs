use std::collections::HashSet;
use std::fs;

fn execute(instructions: &Vec<(&str, i32)>) -> i32 {
    let mut executed_instructions : HashSet<i32> = HashSet::new();
    let mut accumulator = 0;
    let mut pp = 0;

    while !executed_instructions.contains(&pp) {
        executed_instructions.insert(pp);
        match instructions[pp as usize].0 {
            "acc" => accumulator = accumulator + instructions[pp as usize].1,
            "jmp" => pp = pp as i32 + instructions[pp as usize].1 - 1i32,
            "nop" => {},
            _ => {}
        }
        pp = pp + 1;
    }
    return accumulator;
}

fn main() {
    let filename = "input/day08.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let commands = contents.lines().collect::<Vec<_>>();

    let program  = commands.into_iter().map(|it| {
        let split = it.split(" ").collect::<Vec<_>>();
        (split[0], split[1].parse().unwrap())
    }).collect::<Vec<(&str, i32)>>();

    execute(&program);

    println!("{:?}", execute(&program));
}
