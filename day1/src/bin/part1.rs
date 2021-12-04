use std::env;
use std::fs;
use std::process;

fn read_input(filename: &str) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    let content = fs::read_to_string(filename);
    let content = match content {
        Ok(c) => c,
        Err(err) => panic!("Unable to read from file {} {:?}", filename, err),
    };
    for line in content.lines() {
        ret.push(line.parse().expect("Unable to parse int"));
    }

    ret
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let depths = read_input(&args[1]);
    let mut larger: u32 = 0;
    let mut prev: i32 = depths[0];
    for &depth in depths[1..].iter() {
        if depth > prev {
            larger += 1;
        }
        prev = depth;
    }
    println!("{} measurements are larger", larger);
}
