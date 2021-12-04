use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn read_input<P: AsRef<Path>>(filename: P) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    content
        .lines()
        .map(|x| String::from(x))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let diag_lines = read_input(&args[1]);
    let mut gamma  = String::new();
    for i in 0..diag_lines[0].len() {
        let sum: usize = diag_lines.iter()
                        .fold(0,
                            |a, b| if b.chars().nth(i) == Some('1') { a + 1 } else { a }
                        );
        if sum > diag_lines.len()/2 {
            gamma.push('1');
        } else {
            gamma.push('0');
        }
    }
    let epsilon: String = gamma.chars().map(|x| if x == '0' { '1' } else { '0' }).collect();
    println!("{}", gamma);
    println!("{}", epsilon);

    let gamma: i32 = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon: i32 = i32::from_str_radix(&epsilon, 2).unwrap();
    println!("g {}, e {} -> {}", gamma, epsilon, gamma*epsilon);
}
