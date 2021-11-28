use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn find_bag(current : &str, rule_map : &HashMap<&str, Vec<(u32, &str)>>) -> u32 {
    let list = rule_map.get(current);
    match list {
        None => 0,
        Some(list) =>
            list.into_iter().map(|item| {
                item.0 * (1 + find_bag(item.1, rule_map))
            }).sum::<u32>()
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
    let result = find_bag("shiny gold", &rule_map);
    println!("{:?}", result);
}
