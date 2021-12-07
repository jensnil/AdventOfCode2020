use std::collections::HashMap;
use std::fs;

fn get_combinations(mask : Vec<char>) -> Vec<Vec<char>> {
    return if mask.is_empty() {
        vec![vec![]]
    } else {
        let split = mask.split_first().unwrap();
        let lists = get_combinations(split.1.to_vec());
        if *split.0 == 'X' {
            let first = lists.clone().into_iter().map(|mut it| {
                it.insert(0, '2'); // 2 means write 0
                it
            }).collect::<Vec<Vec<char>>>();
            let mut second = lists.clone().into_iter().map(|mut it| {
                it.insert(0, '1');
                it
            }).collect::<Vec<Vec<char>>>();
            let mut to_return = first.to_vec();
            to_return.append(&mut second);
            to_return
        } else {
            let tmp = lists.clone().into_iter().map(|it| {
                let mut new_list  = it.clone();
                new_list.insert(0,*split.0);
                new_list
            }).collect::<Vec<Vec<char>>>();
            tmp
        }
    }
}

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

    let mut masks = vec![vec![]];
    let mut mem : HashMap<usize,usize> = HashMap::new();

    commands.clone().into_iter().for_each(|command| {
        match command.0 {
            "mask" => {
                let value = command.1.chars().collect::<Vec<char>>();
                masks = get_combinations(value);
            },
            _ => {
                let l_index = command.0.find("[").unwrap();
                let r_index = command.0.find("]").unwrap();
                let index : usize = command.0[l_index+1..r_index].parse::<usize>().unwrap();
                let value_as_string = format!("{:036b}", index);
                let value = value_as_string.chars().collect::<Vec<char>>();

                masks.clone().into_iter().for_each(|mask| {
                    let res = mask.into_iter().enumerate().into_iter().map(|(i, ch)| {
                        match ch {
                            '0' => { value[i] },
                            '1' => { '1' },
                            '2' => { '0' },
                            _ => panic!("Stop!")
                        }
                    }).collect::<String>();
                    let number = usize::from_str_radix(res.as_str(), 2).unwrap();
                    mem.insert(number, command.1.parse().unwrap());
                });

            }
        }
    });
    println!("{}", mem.values().sum::<usize>());
}
