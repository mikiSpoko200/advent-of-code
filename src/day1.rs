use std::{ops::RangeInclusive, str::FromStr};

use super::Output;

pub struct Dial {
    state: i32,
    span: RangeInclusive<i32>,
}

impl Dial {
    pub const fn is_zero(&self) -> bool {
        self.state == 0
    }

    pub fn span(&self) -> i32 {
        self.span.end() - self.span.start() + 1
    }

    pub fn split(&self, rot: Rotation) -> (Rotation, usize) {
        if rot.extent().abs
    }
}

impl std::ops::AddAssign<Rotation> for Dial {
    fn add_assign(&mut self, rot: Rotation) {
        let encoding = match rot {
            Rotation::Left(extent) => -(extent as i32),
            Rotation::Right(extent) => extent as _,
        };

        self.state = (self.state + encoding).rem_euclid(self.span());
    }
}

pub enum Rotation {
    Left(u32),
    Right(u32),
}

impl Rotation {
    pub const fn extent(&self) -> u32 {
        match *self {
            Self::Left(extent) | Self::Right(extent) => extent
        }
    }
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (direction, extent) = line.split_at(1);

        extent
            .parse()
            .map(match direction {
                "L" => Self::Left,
                "R" => Self::Right,
                _ => anyhow::bail!("invalid rotation instruction format {line}"),
            })
            .map_err(Into::into)
    }
}

pub fn part1(input: &str) -> anyhow::Result<Output> {
    Ok(input
        .split_whitespace()
        .map(str::parse::<Rotation>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .scan(
            Dial {
                state: 50,
                span: 0..=99,
            },
            |dial, rotation| {
                *dial += rotation;

                Some(dial.is_zero() as Output)
            },
        )
        .sum())
}

#[cfg(test)]
#[test]
fn test_rotation_identity() {
    let mut dial = Dial {
        state: 50,
        span: 0..=99,
    };

    dial += Rotation::Left(1000);

    assert_eq!(dial.state, 50);
}
