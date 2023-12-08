use anyhow::{Context, Result};
advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time_allowed: usize,
    record_distance: usize,
}

impl Race {
    fn ways_to_win(&self) -> usize {
        let mut wins = 0;
        for button_time in 1..(self.time_allowed) {
            let remaining_time = self.time_allowed - button_time;
            if button_time * remaining_time > self.record_distance {
                wins += 1;
            }
        }
        wins
    }
}

fn parse_races(input: &str) -> Vec<Race> {
    let lines: Vec<_> = input.lines().collect();
    let times: Vec<usize> = lines[0]
        .trim_start_matches("Time:")
        .trim_start()
        .split_whitespace()
        .map(|t| t.parse().expect("parse time"))
        .collect();

    let distances: Vec<usize> = lines[1]
        .trim_start_matches("Distance:")
        .trim_start()
        .split_whitespace()
        .map(|t| t.parse().expect("parse distance"))
        .collect();

    debug_assert_eq!(times.len(), distances.len());

    let mut races = Vec::with_capacity(times.len());
    for (i, time) in times.iter().enumerate() {
        races.push(Race {
            time_allowed: *time,
            record_distance: distances[i],
        })
    }
    races
}

fn parse_races_part_two(input: &str) -> Result<Race> {
    let lines: Vec<_> = input.lines().collect();
    let time = lines[0]
        .trim_start_matches("Time:")
        .replace(" ", "")
        .parse::<usize>()?;

    let distance = lines[1]
        .trim_start_matches("Distance:")
        .replace(" ", "")
        .parse::<usize>()?;

    Ok(Race {
        time_allowed: time,
        record_distance: distance,
    })
}

pub fn part_one(input: &str) -> Result<usize> {
    let races = parse_races(input);
    Ok(races
        .iter()
        .map(|r| r.ways_to_win())
        .reduce(|acc, w| acc * w)
        .expect("at least one race"))
}

pub fn part_two(input: &str) -> Result<usize> {
    let race = parse_races_part_two(input)?;
    Ok(race.ways_to_win())
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
        assert_eq!(result.unwrap(), 288);
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
        assert_eq!(result.unwrap(), 71503);
    }
}
