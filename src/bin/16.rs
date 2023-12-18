use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{Context, Result};
advent_of_code::solution!(16);

#[derive(Debug, Clone)]
struct Grid {
    tiles: HashMap<Coordinate, Tile>,
    rows: usize,
    columns: usize,
}

impl Grid {
    fn new(tiles: HashMap<Coordinate, Tile>) -> Self {
        let rows = tiles.iter().map(|(c, _)| c.y).max().unwrap() + 1;
        let columns = tiles.iter().map(|(c, _)| c.x).max().unwrap() + 1;

        Self {
            tiles: tiles,
            rows: rows,
            columns: columns,
        }
    }

    fn propagate(&mut self) {
        self.propagate_from(Coordinate { x: 0, y: 0 }, Direction::East);
    }

    fn propagate_from(&mut self, coordinate: Coordinate, direction: Direction) {
        type Queue = VecDeque<(Coordinate, Direction)>;

        let mut queue: Queue = VecDeque::from([(coordinate, direction)]);
        let mut visited: HashSet<(Coordinate, Direction)> = HashSet::new();

        while let Some((coordinate, direction)) = queue.pop_front() {
            if visited.contains(&(coordinate, direction)) {
                continue;
            }

            let next_coordinates = self.next_coordinate(&coordinate, &direction);
            for next in next_coordinates {
                queue.push_back(next);
            }
            visited.insert((coordinate, direction));
            self.tiles.get_mut(&coordinate).unwrap().energized = true;
        }
    }

    fn next_coordinate(
        &self,
        coordinate: &Coordinate,
        direction: &Direction,
    ) -> Vec<(Coordinate, Direction)> {
        let mut next = Vec::new();
        let tile = self
            .tiles
            .get(&coordinate)
            .expect(format!("No tile at {:?}", coordinate).as_str());
        match tile.kind {
            TileKind::Empty => {
                if let Some(neighbor) = self.neighbor(&coordinate, direction) {
                    next.push((neighbor, *direction));
                }
            }
            TileKind::VeriticalSplitter => match direction {
                Direction::East | Direction::West => {
                    if let Some(neighbor) = self.neighbor(&coordinate, &Direction::North) {
                        next.push((neighbor, Direction::North));
                    }
                    if let Some(neighbor) = self.neighbor(&coordinate, &Direction::South) {
                        next.push((neighbor, Direction::South));
                    }
                }
                Direction::North | Direction::South => {
                    if let Some(neighbor) = self.neighbor(&coordinate, direction) {
                        next.push((neighbor, *direction));
                    }
                }
            },
            TileKind::HorizontalSplitter => match direction {
                Direction::North | Direction::South => {
                    if let Some(neighbor) = self.neighbor(&coordinate, &Direction::East) {
                        next.push((neighbor, Direction::East));
                    }
                    if let Some(neighbor) = self.neighbor(&coordinate, &Direction::West) {
                        next.push((neighbor, Direction::West));
                    }
                }
                Direction::East | Direction::West => {
                    if let Some(neighbor) = self.neighbor(&coordinate, direction) {
                        next.push((neighbor, *direction));
                    }
                }
            },
            TileKind::ForwardMirror => {
                // /
                let reflection = match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                };
                if let Some(neighbor) = self.neighbor(&coordinate, &reflection) {
                    next.push((neighbor, reflection));
                }
            }
            TileKind::BackwardsMirror => {
                // \
                let reflection = match direction {
                    Direction::North => Direction::West,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                };
                if let Some(neighbor) = self.neighbor(&coordinate, &reflection) {
                    next.push((neighbor, reflection));
                }
            }
        }
        next
    }

    fn neighbor(&self, coordinate: &Coordinate, direction: &Direction) -> Option<Coordinate> {
        match direction {
            Direction::North => {
                if coordinate.y > 0 {
                    return Some(Coordinate {
                        x: coordinate.x,
                        y: coordinate.y - 1,
                    });
                }
            }
            Direction::East => {
                if coordinate.x < self.columns - 1 {
                    return Some(Coordinate {
                        x: coordinate.x + 1,
                        y: coordinate.y,
                    });
                }
            }
            Direction::South => {
                if coordinate.y < self.rows - 1 {
                    return Some(Coordinate {
                        x: coordinate.x,
                        y: coordinate.y + 1,
                    });
                }
            }
            Direction::West => {
                if coordinate.x > 0 {
                    return Some(Coordinate {
                        x: coordinate.x - 1,
                        y: coordinate.y,
                    });
                }
            }
        }
        None
    }

    fn num_energized(&self) -> usize {
        self.tiles.values().filter(|tile| tile.energized).count()
    }

    fn print(&self) {
        for y in 0..self.rows {
            for x in 0..self.columns {
                let tile = self.tiles.get(&Coordinate { x, y }).unwrap();
                if tile.energized {
                    print!("E");
                } else {
                    print!("{}", tile.kind as u8 as char);
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Tile {
    kind: TileKind,
    energized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum TileKind {
    Empty = b'.',
    VeriticalSplitter = b'|',
    HorizontalSplitter = b'-',
    ForwardMirror = b'/',
    BackwardsMirror = b'\\',
}

impl TileKind {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '.' => Ok(TileKind::Empty),
            '|' => Ok(TileKind::VeriticalSplitter),
            '-' => Ok(TileKind::HorizontalSplitter),
            '/' => Ok(TileKind::ForwardMirror),
            '\\' => Ok(TileKind::BackwardsMirror),
            _ => Err(anyhow::anyhow!("invalid tile: {}", c)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_grid(input: &str) -> Result<Grid> {
    let tiles = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(column, c)| {
                let coordinate = Coordinate { x: column, y: row };
                Ok((
                    coordinate,
                    Tile {
                        kind: TileKind::from_char(c)?,
                        energized: false,
                    },
                ))
            })
        })
        .collect::<Result<HashMap<_, _>>>()?;

    Ok(Grid::new(tiles))
}

pub fn part_one(input: &str) -> Result<usize> {
    let mut grid = parse_grid(input)?;
    grid.propagate();
    Ok(grid.num_energized())
}

pub fn part_two(input: &str) -> Result<usize> {
    let grid = parse_grid(input)?;

    let mut best = 0;
    for row in 0..grid.rows {
        let mut right = grid.clone();
        right.propagate_from(Coordinate { x: 0, y: row }, Direction::East);
        best = best.max(right.num_energized());

        let mut left = grid.clone();
        left.propagate_from(
            Coordinate {
                x: left.columns - 1,
                y: row,
            },
            Direction::West,
        );
        best = best.max(left.num_energized());
    }

    for column in 0..grid.columns {
        let mut top = grid.clone();
        top.propagate_from(Coordinate { x: column, y: 0 }, Direction::South);
        best = best.max(top.num_energized());

        let mut bottom = grid.clone();
        bottom.propagate_from(
            Coordinate {
                x: column,
                y: bottom.rows - 1,
            },
            Direction::North,
        );
        best = best.max(bottom.num_energized());
    }
    Ok(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 46);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 51);
        Ok(())
    }
}
