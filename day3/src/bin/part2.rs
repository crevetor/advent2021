use std::env;
use std::fs;
use std::path::Path;
use std::process;

enum RatingTypes {
    Oxygen,
    Co2,
}

fn read_input<P: AsRef<Path>>(filename: P) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    content
        .lines()
        .map(|x| String::from(x))
        .collect()
}

fn common(input: &Vec<String>, pos: usize, rtype: &RatingTypes) -> char {
    let sum: usize = input.iter()
                    .fold(0,
                        |a, b| if b.chars().nth(pos) == Some('1') { a + 1 } else { a }
                    );
    let is_eq = input.len() - sum == sum;
    let common = if sum > input.len() - sum { ('1', '0') } else { ('0', '1') };
    match rtype {
        RatingTypes::Oxygen => if is_eq { '1' } else { common.0 },
        RatingTypes::Co2 => if is_eq { '0' } else { common.1 },
    }
}

fn rating(input: &Vec<String>, rtype: &RatingTypes) -> i32 {
    let mut filtered : Vec<String> = input.clone();
    let mut pos = 0;
    while filtered.len() != 1 {
        let common = common(&filtered, pos, rtype);
        filtered = filtered.iter()
                    .filter(|x| x.chars().nth(pos) == Some(common))
                    .cloned().collect();
        pos += 1;
    }
    i32::from_str_radix(&filtered[0], 2).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let diag_lines = read_input(&args[1]);
    let o2_rating = rating(&diag_lines, &RatingTypes::Oxygen);
    let co2_rating = rating(&diag_lines, &RatingTypes::Co2);
    println!("{} {} -> {}", o2_rating, co2_rating, o2_rating*co2_rating);
}
