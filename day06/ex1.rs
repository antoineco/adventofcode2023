use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::ControlFlow;
use std::time::Instant;

fn main() -> io::Result<()> {
    let f = File::open("day06/input")?;
    let begin = Instant::now();
    println!("{}", product(f));
    println!("took: {:?}", Instant::now() - begin);
    Ok(())
}

fn product<R: io::Read>(r: R) -> u32 {
    let mut lines = BufReader::new(r).lines();

    let hdr_col_len = "Distance:".len();
    let times = lines.next().unwrap().unwrap()[hdr_col_len..]
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<_>>();
    let dists = lines.next().unwrap().unwrap()[hdr_col_len..]
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<_>>();

    times
        .iter()
        .zip(dists.iter())
        .map(|race| {
            let race_time = *race.0;
            let race_dist = *race.1;

            /* Alternative: terse bruteforce using map().sum()
             *
             * (1..race_time)
             *     .map(|press_time| is_win(press_time, race_time, race_dist) as u32)
             *     .sum()
             */

            if let ControlFlow::Break(first_win) = (1..race_time).try_for_each(|press_time| {
                if is_win(press_time, race_time, race_dist) {
                    ControlFlow::Break(press_time)
                } else {
                    ControlFlow::Continue(())
                }
            }) {
                (race_time + 1) - first_win * 2
            } else {
                0
            }
        })
        .product()
}

fn is_win(press_time: u32, race_time: u32, race_dist: u32) -> bool {
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
    assert_eq!(product(input), 288)
}
