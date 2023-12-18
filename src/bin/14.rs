use std::collections::HashMap;

use anyhow::{anyhow, Ok, Result};

advent_of_code::solution!(14);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform {
    grid: Vec<Vec<Tile>>,
    view: View,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum View {
    Columns,
    Rows,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    RoundedRock,
    CubeRock,
    Empty,
}

impl Tile {
    fn from_char(c: char) -> Result<Self> {
        match c {
            'O' => Ok(Self::RoundedRock),
            '#' => Ok(Self::CubeRock),
            '.' => Ok(Self::Empty),
            _ => Err(anyhow!("invalid char {c}")),
        }
    }

    fn char(&self) -> char {
        match self {
            Self::RoundedRock => 'O',
            Self::CubeRock => '#',
            Self::Empty => '.',
        }
    }
}

impl Platform {
    fn from_rows(rows: Vec<Vec<Tile>>) -> Self {
        Platform {
            grid: rows,
            view: View::Rows,
        }
    }

    fn part_one(&mut self) -> usize {
        self.transpose();
        for column in self.grid.iter_mut() {
            shift_left(column);
        }
        self.north_beam_load()
    }

    fn north_beam_load(&mut self) -> usize {
        match self.view {
            View::Rows => self.transpose(),
            View::Columns => {}
        }
        debug_assert_eq!(View::Columns, self.view);

        let num_rows = self.grid[0].len();
        let total = self
            .grid
            .iter()
            .flat_map(|column| {
                column.iter().enumerate().map(|(i, &t)| {
                    if t == Tile::RoundedRock {
                        return num_rows - i;
                    }
                    return 0;
                })
            })
            .sum::<usize>();

        self.transpose();
        total
    }

    fn cycle(&mut self) {
        // north
        self.transpose();
        for column in self.grid.iter_mut() {
            shift_left(column);
        }

        // west
        self.transpose();
        for row in self.grid.iter_mut() {
            shift_left(row);
        }

        // south
        self.transpose();
        for column in self.grid.iter_mut() {
            column.reverse();
            shift_left(column);
            column.reverse();
        }

        // east
        self.transpose();
        for row in self.grid.iter_mut() {
            row.reverse();
            shift_left(row);
            row.reverse();
        }
    }

    fn transpose(&mut self) {
        let rows = self.grid.len();
        let cols = self.grid[0].len();

        self.grid = (0..cols)
            .map(|col| (0..rows).map(|row| self.grid[row][col]).collect())
            .collect();

        self.view = match self.view {
            View::Columns => View::Rows,
            View::Rows => View::Columns,
        };
    }

    fn print(&self) {
        for row in &self.grid {
            println!("{}", row.iter().map(|t| t.char()).collect::<String>());
        }
    }
}

fn shift_left(row: &mut Vec<Tile>) {
    let mut empty_tile: Option<usize> = None;
    for i in 0..row.len() {
        match row[i] {
            Tile::CubeRock => {
                empty_tile = None;
            }
            Tile::RoundedRock => {
                if let Some(empty_tile_index) = empty_tile {
                    row[empty_tile_index] = Tile::RoundedRock;
                    row[i] = Tile::Empty;

                    empty_tile = Some(i);
                    for j in (empty_tile_index + 1)..i {
                        if row[j] == Tile::Empty {
                            empty_tile = Some(j);
                            break;
                        }
                    }
                }
            }
            Tile::Empty => {
                if empty_tile.is_none() {
                    empty_tile = Some(i);
                }
            }
        }
    }
}

fn parse_platform(input: &str) -> Result<Platform> {
    Ok(Platform::from_rows(
        input
            .lines()
            .map(|l| l.chars().map(|c| Tile::from_char(c)).collect())
            .collect::<Result<Vec<_>>>()?,
    ))
}

pub fn part_one(input: &str) -> Result<usize> {
    let mut platform = parse_platform(input)?;
    Ok(platform.part_one())
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut platform = parse_platform(input)?;

    let mut cache: HashMap<Platform, usize> = HashMap::new();
    let mut states: Vec<Platform> = Vec::new();

    let mut cycle_length = 0;
    let mut cycle_start = 0;

    // we should hit a cycle before the first 500k iterations
    for i in 1..500_000 {
        if let Some(&c) = cache.get(&platform) {
            cycle_length = i - c;
            cycle_start = c;
            break;
        }
        let original = platform.clone();
        platform.cycle();
        cache.insert(original, i);
        states.push(platform.clone());
    }

    assert_ne!(0, cycle_start);

    let moves_after_cycle = (1_000_000_000 - cycle_start) % cycle_length;
    let mut platform = states[cycle_start - 1 + moves_after_cycle].clone();
    Ok(platform.north_beam_load())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 136);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 64);
        Ok(())
    }
}
