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

fn adjascent_points(point: (usize, usize), grid: &Array2<u32>) -> Vec<(usize, usize)> {
    let y: i32 = point.0.try_into().unwrap();
    let x: i32 = point.1.try_into().unwrap();
    let miny = cmp::max(y - 1, 0);
    let minx = cmp::max(x - 1, 0);
    let maxy = cmp::min(y + 2, grid.shape()[0].try_into().unwrap());
    let maxx = cmp::min(x + 2, grid.shape()[1].try_into().unwrap());
    let mut out: Vec<(usize, usize)> = Vec::new();
    for y in miny..maxy {
        if y != point.0.try_into().unwrap() {
            out.push((y.try_into().unwrap(), point.1));
        }
    }
    for x in minx..maxx {
        if x != point.1.try_into().unwrap() {
            out.push((point.0, x.try_into().unwrap()));
        }
    }
    out
}

fn basin_recurse(point: (usize, usize), grid: &Array2<u32>, visited: &mut Vec<(usize, usize)>) -> i32 {
    println!("Processing {:?} ({})", point, grid.get(point).unwrap());
    if grid.get(point) == Some(&9) || visited.contains(&point) {
        //println!("9 or visited");
        return 0;
    } else {
        let points = adjascent_points(point, grid);
        //println!("Adj {:?}", points);
        visited.push(point);
        return points.iter().fold(0, |acc, p| acc + basin_recurse(*p, grid, visited)) + 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }
    let grid = read_input(&args[1]);
    let mut mins: Vec<(usize, usize)> = Vec::new();
    println!("{:?}", grid);
    println!("{:?}", grid.shape());
    for ((y, x), elt) in grid.indexed_iter() {
        let y: i32 = y.try_into().unwrap();
        let x: i32 = x.try_into().unwrap();
        let miny = cmp::max(y - 1, 0);
        let minx = cmp::max(x - 1, 0);
        let maxy = cmp::min(y + 2, grid.shape()[0].try_into().unwrap());
        let maxx = cmp::min(x + 2, grid.shape()[1].try_into().unwrap());
        let row = grid.slice(s![y, minx..maxx]);
        let col = grid.slice(s![miny..maxy, x]);
        if row.iter().min() == col.iter().min() && row.iter().min() == Some(elt) {
            if col.iter().filter(|x| *x == elt).count() > 1 || row.iter().filter(|x| *x == elt).count() > 1 {
                println!("Duplicate !");
            } else {
                println!("Found low point {}", elt);
                mins.push((y.try_into().unwrap(), x.try_into().unwrap()));
            }
        }
    }
    let mut sizes: Vec<i32> = Vec::new();
    for min in mins {
        let mut visited: Vec<(usize, usize)> = Vec::new();
        sizes.push(basin_recurse(min, &grid, &mut visited));
    }
    sizes.sort();
    println!("{:?} {:?}", sizes, sizes.len()-3);
    println!("Total : {}", (sizes.len()-3..sizes.len()).fold(1, |acc, s| acc*sizes[s]));
}
