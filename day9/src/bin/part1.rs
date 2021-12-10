use std::cmp;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

use ndarray::prelude::*;

fn read_input<P: AsRef<Path>>(filename: P) -> Array2<u32> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    let mut width = 0;
    let mut height = 0;
    let mut nums: Vec<u32> = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            width = line.chars().count();
        }
        nums.extend(line.chars().map(|x| x.to_digit(10).unwrap()));
        height += 1;
    }
    Array::from_shape_vec((height, width), nums).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }
    let grid = read_input(&args[1]);
    let mut mins: Vec<u32> = Vec::new();
    println!("{:?}", grid);
    println!("{:?}", grid.shape());
    for ((y, x), elt) in grid.indexed_iter() {
        let y: i32 = y.try_into().unwrap();
        let x: i32 = x.try_into().unwrap();
        let miny = cmp::max(y - 1, 0);
        let minx = cmp::max(x - 1, 0);
        let maxy = cmp::min(y + 2, grid.shape()[0].try_into().unwrap());
        let maxx = cmp::min(x + 2, grid.shape()[1].try_into().unwrap());
        //println!("{} {} {} {} {} {}", x, y, minx, maxx, miny, maxy);
        let row = grid.slice(s![y, minx..maxx]);
        let col = grid.slice(s![miny..maxy, x]);
        //println!("{:?}", grid.slice(s![miny..maxy, minx..maxx]));
        //println!("{:?} {:?}", row, col);
        if row.iter().min() == col.iter().min() && row.iter().min() == Some(elt) {
            if col.iter().filter(|x| *x == elt).count() > 1 || row.iter().filter(|x| *x == elt).count() > 1 {
                println!("Duplicate !");
            } else {
                println!("Found low point {}", elt);
                mins.push(*elt + 1);
            }
        }
    }
    println!("{}", mins.iter().sum::<u32>());
}
