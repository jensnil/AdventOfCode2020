use std::collections::{HashMap, HashSet};
use std::fs;
use itertools::Itertools;

fn traverse(rules : &HashMap<& str, Vec<Vec<String>>>, current : & str, depth: usize) -> Vec<String> {
    //println!("{}current: {}", vec![" "; depth].join(""), current);
    let test = rules.clone();
    let temp = rules.get(current);
    let current_rule = temp.unwrap();
    let all = if current_rule[0].len() == 1 &&
        current_rule.get(0).unwrap().get(0).is_some() &&
        current_rule.get(0).unwrap().get(0).unwrap().parse::<i32>().is_err() {
        let first = vec![current_rule.get(0).unwrap().get(0).unwrap().clone()];
        first
    } else {
        let recursive_result = current_rule.into_iter().map(|it| {
            let a = it.into_iter().fold(vec!["".to_string()], |agg, it| {
                let result = traverse(rules, &it, depth + 1);
                let b = result.into_iter().map(|it| {
                    let c = agg.clone().into_iter().map(|it2| {
                        format!("{}{}", it2, it)
                    }).collect::<Vec<String>>();
                    c
                }).collect::<Vec<Vec<String>>>();
                let merge = b.into_iter().flatten().collect::<Vec<String>>();
                merge
            });
            a
        }).collect::<Vec<Vec<String>>>();
        let d = recursive_result.into_iter().flatten().collect::<Vec<String>>();
        d
    };
    //println!("{} {}: {:?}", vec!["."; depth].join(""), current, all);
    return all;
}

fn main() {
    let filename = "input/day19.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input = contents.lines().collect::<Vec<&str>>();

    let clone = input.clone();

    let divider_index = &input.into_iter().find_position(|&it| {it == ""}).unwrap();
    let rules = clone[..divider_index.0].iter().map(|&rule| {
        let split = rule.split(":").collect::<Vec<&str>>();
        if split[1].contains("\"") {
            let left = split[1].find("\"").unwrap() + 1;
            let right = split[1].rfind("\"").unwrap();
            let alternatives = vec![vec![split[1][left..right].to_string()]];
            (split[0], alternatives)
        } else {
            let alternatives = split[1].trim().split("|").map(|it| {
                it.trim().split(" ").map(|it| {it.to_string()}).collect::<Vec<String>>()
            }).collect::<Vec<Vec<String>>>();
            (split[0], alternatives)
        }
    }).collect::<HashMap<&str, Vec<Vec<String>>>>();

    let valid_strings = traverse(&rules, "0", 0);

    let valid_strings_set : HashSet<&String> = HashSet::from_iter(&valid_strings);

    //println!("{:?}", &valid_strings);
    //println!("{:?}", valid_strings_set);

    let matches = clone[divider_index.0+1..].iter().filter(|&&line| { valid_strings_set.contains(&line.to_string()) }).collect::<Vec<&&str>>();
    println!("matches: {}", matches.len());

}
