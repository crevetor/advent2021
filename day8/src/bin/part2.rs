use itertools::Itertools;

use std::collections::{HashSet, HashMap};
use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Debug)]
struct Segments {
    digits: Vec<String>,
    output: Vec<String>,
    mapping: HashMap<String, char>,
}

impl Segments {
    fn new(digits: Vec<String>, output: Vec<String>) -> Segments {
        let mut out = Segments {
            digits,
            output,
            mapping: HashMap::new(),
        };
        out.decode();
        out
    }

    fn decode(&mut self) {
        let mut digits: Vec<HashSet<char>> = (0..10).map(|_| "".chars().collect::<HashSet<char>>()).collect();
        for digit in self.digits.iter() {
            match digit.len() {
                2 => digits[1] = digit.chars().collect(),
                4 => digits[4] = digit.chars().collect(),
                3 => digits[7] = digit.chars().collect(),
                7 => digits[8] = digit.chars().collect(),
                _ => (),
            }
        }

        for digit in self.digits.iter().filter(|x| x.len() == 6) {
            if digits[4].intersection(&digit.chars().collect::<HashSet<char>>()).cloned().collect::<HashSet<char>>() == digits[4] {
                digits[9] = digit.chars().collect();
            } else if digits[1].intersection(&digit.chars().collect::<HashSet<char>>()).cloned().collect::<HashSet<char>>() == digits[1] {
                digits[0] = digit.chars().collect();
            } else {
                digits[6] = digit.chars().collect();
            }
        }

        for digit in self.digits.iter().filter(|x| x.len() == 5) {
            if digits[1].intersection(&digit.chars().collect::<HashSet<char>>()).cloned().collect::<HashSet<char>>() == digits[1] {
                digits[3] = digit.chars().collect();
            } else if digits[6].intersection(&digit.chars().collect::<HashSet<char>>()).cloned().collect::<HashSet<char>>() == digit.chars().collect() {
                digits[5] = digit.chars().collect();
            } else {
                digits[2] = digit.chars().collect();
            }
        }

        for (i, digit) in digits.iter().enumerate() {
            self.mapping.insert(digit.iter().sorted().collect(), char::from_digit(i.try_into().unwrap(), 10).unwrap());
        }
        println!("{:?}", self.mapping);
    }

    fn output_num(&self) -> i32 {
        let mut num_str = String::new();
        for n in self.output.iter() {
            println!("Finding {}", n.chars().sorted().collect::<String>());
            num_str.push(*self.mapping.get(&n.chars().sorted().collect::<String>()).unwrap());
        }
        i32::from_str_radix(&num_str, 10).unwrap()
    }
}

fn read_input<P: AsRef<Path>>(filename: P) -> Vec<Segments> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    let mut out: Vec<Segments> = Vec::new();
    for line in content.lines() {
        let (digit, output) = line.split_once('|').unwrap();
        out.push(Segments::new(digit.split_whitespace().map(|x| String::from(x)).collect(), output.split_whitespace().map(|x| String::from(x)).collect()));
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }
    let segs = read_input(&args[1]);
    let mut outs: Vec<i32> = Vec::new();
    for seg in segs {
        outs.push(seg.output_num());
        println!("{:?} {}", seg, outs.last().unwrap());
    }
    println!("{}", outs.iter().sum::<i32>());
}
