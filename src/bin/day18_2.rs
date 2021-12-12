use std::fs;

fn tokenize(input : &str) -> Vec<String> {
    let clone = input.replace(" ", "").chars().map(|it| {
        it.to_string()
    }).collect::<Vec<String>>().clone();
    return clone.iter().map(|it| {it.clone()}).collect::<Vec<String>>();
}

fn find_matching_parenthesis(input : Vec<String>) -> usize {
    let mut counter = 1;
    let mut index : usize = 1;
    while counter > 0 {
        match input.get(index).unwrap().as_str() {
            "(" => counter = counter + 1,
            ")" => counter = counter - 1,
            _ => {}
        }
        index = index + 1;
    }
    return index - 1;
}

fn evaluate(input : Vec<String>) -> String {
    let mut current = input.clone().into_iter().map(|it|{it.to_string()}).collect::<Vec<String>>();
    let mut index = 0;
    while index < current.len() {
        if current[index] == "(" {
            let end = index + find_matching_parenthesis(current[index..].to_vec());
            let new_vec = &current[index + 1..end];
            let inner_result = evaluate(new_vec.to_vec()).to_string();
            current.splice(index..=end, vec![inner_result]);
        } else {
            index = index + 1;
        }
    }
    index = 0;
    while index <current.len() {
        if current[index] == "+" {
            let left : i64 = current[index-1].parse().unwrap();
            let right : i64 = current[index+1].parse().unwrap();
            current.splice(index-1..=index+1, [(left + right).to_string()]);
        } else {
            index = index + 1;
        }
    }
    index = 0;
    while index < current.len() {
        if current[index] == "*" {
            let left : i64 = current[index-1].parse().unwrap();
            let right : i64 = current[index+1].parse().unwrap();
            current.splice(index-1..=index+1, [(left * right).to_string()]);
        } else {
            index = index + 1;
        }
    }
    return current[0].clone();
}

fn main() {
    let filename = "input/day18.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input = contents.lines().collect::<Vec<&str>>();

    let sum :i64 = input.into_iter().map(|line|  {
        evaluate(tokenize(line)).parse::<i64>().unwrap()
    }).sum();

    println!("Sum: {:?}", sum);
}