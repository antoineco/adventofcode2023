use std::collections::HashMap;
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
const NODE_CHARS: usize = 3;

fn steps<R: io::Read>(r: R) -> u32 {
    let mut lines = BufReader::new(r).lines();

    let binding = lines.next().unwrap().unwrap();
    let mut steps = binding.chars().cycle();

    lines.next();

    let mut network = HashMap::with_capacity(MAX_NODES);
    for l in lines {
        match l {
            Ok(l) => {
                let e = parse_network_entry(l);
                network.insert(e.0, e.1);
            }
            Err(e) => panic!("{e}"),
        }
    }

    let mut n_steps = 0u32;
    let mut next_step = "AAA";
    loop {
        if next_step == "ZZZ" {
            break n_steps;
        }
        n_steps += 1;
        if let Some(s) = steps.next() {
            next_step = match s {
                'L' => &network.get(next_step).unwrap().0,
                'R' => &network.get(next_step).unwrap().1,
                _ => panic!("should be either L or R"),
            }
        }
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
fn sample_input1() {
    let input = "\
        RL\n\
        \n\
        AAA = (BBB, CCC)\n\
        BBB = (DDD, EEE)\n\
        CCC = (ZZZ, GGG)\n\
        DDD = (DDD, DDD)\n\
        EEE = (EEE, EEE)\n\
        GGG = (GGG, GGG)\n\
        ZZZ = (ZZZ, ZZZ)\n\
        "
    .as_bytes();
    assert_eq!(steps(input), 2)
}

#[test]
fn sample_input2() {
    let input = "\
        LLR\n\
        \n\
        AAA = (BBB, BBB)\n\
        BBB = (AAA, ZZZ)\n\
        ZZZ = (ZZZ, ZZZ)\n\
        "
    .as_bytes();
    assert_eq!(steps(input), 6)
}
