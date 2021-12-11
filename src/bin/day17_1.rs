use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

fn print(space : &HashMap<(i32,i32,i32),char>) {
    let min_x = space.keys().map(|&it| { it.0 }).min().unwrap();
    let max_x = space.keys().map(|&it| { it.0 }).max().unwrap();
    let min_y = space.keys().map(|&it| { it.1 }).min().unwrap();
    let max_y = space.keys().map(|&it| { it.1 }).max().unwrap();
    let min_z = space.keys().map(|&it| { it.2 }).min().unwrap();
    let max_z = space.keys().map(|&it| { it.2 }).max().unwrap();
    for space_z in min_z..=max_z {
        println!("z = {}", space_z);
        for space_y in min_y..=max_y {
            for space_x in min_x..=max_x {
                if space.contains_key(&(space_x,space_y,space_z)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
    println!("=============");

}


fn main() {
    let filename = "input/day17.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input = contents.lines().collect::<Vec<&str>>();

    let mut space = input.into_iter().enumerate()
        .map(|line| {
            line.1.chars().enumerate().filter(|&it| { it.1 == '#' })
                .map(|column| { ((column.0 as i32, line.0 as i32, 0_i32), column.1) })
                .collect::<Vec<(_,_)>>()
        })
        .flatten()
        .collect::<HashMap<(i32,i32,i32),char>>();

    for _ in 1..=6 {
        //print(&space);
        let mut next_space : HashMap<(i32,i32,i32),char> = HashMap::new();

        let min_x = space.keys().map(|&it| { it.0 }).min().unwrap();
        let max_x = space.keys().map(|&it| { it.0 }).max().unwrap();
        let min_y = space.keys().map(|&it| { it.1 }).min().unwrap();
        let max_y = space.keys().map(|&it| { it.1 }).max().unwrap();
        let min_z = space.keys().map(|&it| { it.2 }).min().unwrap();
        let max_z = space.keys().map(|&it| { it.2 }).max().unwrap();

        for space_x in min_x-1..=max_x+1 {
            for space_y in min_y-1..=max_y+1 {
                for space_z in min_z-1..=max_z+1 {
                    let position = (space_x,space_y,space_z);
                    let x = (space_x-1..=space_x+1).collect::<Vec<i32>>();
                    let y = (space_y-1..=space_y+1).collect::<Vec<i32>>();
                    let z = (space_z-1..=space_z+1).collect::<Vec<i32>>();
                    let active_counter = x.iter()
                        .cartesian_product(y.iter()).cartesian_product(z.iter())
                        .map(|it| { (*it.0.0, *it.0.1, *it.1) })
                        .filter(|&it| { it != position })
                        .filter(|surrounding_position| {
                            space.contains_key(&surrounding_position)
                        }).count();

                    match space.get(&position) {
                        None => {
                            if active_counter == 3 {
                                next_space.insert(position, '#');
                            }
                        }
                        Some(_) => {
                            if active_counter == 2 || active_counter == 3 {
                                next_space.insert(position, '#');
                            }
                        }
                    }
                }
            }
        }
        space = next_space.clone();
    }

    //print(&space);
    println!("{:?}", space.len());
}
