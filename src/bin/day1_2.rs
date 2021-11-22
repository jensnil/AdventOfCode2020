use advent_of_code2020::day1_input::get_input_day_01;

fn main() {
    let values = get_input_day_01().split("\n").map(| a| a.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    'outer: for i in &values {
        for j in &values {
            for k in &values {
                if i + j + k == 2020 {
                    println!("{}", i * j * k);
                    break 'outer
                }
            }
        }
    }
}
