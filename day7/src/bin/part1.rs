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

    let mut mid = Vec::new();

    if crabpos.len() % 2 == 0 {
        mid.push(crabpos[crabpos.len() / 2 - 1]);
    } else {
        let a = crabpos[crabpos.len() / 2];
        let b = crabpos[crabpos.len() / 2 - 1];
        if (a+b)%2 == 0 {
            mid.push((a+b)/2);
        } else {
            mid.push((a+b)/2);
            mid.push((a+b)/2 + 1);
        }
    }
    let gas = mid
            .iter()
            .map(|m| crabpos
                        .iter()
                        .fold(0, |acc, c| acc + (m - c).abs())
            )
            .min()
            .unwrap();
    println!("{}", gas);
}
