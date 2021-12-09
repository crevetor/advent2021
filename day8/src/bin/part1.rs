use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn read_input<P: AsRef<Path>>(filename: P) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    let mut out: Vec<String> = Vec::new();
    for line in content.lines() {
        line.split('|').nth(1).unwrap().split(' ').for_each(|x| out.push(String::from(x)));
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }
    let words = read_input(&args[1]);
    println!("{:?}", words);
    let num = words.iter().filter(|x| [2,4,3,7].contains(&x.chars().count())).count();
    println!("{}", num);
}
