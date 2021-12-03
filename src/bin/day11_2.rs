use std::fs;
use itertools::Itertools;


fn rule1(seats : &Vec<Vec<char>>, row : usize, col : usize) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if i != 1 || j != 1 {
                let mut k :i32 = row as i32 + (i as i32 - 1);
                let mut l : i32 = col as i32 + (j as i32 - 1);
                while (0..seats.len()).into_iter().map(|it| it as i32).contains(&k)
                    && (0..seats[k as usize].len()).into_iter().map(|it| it as i32).contains(&l) {
                    if seats[k as usize][l as usize] == '#' {
                        return false;
                    } else if seats[k as usize][l as usize] == 'L' {
                        break;
                    }
                    k = k + i - 1;
                    l = l + j - 1;
                }
            }
        }
    }
    return true;
}

fn rule2(seats : &Vec<Vec<char>>, row : usize, col : usize) -> bool {
    let mut occupied_seats = 0;
    for i in 0..3 {
        for j in 0..3 {
            if i != 1 || j != 1 {
                let mut k :i32 = row as i32 + (i as i32 - 1);
                let mut l : i32 = col as i32 + (j as i32 - 1);
                while (0..seats.len()).into_iter().map(|it| it as i32).contains(&k)
                    && (0..seats[k as usize].len()).into_iter().map(|it| it as i32).contains(&l) {
                    if seats[k as usize][l as usize] != '.' {
                        if seats[k as usize][l as usize] == '#' {
                            occupied_seats = occupied_seats + 1;
                        }
                        break;
                    }
                    k = k + i - 1;
                    l = l + j - 1;
                }
            }
        }
    }
    return occupied_seats >= 5;
}

fn print(seats: &Vec<Vec<char>>) {
    seats.iter().for_each(|row| {
        row.iter().for_each(|ch| {
            print!("{}", ch);
        });
        println!();
    });
    println!();
}

fn main() {
    let filename = "input/day11.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input = contents.lines().collect::<Vec<&str>>();

    let mut seats  = input.into_iter().map(|it| { it.chars().collect::<Vec<char>>() }).collect::<Vec<Vec<char>>>();

    loop {
        let mut next_seats = seats.clone();
        for i in 0..seats.len() {
            for j in 0..seats[i].len() {
                if seats[i][j] == 'L' {
                    if rule1(&seats, i, j) {
                        next_seats[i][j] = '#';
                    }
                } else if seats[i][j] == '#' {
                    if rule2(&seats, i, j) {
                        next_seats[i][j] = 'L';
                    }
                }
            }
        }

        //print(&next_seats);

        if seats == next_seats {
            break;
        }
        seats = next_seats;
    }
    println!("{}", seats.iter().flatten().filter(|&&it| { it == '#' }).count());
}
