use std::collections::HashSet;
use std::fs;
use regex::Regex;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

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

    let my_ticket = input[rule_stop+2..your_ticket_stop].first().unwrap().split(",").map(|it| { it.parse().unwrap() }).collect::<Vec<i64>>();

    let all_tickets = input[your_ticket_stop+2..].iter().map(|&it| {
        it.split(",").map(|it| { it.parse().unwrap() }).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();

    let valid_numbers = &rules.clone().into_iter().map(|it| { it.1 }).flatten().collect::<HashSet<i32>>();
    let valid_all_tickets = all_tickets.into_iter().filter(|ticket| { ticket.into_iter().all(|it| { valid_numbers.contains(it) }) }).collect::<Vec<Vec<i32>>>();

    let tickets_per_column = transpose(valid_all_tickets.clone());

    let result =
        &tickets_per_column.iter().map(|column| {
            let clone = rules.clone();
            clone.into_iter().filter(|rule| {
                column.into_iter().all(|item| { rule.1.contains(&item) })
            }).map(|it| { it.0 }).collect::<Vec<&str>>()
        }).collect::<Vec<Vec<&str>>>();

    let mut product = 1_i64;
    let mut used : HashSet<&str> = HashSet::new();
    loop {
        let singles = result.into_iter().enumerate()
            .map(|it| {
                let tmp = it.1.into_iter().map(|&it| { it }).filter(|&it| { !used.contains(it) }).collect::<Vec<&str>>();
                (it.0, tmp)
            })
            .filter(|it| {
                it.1.len() == 1
            })
            .collect::<Vec<_>>();
        let first = singles.first();
        match first {
            None => {break},
            Some(value) => {
                let dimension = *value.1.first().unwrap();
                let first_index = (*first.unwrap()).0;
                used.insert(dimension);
                if dimension.starts_with("departure") {
                    product = product * my_ticket[first_index];
                }
            }
        }
    }
    println!("{:?}", product);
}
