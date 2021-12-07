use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "input/day14.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input = contents.lines().collect::<Vec<&str>>();

    let commands = input.into_iter().map(|command| {
        let s = command.split(" = ").collect::<Vec<&str>>();
        (s[0], s[1])
    }).collect::<Vec<(&str,&str)>>();

    let mut mask = "";
    let mut mem : HashMap<usize,usize> = HashMap::new();

    commands.clone().into_iter().for_each(|command| {
        match command.0 {
            "mask" => {
                mask = &command.1;
            },
            _ => {
                let l_index = command.0.find("[").unwrap();
                let r_index = command.0.find("]").unwrap();
                let index : usize = command.0[l_index+1..r_index].parse::<usize>().unwrap();
                let value_as_string = format!("{:036b}", command.1.parse::<usize>().unwrap());
                let value = value_as_string.chars().collect::<Vec<char>>();
                let res = mask.chars().enumerate().into_iter().map(|(i, ch)| {
                    match ch {
                        'X' => { value[i] },
                        _ => ch
                    }
                }).collect::<String>();
                let number = usize::from_str_radix(res.as_str(), 2).unwrap();
                mem.insert(index, number);
            }
        }
    });
    println!("{:?}", &mem);
    println!("{}", mem.values().sum::<usize>());
}
