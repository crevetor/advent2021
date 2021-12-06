use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
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
        let mut x = self.start.x;
        let mut y = self.start.y;
        while !(x == self.end.x && y == self.end.y) {
            out.insert(Point { x, y });
            if x < self.end.x {
                x += 1;
            } else  if x > self.end.x {
                x -= 1;
            }
            if y < self.end.y {
                y += 1;
            } else if y > self.end.y {
                y -= 1;
            }
        }
        out.insert(Point {x: self.end.x, y: self.end.y});
        out
    }
}

fn read_input<P: AsRef<Path>>(filename: P) -> Vec<Line> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    let mut out: Vec<Line> = Vec::new();
    for line in content.lines() {
        let mut points: Vec<Point> = line
            .split(" -> ")
            .map(|point| {
                let (newx, newy) = point.split_once(',').unwrap();
                Point{ x: i32::from_str_radix(newx, 10).unwrap(), y: i32::from_str_radix(newy, 10).unwrap()}
            })
            .collect();
            points.sort();
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
    let lines = read_input(&args[1]);
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
