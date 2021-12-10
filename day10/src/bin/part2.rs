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
    let revmatching = HashMap::from([
        ('{', '}'),
        ('[', ']'),
        ('(', ')'),
        ('<', '>'),
    ]);
    let points = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }
    let lines = read_input(&args[1]);
    let mut scores: Vec<i64> = Vec::new();
    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        let mut valid = true;
        for c in line.chars() {
            match c {
                '{'|'('|'['|'<' => stack.push(c),
                '}'|')'|']'|'>' => {
                    let p = stack.pop().unwrap();
                    if p != *matching.get(&c).unwrap() {
                        valid = false;
                        break;
                    }
                },
                _ => (),
            }
        }
        if valid {
            stack.reverse();
            let missing: String = stack.iter().map(|x| revmatching.get(x).unwrap()).collect();
            scores.push(missing.chars().fold(0, |acc, c| acc*5 + points.get(&c).unwrap()));
            println!("Missing {}", missing);
        }
    }
    scores.sort();
    println!("{}", scores[scores.len()/2]);
}
