use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

const MAX_DIGITS_PER_LINE: usize = 10;

fn main() -> io::Result<()> {
    let f = File::open("day01/input")?;
    println!("{}", sum(f));
    Ok(())
}

fn sum<R: io::Read>(r: R) -> u64 {
    let r = BufReader::new(r);
    let mut digits = Vec::with_capacity(MAX_DIGITS_PER_LINE);
    r.lines().fold(0, |acc, l| {
        let line = l.unwrap();
        let mut iter = line.chars().enumerate();
        digits.clear();
        // Possible improvement: interrupt the iteration and iterate again in reverse as soon as we
        // find the first digit.
        while let Some((i, c)) = iter.next() {
            if c.is_ascii_digit() {
                digits.push((c as u8 - b'0') as u64);
                continue;
            }
            // The set of possible words is small (lowercase single digit numbers in English), so
            // we can reasonably enumerate all possible substrings based on the currently visited
            // character, and swallow the rest of the word if we find a match.
            let remain_chars = line[i..].len();
            if remain_chars.lt(&3) {
                continue;
            }
            if let Some(n) = match c {
                'o' => {
                    if "one" == &line[i..i + 3] {
                        iter.next(); // don't swallow 'e', it could be the start of "eight"
                        Some(1)
                    } else {
                        None
                    }
                }
                't' => {
                    if remain_chars.ge(&5) && "three" == &line[i..i + 5] {
                        iter.nth(2); // don't swallow 'e', it could be the start of "eight"
                        Some(3)
                    } else if "two" == &line[i..i + 3] {
                        iter.next(); // don't swallow 'o', it could be the start of "one"
                        Some(2)
                    } else {
                        None
                    }
                }
                'f' => {
                    if remain_chars.ge(&4) {
                        if "four" == &line[i..i + 4] {
                            iter.nth(2);
                            Some(4)
                        } else if "five" == &line[i..i + 4] {
                            iter.nth(1); // don't swallow 'e', it could be the start of "eight"
                            Some(5)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                's' => {
                    if remain_chars.ge(&5) && "seven" == &line[i..i + 5] {
                        iter.nth(2); // don't swallow 'n', it could be the start of "nine"
                        Some(7)
                    } else if "six" == &line[i..i + 3] {
                        iter.nth(1);
                        Some(6)
                    } else {
                        None
                    }
                }
                'e' => {
                    if remain_chars.ge(&5) && "eight" == &line[i..i + 5] {
                        iter.nth(2); // don't swallow 't', it could be the start of {"two","three"}
                        Some(8)
                    } else {
                        None
                    }
                }
                'n' => {
                    if remain_chars.ge(&4) && "nine" == &line[i..i + 4] {
                        iter.nth(1); // don't swallow 'e', it could be the start of "eight"
                        Some(9)
                    } else {
                        None
                    }
                }
                _ => None,
            } {
                digits.push(n)
            };
        }
        if let Some(first) = digits.first() {
            acc + first * 10 + digits.last().unwrap()
        } else {
            acc
        }
    })
}

#[test]
fn mixed_patterns() {
    let input = "x2xxoneeight\n2xtwo\nonexx21x\n".as_bytes();
    assert_eq!(sum(input), 61);
}
