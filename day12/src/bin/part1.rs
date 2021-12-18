use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    neighbors: Vec<Node>,
    isbig: bool,
    isstart: bool,
    isend: bool,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
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

    fn visit(self, visited: &mut Vec<Node>) -> i32 {
        if self.isend {
            return 1;
        }
        let mut paths = 0;
        visited.push(self.clone());
        for node in self.neighbors {
            if (!node.isbig && !visited.contains(&&node)) || node.isbig {
                paths += node.visit(visited);
            }
        }
        return paths;
    }
}

fn read_input<P: AsRef<Path>>(filename: P) -> HashMap<String, Node> {
    let content = fs::read_to_string(filename).expect("Couldn't read from file");
    let mut nodemap: HashMap<String, Node> = HashMap::new();
    for line in content.lines() {
        let (from, to) = line.split_once("-").unwrap();
        let mut fromnode = match nodemap.get(from) {
            None => Node::new(from.to_string()),
            Some(a) => a.clone(),
        };
        let tonode = match nodemap.get(to) {
            None => Node::new(to.to_string()),
            Some(a) => a.clone(),
        };
        fromnode.neighbors.push(tonode.clone());
        nodemap.insert(from.to_string(), fromnode);
        nodemap.insert(to.to_string(), tonode);
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
