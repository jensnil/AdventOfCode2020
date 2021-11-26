use std::collections::HashSet;
use std::fs;

fn main() {
    let filename = "input/day06.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let groups = contents.split("\n\n").collect::<Vec<_>>();
    let person_in_group = groups.iter().map(|group| {
        let persons = group.split("\n").collect::<Vec<_>>();
        let persons_with_chars = persons.iter().map(|person| {
            let chars = person.chars().collect::<Vec<char>>();
            HashSet::from_iter(chars.iter().cloned())
        }).collect::<Vec<HashSet<char>>>();
        let persons_with_sets = persons_with_chars.into_iter().reduce(|c, n| {
            let union = c.intersection(&n).cloned().collect::<Vec<char>>();
            HashSet::from_iter(union)
        }).unwrap();
        persons_with_sets.into_iter().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();
    let flattened = person_in_group.iter().flatten().cloned().collect::<Vec<char>>();
    println!("{:?}", flattened.len() );
}
