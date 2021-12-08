use std::collections::HashSet;
use std::fs;
use regex::Regex;

fn main() {
    let filename = "input/day16.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input = contents.lines().collect::<Vec<&str>>();

    let mut line_number = 0;
    while line_number < input.len() {
        if input[line_number] == "" {
            break;
        }
        line_number = line_number + 1;
    }
    let rule_stop = line_number;
    line_number = line_number + 1;
    while line_number < input.len() {
        if input[line_number] == "" {
            break;
        }
        line_number = line_number + 1;
    }
    let your_ticket_stop = line_number;

    let rules = input[0..rule_stop].iter().map(|it| {
        let re = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        let cap = re.captures(it).unwrap();
        let first_start = cap.get(2).unwrap().as_str().parse().unwrap();
        let first_end = cap.get(3).unwrap().as_str().parse().unwrap();
        let second_start = cap.get(4).unwrap().as_str().parse().unwrap();
        let second_end = cap.get(5).unwrap().as_str().parse().unwrap();
        let first = (first_start..=first_end).into_iter().map(|it| it).collect::<HashSet<i32>>();
        let second = (second_start..=second_end).into_iter().map(|it| it).collect::<HashSet<i32>>();
        let all : HashSet<_> = first.union(&second).into_iter().map(|it| { *it}).collect::<HashSet<i32>>();
        return (cap.get(1).unwrap().as_str(), all);
    }).collect::<Vec<_>>();

    //let my_ticket = input[rule_stop+2..your_ticket_stop].first().unwrap().split(",").map(|it| { it.parse().unwrap() }).collect::<Vec<i32>>();

    let all_tickets = input[your_ticket_stop+2..].iter().map(|&it| {
        it.split(",").map(|it| { it.parse().unwrap() }).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();

    let valid_numbers = &rules.into_iter().map(|it| { it.1 }).flatten().collect::<HashSet<i32>>();
    let ticket_scanning_error_rate = all_tickets.into_iter().flatten().filter(|it| { !valid_numbers.contains(it) }).collect::<Vec<i32>>();

    println!("{:?}", ticket_scanning_error_rate.into_iter().sum::<i32>());
}
