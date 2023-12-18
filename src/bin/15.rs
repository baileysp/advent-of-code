use std::collections::HashMap;

use anyhow::Result;
advent_of_code::solution!(15);

#[derive(Debug, Clone)]
struct Sequence1<'a> {
    steps: Vec<Step1<'a>>,
}

#[derive(Debug, Clone)]
struct Step1<'a> {
    step: &'a str,
}

impl<'a> Step1<'a> {
    fn hash(&self) -> u32 {
        self.step.chars().fold(0, |mut acc, c| {
            if c == '\n' {
                return acc;
            }

            acc += c as u32;
            acc *= 17;
            acc % 256
        })
    }
}

#[derive(Debug, Clone)]
struct Sequence2 {
    steps: Vec<Step2>,
}

#[derive(Debug, Clone)]
struct Step2 {
    label: String,
    operation: Operation,
    hash: u32,
}

impl Step2 {
    fn new(step: String) -> Self {
        let (label, operation) = match step.chars().last().unwrap() {
            '-' => (step.replace('-', ""), Operation::Dash),
            _ => {
                let (label, focal_length) = step.split_once('=').unwrap();
                (
                    label.to_string(),
                    Operation::Equals {
                        focal_length: focal_length.parse().unwrap(),
                    },
                )
            }
        };

        Self {
            label: label.to_string(),
            hash: hash(label),
            operation: operation,
        }
    }

    fn focal_length(&self) -> u32 {
        match self.operation {
            Operation::Equals { focal_length } => return focal_length,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Dash,
    Equals { focal_length: u32 },
}

fn hash(input: String) -> u32 {
    input.chars().fold(0, |mut acc, c| {
        if c == '\n' {
            return acc;
        }

        acc += c as u32;
        acc *= 17;
        acc % 256
    })
}

fn parse_sequence_part_one(input: &str) -> Sequence1 {
    Sequence1 {
        steps: input.split(",").map(|step| Step1 { step: step }).collect(),
    }
}

fn parse_sequence_part_two(input: &str) -> Sequence2 {
    Sequence2 {
        steps: input
            .split(",")
            .map(|step| Step2::new(step.replace('\n', "")))
            .collect(),
    }
}

pub fn part_one(input: &str) -> Result<u32> {
    let sequence = parse_sequence_part_one(input);
    Ok(sequence.steps.iter().map(|s| s.hash()).sum::<u32>())
}

pub fn part_two(input: &str) -> Result<u32> {
    let sequence = parse_sequence_part_two(input);

    let mut boxes: HashMap<u32, Vec<Step2>> = HashMap::new();
    for step in sequence.steps.as_slice() {
        match step.operation {
            Operation::Dash => {
                if let Some(steps) = boxes.get_mut(&step.hash) {
                    if let Some(index) = steps.iter().position(|s| *s.label == step.label) {
                        steps.remove(index);
                    }
                }
            }
            Operation::Equals { focal_length: _ } => match boxes.get_mut(&step.hash) {
                Some(steps) => {
                    if let Some(index) = steps.iter().position(|s| *s.label == step.label) {
                        steps[index] = step.clone();
                    } else {
                        steps.push(step.clone());
                    }
                }
                None => {
                    boxes.insert(step.hash, vec![step.clone()]);
                }
            },
        }
    }

    Ok(boxes.iter().fold(0, |acc, (&i, steps)| {
        acc + steps
            .iter()
            .enumerate()
            .map(|(step_index, s)| (1 + i as u32) * (step_index as u32 + 1) * s.focal_length())
            .sum::<u32>()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 1320);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 145);
        Ok(())
    }
}
