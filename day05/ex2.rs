use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const MAX_SEEDS: usize = 20;
const MAX_SEED_PAIRS: usize = MAX_SEEDS / 2;
const MAX_N_DIGITS: usize = 10;

fn main() -> io::Result<()> {
    let f = File::open("day05/input")?;
    let begin = Instant::now();
    println!("{}", lowest(f));
    println!("took: {:?}", Instant::now() - begin);
    Ok(())
}

struct SeedRange(u64, usize); // seed, range_len

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct MapRange(u64, u64, usize); // src_start, dst_start, range_len

// Required to avoid error:
//    cannot define inherent `impl` for a type outside of the crate where the type is defined
trait Mapper {
    fn new() -> Self;
    fn map(&self, src: u64) -> u64;
}

type MapRanges = Vec<MapRange>;

impl Mapper for MapRanges {
    fn new() -> Self {
        const MAX_MAP_SIZE: usize = 50;
        Vec::with_capacity(MAX_MAP_SIZE)
    }

    fn map(&self, src: u64) -> u64 {
        for mr in self {
            if src < mr.0 {
                // src is between two ranges. Ranges are sorted and do not overlap.
                return src;
            } else if src <= mr.0 + (mr.2 - 1) as u64 {
                return mr.1 + (src - mr.0);
            }
        }
        src
    }
}

fn lowest<R: io::Read>(r: R) -> u64 {
    let mut lines = BufReader::new(r).lines();

    let mut seed_rngs = Vec::with_capacity(MAX_SEED_PAIRS);

    let mut seed_str: String = String::with_capacity(MAX_N_DIGITS);
    if let Some(Ok(l_seeds)) = lines.next() {
        let mut seed_chars = l_seeds.chars();
        seed_chars.nth("seeds:".len());

        let mut seed: u64 = u64::MAX;
        for c in seed_chars {
            if c == ' ' {
                if seed == u64::MAX {
                    seed = match seed_str.parse() {
                        Ok(v) => v,
                        Err(e) => panic!("{e}"),
                    }
                } else {
                    match seed_str.parse::<usize>() {
                        Ok(v) => seed_rngs.push(SeedRange(seed, v)),
                        Err(e) => panic!("{e}"),
                    };
                    seed = u64::MAX;
                }
                seed_str.clear();
                continue;
            }
            seed_str.push(c);
        }
        match seed_str.parse::<usize>() {
            Ok(v) => seed_rngs.push(SeedRange(seed, v)),
            Err(e) => panic!("{e}"),
        };
    }
    lines.nth(1); // step over the first "SRC-to-DST map:" line

    let maps: [&mut MapRanges; 7] = [
        &mut Mapper::new(), // seed to soil
        &mut Mapper::new(), // soil to fertilizer
        &mut Mapper::new(), // fertilizer to water
        &mut Mapper::new(), // water to light
        &mut Mapper::new(), // light to temperature
        &mut Mapper::new(), // temperature to humidity
        &mut Mapper::new(), // humidity to location
    ];

    let mut cur_map = 0;
    let mut n_str = String::with_capacity(MAX_N_DIGITS);
    while let Some(Ok(l)) = lines.next() {
        if l.is_empty() {
            maps[cur_map].sort();
            cur_map += 1;
            lines.next(); // causes the next iteration to step over the "SRC-to-DST map:" line
            continue;
        }
        n_str.clear();

        let mut src_start: u64 = 0;
        let mut dst_start: u64 = 0;

        let mut parsed_first = false;
        for c in l.chars() {
            if c == ' ' {
                let v: u64 = match n_str.parse() {
                    Ok(v) => v,
                    Err(e) => panic!("{e}"),
                };
                if parsed_first {
                    src_start = v;
                } else {
                    parsed_first = true;
                    dst_start = v;
                };
                n_str.clear();
            } else {
                n_str.push(c);
            }
        }

        let range_len: usize = match n_str.parse() {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        };

        maps[cur_map].push(MapRange(src_start, dst_start, range_len));
    }
    maps[cur_map].sort();

    // TODO: optimize runtime, ran in 3m0s
    seed_rngs
        .iter()
        .map(|r| {
            (r.0..(r.0 + r.1 as u64))
                .map(|s| maps.iter().fold(s, |acc, e| e.map(acc)))
                .min()
                .unwrap_or_default()
        })
        .min()
        .unwrap_or_default()
}

#[test]
fn sample_input_no_gaps() {
    let input = "\
        seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4\n\
        "
    .as_bytes();
    assert_eq!(lowest(input), 46)
}
