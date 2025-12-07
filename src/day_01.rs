use std::{
    num::ParseIntError,
    ops::{Add, AddAssign},
    str::FromStr,
};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum Direction {
    Left(i64),
    Right(i64),
}

impl Direction {
    fn minimize(&self) -> Direction {
        match self {
            Direction::Left(inner) => Direction::Left(inner % 100),
            Direction::Right(inner) => Direction::Right(inner % 100),
        }
    }

    fn normalize(&self) -> Direction {
        match self {
            Direction::Right(inner) => Direction::Right(*inner),
            Direction::Left(inner) => Direction::Right(100 - inner),
        }
    }

    fn inner(&self) -> i64 {
        match self {
            Direction::Left(inner) => *inner,
            Direction::Right(inner) => *inner,
        }
    }
}

impl FromStr for Direction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..1] {
            "L" => s[1..].parse::<i64>().map(Direction::Left),
            "R" => s[1..].parse::<i64>().map(Direction::Right),
            other => unreachable!("{other}"),
        }
    }
}

impl Add<Direction> for i64 {
    type Output = i64;
    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Left(inner) => self - inner,
            Direction::Right(inner) => self + inner,
        }
    }
}

impl AddAssign<Direction> for i64 {
    fn add_assign(&mut self, rhs: Direction) {
        let res = *self + rhs;
        *self = res;
    }
}
impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.minimize().normalize().inner() == other.minimize().normalize().inner()
    }
}

#[aoc_generator(day01, part1)]
pub fn generator_part1(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| line.parse::<Direction>().unwrap())
        .map(|direction| direction.minimize().normalize())
        .collect()
}

#[aoc(day01, part1)]
pub fn part1(directions: &[Direction]) -> u64 {
    let mut result = 0;

    let mut counter = 50;

    for direction in directions {
        counter = (counter + *direction) % 100;

        if counter == 0 {
            result += 1;
        }
    }

    result
}

#[aoc_generator(day01, part2)]
pub fn generator_part2(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| line.parse::<Direction>().unwrap())
        .collect()
}

#[aoc(day01, part2)]
pub fn part2(directions: &[Direction]) -> u64 {
    let mut result: u64 = 0;
    let mut counter = 50;

    for direction in directions {
        let total = direction.inner();
        let offset = match direction {
            Direction::Left(_) => -1,
            Direction::Right(_) => 1,
        };

        // Yes. We do it like this. Simply, because I do not care enough
        for _ in 0..total {
            counter += offset;
            if counter == 0 {
                result += 1;
            } else if counter == 100 || counter == -100 {
                result += 1;
                counter = 0;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{generator_part1, generator_part2, part1, part2};

    const SAMPLE_INPUT: &str = "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82";

    #[test]
    fn test_generator_part1() {
        use super::Direction::*;
        let parsed = generator_part1(SAMPLE_INPUT);

        assert_eq!(
            parsed,
            vec![
                Left(68),
                Left(30),
                Right(48),
                Left(5),
                Right(60),
                Left(55),
                Left(1),
                Left(99),
                Right(14),
                Left(82)
            ]
        );
    }

    #[test]
    fn test_part1() {
        let generated = generator_part1(SAMPLE_INPUT);
        let result = part1(&generated);

        assert_eq!(result, 3)
    }

    #[test]
    fn test_part2() {
        let generated = generator_part2(SAMPLE_INPUT);
        let result = part2(&generated);

        assert_eq!(result, 6)
    }

    #[test]
    fn test_part2_large_rotation() {
        // Test with a large rotation that wraps multiple times
        let large_input = "R1000";
        let generated = generator_part2(large_input);
        let result = part2(&generated);

        // Starting at 50, R1000 should pass through 0 ten times
        assert_eq!(result, 10)
    }
}
