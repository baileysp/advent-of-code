use anyhow::Result;
advent_of_code::solution!(9);

struct History {
    numbers: Vec<i64>,
}

impl History {
    fn sequences(&self) -> Vec<Vec<i64>> {
        let mut sequences: Vec<Vec<_>> = vec![self.numbers.clone()];
        let mut index = 0;

        while !sequences[index].iter().all(|s| *s == 0) {
            sequences.push(sequences[index].windows(2).map(|w| w[1] - w[0]).collect());
            index += 1;
        }

        sequences.pop();
        sequences
    }

    fn extrapolate_forwards(&self) -> i64 {
        let mut last = 0;
        for s in self.sequences().iter().rev() {
            if last == 0 {
                last = s[s.len() - 1]
            } else {
                last += s[s.len() - 1]
            }
        }
        last
    }

    fn extrapolate_backwards(&self) -> i64 {
        let mut first = 0;
        for s in self.sequences().iter().rev() {
            if first == 0 {
                first = s[0]
            } else {
                first = s[0] - first
            }
        }
        first
    }
}

fn parse_histories(input: &str) -> Result<Vec<History>> {
    input
        .lines()
        .map(|l| {
            Ok(History {
                numbers: l
                    .split_whitespace()
                    .map(|d| d.parse::<i64>())
                    .collect::<Result<Vec<_>, _>>()?,
            })
        })
        .collect()
}

pub fn part_one(input: &str) -> Result<i64> {
    let histories = parse_histories(input)?;
    Ok(histories.iter().map(|h| h.extrapolate_forwards()).sum())
}

pub fn part_two(input: &str) -> Result<i64> {
    let histories = parse_histories(input)?;
    Ok(histories.iter().map(|h| h.extrapolate_backwards()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 114);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 2);
        Ok(())
    }
}
