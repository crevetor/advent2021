use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn read_input<P: AsRef<Path>>(filename: P) -> Vec<usize> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    content.lines().next().unwrap().split(',').map(|x| usize::from_str_radix(x, 10).unwrap()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }
    let ini_fish: Vec<usize> = read_input(&args[1]);
    let mut fish = [0; 9];
    for i in ini_fish {
        fish[i] += 1;
    }
    for _day in 0..256 {
        let numspawning = fish[0];
        for i in 1..9 {
            fish[i-1] = fish[i];
        }
        fish[6] += numspawning;
        fish[8] = numspawning;
    }
    println!("{}", fish.iter().sum::<i64>());
}
