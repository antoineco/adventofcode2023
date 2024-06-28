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

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Range(u64, u64); // [rng_start, rng_end)

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct MapRange(u64, u64, usize); // src_start, dst_start, range_len

// Required to avoid error:
//    cannot define inherent `impl` for a type outside of the crate where the type is defined
trait Mapper {
    fn new() -> Self;
    fn map(&self, input: Vec<Range>) -> Vec<Range>;
}

type MapRanges = Vec<MapRange>;

impl Mapper for MapRanges {
    fn new() -> Self {
        const MAX_MAP_SIZE: usize = 50;
        Vec::with_capacity(MAX_MAP_SIZE)
    }

    fn map(&self, mut input: Vec<Range>) -> Vec<Range> {
        let mut transformed_rngs = Vec::with_capacity(input.len() * 2); // empirical cap value
        for mr in self {
            if input.is_empty() {
                break;
            }
            // worst case: before+after ranges for each input
            let mut next_rngs = Vec::with_capacity(2 * input.len());
            while let Some(i) = input.pop() {
                /* Visualization of ranges:
                 * https://www.reddit.com/r/adventofcode/comments/18b82w0/2023_day_5_part_2_visualizing_all_the_mapping/
                 *
                 * Possible cases:
                 *
                 * mr.src range fully contained inside i range
                 *     [i_start                                            i_end]
                 *                       [src_start      src_end]
                 *     [BEFORE          ][INTER                 ][AFTER         ]
                 *
                 * mr.src range overlaps i range on one side
                 *               [i_start                                  i_end]
                 *     [src_start      src_end]
                 *              x[INTER       ][AFTER                           ]
                 *
                 * mr.src range fully contains i range
                 *                       [i_start          i_end]
                 *     [src_start                                        src_end]
                 *                      x[INTER                 ]x
                 *
                 * mr.src range and i range do not overlap
                 *     [i_start        i_end]
                 *                              [src_start               src_end]
                 *     [BEFORE              ]x
                 *
                 * Inter ranges are shifted and returned as input for the next Mapper.
                 *
                 * Before/after ranges are either:
                 *   - tried against the next range in the current Mapper.
                 *   - returned as input for the next Mapper, if no more range is to be tried in
                 *     the current one (no map = same dest per puzzle description).
                 */

                // In all of the cases below, the range is considered empty if start >= end.
                let before = Range(i.0, i.1.min(mr.0));
                let inter = Range(i.0.max(mr.0), i.1.min(mr.0 + mr.2 as u64));
                let after = Range(i.0.max(mr.0 + mr.2 as u64), i.1);

                if before.1 > before.0 {
                    next_rngs.push(before);
                }
                if inter.1 > inter.0 {
                    // shifted inner range
                    transformed_rngs.push(Range(mr.1 + (inter.0 - mr.0), mr.1 + (inter.1 - mr.0)));
                }
                if after.1 > after.0 {
                    next_rngs.push(after);
                }
            }
            input = next_rngs; // reuse outer variable as input for the next range
        }
        [transformed_rngs, input].concat()
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
                        Ok(v) => seed_rngs.push(Range(seed, seed + v as u64)),
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
            Ok(v) => seed_rngs.push(Range(seed, seed + v as u64)),
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

    seed_rngs
        .iter()
        .map(|r| {
            maps.iter()
                .fold(vec![r.clone()], |acc, e| e.map(acc))
                .iter()
                .min()
                .unwrap()
                .0
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
