use std::fs;

fn tokenize(input : &str) -> Vec<char> {
    return input.replace(" ", "").chars().map(|it| { it }).collect::<Vec<char>>();
}

fn find_matching_parenthesis(input : Vec<char>) -> usize {
    let mut counter = 1;
    let mut index : usize = 1;
    while counter > 0 {
        match input[index] {
            '(' => counter = counter + 1,
            ')' => counter = counter - 1,
            _ => {}
        }
        index = index + 1;
    }
    return index - 1;
}

fn evaluate(input : Vec<char>) -> i64 {
    let mut index = 0;
    let mut result = 0;
    let mut current_operator = '+';
    while index < input.len() {
        match input[index] {
            '0'..='9' => {
                match current_operator {
                    '+' => {
                        let number : i64 = input[index].to_string().parse().unwrap();
                        result = result + number;
                    },
                    '*' => {
                        let number : i64 = input[index].to_string().parse().unwrap();
                        result = result * number;
                    },
                    _ => println!("Boom!")
                }
            },
            '+' | '*' => current_operator = input[index],
            '(' => {
                let end = index + find_matching_parenthesis(input[index..].to_vec());
                let new_vec = &input[index+1..end];
                let inner_result = evaluate(new_vec.to_vec());
                match current_operator {
                    '+' => result = result + inner_result,
                    '*' => result = result * inner_result,
                    _ => println!("Booom!")
                }
                index = end;
            },
            _ => println!("Boooom!")
        }

        index = index + 1;
    }
    println!("{:?}: {}", input, result);
    return result;
}

fn main() {
    let filename = "input/day18.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input = contents.lines().collect::<Vec<&str>>();

    let sum :i64 = input.into_iter().map(|line|  {
        let res = evaluate(tokenize(line));
        println!("Result: {}", res);
        res
    }).sum();

    println!("Sum: {:?}", sum);
}
