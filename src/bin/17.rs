use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use anyhow::{anyhow, Result};
advent_of_code::solution!(17);

#[derive(Debug, Clone)]
struct Grid {
    tiles: HashMap<Coordinate, u32>,
    rows: usize,
    columns: usize,
}

impl Grid {
    fn new(tiles: HashMap<Coordinate, u32>) -> Self {
        let rows = tiles.iter().map(|(c, _)| c.y).max().unwrap() + 1;
        let columns = tiles.iter().map(|(c, _)| c.x).max().unwrap() + 1;

        Self {
            tiles: tiles,
            rows: rows,
            columns: columns,
        }
    }

    fn search<F>(&self, neighbors: F) -> u32
    where
        F: Fn(&Self, &SearchState) -> Vec<SearchState>,
    {
        let start = Coordinate::new(0, 0);
        let goal = Coordinate::new(self.columns - 1, self.rows - 1);

        let mut queue: BinaryHeap<SearchState> = BinaryHeap::from([SearchState::new_start(start, Direction::South), SearchState::new_start(start, Direction::East)]);
        let mut visited: HashSet<(Coordinate, Direction, u32)> = HashSet::new();

        while let Some(state) = queue.pop() {
            if let Some(_v) = visited.get(&(state.current, state.direction, state.num_straight_steps)) {
                continue;
            }
            if state.current == goal {
                return state.exact_cost;
            }

            for neighbor in neighbors(self, &state) {
                queue.push(neighbor);
            }
            visited.insert((state.current, state.direction, state.num_straight_steps));
        }
        unreachable!()
    }

    fn neighbors(&self, state: &SearchState) -> Vec<SearchState> {
        let mut neighbors = Vec::new();

        for orthogonal in state.direction.orthoganal() {
            if let Some(next) = self.neighbor(state.current, orthogonal) {
                let cost = *self.tiles.get(&next).unwrap();
                neighbors.push(state.new_turn(next, orthogonal, cost));
            }
        }

        if state.num_straight_steps < 3 {
            if let Some(next) = self.neighbor(state.current, state.direction) {
                let cost = *self.tiles.get(&next).unwrap();
                neighbors.push(state.new_straight(next, cost));
            }
        }
        neighbors
    }

    fn neighbors_ultra(&self, state: &SearchState) -> Vec<SearchState> {
        let mut neighbors = Vec::new();

        if state.num_straight_steps >= 4 {
            for orthogonal in state.direction.orthoganal() {
                if let Some(next) = self.neighbor(state.current, orthogonal) {
                    let cost = *self.tiles.get(&next).unwrap();
                    neighbors.push(state.new_turn(next, orthogonal, cost));
                }
            }
        }

        if state.num_straight_steps < 10 {
            if let Some(next) = self.neighbor(state.current, state.direction) {
                let cost = *self.tiles.get(&next).unwrap();
                neighbors.push(state.new_straight(next, cost));
            }
        }
        neighbors
    }

    fn neighbor(&self, coordinate: Coordinate, direction: Direction) -> Option<Coordinate> {
        match direction {
            Direction::North => {
                if coordinate.y > 0 {
                    return Some(Coordinate::new(coordinate.x, coordinate.y - 1));
                }
            }
            Direction::East => {
                if coordinate.x < self.columns - 1 {
                    return Some(Coordinate::new(coordinate.x + 1, coordinate.y));
                }
            }
            Direction::South => {
                if coordinate.y < self.rows - 1 {
                    return Some(Coordinate::new(coordinate.x, coordinate.y + 1));
                }
            }
            Direction::West => {
                if coordinate.x > 0 {
                    return Some(Coordinate::new(coordinate.x - 1, coordinate.y));
                }
            }
        }
        None
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn orthoganal(&self) -> Vec<Self> {
        match self {
            Direction::North | Direction::South => vec![Direction::East, Direction::West],
            Direction::East | Direction::West => vec![Direction::North, Direction::South],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SearchState {
    current: Coordinate,
    direction: Direction,
    num_straight_steps: u32,
    exact_cost: u32,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.exact_cost.cmp(&self.exact_cost)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SearchState {
    fn new_start(coordinate: Coordinate, direction: Direction) -> SearchState {
        Self {
            current: coordinate,
            direction,
            num_straight_steps: 2,
            exact_cost: 0,
        }
    }

    fn new_straight(&self, next: Coordinate, cost: u32) -> Self {
        Self {
            current: next,
            direction: self.direction,
            num_straight_steps: self.num_straight_steps + 1,
            exact_cost: self.exact_cost + cost,
        }
    }

    fn new_turn(&self, next: Coordinate, direction: Direction, cost: u32) -> Self {
        Self {
            current: next,
            direction,
            num_straight_steps: 1,
            exact_cost: self.exact_cost + cost,
        }
    }
}

fn parse_grid(input: &str) -> Result<Grid> {
    let tiles = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(column, c)| {
                let coordinate = Coordinate { x: column, y: row };
                Ok((coordinate, c.to_digit(10).ok_or(anyhow!("invalid digit: {}", c))?))
            })
        })
        .collect::<Result<HashMap<_, _>>>()?;
    Ok(Grid::new(tiles))
}

pub fn part_one(input: &str) -> Result<u32> {
    let grid = parse_grid(input)?;
    Ok(grid.search(Grid::neighbors))
}

pub fn part_two(input: &str) -> Result<u32> {
    let grid = parse_grid(input)?;
    Ok(grid.search(Grid::neighbors_ultra))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 102);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 94);
        Ok(())
    }
}
