use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use std::io::prelude::*;
use std::vec;

use anyhow::{anyhow, Result};
advent_of_code::solution!(10);

#[derive(Debug, Clone, PartialEq)]
struct Tiles {
    inner: Vec<Vec<Tile>>,
}

impl Tiles {
    fn rows(&self) -> &Vec<Vec<Tile>> {
        &self.inner
    }

    fn num_rows(&self) -> usize {
        self.inner.len()
    }

    fn num_columns(&self) -> usize {
        self.inner[0].len()
    }

    fn get(&self, row: usize, column: usize) -> Option<&Tile> {
        let r = self.inner.get(row)?;
        r.get(column)
    }

    fn set(&mut self, row: usize, column: usize, tile: Tile) {
        debug_assert!(self.get(row, column).is_some());
        self.inner[row][column] = tile;
    }

    fn insert(&mut self, row: usize, column: usize, tile: Tile) {
        debug_assert!(self.get(row, column).is_some());
        self.inner[row].insert(column, tile);
    }

    fn insert_row(&mut self, row: usize, tiles: Vec<Tile>) {
        self.inner.insert(row, tiles);
    }

    fn start(&self) -> Coordinate {
        for (i, row) in self.inner.iter().enumerate() {
            for (j, &tile) in row.iter().enumerate() {
                if tile == Tile::Start {
                    return Coordinate::new(i, j);
                }
            }
        }
        unreachable!()
    }

    fn neighbor_pipes(&self, c: &Coordinate) -> Vec<Coordinate> {
        let tile = self.get(c.row, c.column).unwrap();
        let mut neighbors = Vec::new();

        match tile {
            Tile::VerticalPipe => {
                neighbors.push(Coordinate::new(c.row - 1, c.column));
                neighbors.push(Coordinate::new(c.row + 1, c.column));
            }
            Tile::HorizontalPipe => {
                neighbors.push(Coordinate::new(c.row, c.column - 1));
                neighbors.push(Coordinate::new(c.row, c.column + 1));
            }
            Tile::NorthEastBend => {
                neighbors.push(Coordinate::new(c.row - 1, c.column));
                neighbors.push(Coordinate::new(c.row, c.column + 1));
            }
            Tile::NorthWestBend => {
                neighbors.push(Coordinate::new(c.row - 1, c.column));
                neighbors.push(Coordinate::new(c.row, c.column - 1));
            }
            Tile::SouthWestBend => {
                neighbors.push(Coordinate::new(c.row + 1, c.column));
                neighbors.push(Coordinate::new(c.row, c.column - 1));
            }
            Tile::SouthEastBend => {
                neighbors.push(Coordinate::new(c.row + 1, c.column));
                neighbors.push(Coordinate::new(c.row, c.column + 1));
            }
            Tile::Start => {
                if c.row > 0 {
                    if let Some(n) = self.get(c.row - 1, c.column) {
                        match *n {
                            Tile::VerticalPipe | Tile::SouthEastBend | Tile::SouthWestBend => {
                                neighbors.push(Coordinate::new(c.row - 1, c.column));
                            }
                            _ => {}
                        }
                    }
                }
                if c.row < self.num_rows() - 1 {
                    if let Some(n) = self.get(c.row + 1, c.column) {
                        match *n {
                            Tile::VerticalPipe | Tile::NorthEastBend | Tile::NorthWestBend => {
                                neighbors.push(Coordinate::new(c.row + 1, c.column));
                            }
                            _ => {}
                        }
                    }
                }
                if c.column > 0 {
                    if let Some(n) = self.get(c.row, c.column - 1) {
                        match *n {
                            Tile::HorizontalPipe | Tile::SouthEastBend | Tile::NorthEastBend => {
                                neighbors.push(Coordinate::new(c.row, c.column - 1));
                            }
                            _ => {}
                        }
                    }
                }
                if c.column < self.num_columns() - 1 {
                    if let Some(n) = self.get(c.row, c.column + 1) {
                        match *n {
                            Tile::HorizontalPipe | Tile::SouthWestBend | Tile::NorthWestBend => {
                                neighbors.push(Coordinate::new(c.row, c.column + 1));
                            }
                            _ => {}
                        }
                    }
                }
            }
            Tile::Ground => {}
        };
        neighbors
            .iter()
            .filter_map(|c| {
                let &tile = self.get(c.row, c.column).unwrap();
                if tile != Tile::Ground {
                    return Some(*c);
                }
                None
            })
            .collect()
    }

