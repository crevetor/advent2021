use std::env;
use std::fs;
use std::path::Path;
use std::process;

struct Fish {
    daysremaining: i32,
}

impl Fish {
    fn spawn() -> Fish {
        Fish {
            daysremaining: 8,
        }
    }

    fn day(&mut self) -> i32 {
        if self.daysremaining == 0 {
            self.daysremaining = 6;
            return 1;
        }
        self.daysremaining -= 1;
        return 0;
    }
}

fn read_input<P: AsRef<Path>>(filename: P) -> Vec<Fish> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    content.lines().next().unwrap().split(',').map(|x| Fish { daysremaining: i32::from_str_radix(x, 10).unwrap() }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }
    let mut fish: Vec<Fish> = read_input(&args[1]);
    for _ in 0..80 {
        let numnew = fish.iter_mut().fold(0, |acc, f| acc + f.day() );
        fish.append(&mut (0..numnew).map(|_| Fish::spawn()).collect());
    }
    println!("After 80 days {}", fish.len());
}
