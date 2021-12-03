use std::fs;

fn rule1(seats : &Vec<Vec<char>>, row : usize, col : usize) -> bool {
    let row_start = if row <= 0 {0} else {row - 1};
    let row_end = if row >= seats.len() - 1 {seats.len() - 1} else {row + 1};
    let col_start = if col <= 0 {0} else {col - 1};
    let col_end = if col >= seats[row].len() - 1 {seats[row].len() - 1} else {col + 1};

    for i in row_start..=row_end {
        for j in col_start..=col_end {
            if (i != row || j != col) && seats[i][j] == '#' {
                return false;
            }
        }
    }
    return true;
}

fn rule2(seats : &Vec<Vec<char>>, row : usize, col : usize) -> bool {
    let row_start = if row <= 0 {0} else {row - 1};
    let row_end = if row >= seats.len() - 1 {seats.len() - 1} else {row + 1};
    let col_start = if col <= 0 {0} else {col - 1};
    let col_end = if col >= seats[row].len() - 1 {seats[row].len() - 1} else {col + 1};
    let mut occupied_seats = 0;

    for i in row_start..=row_end {
        for j in col_start..=col_end {
            if (i != row || j != col) && seats[i][j] == '#' {
                occupied_seats = occupied_seats + 1;
            }
        }
    }
    return occupied_seats >= 4;
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

        if  seats == next_seats {
            break;
        }
        seats = next_seats;
    }
    println!("{}", seats.iter().flatten().filter(|&&it| { it == '#' }).count());
}
