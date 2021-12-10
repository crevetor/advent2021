use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn read_input<P: AsRef<Path>>(filename: P) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    content.lines().map(|x| String::from(x)).collect()
}

fn main() {
    let matching = HashMap::from([
        ('}', '{'),
        (']', '['),
        (')', '('),
        ('>', '<'),
    ]);
    let points = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }
    let lines = read_input(&args[1]);
    let mut score = 0;
    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '{'|'('|'['|'<' => stack.push(c),
                '}'|')'|']'|'>' => {
                    let p = stack.pop().unwrap();
                    if p != *matching.get(&c).unwrap() {
                        println!("Found {} when expecting {}", p, matching.get(&c).unwrap());
                        score += points.get(&c).unwrap();
                    }
                },
                _ => (),
            }
        }
    }
    println!("{}", score);
}
