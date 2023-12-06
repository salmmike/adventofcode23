use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
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

fn parse_seeds(input: &Vec<String>) -> Vec<u64> {
    let mut seeds = Vec::new();
    for line in input.iter() {
        if line.starts_with("seeds:") {
            let seedstring = line.split(':').nth(1).unwrap();
            for seed in seedstring.split(' ') {
                let val = seed.trim().parse();
                if val.is_ok() {
                    seeds.push(val.unwrap());
                }
            }
        }
    }
    seeds
}

fn parse_map_name(line: &String) -> String {
    line.split(' ').nth(0).unwrap().to_string()
}

fn _print_seeds(input: &Vec<u64>) {
    print!("seeds:");
    for i in input.iter() {
        print!(" {}", i);
    }
    println!("");
}

fn _print_map(map: &MapEntry) {
    println!("{} {} {}", map.source, map.destination, map.range);
}

fn _print_maps(input: &Vec<ValueMap>) {
    for map in input.iter() {
        println!("\n{} map:", map.name);
        for values in map.map_entry.iter() {
            _print_map(&values);
        }
    }
}

fn parse_seed_map(line: &String) -> Option<MapEntry> {
    if (line.is_empty())
        | (line.chars().nth(0).unwrap() < '0')
        | (line.chars().nth(0).unwrap() > '9')
    {
        return None;
    }
    let vals: std::str::Split<'_, char> = line.split(' ');

    let mut source = 0;
    let mut dest = 0;
    let mut range = 0;

    for (i, val) in vals.clone().enumerate() {
        match i {
            0 => {
                dest = val.parse().unwrap();
            }
            1 => source = val.parse().unwrap(),
            2 => range = val.parse().unwrap(),
            _ => {}
        }
    }

    Some(MapEntry {
        destination: dest,
        source: source,
        range: range,
    })
}


fn get_seedmaps(input: &Vec<String>) -> Vec<ValueMap> {
    let mut seedmaps: Vec<ValueMap> = Vec::new();

    //_print_seeds(&seeds);
    let mut current_map: Option<ValueMap> = None;


    for line in input.iter() {
        if line.ends_with("map:") {
            if current_map.is_some() {
                seedmaps.push(current_map.unwrap());
            }
            current_map = Some(ValueMap {
                name: parse_map_name(&line),
                map_entry: Vec::new(),
            })
        }

        let res: Option<MapEntry> = parse_seed_map(line);

        if res.is_some() {
            let map: MapEntry = res.unwrap();
            current_map.as_mut().unwrap().map_entry.push(map);
        }
    }

    seedmaps.push(current_map.unwrap());
    seedmaps

}

/// STRUCTURES

// Map format is: Source Destination Range (S, D, R)
struct MapEntry {
    source: u64,
    destination: u64,
    range: u64,
}

impl MapEntry {
    fn source_end(&self) -> u64 {
        self.source + self.range
    }
    fn effect_range(&self) -> ValueRange {
        ValueRange::new(self.source, self.source_end())
    }
}

struct ValueMap {
    name: String,
    map_entry: Vec<MapEntry>,
}

#[derive(Clone, PartialEq, Eq)]
struct ValueRange {
    start: u64,
    end: u64,
}

impl ValueRange {
    fn new(start: u64, end: u64) -> ValueRange {
        assert!(start <= end);
        ValueRange {
            start: start,
            end: end,
        }
    }

    fn overlaps(&self, other: &ValueRange) -> bool {
        return in_between(self.start, other.start, self.end)
            || in_between(other.start, self.start, other.end)
            || in_between(other.start, self.end, other.end);
    }

    fn merge(&mut self, other: &ValueRange) {
        self.start = min(self.start, other.start);
        self.end = max(self.end, other.end);
    }
    fn print(&self) {
        println!("[{} - {}]", self.start, self.end);
    }
    fn _to_string(&self) -> String {
        return format!("[{} - {}]", self.start, self.end).to_string();
    }
}

impl PartialOrd for ValueRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.start.cmp(&other.start))
    }
}

impl Ord for ValueRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

#[derive(Clone, PartialEq, Eq)]
struct RangeCollection {
    ranges: Vec<ValueRange>,
}

impl RangeCollection {
    fn sort(&mut self) {
        self.ranges.sort();
    }

    fn add(&mut self, range: &ValueRange) {
        if self.ranges.len() == 0 {
            self.ranges.push(range.clone());
        } else {
            let mut was_merged = false;
            for mrg in self.ranges.iter_mut() {
                if mrg.overlaps(range) {
                    mrg.merge(range);
                    was_merged = true;
                }
            }
            if !was_merged {
                self.ranges.push(range.clone());
            }
        }
        self.sort();
        self.merge_all();
    }

    fn addv(&mut self, ranges: &Vec<ValueRange>) {
        for range in ranges.iter() {
            self.add(range);
        }
    }

    fn new() -> RangeCollection {
        RangeCollection { ranges: Vec::new() }
    }
    fn _from(val: &Vec<ValueRange>) -> RangeCollection {
        RangeCollection {
            ranges: val.clone(),
        }
    }

    fn merge_all(&mut self) {
        let mut new_range: Vec<ValueRange> = Vec::new();
        new_range.push(self.ranges[0].clone());
        for mrg in self.ranges.iter() {
            if new_range.last().unwrap().overlaps(&mrg) {
                new_range.last_mut().unwrap().merge(mrg);
            } else {
                new_range.push(mrg.clone());
            }
        }
        self.ranges = new_range;
    }

