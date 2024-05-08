use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("day01/input")?;
    println!("{}", sum(f));
    Ok(())
}

fn sum<R: io::Read>(r: R) -> u64 {
    let r = BufReader::new(r);
    r.lines().fold(0, |acc, l| {
        let line = l.unwrap();
        // Possible improvement: interrupt the iteration and iterate again in reverse as soon as we
        // find the first digit.
        let mut digits = line.chars().filter_map(|c| {
            if c.is_ascii_digit() {
                Some((c as u8 - b'0') as u64)
            } else {
                None
            }
        });
        if let Some(first) = digits.next() {
            acc + first * 10
                + if let Some(last) = digits.last() {
                    last
                } else {
                    first
                }
        } else {
            acc
        }
    })
}

#[test]
fn mixed_patterns() {
    let input = "x2xx1\n2x\nxx11x\n".as_bytes();
    assert_eq!(sum(input), 54);
}
