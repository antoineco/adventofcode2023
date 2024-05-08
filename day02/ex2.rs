use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("day02/input")?;
    println!("{}", sum(f));
    Ok(())
}

fn sum<R: io::Read>(r: R) -> u32 {
    let r = BufReader::new(r);

    let mut cubes = String::with_capacity(2);
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

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
        min_red = 0;
        min_green = 0;
        min_blue = 0;

        while let Some(c) = iter.next() {
            match c {
                '0'..='9' => {
                    cubes.push(c);
                }
                ' ' => {
                    match iter.next().unwrap() {
                        'r' => {
                            let n = cubes.parse::<u32>().unwrap();
                            if n > min_red {
                                min_red = n;
                            }
                            cubes.clear();
                            iter.nth("red".len() - 2);
                        }
                        'g' => {
                            let n = cubes.parse::<u32>().unwrap();
                            if n > min_green {
                                min_green = n;
                            }
                            cubes.clear();
                            iter.nth("green".len() - 2);
                        }
                        'b' => {
                            let n = cubes.parse::<u32>().unwrap();
                            if n > min_blue {
                                min_blue = n;
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

        acc + (min_red * min_green * min_blue)
    })
}

#[test]
fn zero_no_green() {
    let input = "\
        Game 1: 10 blue; 1 blue\n\
        Game 2: 10 blue, 1 red, 2 green\n\
        Game 3: 1 blue, 2 red, 3 green\n\
        "
    .as_bytes();
    assert_eq!(sum(input), 26)
}
