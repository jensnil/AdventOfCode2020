use std::fs;

fn find_alignment(first : (i128, i128), second : (i128, i128)) -> (i128, i128) {
    let a_period = first.0;
    let a_phase = first.1;
    let b_period = second.0;
    let b_phase = second.1;
    let (gcd, s, _) = extended_euclidean_algorithm(a_period, b_period);
    let phase_difference = a_phase - b_phase;
    if phase_difference % gcd != 0 {
        panic!("Stop!")
    }
    let z = phase_difference / gcd;
    let m = z * s;
    let combined_period = a_period * b_period / gcd;
    let combined_phase = (-m * a_period + a_phase) % combined_period;

    return (combined_period, combined_phase);
}

fn extended_euclidean_algorithm(a : i128, b : i128) -> (i128, i128, i128) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s= 1_i128;
    let mut s = 0_i128;
    let mut old_t = 0_i128;
    let mut t = 1_i128;

    while r != 0 {
        let quotient = old_r / r;
        let remainder = old_r % r;
        old_r = r;
        r = remainder;

        let temp_s= old_s;
        old_s = s;
        s = temp_s - quotient * s;

        let temp_t = old_t;
        old_t = t;
        t = temp_t - quotient * t;
    }
    return (old_r, old_s, old_t);
}

fn main() {
    let filename = "input/day13.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input = contents.lines().collect::<Vec<&str>>();

    let buses = input[1]
        .split(",")
        .enumerate()
        .filter(|&it| { it.1 != "x" })
        .map(|it| { (it.1.parse().unwrap(), it.0 as i128) })
        .collect::<Vec<(i128, i128)>>();

    let first = buses.first().unwrap().clone();
    let folding = buses.into_iter().skip(1).fold(first, |agg, it| {
        find_alignment(agg, it)
    });
    println!("Done: {}", folding.0 - folding.1.rem_euclid(folding.0));

}
