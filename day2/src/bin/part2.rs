use std::env;
use std::fmt;
use std::fs;
use std::path::Path;
use std::process;


struct Submarine {
    aim: i32,
    depth: i32,
    horizontal: i32,
}

impl Submarine {
    fn forward(&mut self, distance: i32) {
        self.horizontal += distance;
        self.depth += self.aim*distance;
    }

    fn down(&mut self, distance: i32) {
        self.aim += distance;
    }

    fn up(&mut self, distance: i32) {
        self.aim -= distance;
    }

    fn process_move(&mut self, movestr: &str) {
        let (direction, distance) = movestr.split_once(' ').expect("Couldn't split string");
        let distance: i32 = distance.parse().expect("Couldn't parse integer");
        match direction {
            "forward" => self.forward(distance),
            "down" => self.down(distance),
            "up" => self.up(distance),
            _ => (),
        };
    }
}

impl fmt::Display for Submarine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}) -> {}", self.depth, self.horizontal, self.aim, self.depth*self.horizontal)
    }
}

fn read_input<P: AsRef<Path>>(filename: P) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    content.lines().map(|x| String::from(x)).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let directions = read_input(&args[1]);
    let mut s = Submarine{depth: 0, horizontal: 0, aim: 0};
    for direction in directions {
        s.process_move(&direction);
    }
    println!("{}", s);
}
