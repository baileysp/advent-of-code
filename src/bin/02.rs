use anyhow::{Context, Result};
advent_of_code::solution!(2);

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    pulls: Vec<Vec<Pull>>,
}

impl Game {
    fn possible(&self, blue: u32, green: u32, red: u32) -> bool {
        // for each pull set, do we have a pull geater than
        let color_possible = |color: Color, count: u32| -> bool {
            self.pulls
                .clone()
                .into_iter()
                .flatten()
                .filter(|p| p.color == color)
                .all(|p| p.count <= count)
        };

        color_possible(Color::Blue, blue)
            && color_possible(Color::Green, green)
            && color_possible(Color::Red, red)
    }

    fn min_cube_set_power(&self) -> u32 {
        let min = |color: Color| -> u32 {
            let x = self
                .pulls
                .clone()
                .into_iter()
                .flatten()
                .filter(|p| p.color == color)
                .max_by(|x, y| x.count.cmp(&y.count));

            match x {
                Some(Pull { count, color: _ }) => count,
                _ => 0,
            }
        };
        min(Color::Blue) * min(Color::Green) * min(Color::Red)
    }
}

#[derive(Debug, Clone, Copy)]
struct Pull {
    color: Color,
    count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Blue,
    Green,
    Red,
}

fn parse_games(input: &str) -> Result<Vec<Game>> {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s| {
            let (game, rest) = s.split_once(':').context("split game")?;
            let num = game
                .split_whitespace()
                .last()
                .expect("game number")
                .parse::<u32>()
                .expect("parse game number");
            let pulls: Vec<_> = rest
                .split_terminator(';')
                .map(|s| {
                    s.split(',')
                        .map(|p| {
                            let x: Vec<_> = p.trim().split_whitespace().collect();
                            let num = x
                                .first()
                                .expect("pull count")
                                .parse::<u32>()
                                .expect("parse pull count");

                            let color = match x.last().expect("color") {
                                &"blue" => Color::Blue,
                                &"green" => Color::Green,
                                &"red" => Color::Red,
                                &&_ => todo!(),
                            };
                            Pull { color, count: num }
                        })
                        .collect::<Vec<Pull>>()
                })
                .collect();

            Ok(Game { id: num, pulls })
        })
        .collect()
}

pub fn part_one(input: &str) -> Result<u32> {
    let games = parse_games(input)?;

    // only 12 red cubes, 13 green cubes, and 14 blue cubes?
    let mut sum = 0;
    for game in games {
        if game.possible(14, 13, 12) {
            sum += game.id
        }
    }

    Ok(sum)
}

pub fn part_two(input: &str) -> Result<u32> {
    Ok(parse_games(input)?
        .into_iter()
        .map(|g| g.min_cube_set_power())
        .sum())
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
        assert_eq!(result.unwrap(), 8);
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
        assert_eq!(result.unwrap(), 2286);
    }
}
