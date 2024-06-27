use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    let f = File::open("day08/input")?;
    let begin = Instant::now();
    println!("{}", steps(f));
    println!("took: {:?}", Instant::now() - begin);
    Ok(())
}

const MAX_NODES: usize = 790;
const MAX_START_NODES: usize = 6;
const NODE_CHARS: usize = 3;

fn steps<R: io::Read>(r: R) -> u32 {
    let mut lines = BufReader::new(r).lines();

    let binding = lines.next().unwrap().unwrap();
    let mut steps = binding.chars().cycle();

    lines.next();

    let mut start_nodes = Vec::with_capacity(MAX_START_NODES);
    let mut network = HashMap::with_capacity(MAX_NODES);
    for l in lines {
        match l {
            Ok(l) => {
                let e = parse_network_entry(l);
                if e.0.ends_with('A') {
                    start_nodes.push(e.0.clone());
                }
                network.insert(e.0, e.1);
            }
            Err(e) => panic!("{e}"),
        }
    }

    let mut n_steps = 0u32;
    let mut cur_last_chars = HashSet::with_capacity(start_nodes.len());
    let mut cur_nodes = start_nodes;
    //println!("{:?} {:?}", cur_nodes, cur_last_chars);
    loop {
        if cur_last_chars.len() == 1 && cur_last_chars.contains(&'Z') {
            break n_steps;
        }
        cur_last_chars.clear();
        n_steps += 1;
        if let Some(s) = steps.next() {
            cur_nodes = cur_nodes
                .into_iter()
                .map(|node| {
                    let next_node = match s {
                        'L' => &network.get(&node).unwrap().0,
                        'R' => &network.get(&node).unwrap().1,
                        _ => panic!("should be either L or R"),
                    };
                    cur_last_chars.insert(next_node.chars().last().unwrap());
                    next_node.clone()
                })
                .collect();
            //println!("{s} {:?} {:?}", cur_nodes, cur_last_chars);
        };
    }
}

fn parse_network_entry(l: String) -> (String, (String, String)) {
    let mut chars = l.chars();

    let mut node = String::with_capacity(NODE_CHARS);
    (0..3).for_each(|_| node.push(chars.next().unwrap()));

    chars.nth(" = (".len() - 1);

    let mut left = String::with_capacity(NODE_CHARS);
    (0..3).for_each(|_| left.push(chars.next().unwrap()));

    chars.nth(", ".len() - 1);

    let mut right = String::with_capacity(NODE_CHARS);
    (0..3).for_each(|_| right.push(chars.next().unwrap()));

    (node, (left, right))
}

#[test]
fn sample_input() {
    let input = "\
        LR\n\
        \n\
        11A = (11B, XXX)\n\
        11B = (XXX, 11Z)\n\
        11Z = (11B, XXX)\n\
        22A = (22B, XXX)\n\
        22B = (22C, 22C)\n\
        22C = (22Z, 22Z)\n\
        22Z = (22B, 22B)\n\
        XXX = (XXX, XXX)\n\
        "
    .as_bytes();
    assert_eq!(steps(input), 6)
}
