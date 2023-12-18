
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::vec;

use anyhow::{anyhow, Ok, Result};
advent_of_code::solution!(12);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => Err(anyhow!("invalid char {c}")),
        }
    }

    fn char(&self) -> char {
        match self {
            Self::Operational => '.',
            Self::Damaged => '#',
            Self::Unknown => '?',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
struct SpringRow {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Display for SpringRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SpringRow { springs: ")?;
        f.write_str(
            self.springs
                .iter()
                .map(|s| s.char())
                .collect::<String>()
                .as_str(),
        )?;
        f.write_str(", groups: ")?;
        f.debug_list().entries(self.groups.iter()).finish()?;
        f.write_str(" }")
    }
}

impl SpringRow {
    fn unfold(&mut self, factor: usize) {
        let original = self.clone();
        for i in 0..(factor - 1) {
            if i != (factor - 1) {
                self.springs.push(Spring::Unknown);
            }
            self.springs.append(&mut original.springs.clone());
            self.groups.append(&mut original.groups.clone());
        }
    }
}

struct Searcher {
    cache: HashMap<SpringRow, usize>,
}

impl Searcher {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    fn possible_arrangements(&mut self, row: &SpringRow) -> usize {
        if row.springs.is_empty() {
            if row.groups.is_empty() {
                return 1;
            }
            return 0;
        }

        if row.groups.is_empty() {
            if row.springs.contains(&Spring::Damaged) {
                return 0;
            }
            return 1;
        }

        if let Some(&cached) = self.cache.get(row) {
            return cached;
        }

        let mut total = 0;

        let group_size = row.groups[0];

        // consume n damaged springs
        if row.springs[0] == Spring::Damaged || row.springs[0] == Spring::Unknown {
            // more springs then the current group
            if group_size <= row.springs.len() {
                // need to consume n (damaged or unknown) springs
                if !row.springs[..group_size].contains(&Spring::Operational) {
                    // group needs to terminate with an operational spring or the end of the row
                    if group_size == row.springs.len() {
                        total += self.possible_arrangements(&SpringRow {
                            springs: vec![],
                            groups: row.groups[1..].to_vec(),
                        });
                    } else if row.springs[group_size] != Spring::Damaged {
                        total += self.possible_arrangements(&SpringRow {
                            springs: row.springs[(row.groups[0] + 1)..].to_vec(),
                            groups: row.groups[1..].to_vec(),
                        });
                    }
                }
            }
        }

        // skip over operational springs and treat unknown spring as operational
        if row.springs[0] == Spring::Operational || row.springs[0] == Spring::Unknown {
            total += self.possible_arrangements(&SpringRow {
                springs: row.springs[1..].to_vec(),
                groups: row.groups.clone(),
            });
        }

        self.cache.insert(row.clone(), total);

        total
    }
}

fn parse_spring_rows(input: &str) -> Result<Vec<SpringRow>> {
    let rows: Vec<SpringRow> = input
        .lines()
        .map(|l| {
            let (row, groups) = l.split_once(" ").unwrap();
            Ok(SpringRow {
                springs: row
                    .chars()
                    .map(|c| Spring::from_char(c))
                    .collect::<Result<Vec<_>>>()?,
                groups: groups
                    .split(",")
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect(),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(rows)
}

pub fn part_one(input: &str) -> Result<usize> {
    let rows = parse_spring_rows(input)?;
    let mut total = 0;
    let mut searcher = Searcher::new();
    for row in rows {
        total += searcher.possible_arrangements(&row);
    }
    Ok(total)
}

pub fn part_two(input: &str) -> Result<usize> {
    let rows = parse_spring_rows(input)?;
    let mut total = 0;
    let mut searcher = Searcher::new();
    for mut row in rows {
        row.unfold(10);
        total += searcher.possible_arrangements(&row);
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 21);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 525152);
        Ok(())
    }
}
