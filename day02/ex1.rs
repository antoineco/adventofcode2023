use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

const MAX_REDS: u8 = 12;
const MAX_GREENS: u8 = 13;
const MAX_BLUES: u8 = 14;

fn main() -> io::Result<()> {
    let f = File::open("day02/input")?;
    println!("{}", sum(f));
    Ok(())
}

fn sum<R: io::Read>(r: R) -> u32 {
    let r = BufReader::new(r);
    let mut cubes = String::with_capacity(2);
    r.lines().enumerate().fold(0, |acc, (i, l)| {
        let line = l.unwrap();
        let mut iter = line.chars();

        // swallow "Game <id>: "
        iter.nth(
            "Game :".len()
                + if i + 1 < 10 {
                    1
                } else if i + 1 < 100 {
                    2
                } else {
                    3
                },
        );

        cubes.clear();
        while let Some(c) = iter.next() {
            match c {
                '0'..='9' => {
                    cubes.push(c);
                }
                ' ' => {
                    match iter.next().unwrap() {
                        'r' => {
                            if cubes.parse::<u8>().unwrap() > MAX_REDS {
                                return acc;
                            }
                            cubes.clear();
                            iter.nth("red".len() - 2);
                        }
                        'g' => {
                            if cubes.parse::<u8>().unwrap() > MAX_GREENS {
                                return acc;
                            }
                            cubes.clear();
                            iter.nth("green".len() - 2);
                        }
                        'b' => {
                            if cubes.parse::<u8>().unwrap() > MAX_BLUES {
                                return acc;
                            }
                            cubes.clear();
                            iter.nth("blue".len() - 2);
                        }
                        _ => (),
                    };
                }
                ',' | ';' => {
                    iter.next();
                }
                _ => (),
            }
        }

        acc + (i + 1) as u32
    })
}

#[test]
fn basic() {
    let input = "\
        Game 1: 10 blue; 99 blue\n\
        Game 2: 10 blue, 1 red\n\
        Game 3: 1 blue, 10 red, 99 green\n\
        "
    .as_bytes();
    assert_eq!(sum(input), 2)
}
