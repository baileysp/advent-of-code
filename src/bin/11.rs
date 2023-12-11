use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    vec,
};

use anyhow::{anyhow, Result};
advent_of_code::solution!(11);

struct Image {
    tiles: Tiles,

    expanse_factor: usize,
    expanse_rows: Vec<usize>,
    expanse_columns: Vec<usize>,
}

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
}

impl Image {
    fn from_tiles(tiles: Tiles, expanse_factor: usize) -> Self {
        let mut expanse_rows = vec![];
        for (i, row) in tiles.rows().iter().enumerate() {
            if row.iter().all(|&t| t == Tile::EmptySpace) {
                expanse_rows.push(i);
            }
        }

        let mut expanse_columns = vec![];
        for column in 0..tiles.num_columns() {
            let mut empty = true;
            for row in 0..tiles.num_rows() {
                match tiles.get(row, column).expect("valid tile") {
                    Tile::EmptySpace => continue,
                    Tile::Galaxy => {
                        empty = false;
                        break;
                    }
                }
            }
            if empty {
                expanse_columns.push(column);
            }
        }

        Image {
            tiles: tiles,
            expanse_factor: expanse_factor,
            expanse_rows: expanse_rows,
            expanse_columns: expanse_columns,
        }
    }

    fn galaxy_distances(&self) -> Vec<usize> {
        let galaxies = self.galaxies();
        let mut distances: Vec<usize> = vec![];
        for (g1, &start) in galaxies.iter().enumerate() {
            for (g2, &end) in galaxies.iter().enumerate() {
                if g1 == g2 || g2 < g1 {
                    continue;
                }

                let distance = self.distance(start, end);
                distances.push(distance);
            }
        }
        distances
    }

    fn galaxies(&self) -> Vec<Coordinate> {
        let mut galaxies = vec![];
        for row in 0..self.tiles.num_rows() {
            for column in 0..self.tiles.num_columns() {
                match self.tiles.get(row, column).expect("valid tile") {
                    Tile::Galaxy => galaxies.push(Coordinate::new(row, column)),
                    Tile::EmptySpace => {}
                }
            }
        }
        galaxies
    }

    fn distance(&self, start: Coordinate, end: Coordinate) -> usize {
        let mut dist: HashMap<Coordinate, _> = HashMap::new();
        for row in 0..self.tiles.num_rows() {
            for column in 0..self.tiles.num_columns() {
                dist.insert(Coordinate::new(row, column), usize::MAX);
            }
        }

        dist.insert(start, 0);
        let mut to_search: BinaryHeap<SearchState> = BinaryHeap::from([SearchState {
            cost: 0,
            heuristic: 0,
            coords: start,
        }]);

        while let Some(current) = to_search.pop() {
            if current.coords == end {
                return current.cost;
            }
            if current.cost > *dist.get(&current.coords).expect("valid coords") {
                continue;
            }

            for neighbor in self.neighbors(&current.coords) {
                let next = SearchState {
                    cost: current.cost + self.cost(&current.coords),
                    heuristic: neighbor.manhattan_distance(end),
                    coords: neighbor,
                };

                let distance = *dist
                    .get(&neighbor)
                    .expect(format!("valid coords {:?}", neighbor).as_str());
                if next.cost < distance {
                    to_search.push(next);
                    dist.insert(next.coords, next.cost);
                }
            }
        }
        unreachable!()
    }

    fn neighbors(&self, c: &Coordinate) -> Vec<Coordinate> {
        let mut neighbors = vec![];
        if c.row > 0 {
            neighbors.push(Coordinate::new(c.row - 1, c.column));
        }
        if c.row < self.tiles.num_rows() - 1 {
            neighbors.push(Coordinate::new(c.row + 1, c.column));
        }
        if c.column > 0 {
            neighbors.push(Coordinate::new(c.row, c.column - 1));
        }
        if c.column < self.tiles.num_columns() - 1 {
            neighbors.push(Coordinate::new(c.row, c.column + 1));
        }
        neighbors
    }

    fn cost(&self, from: &Coordinate) -> usize {
        if self.expanse_rows.contains(&from.row) {
            return self.expanse_factor;
        }
        if self.expanse_columns.contains(&from.column) {
            return self.expanse_factor;
        }
        1
    }

    fn print(&self) {
        println!(
            "rows: {} columns: {}",
            self.tiles.num_rows(),
            self.tiles.num_columns()
        );

        println!(
            "expanse rows: {:?} expanse columns: {:?}",
            self.expanse_rows, self.expanse_columns
        );

        for row in self.tiles.rows() {
            for tile in row {
                match tile {
                    Tile::EmptySpace => print!("."),
                    Tile::Galaxy => print!("#"),
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct SearchState {
    cost: usize,
    heuristic: usize,
    coords: Coordinate,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

    fn manhattan_distance(&self, other: Coordinate) -> usize {
        ((self.row as i32 - other.row as i32).abs()
            + (self.column as i32 - other.column as i32).abs()) as usize
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Galaxy,
    EmptySpace,
}

impl Tile {
    fn from_char(c: char) -> Result<Self> {
        let tile = match c {
            '#' => Self::Galaxy,
            '.' => Self::EmptySpace,
            _ => return Err(anyhow!("invalid char: {c}")),
        };
        Ok(tile)
    }
}

fn parse_image(input: &str, expanse_factor: usize) -> Result<Image> {
    let tiles = input
        .lines()
        .map(|l| l.chars().map(move |c| Ok(Tile::from_char(c)?)).collect())
        .collect::<Result<Vec<Vec<Tile>>>>()?;
    Ok(Image::from_tiles(Tiles { inner: tiles }, expanse_factor))
}

pub fn part_one(input: &str) -> Result<usize> {
    let image = parse_image(input, 2)?;
    let distances = image.galaxy_distances();
    Ok(distances.iter().sum())
}

pub fn part_two(input: &str) -> Result<usize> {
    let image = parse_image(input, 1000000)?;
    let distances = image.galaxy_distances();
    Ok(distances.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 374);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 1030);
        Ok(())
    }
}
