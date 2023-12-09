use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
advent_of_code::solution!(8);

#[derive(Debug)]
enum Instruction {
    Right,
    Left,
}

impl Instruction {
    fn from_char(c: char) -> Result<Self> {
        let instruction = match c {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => bail!("invalid instruction char: {}", c),
        };
        Ok(instruction)
    }
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    right: &'a str,
    left: &'a str,
}

impl<'a> Node<'a> {
    fn from_str(s: &'a str) -> Result<Self> {
        let (n, pair) = s.split_once("=").ok_or(anyhow!("invalid node"))?;
        let (left, right) = pair
            .trim()
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split_once(",")
            .ok_or(anyhow!("invalid node"))?;

        Ok(Node {
            name: n.trim(),
            right: right.trim(),
            left: left.trim(),
        })
    }
}

#[derive(Debug)]
struct Network<'a> {
    instructions: Vec<Instruction>,
    nodes: HashMap<&'a str, Node<'a>>,
}

fn parse_network(input: &str) -> Network {
    let lines: Vec<_> = input.lines().collect();
    let instructions = lines[0]
        .chars()
        .map(|c| Instruction::from_char(c).unwrap())
        .collect();

    let nodes = lines[2..]
        .iter()
        .map(|l| {
            let node = Node::from_str(l).unwrap();
            (node.name, node)
        })
        .collect();

    Network {
        instructions: instructions,
        nodes: nodes,
    }
}

pub fn part_one(input: &str) -> Result<u32> {
    let network = parse_network(input);

    let mut count = 0;
    let mut current = "AAA";
    for instruction in network.instructions.iter().cycle() {
        let node = network.nodes.get(current).ok_or(anyhow!("missing node"))?;
        current = match instruction {
            Instruction::Right => node.right,
            Instruction::Left => node.left,
        };
        count += 1;

        if current == "ZZZ" {
            break;
        }
    }

    Ok(count)
}

pub fn part_two(input: &str) -> Result<usize> {
    let network = parse_network(input);

    let mut count = 0;
    let mut currents: Vec<&str> = network
        .nodes
        .keys()
        .filter_map(|s| {
            if s.ends_with("A") {
                return Some(*s);
            }
            None
        })
        .collect();

    let num_starts = currents.len();
    let mut ends = vec![];

    for instruction in network.instructions.iter().cycle() {
        currents = currents
            .iter()
            .filter_map(|c| {
                let node = network.nodes.get(*c).expect("missing node");

                if node.name.ends_with("Z") {
                    ends.push(count);
                    return None;
                }
                return Some(match instruction {
                    Instruction::Right => node.right,
                    Instruction::Left => node.left,
                });
            })
            .collect();

        count += 1;
        if ends.len() == num_starts {
            break;
        }
    }

    Ok(lcm(&ends))
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 6);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 6);
        Ok(())
    }
}
