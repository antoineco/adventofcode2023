use core::panic;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    let f = File::open("day06/input")?;
    let begin = Instant::now();
    println!("{}", ways(f));
    println!("took: {:?}", Instant::now() - begin);
    Ok(())
}

fn ways<R: io::Read>(r: R) -> u64 {
    let mut lines = BufReader::new(r).lines();

    let hdr_col_len = "Distance:".len();
    let race_time = match lines.next().unwrap().unwrap()[hdr_col_len..]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
    {
        Ok(v) => v,
        Err(e) => panic!("{e}"),
    };
    let race_dist = match lines.next().unwrap().unwrap()[hdr_col_len..]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
    {
        Ok(v) => v,
        Err(e) => panic!("{e}"),
    };

    /* Alternative: terse bruteforce using map().sum()
     *
     * (1..race_time)
     *     .map(|press_time| is_win(press_time, race_time, race_dist) as u64)
     *     .sum()
     */

    let mut i = 0;
    let mut j = race_time / 2;
    let first_win = loop {
        if i > j {
            break 0;
        }
        let mid = i + (j - i) / 2;
        if is_win(mid, race_time, race_dist) {
            if !is_win(mid - 1, race_time, race_dist) {
                break mid;
            }
            j = mid;
        } else {
            if is_win(mid + 1, race_time, race_dist) {
                break mid + 1;
            }
            i = mid;
        }
    };
    (race_time + 1) - first_win * 2
}

fn is_win(press_time: u64, race_time: u64, race_dist: u64) -> bool {
    let speed = press_time;
    let move_time = race_time - press_time;
    move_time * speed > race_dist
}

#[test]
fn sample_input() {
    let input = "\
        Time:      7  15   30\n\
        Distance:  9  40  200\n\
        "
    .as_bytes();
    assert_eq!(ways(input), 71503)
}
