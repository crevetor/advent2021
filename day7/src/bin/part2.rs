use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn read_input<P: AsRef<Path>>(filename: P) -> Vec<i32> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    content.lines().next().unwrap().split(',').map(|x| i32::from_str_radix(x, 10).unwrap() ).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }
    let mut crabpos = read_input(&args[1]);
    crabpos.sort();

    let crabsum = crabpos.iter().sum::<i32>();
    let crablen: i32 = crabpos.len().try_into().unwrap();
    let mut avgs: Vec<i32> = Vec::new();
    avgs.push(crabsum/crablen);
    if crabsum % crablen != 0 {
        avgs.push(crabsum/crablen + 1);
    }
    let gas = avgs
                .iter()
                .map(
                    |avg| crabpos
                            .iter()
                            .fold(0,
                                |acc, c| acc + (0..=(c - avg).abs()).sum::<i32>()
                            )
                    )
                .min()
                .unwrap();
    println!("{}", gas);
}
