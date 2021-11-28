use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn find_bag(search: &str, current : &str, rule_map : &HashMap<&str, Vec<(u32, &str)>>) -> bool {
    if search == current {
        false
    } else {
        let list = rule_map.get(current);
        match list {
            None => false,
            Some(list) =>
                if list.into_iter().map(|it| { it.1 }).any(|it| { it == search }) {
                    true
                }  else {
                    list.into_iter().any(|item| {
                        find_bag(search, item.1, rule_map)
                    })
                }
        }
    }
}

fn main() {
    let filename = "input/day07.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let rules = contents.lines().collect::<Vec<_>>();
    let rule_map = rules.iter().cloned().map(|rule| {
        let split = rule.split(" bags contain").collect::<Vec<_>>();
        let key = split[0];
        let value = split[1].split(",").collect::<Vec<_>>();
        let values = value.into_iter().map(|item| {
            let re = Regex::new(r"^(\d+) (.*)$").unwrap();
            let cap = re.captures(item.trim());
            match cap {
                None => (0 as u32, item),
                Some(result) => {
                    let bag_count : u32 = result.get(1).unwrap().as_str().parse().unwrap();
                    let type_str = result.get(2).unwrap().as_str();
                    let last_space = type_str.rfind(' ').unwrap();
                    let bag_type = &type_str[..last_space];
                    (bag_count, bag_type)
                }
            }
        }).collect::<Vec<_>>();
        (key, values)
    }).collect::<HashMap<_,_>>();
    let result = rule_map.keys().map(|bag| {
        find_bag("shiny gold", bag, &rule_map)
    }).collect::<Vec<_>>();
    println!("{:?}", rule_map.keys() );
    println!("{:?}", result.into_iter().filter(|&it| { it }).collect::<Vec<_>>().len());
}