    fn neighbor_grounds(&self, c: &Coordinate) -> Vec<Coordinate> {
        let mut neighbors = Vec::new();

        if c.row > 0 {
            neighbors.push(Coordinate::new(c.row - 1, c.column));
        }
        if c.row < self.num_rows() - 1 {
            neighbors.push(Coordinate::new(c.row + 1, c.column));
        }
        if c.column > 0 {
            neighbors.push(Coordinate::new(c.row, c.column - 1));
        }
        if c.column < self.num_columns() - 1 {
            neighbors.push(Coordinate::new(c.row, c.column + 1));
        }
        neighbors
            .iter()
            .filter_map(|c| {
                if *self.get(c.row, c.column).unwrap() == Tile::Ground {
                    return Some(*c);
                }
                None
            })
            .collect()
    }
}

struct Grid {
    tiles: Tiles,
}

impl Grid {
    fn search_from_start(&self) -> HashMap<Coordinate, usize> {
        let mut to_search: VecDeque<(Coordinate, usize)> =
            VecDeque::from([(self.tiles.start(), 0)]);
        let mut searched: HashMap<Coordinate, usize> = HashMap::new();

        while !to_search.is_empty() {
            let (current, distance) = to_search.pop_front().expect("non empty to search queue");
            searched.insert(current, distance);

            for neighbor in self.tiles.neighbor_pipes(&current) {
                if !searched.contains_key(&neighbor) {
                    to_search.push_back((neighbor, distance + 1));
                }
            }
        }
        searched
    }

    fn furthest_tile_distance(&self) -> usize {
        *self
            .search_from_start()
            .values()
            .max()
            .expect("at least one tile")
    }

    fn enclosed_tiles(&self) -> usize {
        // search main loop
        let main_loop = self.search_from_start();

        // replace all non main loop tiles with ground
        let mut double_scale = self.tiles.clone();
        for (i, row) in self.tiles.rows().iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if !main_loop.contains_key(&Coordinate::new(i, j)) {
                    double_scale.set(i, j, Tile::Ground);
                }
            }
        }

        // replace the start tile with his pipe equivalent
        let start = double_scale.start();
        let start_neighbors = self.tiles.neighbor_pipes(&start);
        assert_eq!(2, start_neighbors.len());

        let start_pipe = start_pipe(start, start_neighbors);
        double_scale.set(start.row, start.column, start_pipe);

        // expand the grid into 2x scale
        let num_columns = double_scale.num_columns();
        for row in 0..double_scale.num_rows() {
            for column in (0..num_columns).rev() {
                let &current = double_scale.get(row, column).unwrap();
                if column > 0 {
                    double_scale.insert(row, column, current);
                    let pipe = match current {
                        Tile::HorizontalPipe | Tile::NorthWestBend | Tile::SouthWestBend => {
                            Tile::HorizontalPipe
                        }
                        _ => Tile::Ground,
                    };
                    double_scale.set(row, column, pipe);
                }
            }
        }

        let num_rows = double_scale.num_rows();
        for row in (1..num_rows).rev() {
            let mut new_row = vec![];
            for column in 0..double_scale.num_columns() {
                let &current = double_scale.get(row, column).unwrap();
                let pipe = match current {
                    Tile::VerticalPipe | Tile::NorthEastBend | Tile::NorthWestBend => {
                        Tile::VerticalPipe
                    }
                    _ => Tile::Ground,
                };
                new_row.push(pipe);
            }
            double_scale.insert_row(row, new_row)
        }

        // add the bottom and top row as well as the sides
        let mut to_search: Vec<Coordinate> = Vec::from([]);
        for column in 0..double_scale.num_columns() {
            if !main_loop.contains_key(&Coordinate::new(0, column / 2)) {
                to_search.push(Coordinate::new(0, column));
            }
            if !main_loop.contains_key(&Coordinate::new(self.tiles.num_rows() - 1, column / 2)) {
                to_search.push(Coordinate::new(double_scale.num_rows() - 1, column));
            }
        }
        for row in 0..double_scale.num_rows() {
            if !main_loop.contains_key(&Coordinate::new(row / 2, 0)) {
                to_search.push(Coordinate::new(row, 0));
            }
            if !main_loop.contains_key(&Coordinate::new(row / 2, self.tiles.num_columns() - 1)) {
                to_search.push(Coordinate::new(row, double_scale.num_columns() - 1));
            }
        }

        let mut searched: HashSet<Coordinate> = HashSet::new();
        while !to_search.is_empty() {
            let current = to_search.pop().expect("non empty to search queue");
            searched.insert(current);

            for neighbor in double_scale.neighbor_grounds(&current) {
                if !searched.contains(&neighbor) {
                    to_search.push(neighbor);
                }
            }
        }

        let outside: HashSet<_> = searched
            .iter()
            .filter_map(|s| {
                if s.row % 2 == 0 && s.column % 2 == 0 {
                    Some(Coordinate::new(s.row / 2, s.column / 2))
                } else {
                    None
                }
            })
            .collect();

        (num_rows * num_columns) - (main_loop.len() + outside.len())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Start,
}

