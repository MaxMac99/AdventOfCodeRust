use std::cmp::{max, min};
use std::error::Error;
use std::fs;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct Input {
    source: u64,
    destination: u64,
    range: u64,
}

pub fn parse_file(filename: &str) -> Result<(Vec<u64>, Vec<Vec<Input>>), Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;

    let blocks: Vec<String> = content.split("\n\n").map(String::from).collect();

    let seeds: Vec<u64> = blocks
        .get(0)
        .expect("Could not find blocks")
        .split_whitespace()
        .skip(1)
        .map(|val| val.parse::<u64>().expect("Could not parse seed"))
        .collect();

    let maps = parse_maps(&blocks);
    Ok((seeds, maps))
}

fn parse_maps(blocks: &Vec<String>) -> Vec<Vec<Input>> {
    let mut maps: Vec<Vec<Input>> = Vec::new();
    for block in blocks
        .iter()
        .skip(1) {
        let mut map = Vec::new();
        for line in block.split("\n")
            .skip(1) {
            let items: Vec<u64> = line.split_whitespace()
                .map(|val| val.parse::<u64>().expect("Could not "))
                .collect();
            map.push(Input {
                source: *items.get(1).unwrap(),
                destination: *items.get(0).unwrap(),
                range: *items.get(2).unwrap(),
            })
        }
        maps.push(map);
    }

    maps
}

pub fn find_min_solution(seeds: &Vec<(u64, u64)>, maps: &Vec<Vec<Input>>) -> u64 {
    seeds.par_iter()
        .flat_map(|val| find_location(*val, maps))
        .min()
        .expect("Could not find solution")
}

fn find_location(seed: (u64, u64), maps: &Vec<Vec<Input>>) -> Vec<u64> {
    let mut destination: Vec<(u64, u64)> = Vec::new();
    destination.push(seed);
    for map in maps {
        destination = find_destination_range(destination, map);
    }
    destination.iter()
        .map(|val| val.0)
        .collect()
}

fn find_destination_range(sources: Vec<(u64, u64)>, map: &Vec<Input>) -> Vec<(u64, u64)> {
    let mut destinations: Vec<(u64, u64)> = Vec::new();
    for source in sources {
        let mut destination_ranges = map.iter()
            .filter_map(|input| input.find_destination_range(source))
            .collect::<Vec<(u64, u64)>>();
        if destination_ranges.is_empty() {
            destinations.push(source);
        }
        destinations.append(&mut destination_ranges);
    }
    destinations
}

impl Input {
    fn find_destination_range(&self, source: (u64, u64)) -> Option<(u64, u64)> {
        let source_upper = source.0 + source.1;
        let self_upper = self.source + self.range;
        if source.0 <= self_upper && self.source <= source_upper {
            let min_source = max(source.0, self.source);
            let upper_bound = min(source_upper, self_upper);
            let target_range = upper_bound - min_source;
            let destination = self.destination + (min_source - self.source);
            return Some((destination, target_range));
        }
        None
    }
}
