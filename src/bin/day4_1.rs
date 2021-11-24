use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "input/day04.txt";
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let passports  = contents.split("\n\n").collect::<Vec<_>>();

    let hash_mapped_passports = (&passports).into_iter().map(| passport | {
        passport.split(|c| { c == ' ' || c == '\n'}).map(|item| {
            let tuple = item.split(":").collect::<Vec<_>>();
            (tuple[0], tuple[1])
        }).collect::<HashMap<_,_>>()
    }).collect::<Vec<HashMap<_, _>>>();

    let passports_with_present_required_fields = hash_mapped_passports.into_iter().filter(|passport| {
        passport.contains_key("byr") &&
            passport.contains_key("iyr") &&
            passport.contains_key("eyr") &&
            passport.contains_key("hgt") &&
            passport.contains_key("hcl") &&
            passport.contains_key("ecl") &&
            passport.contains_key("pid")
    }).collect::<Vec<HashMap<_,_>>>();
    println!("{}", passports_with_present_required_fields.len());
}
