use std::collections::HashSet;
use std::fs;

fn execute<'a>(instructions: &'a Vec<(&str, i32)>) -> Result<i32, &'a str> {
    let mut executed_instructions : HashSet<i32> = HashSet::new();
    let mut accumulator = 0;
    let mut pp: i32 = 0;

    while pp < instructions.len() as i32 && !executed_instructions.contains(&pp) {
        executed_instructions.insert(pp);
        match instructions[pp as usize].0 {
            "acc" => accumulator = accumulator + instructions[pp as usize].1,
            "jmp" => pp = pp + instructions[pp as usize].1 - 1i32,
            "nop" => {},
            _ => {}
        }
        pp = pp + 1;
    }
    return if pp != instructions.len() as i32 {
        Err("failed")
    } else {
        Ok(accumulator)
    }
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

    for i in 0..program.len() {
        if program[i].0 == "jmp" {
            let mut modified_program = program.clone();
            modified_program[i].0 = "nop";
            let result = execute(&modified_program);
            match result {
                Ok(acc) => { println!("{}", acc); break; },
                _ => {}
            }
        } else if program[i].0 == "nop" {
            let mut modified_program = program.clone();
            modified_program[i].0 = "jmp";
            let result = execute(&modified_program);
            match result {
                Ok(acc) => { println!("{}", acc); break; },
                _ => {}
            }
        }
    }
}
