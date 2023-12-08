use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

/// INPUT PARSES

/// Read file filename into a vector, with each line as on element.
fn parse_inputfile(filename: &String) -> Vec<String> {
    if filename.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        if !line.is_empty() {
            result.push(line.to_string().trim().to_string());
        }
    }
    result
}

fn get_filename() -> String {
    for (i, arg) in env::args().enumerate() {
        if i == 1 {
            return arg.to_string();
        }
    }
    String::new()
}

#[derive(PartialEq, Eq)]
struct Card {
    val: u32,
    letter: char,
}

impl Card {
    fn from_char(c: char) -> Card {
        let mut val: u32 = 0;
        if (c >= '0') && (c <= '9') {
            val = c.to_string().parse().unwrap();
        } else {
            match c {
                'T' => {
                    val = 10;
                }
                'J' => {
                    val = 11;
                }
                'Q' => {
                    val = 12;
                }
                'K' => {
                    val = 13;
                }
                'A' => {
                    val = 14;
                }
                _ => {
                    assert!(false);
                }
            }
        }

        Card {
            val: val,
            letter: c,
        }
    }

    fn from_char_as_jokers(c: char) -> Card {
        let mut val: u32 = 0;
        if (c >= '0') && (c <= '9') {
            val = c.to_string().parse().unwrap();
        } else {
            match c {
                'T' => {
                    val = 10;
                }
                'J' => {
                    val = 1;
                }
                'Q' => {
                    val = 12;
                }
                'K' => {
                    val = 13;
                }
                'A' => {
                    val = 14;
                }
                _ => {
                    assert!(false);
                }
            }
        }

        Card {
            val: val,
            letter: c,
        }
    }
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    joker_count: u32,
    h_map: HashMap<u32, u32>,
}

fn to_hashmap(cards: &Vec<Card>, joker_count: u32) -> HashMap<u32, u32> {
    let mut valmap: HashMap<u32, u32> = HashMap::new();
    for card in cards.iter() {
        let existing = valmap.get(&card.val);
        if (joker_count > 0) && card.letter == 'J' {
            continue;
        }
        if existing.is_some() {
            valmap.insert(card.val, existing.unwrap() + 1);
        } else {
            valmap.insert(card.val, 1);
        }
    }
    valmap
}

impl Hand {
    fn from_string(input: &String) -> Hand {
        let mut cards: Vec<Card> = Vec::new();
        let mut h_b = input.split_whitespace();

        for c in h_b.clone().nth(0).unwrap().chars() {
            cards.push(Card::from_char(c));
        }

        let bid: u32 = h_b.nth(1).unwrap().parse().unwrap();
        let hmap: HashMap<u32, u32> = to_hashmap(&cards, 0);

        Hand {
            cards: cards,
            bid: bid,
            joker_count: 0,
            h_map: hmap,
        }
    }
    fn from_string_as_jokers(input: &String) -> Hand {
        let mut cards: Vec<Card> = Vec::new();
        let mut h_b = input.split_whitespace();
        let mut joker_count = 0;

        for c in h_b.clone().nth(0).unwrap().chars() {
            if c == 'J' {
                joker_count += 1;
            }
            cards.push(Card::from_char_as_jokers(c));
        }

        let bid: u32 = h_b.nth(1).unwrap().parse().unwrap();
        let h_map: HashMap<u32, u32> = to_hashmap(&cards, joker_count);

        Hand {
            cards: cards,
            bid: bid,
            joker_count: joker_count,
            h_map: h_map,
        }
    }

    fn max_same(&self) -> u32 {
        let mmap = &self.h_map;
        let key = mmap.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k);
        if key.is_none() {
            return 0;
        }

        mmap.get(key.unwrap()).unwrap().clone()
    }

    fn to_string(&self) -> String {
        let mut r = String::new();
        for card in self.cards.iter() {
            r += &card.letter.to_string();
        }
        format!("{} {}", r, self.bid.to_string())
    }
    fn print(&self) {
        println!("{}", self.to_string());
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let same_s = self.max_same() + self.joker_count;
        let same_o = other.max_same() + other.joker_count;

        if same_s > same_o {
            return Ordering::Greater;
        }
        if same_s < same_o {
            return Ordering::Less;
        }

        if (same_s == 2) || (same_s == 3) {
            let c_s = self.h_map.len();
            let c_o = other.h_map.len();
            if c_s != c_o {
                return if c_s < c_o {
                    Ordering::Greater
                } else {
                    Ordering::Less
                };
            }
        }

        for i in 0..self.cards.len() {
            let sv = self.cards[i].val;
            let ov = other.cards[i].val;
            if sv > ov {
                return Ordering::Greater;
            }
            if ov > sv {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

fn main() {
    let input: Vec<String> = parse_inputfile(&get_filename());
    let mut hands: Vec<Hand> = Vec::new();

    let part1_start: std::time::Instant = std::time::Instant::now();

    for line in input.iter() {
        hands.push(Hand::from_string(line));
    }

    hands.sort();

    let mut score: u32 = 0;

    for (i, hand) in hands.iter().enumerate() {
        score += (i as u32 + 1) * hand.bid;
    }

    println!("Part 1 took: {:?}, score {}", part1_start.elapsed(), score);

    score = 0;
    let mut hands: Vec<Hand> = Vec::new();

    let part2_start: std::time::Instant = std::time::Instant::now();

    for line in input.iter() {
        hands.push(Hand::from_string_as_jokers(line));
    }
    hands.sort();

    for (i, hand) in hands.iter().enumerate() {
        score += (i as u32 + 1) * hand.bid;
    }

    println!("Part 2 took {:?}. Score: {}", part2_start.elapsed(), score);
}
