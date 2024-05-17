use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const NUM_LINES: usize = 140;
const LINE_LENGTH: usize = 140;
const MAX_NUM_LEN: usize = 3;

fn main() -> io::Result<()> {
    let f = File::open("day03/input")?;
    let begin = Instant::now();
    println!("{}", sum(f));
    println!("took: {:?}", Instant::now() - begin);
    Ok(())
}

type Symbols = HashSet<i16 /*char idx*/>;
type Numbers = Vec<((usize /*start idx*/, usize /*end idx*/), String)>;

fn sum<R: io::Read>(r: R) -> u32 {
    let r = BufReader::new(r);
    let mut iter = r.lines();

    let mut cur_num_start = usize::MAX;
    let mut cur_num_str = String::with_capacity(MAX_NUM_LEN);

    let symbols_numbers: [(Symbols, Numbers); NUM_LINES] = core::array::from_fn(|_| {
        match iter.next() {
            None => (HashSet::new(), Vec::new()), // tests cannot realistically be NUM_LINES long
            Some(line) => match line {
                Ok(line) => {
                    // In the worst case, every character is a symbol
                    let mut symbols = HashSet::with_capacity(LINE_LENGTH);
                    // In the worst case, only single digit numbers separated by symbols
                    let mut numbers = Vec::with_capacity(LINE_LENGTH / 2);

                    for (i, b) in line.bytes().enumerate() {
                        match b {
                            b'0'..=b'9' => {
                                if cur_num_str.is_empty() {
                                    cur_num_start = i;
                                }
                                cur_num_str.push(char::from(b));
                            }
                            b'!'..=b'-' | b'/' | b':'..=b'@' => {
                                symbols.insert(i as i16);
                                if !cur_num_str.is_empty() {
                                    numbers.push(((cur_num_start, i - 1), cur_num_str.clone()));
                                    cur_num_str.clear();
                                }
                            }
                            b'.' => {
                                if !cur_num_str.is_empty() {
                                    numbers.push(((cur_num_start, i - 1), cur_num_str.clone()));
                                    cur_num_str.clear();
                                }
                            }
                            _ => panic!("unexpected char"),
                        };
                    }

                    if !cur_num_str.is_empty() {
                        numbers.push(((cur_num_start, line.len() - 1), cur_num_str.clone()));
                        cur_num_str.clear();
                    }

                    (symbols, numbers)
                }
                Err(e) => panic!("{e}"),
            },
        }
    });

    symbols_numbers
        .iter()
        .enumerate()
        .fold(0u32, |acc, (l, sn)| {
            let mut acc_line = 0;
            for num in &sn.1 {
                // Cheap optimized case if the number has a symbol directly to its left or right.
                if sn.0.contains(&(num.0 .0 as i16 - 1)) || sn.0.contains(&(num.0 .1 as i16 + 1)) {
                    acc_line += num.1.parse::<u32>().unwrap();
                } else {
                    // Numbers are at most 3 digits long, so we iterate at most 5 times per number here.
                    for i in num.0 .0 as i16 - 1..=num.0 .1 as i16 + 1 {
                        if (l > 0 && symbols_numbers[l - 1].0.contains(&i))
                            || (l < symbols_numbers.len() - 1
                                && symbols_numbers[l + 1].0.contains(&i))
                        {
                            acc_line += num.1.parse::<u32>().unwrap();
                            break;
                        }
                    }
                }
            }
            acc + acc_line
        })
}

#[test]
fn sample_input_with_sandwiched_numbers() {
    let input = "\
        467#.114..\n\
        ...*......\n\
        ..35..633.\n\
        ...#..#...\n\
        617*......\n\
        .....+..58\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..\n\
        "
    .as_bytes();
    assert_eq!(sum(input), 4361)
}
