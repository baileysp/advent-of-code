use std::collections::HashMap;

use anyhow::{Result};
advent_of_code::solution!(5);

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,

    // keyed by source category
    maps: HashMap<String, Map>,
}

impl Almanac {
    fn mapped_seeds(&self, seeds: &Vec<u32>) -> Vec<u32> {
        let mut mapped_seeds: Vec<u32> = vec![];
        for seed in seeds {
            // start with seed map
            let mut current_category = "seed";
            let mut current = *seed;
            while current_category != "location" {
                let m = self.maps.get(current_category).expect("seed map");
                current = m.map(current);
                current_category = m.destination_category.as_str();
            }
            mapped_seeds.push(current);
        }
        mapped_seeds
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
    source_category: String,
    destination_category: String,
}

impl Map {
    fn map(&self, start: u32) -> u32 {
        for range in &self.ranges {
            if range.contains(start) {
                return range.map(start);
            }
        }
        start
    }
}

#[derive(Debug)]
struct Range {
    destination_start: u32,
    source_start: u32,
    length: u32,
}

impl Range {
    fn contains(&self, u: u32) -> bool {
        if self.source_start > u {
            return false;
        }
        return self.source_start + self.length > u;
    }
    fn map(&self, start: u32) -> u32 {
        debug_assert!(self.contains(start));
        self.destination_start + (start - self.source_start)
    }
}

fn parse_almanac(input: &str) -> Almanac {
    let a = input.split("\n\n").collect::<Vec<_>>();

    let (seeds_str, maps_str) = a.split_at(1);
    let seeds: Vec<_> = seeds_str
        .first()
        .expect("slice 1")
        .split(":")
        .last()
        .expect("seeds")
        .split_whitespace()
        .map(|d| d.parse::<u32>().expect("seed"))
        .collect();

    let maps_vec: Vec<_> = maps_str
        .iter()
        .map(|l| {
            let (name, rest) = l.split_once(":\n").expect("map format");
            let (source, destination) = name
                .split_whitespace()
                .collect::<Vec<_>>()
                .first()
                .expect("name")
                .split_once("-to-")
                .expect("name format");

            let ranges = rest
                .lines()
                .map(|l| {
                    let mut nums = l.split_whitespace();
                    Range {
                        destination_start: nums
                            .next()
                            .expect("destination start")
                            .parse::<u32>()
                            .expect("destination start"),
                        source_start: nums
                            .next()
                            .expect("source start")
                            .parse::<u32>()
                            .expect("source start"),
                        length: nums.next().expect("length").parse::<u32>().expect("length"),
                    }
                })
                .collect();
            Map {
                source_category: source.to_string(),
                destination_category: destination.to_string(),
                ranges: ranges,
            }
        })
        .collect();

    let mut maps = HashMap::new();
    for map in maps_vec {
        maps.insert(map.source_category.clone(), map);
    }

    Almanac {
        seeds: seeds,
        maps: maps,
    }
}

pub fn part_one(input: &str) -> Result<u32> {
    let almanac = parse_almanac(input);
    let mapped_seeds = almanac.mapped_seeds(&almanac.seeds);
    Ok(*mapped_seeds.iter().min().expect("at least one seed"))
}

pub fn part_two(input: &str) -> Result<u32> {
    let almanac = parse_almanac(input);
    let mut seeds = vec![];

    for chunk in almanac.seeds.chunks(2) {
        let (start, length) = (chunk[0], chunk[1]);

        for i in start..(start + length) {
            seeds.push(i);
        }
    }

    let mapped_seeds = almanac.mapped_seeds(&seeds);
    Ok(*mapped_seeds.iter().min().expect("at least one seed"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            true,
            result.is_ok(),
            "part one failed: {}",
            result.err().unwrap()
        );
        assert_eq!(result.unwrap(), 35);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            true,
            result.is_ok(),
            "part two failed: {}",
            result.err().unwrap()
        );
        assert_eq!(result.unwrap(), 46);
    }
}
