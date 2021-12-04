use std::env;
use std::fmt;
use std::fs;
use std::path::Path;
use std::process;

use ndarray::prelude::*;

#[derive(Clone)]
struct BingoNumber {
    number: i32,
    is_marked: bool,
}

impl BingoNumber {
    fn new(number: i32) -> BingoNumber {
        BingoNumber {
            number: number,
            is_marked: false,
        }
    }
}

impl fmt::Display for BingoNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({:0>3}, {})", self.number, if self.is_marked { 'x' } else { '-' })
    }
}

impl fmt::Debug for BingoNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:0>3}, {})", self.number, if self.is_marked { 'x' } else { '-' })
    }
}

trait BingoBoardTrait {
    fn is_winner(&self) -> bool;
    fn mark(&mut self, number: i32);
    fn score(&self, number: i32) -> i32;
}

impl BingoBoardTrait for Array2<BingoNumber> {
    fn is_winner(&self) -> bool {
        if self.rows().into_iter().any(|row| row.iter().all(|x| x.is_marked)) {
            return true;
        }
        if self.columns().into_iter().any(|column| column.iter().all(|x| x.is_marked)) {
            return true;
        }
        return false;
    }

    fn mark(&mut self, number: i32) {
        self.iter_mut().for_each(|x| if x.number == number { x.is_marked = true; });
    }

    fn score(&self, number: i32) -> i32 {
        self.iter().fold(0, |a, b| if !b.is_marked { a + b.number } else { a }) * number
    }
}

fn read_input<P: AsRef<Path>>(filename: P) -> (Vec<i32>, Vec<Array2<BingoNumber>>) {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    let mut line_iter = content.lines();
    let gen_nums = line_iter.next().unwrap().split(',').map(|x| i32::from_str_radix(x, 10).unwrap()).collect();
    let mut boards = Vec::new();
    let mut board: Vec<BingoNumber> = Vec::new();
    line_iter.next();
    for line in line_iter {
        if line.is_empty() {
            boards.push(Array::from_shape_vec((5, 5), board.clone()).unwrap());
            board.clear();
        } else {
            board.extend(
                line.split_whitespace()
                .map(|x| BingoNumber::new(i32::from_str_radix(x, 10).unwrap()))
            );
        }
    }
    boards.push(Array::from_shape_vec((5, 5), board.clone()).unwrap());
    (gen_nums, boards)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }
    let (nums, mut boards) = read_input(&args[1]);
    for num in nums {
        for board in boards.iter_mut() {
            board.mark(num);
            if board.is_winner() {
                println!("Found winner {}", board.score(num));
                process::exit(0);
            }
        }
        println!("{}", num);
        println!("{:?}", boards);
        println!("------------------");
    }
}