    fn print(&self) {
        for range in self.ranges.iter() {
            range.print();
        }
    }
    fn min(&self) -> u64 {
        self.ranges[0].start
    }
}

// ########### HELPER FUNCTIONS ##################

/// Check value c is between s:e
fn in_between(s: u64, c: u64, e: u64) -> bool {
    return (s <= c) && (e >= c);
}

/// Checks that value is within source + range
fn check_in_source_range(value: u64, map_entry: &MapEntry) -> bool {
    return in_between(map_entry.source, value, map_entry.source_end());
}

// Destination X = V + D - S if S <= V <= S + R else V
// If D > S: X >= V
// If D < S: X <= V
fn get_destination(value: u64, map_entry: &MapEntry) -> u64 {
    //println!("Checking value {}: {}, {}, {}", value, map_entry.destination, map_entry.source, map_entry.range);
    let mut res = value;
    if check_in_source_range(value, map_entry) {
        res = value + map_entry.destination - map_entry.source;
    }
    //println!("Destination of {} is {}", value, res);
    res
}

// ############# PROBLEM 1 FUNCTIONS ###############

fn get_next_value(value: u64, next_maps: &Vec<MapEntry>) -> u64 {
    let mut next_location = value;
    for map_entry in next_maps.iter() {
        next_location = get_destination(value, map_entry);
        if next_location != value {
            return next_location;
        }
    }

    next_location
}

/// Go through maps, find the location number for seed.
/// Iterateing over positions, then maps.
fn get_seed_location(seed: u64, maps: &Vec<ValueMap>) -> u64 {
    let mut next_location = seed;

    for map in maps.iter() {
        next_location = get_next_value(next_location, &map.map_entry);
    }
    next_location
}

/// Iterates over
/// seed in seeds {
///     map in maps {
///         map_line in map {
///             next_location = destination from 1st map matching.
///         }
///     }
/// }
fn get_min_location(seeds: &Vec<u64>, seedmaps: &Vec<ValueMap>) -> u64 {
    let mut min_location = 0;

    for seed in seeds.iter() {
        let seed_location = get_seed_location(seed.clone(), &seedmaps);
        if min_location == 0 {
            min_location = seed_location;
        }
        min_location = min(min_location, seed_location);
    }
    min_location
}

// ############ PROBLEM 2 CODE ###################

/// Get ranges of seeds
fn parse_seed_ranges(seeds: Vec<u64>) -> RangeCollection {
    let mut seed_ranges: RangeCollection = RangeCollection::new();
    let mut start: u64 = 0;

    let mut mega_seedmap_len = 0;

    for (i, seed) in seeds.iter().enumerate() {
        if i % 2 == 1 {
            mega_seedmap_len += seed;
            seed_ranges.add(&ValueRange::new(start, start + seed - 1));
        } else {
            start = *seed;
        }
    }
    seed_ranges
}

fn gets_split(vr: &ValueRange, mapping: &MapEntry) -> bool {
    mapping.effect_range().overlaps(vr)
}

/// Split range to multiple ranges, based on mapping_to_next
fn split_range(vr: &ValueRange, map_entry: &MapEntry) -> Vec<ValueRange> {
    let mut res: Vec<ValueRange> = Vec::new();
    // Range before the mapping comes out unchanged:
    if vr.start < map_entry.source {
        res.push(ValueRange::new(vr.start, min(vr.end, map_entry.source - 1)))
    }

    res.push(ValueRange::new(
        get_destination(max(vr.start, map_entry.source), map_entry),
        get_destination(min(vr.end, map_entry.source_end()), map_entry),
    ));

    // Range after the mapping comes out unchanged
    if vr.end > map_entry.source_end() {
        res.push(ValueRange::new(map_entry.source_end() + 1, vr.end))
    }

    res
}

fn get_ranges(input: &RangeCollection, map: &ValueMap) -> RangeCollection {
    let mut next_range: RangeCollection = RangeCollection::new();

    for range in input.ranges.iter() {
        let mut was_found: bool = false;
        for map_entry in map.map_entry.iter() {
            if gets_split(range, map_entry) {
                next_range.addv(&split_range(range, map_entry));
                was_found = true;
                break;
            }
        }
        if !was_found {
            next_range.add(range);
        }
    }

    next_range
}


fn main() {
    let input: Vec<String> = parse_inputfile(&get_filename());
    let seeds: Vec<u64> = parse_seeds(&input);

    let parsing_start: std::time::Instant = std::time::Instant::now();
    let seedmaps: Vec<ValueMap> = get_seedmaps(&input);
    println!("Parsing: {:?}", parsing_start.elapsed());

    let part1_start: std::time::Instant = std::time::Instant::now();
    let min_location = get_min_location(&seeds, &seedmaps);
    println!("Part 1: {} ({:?})", min_location, part1_start.elapsed());

    let seed_ranges: RangeCollection = parse_seed_ranges(seeds);
    let mut current_ranges: RangeCollection = seed_ranges.clone();

    let part2_start: std::time::Instant = std::time::Instant::now();

    for map in seedmaps.iter() {
        current_ranges = get_ranges(&current_ranges, map);
    }

    println!("Part 2 took {:?}. Min: {}", part2_start.elapsed(), current_ranges.min());
}
