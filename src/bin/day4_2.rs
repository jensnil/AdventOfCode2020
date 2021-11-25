use std::collections::HashMap;
use std::fs;
use regex::Regex;

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

    let passports_with_valid_fields = passports_with_present_required_fields.into_iter()
        .filter(|passport| {
            let byr = passport["byr"].parse().unwrap();
            1920 <= byr && byr <= 2002
        })
        .filter(|passport| {
            let iyr = passport["iyr"].parse().unwrap();
            2010 <= iyr && iyr <= 2020
        })
        .filter(|passport| {
            let eyr = passport["eyr"].parse().unwrap();
            2020 <= eyr && eyr <= 2030
        })
        .filter(|passport| {
            let re = Regex::new(r"^(\d+)(\w+)$").unwrap();
            let result = re.captures(passport["hgt"]).unwrap();
            let hgt = result.get(1).unwrap().as_str().parse().unwrap();
            if result.get(2).unwrap().as_str() == "cm" {
                150 <= hgt && hgt <= 193
            } else if result.get(2).unwrap().as_str() == "in" {
                59 <= hgt && hgt <= 76
            } else {
                false
            }
        })
        .filter(|passport| {
            let re = Regex::new(r"^#([0-9a-f]+)$").unwrap();
            re.is_match(passport["hcl"])
        })
        .filter(|passport| {
            let ecl = passport["ecl"];
            ecl == "amb" || ecl == "blu" || ecl == "brn" || ecl == "gry" || ecl == "grn" || ecl == "hzl" || ecl == "oth"
        })
        .filter(|passport| {
            let re = Regex::new(r"^(\d{9})$").unwrap();
            re.is_match(passport["pid"])
        })
        .collect::<Vec<HashMap<_,_>>>();
    println!("{}", passports_with_valid_fields.len());
}
