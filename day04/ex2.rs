use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const NUM_LEN: usize = 2;
const MAX_WINNING_NUMS: usize = 10;

fn main() -> io::Result<()> {
    let f = File::open("day04/input")?;
    let begin = Instant::now();
    println!("{}", cards(f));
    println!("took: {:?}", Instant::now() - begin);
    Ok(())
}

fn cards<R: io::Read>(r: R) -> u64 {
    let r = BufReader::new(r);

    let data_start = "Card xxx:".len();

    let mut cur_num = String::with_capacity(NUM_LEN);
    let mut winning_nums: HashSet<u32> = HashSet::with_capacity(MAX_WINNING_NUMS);
    let mut cur_winning = 0;

    let mut cur_copies = 0;
    let mut next_copies = HashMap::with_capacity(MAX_WINNING_NUMS);

    r.lines().enumerate().fold(0, |acc, (i, l)| match l {
        Ok(l) => {
            let mut iter = l.chars();

            iter.nth(data_start - 1);

            winning_nums.clear();
            while iter.next().is_some() {
                let c = iter.next().unwrap();
                if c == '|' {
                    break;
                }
                cur_num.clear();

                if c.is_ascii_digit() {
                    cur_num.push(c);
                }
                cur_num.push(iter.next().unwrap());

                winning_nums.insert(cur_num.parse().unwrap());
            }

            cur_copies = next_copies.remove(&i).unwrap_or(0) + 1;

            cur_winning = 0;
            while iter.next().is_some() {
                cur_num.clear();

                let c = iter.next().unwrap();
                if c.is_ascii_digit() {
                    cur_num.push(c);
                }
                cur_num.push(iter.next().unwrap());

                if winning_nums.contains(&cur_num.parse().unwrap()) {
                    cur_winning += 1;
                    next_copies
                        .entry(i + cur_winning)
                        .and_modify(|n| *n += cur_copies)
                        .or_insert(cur_copies);
                }
            }

            acc + cur_copies
        }
        Err(e) => panic!("{e}"),
    })
}

#[test]
fn sample_input() {
    let input = "\
        Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card   2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card   3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card   4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card   5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card   6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n\
        "
    .as_bytes();
    assert_eq!(cards(input), 30)
}
