use std::time::Instant;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

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

type Symbols = Vec<usize /*symbol idx*/>;
type Numbers = Vec<((usize /*start idx*/, usize /*end idx*/), String)>;

fn sum<R: io::Read>(r: R) -> u32 {
    let r = BufReader::new(r);
    let mut iter = r.lines();

    let mut cur_num_start = usize::MAX;
    let mut cur_num_str = String::with_capacity(MAX_NUM_LEN);

    let symbols_numbers: [(Symbols, Numbers); NUM_LINES] = core::array::from_fn(|_| {
        match iter.next() {
            None => (Vec::new(), Vec::new()), // tests cannot realistically be NUM_LINES long
            Some(line) => match line {
                Ok(line) => {
                    // In the worst case, every character is a symbol
                    let mut symbols = Vec::with_capacity(LINE_LENGTH);
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
                            b'*' => {
                                symbols.push(i);
                                if !cur_num_str.is_empty() {
                                    numbers.push(((cur_num_start, i - 1), cur_num_str.clone()));
                                    cur_num_str.clear();
                                }
                            }
                            b'!'..=b')' | b'+'..=b'/' | b':'..=b'@' => {
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

    let mut surrounding_nums = Vec::with_capacity(2);

    symbols_numbers
        .iter()
        .enumerate()
        .fold(0u32, |acc, (l, sn)| {
            let mut acc_line = 0;
            'sym: for sym in &sn.0 {
                surrounding_nums.clear();
                // Possible improvement: use a data structure which allows indexing the positions
                // of numbers to avoid iterating all numbers found in surrounding lines.
                // This would however be an overkill for the given input because:
                // - The count of numbers per line is relatively small (the longest line has 15 numbers).
                // - We break out of the outer loop as soon as we discover more than 2 surrounding numbers.
                // - We break out of inner loops as soon as a number is further to the right than the evaluated symbol.
                if l > 0 {
                    for num in &symbols_numbers[l - 1].1 {
                        if num.0 .0 > sym + 1 {
                            // Cheap optimization, only interesting when there are a lot of numbers
                            // on the right side of the symbol.
                            break;
                        }
                        if num.0 .0 as i16 - 1 <= *sym as i16 && *sym as i16 <= num.0 .1 as i16 + 1
                        {
                            if surrounding_nums.len() == 2 {
                                break 'sym;
                            }
                            surrounding_nums.push(num);
                        }
                    }
                }
                for num in &sn.1 {
                    if num.0 .0 > sym + 1 {
                        // Cheap optimization, only interesting when there are a lot of numbers
                        // on the right side of the symbol.
                        break;
                    }
                    if num.0 .0 as i16 - 1 <= *sym as i16 && *sym as i16 <= num.0 .1 as i16 + 1 {
                        if surrounding_nums.len() == 2 {
                            break 'sym;
                        }
                        surrounding_nums.push(num);
                    }
                }
                if l < symbols_numbers.len() - 1 {
                    for num in &symbols_numbers[l + 1].1 {
                        if num.0 .0 > sym + 1 {
                            // Cheap optimization, only interesting when there are a lot of numbers
                            // on the right side of the symbol.
                            break;
                        }
                        if num.0 .0 as i16 - 1 <= *sym as i16 && *sym as i16 <= num.0 .1 as i16 + 1
                        {
                            if surrounding_nums.len() == 2 {
                                break 'sym;
                            }
                            surrounding_nums.push(num);
                        }
                    }
                }
                if surrounding_nums.len() == 2 {
                    acc_line += surrounding_nums
                        .iter()
                        .fold(1, |acc, num| acc * num.1.parse::<u32>().unwrap());
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
    assert_eq!(sum(input), 467835)
}
