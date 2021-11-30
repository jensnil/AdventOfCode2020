use std::fs;


fn get_number<'a>(numbers : &Vec<i64>, range : usize) -> Result<usize, &'a str> {
    for i in range..numbers.len() {
        let slice = numbers[i - range .. i].iter().map(|it| { *it }).collect::<Vec<i64>>();
        let sums = slice.iter().map(|first| {
            slice.iter().filter(|second| { &first != second }).map(|second| {
                first + second
            }).collect::<Vec<i64>>()
        }).flatten().collect::<Vec<i64>>();
        if !sums.contains(&numbers[i]) {
            return Ok(i);
        }
    }
    return Err("nope");
}

fn main() {
    let filename = "input/day09.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let commands = contents.lines().collect::<Vec<_>>();

    let numbers  = commands.into_iter().map(|it| { it.parse().unwrap() }).collect::<Vec<i64>>();

    let index = get_number(&numbers, 25).unwrap();
    let magic_number = *numbers.get(index).unwrap();

    let mut i : usize = 0;
    while i < index {
        let mut j = i;
        let mut sum : i64 = 0;
        while sum < magic_number {
            sum = sum + numbers.get(j).unwrap();
            j = j + 1;
        }
        if sum == magic_number {
            let min = numbers[i..j-1].iter().min().unwrap();
            let max = numbers[i..j-1].iter().max().unwrap();
            println!("{} + {} = {}", min, max, min + max);
            break;
        }
        i = i + 1;
    }
}
