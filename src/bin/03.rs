use std::{
    collections::{HashMap, HashSet},
    vec,
};

use anyhow::Result;
advent_of_code::solution!(3);

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<Cell>>,

    part_numbers: Vec<PartNumber>,
    part_number_map: HashMap<(usize, usize), PartNumber>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct PartNumber {
    num: u32,
    indices: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Num(u32),
    Symbol,
    Dot,
}

impl Grid {
    fn from_cells(cells: Vec<Vec<Cell>>) -> Self {
        let mut grid = Grid {
            cells: cells.clone(),
            part_numbers: vec![],
            part_number_map: HashMap::new(),
        };

        for (row, r) in cells.iter().enumerate() {
            let mut part_number: Option<PartNumber> = None;

            for (column, cell) in r.iter().enumerate() {
                let digit = match cell {
                    Cell::Num(num) => {
                        match part_number {
                            Some(ref mut p) => {
                                p.num *= 10;
                                p.num += num;
                                p.indices.push((row, column));
                            }
                            None => {
                                part_number = Some(PartNumber {
                                    num: *num,
                                    indices: vec![(row, column)],
                                });
                            }
                        };
                        true
                    }
                    Cell::Dot | Cell::Symbol => false,
                };

                if let Some(ref mut p) = part_number {
                    if digit && column < r.len() - 1 {
                        continue;
                    }

                    let symbol_adjacent = p.indices.iter().any(|(cell_row, cell_column)| {
                        adjacent_cells(&cells, *cell_row, *cell_column)
                            .iter()
                            .any(|(_, c)| match c {
                                Cell::Symbol => true,
                                _ => false,
                            })
                    });

                    if symbol_adjacent {
                        grid.part_numbers.push(p.clone());
                        for i in &p.indices {
                            grid.part_number_map.insert(*i, p.clone());
                        }
                    }
                    part_number = None;
                }
            }
        }

        grid
    }

    fn gear_ratios_sum(&self) -> u32 {
        let mut sum = 0;

        for (row, r) in self.cells.iter().enumerate() {
            for (column, cell) in r.iter().enumerate() {
                match cell {
                    Cell::Symbol => {
                        // does it have exactly 2 adjacent part numbers
                        let part_numbers: Vec<_> = adjacent_cells(&self.cells, row, column)
                            .iter()
                            .filter_map(|((row, column), _)| {
                                self.part_number_map.get(&(*row, *column))
                            })
                            .collect::<HashSet<_>>()
                            .into_iter()
                            .collect();

                        if part_numbers.len() != 2 {
                            continue;
                        }
                        sum += part_numbers[0].num * part_numbers[1].num
                    }
                    _ => {}
                }
            }
        }
        sum
    }
}

fn adjacent_cells(cells: &Vec<Vec<Cell>>, row: usize, col: usize) -> HashMap<(usize, usize), Cell> {
    let mut adjacents = HashMap::new();
    let mut offsets: Vec<(usize, usize)> = vec![(row, col + 1), (row + 1, col), (row + 1, col + 1)];

    if row > 0 {
        offsets.push((row - 1, col));
        offsets.push((row - 1, col + 1));
        if col > 0 {
            offsets.push((row - 1, col - 1));
        }
    }
    if col > 0 {
        offsets.push((row, col - 1));
        offsets.push((row + 1, col - 1));
    }

    for offset in offsets {
        if let Some(r) = cells.get(offset.0) {
            if let Some(cell) = r.get(offset.1) {
                adjacents.insert((offset.0, offset.1), *cell);
            }
        }
    }
    adjacents
}

fn parse_grid(input: &str) -> Result<Grid> {
    let cells: Vec<_> = input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '0'..='9' => Cell::Num(c.to_digit(10).expect("parse digit")),
                    '.' => Cell::Dot,
                    _ => Cell::Symbol,
                })
                .collect()
        })
        .collect();

    Ok(Grid::from_cells(cells))
}

pub fn part_one(input: &str) -> Result<u32> {
    let grid = parse_grid(input)?;
    Ok(grid.part_numbers.iter().fold(0, |acc, p| acc + p.num))
}

pub fn part_two(input: &str) -> Result<u32> {
    let grid = parse_grid(input)?;
    Ok(grid.gear_ratios_sum())
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
        assert_eq!(result.unwrap(), 4361);
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
        assert_eq!(result.unwrap(), 467835);
    }
}
