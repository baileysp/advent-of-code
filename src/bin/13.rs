use anyhow::Result;
advent_of_code::solution!(13);

enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Debug, Clone)]
struct Pattern {
    rows: Vec<String>,
}

impl Pattern {
    fn summary(&mut self) -> usize {
        let mut total = 0;
        total += reflection(&self.rows).unwrap_or(0) * 100;

        self.transpose();
        total += reflection(&self.rows).unwrap_or(0);
        total
    }

    fn summary_part_two(&mut self) -> usize {
        let mut total = 0;
        total += reflection_part_two(&self.rows).unwrap_or(0) * 100;

        self.transpose();
        total += reflection_part_two(&self.rows).unwrap_or(0);
        total
    }

    fn transpose(&mut self) {
        let rows = self.rows.len();
        let cols = self.rows[0].len();

        self.rows = (0..cols)
            .map(|col| {
                (0..rows)
                    .map(|row| self.rows[row].chars().collect::<Vec<_>>()[col])
                    .collect()
            })
            .collect();
    }

    fn print(&self) {
        for row in &self.rows {
            println!("{}", row);
        }
    }
}

fn reflection(rows: &Vec<String>) -> Option<usize> {
    for row in 1..rows.len() {
        let (top, bottom) = rows.split_at(row);

        let min = top.len().min(bottom.len());
        let mut x = bottom[..min].to_vec();
        x.reverse();

        if &top[top.len() - min..] == x {
            return Some(row);
        }
    }
    None
}

fn reflection_part_two(rows: &Vec<String>) -> Option<usize> {
    for row in 1..rows.len() {
        let (top, bottom) = rows.split_at(row);

        let min = top.len().min(bottom.len());
        let mut x = bottom[..min].to_vec();
        x.reverse();

        let mut differences = 0;

        for (c1, c2) in (&top[top.len() - min..])
            .iter()
            .flat_map(|s| s.chars())
            .zip(x.iter().flat_map(|s| s.chars()))
        {
            if c1 != c2 {
                differences += 1;
            }

            if differences > 1 {
                break;
            }
        }

        if differences == 1 {
            return Some(row);
        }
    }
    None
}

fn parse_patterns(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|p| Pattern {
            rows: p.lines().map(|l| l.chars().collect()).collect(),
        })
        .collect()
}

pub fn part_one(input: &str) -> Result<usize> {
    Ok(parse_patterns(input)
        .iter_mut()
        .map(|p| p.summary())
        .sum::<usize>())
}

pub fn part_two(input: &str) -> Result<usize> {
    Ok(parse_patterns(input)
        .iter_mut()
        .map(|p| p.summary_part_two())
        .sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 405);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 400);
        Ok(())
    }
}
