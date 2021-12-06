use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn overlapping_points(&self, other: &Line) -> Vec<Point> {
        self.all_points().intersection(&other.all_points()).cloned().collect()
    }

    fn all_points(&self) -> HashSet<Point> {
        let mut out = HashSet::new();
        let mut xs = [self.start.x, self.end.x];
        let mut ys = [self.start.y, self.end.y];
        xs.sort();
        ys.sort();
        for x in xs[0]..=xs[1] {
            for y in ys[0]..=ys[1] {
                out.insert(Point { x, y });
            }
        }
        out
    }

    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

fn read_input<P: AsRef<Path>>(filename: P) -> Vec<Line> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    let mut out: Vec<Line> = Vec::new();
    for line in content.lines() {
        let points: Vec<Point> = line
            .split(" -> ")
            .map(|point| {
                let (newx, newy) = point.split_once(',').unwrap();
                Point{ x: i32::from_str_radix(newx, 10).unwrap(), y: i32::from_str_radix(newy, 10).unwrap()}
            })
            .collect();
            out.push(Line{ start: points[0], end: points[1]});
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }
    let lines: Vec<Line> = read_input(&args[1]).iter().filter(|x| x.is_straight()).cloned().collect();
    let mut overlapping:Vec<Point> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for j in i+1..lines.len() {
            overlapping.append(&mut line.overlapping_points(&lines[j]));
            //println!("{:?}", line);
            //println!("{:?}", lines[j]);
            //println!("{:?}", overlapping);
        }
    }
    let overlapping: HashSet<Point> = overlapping.iter().cloned().collect();
    println!("{}", overlapping.len());
}
