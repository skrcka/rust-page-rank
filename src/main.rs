use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


struct Graph {
    edges: HashMap<i64, Vec<i64>>,
}

impl Graph {
    fn from_file(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let lines = reader.lines().map(|l| l.unwrap());

        let mut g = Graph {
            edges: HashMap::new(),
        };
        for line in lines {
            let mut parts = line.split_whitespace();
            let source = parts.next().unwrap().parse::<i64>().unwrap();
            let target = parts.next().unwrap().parse::<i64>().unwrap();
            g.edges.entry(source).or_insert(Vec::new()).push(target);
        }
        g
    }

    fn page_rank(&self) -> HashMap<i64, f64> {
        let n = self.edges.len();
        let mut pr = self.edges.keys().map(|&k| (k, 1.0 / n as f64)).collect::<HashMap<_, _>>();

        for _ in 0..100 {
            pr = self.edges.par_iter().map(|(&u, v)| {
                let mut contrib = v.par_iter().map(|&v| pr[&v] / self.edges[&v].len() as f64).sum();
                contrib = 0.85 * contrib + 0.15 / n as f64;
                (u, contrib)
            }).collect();
        }
        pr
    }
}

fn main() {
    let g = Graph::from_file("./web-BerkStan.txt");

    let pr = g.page_rank();

    for (node, rank) in pr.iter().take(100) {
        println!("{}: {}", node, rank);
    }
}
