use std::cmp::{Ord, Ordering};
use std::collections::HashMap;
use std::convert::Into;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::ControlFlow;
use std::time::Instant;

const HAND_SIZE: usize = 5;
const MAX_HANDS: usize = 1000;

fn main() -> io::Result<()> {
    let f = File::open("day07/input")?;
    let begin = Instant::now();
    println!("{}", total(f));
    println!("took: {:?}", Instant::now() - begin);
    Ok(())
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(PartialEq, Eq)]
struct Hand([char; HAND_SIZE], u32); // cards, bid

impl From<&Hand> for HandType {
    fn from(h: &Hand) -> Self {
        let mut card_types = HashMap::with_capacity(h.0.len());
        for c in h.0 {
            card_types
                .entry(c)
                .and_modify(|n| *n += 1)
                .or_insert(1usize);
        }

        match card_types.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_types.contains_key(&'J') {
                    HandType::FiveOfAKind
                } else if *card_types.values().max().unwrap() == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if *card_types.values().max().unwrap() == 3 {
                    if card_types.contains_key(&'J') {
                        HandType::FourOfAKind
                    } else {
                        HandType::ThreeOfAKind
                    }
                } else if let Some(&n) = card_types.get(&'J') {
                    if n == 2 {
                        HandType::FourOfAKind
                    } else {
                        HandType::FullHouse
                    }
                } else {
                    HandType::TwoPair
                }
            }
            4 => {
                if card_types.contains_key(&'J') {
                    HandType::ThreeOfAKind
                } else {
                    HandType::OnePair
                }
            }
            _ => {
                if card_types.contains_key(&'J') {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type: HandType = self.into();
        let other_type: HandType = other.into();

        match self_type.cmp(&other_type) {
            Ordering::Equal => {
                let cmp = self.0.into_iter().zip(other.0).try_for_each(|cards| {
                    fn card_value(c: char) -> u8 {
                        match c {
                            'J' => 0,
                            '2'..='9' => c as u8 - b'1',
                            'T' => 9,
                            'Q' => 11,
                            'K' => 12,
                            'A' => 13,
                            _ => u8::MAX,
                        }
                    }
                    match card_value(cards.0).cmp(&card_value(cards.1)) {
                        Ordering::Equal => ControlFlow::Continue(()),
                        o => ControlFlow::Break(o),
                    }
                });
                if let ControlFlow::Break(cmp) = cmp {
                    cmp
                } else {
                    Ordering::Equal
                }
            }
            o => o,
        }
    }
}

fn total<R: io::Read>(r: R) -> u32 {
    let mut lines = BufReader::new(r).lines();

    let mut hands = Vec::with_capacity(MAX_HANDS);
    while let Some(Ok(l)) = lines.next() {
        hands.push(parse_draw(l));
    }
    hands.sort();

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, e| acc + (e.0 as u32 + 1) * e.1 .1)
}

fn parse_draw(l: String) -> Hand {
    let mut split = l.split(' ');

    let cards = split
        .next()
        .expect("line should start with hand string")
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .expect("hand should be convertible into and array of 5 chars");

    let bid = split
        .next()
        .expect("line should end with bid string")
        .parse()
        .expect("bid string should be parseable as u32");

    Hand(cards, bid)
}

#[test]
fn sample_input() {
    let input = "\
        32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483\n\
        "
    .as_bytes();
    assert_eq!(total(input), 5905)
}
