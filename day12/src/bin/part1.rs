use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(PartialEq, Debug)]
struct Node {
    name: String,
    neighbors: Vec<Node>,
    isbig: bool,
    isstart: bool,
    isend: bool,
}

impl Node {
    fn new(name: String) -> Node {
        let isstart = name == "start";
        let isend = name == "end";
        let isbig = !isstart && !isend && name.chars().nth(0).unwrap().is_ascii_uppercase();
        Node {
            name,
            neighbors: Vec::new(),
            isbig,
            isstart,
            isend,
        }
    }

    fn visit(&self, mut visited: Vec<Node>) -> i32 {
        if self.isend {
            return 1;
        }
        let paths = 0;
        visited.push(*self);
        for node in self.neighbors {
            if (!node.isbig && !visited.contains(&node)) || node.isbig {
                paths += node.visit(visited);
            }         
        }
        return paths;
    }
}

fn read_input<P: AsRef<Path>>(filename: P) -> HashMap<String, Node> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    let nodemap: HashMap<String, Node> = HashMap::new();
    for line in content.lines() {
        let (from, to) = line.split_once("-").unwrap();
        if !nodemap.contains_key(from) {
            nodemap.insert(from.to_string(), Node::new(from.to_string()));
        }
        if !nodemap.contains_key(to) {
            nodemap.insert(to.to_string(), Node::new(to.to_string()));
        }
        nodemap.get_mut(from).unwrap().neighbors.push(*nodemap.get(to).unwrap());
    }
    nodemap
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }
    let nodemap = read_input(&args[1]);
    println!("{:?}", nodemap);
}