impl Tile {
    fn from_char(c: char) -> Result<Self> {
        let tile = match c {
            '|' => Self::VerticalPipe,
            '-' => Self::HorizontalPipe,
            'L' => Self::NorthEastBend,
            'J' => Self::NorthWestBend,
            '7' => Self::SouthWestBend,
            'F' => Self::SouthEastBend,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => return Err(anyhow!("invalid char: {c}")),
        };
        Ok(tile)
    }

    fn to_char(&self) -> char {
        return match self {
            Self::VerticalPipe => '│',
            Self::HorizontalPipe => '─',
            Self::NorthEastBend => '└',
            Self::NorthWestBend => '┘',
            Self::SouthWestBend => '┐',
            Self::SouthEastBend => '┌',
            Self::Ground => '*',
            Self::Start => 'S',
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    row: usize,
    column: usize,
}

impl Coordinate {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn start_pipe(start: Coordinate, start_neighbors: Vec<Coordinate>) -> Tile {
    let mut directions = start_neighbors
        .iter()
        .map(|n| {
            if start.row < n.row {
                Direction::South
            } else if start.row > n.row {
                Direction::North
            } else if start.column < n.column {
                Direction::East
            } else {
                Direction::West
            }
        })
        .collect::<Vec<_>>();
    directions.sort();
    match directions[0] {
        Direction::North => match directions[1] {
            Direction::South => Tile::VerticalPipe,
            Direction::East => Tile::NorthEastBend,
            Direction::West => Tile::NorthWestBend,
            _ => unreachable!(),
        },
        Direction::South => match directions[1] {
            Direction::North => Tile::VerticalPipe,
            Direction::East => Tile::SouthEastBend,
            Direction::West => Tile::SouthWestBend,
            _ => unreachable!(),
        },
        Direction::East => match directions[1] {
            Direction::North => Tile::NorthEastBend,
            Direction::South => Tile::SouthEastBend,
            Direction::West => Tile::HorizontalPipe,
            _ => unreachable!(),
        },
        Direction::West => match directions[1] {
            Direction::North => Tile::NorthEastBend,
            Direction::South => Tile::SouthEastBend,
            Direction::East => Tile::HorizontalPipe,
            _ => unreachable!(),
        },
    }
}

fn parse_grid(input: &str) -> Result<Grid> {
    let tiles = input
        .lines()
        .map(|l| l.chars().map(|c| Ok(Tile::from_char(c)?)).collect())
        .collect::<Result<Vec<Vec<Tile>>>>()?;
    Ok(Grid {
        tiles: Tiles { inner: tiles },
    })
}

pub fn part_one(input: &str) -> Result<usize> {
    let grid = parse_grid(input)?;
    Ok(grid.furthest_tile_distance())
}

pub fn part_two(input: &str) -> Result<usize> {
    let grid = parse_grid(input)?;
    Ok(grid.enclosed_tiles())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 70);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 8);
        Ok(())
    }
}
