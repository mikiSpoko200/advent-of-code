
use std::{cmp::Ordering, str::FromStr};
use arrayvec::ArrayVec;

pub type Level = u32;

pub mod func {
    pub fn pred_with_tolerance<T>(mut pred: impl FnMut(T) -> bool, mut tolerance: usize) -> impl FnMut(T) -> bool {
        move |arg| (pred(arg) == false)
            .then(|| if tolerance > 0 { tolerance -= 1; true } else { false })
            .unwrap_or(true)
    }
}

#[derive(Debug)]
pub struct Report<const STACK_VEC_CAPACITY: usize = 16> {
    levels: ArrayVec<Level, STACK_VEC_CAPACITY>
}

impl<const STACK_VEC_CAPACITY: usize> Report<STACK_VEC_CAPACITY> {
    const SAFETY_RANGE: std::ops::RangeInclusive<Level> = 1..=3;

    fn adjacent(&self) -> impl Iterator<Item = [Level; 2]> + '_ {
        self.levels.windows(2).map(|pair| pair.try_into().expect("windows are of size 2"))
    }

    fn all_adjacent(&self, pred: impl FnMut([Level; 2]) -> bool) -> bool {
        self.adjacent().all(pred)
    }
 
    fn are_adjacent_in_range([prev, next]: [Level; 2]) -> bool {
        Self::SAFETY_RANGE.contains(&prev.abs_diff(next))
    }

    pub fn is_safe_by<const PART: usize>(&self) -> bool
    where
        Self: IsSafe<PART>
    {
        self.is_safe()
    }

    const fn monotonic_cmp(ordering: Ordering) -> impl FnMut([Level; 2]) -> bool {
        move |[prev, next]| prev.cmp(&next) != ordering
    }
}

impl<const STACK_VEC_CAPACITY: usize> FromIterator<Level> for Report<STACK_VEC_CAPACITY> {
    fn from_iter<T: IntoIterator<Item = Level>>(iter: T) -> Self {
        Self {
            levels: iter
                .into_iter()
                .collect::<ArrayVec<_, STACK_VEC_CAPACITY>>()
        }
    }
}

impl<const STACK_VEC_CAPACITY: usize> FromStr for Report<STACK_VEC_CAPACITY> {
    type Err = eyre::Report;

    fn from_str(line: &str) -> eyre::Result<Self> {
        line.split_whitespace()
            .take(STACK_VEC_CAPACITY)
            .map(str::parse)
            .collect::<Result<Self, _>>()
            .map_err(Into::into)
    }
}

pub trait IsSafe<const PART: usize> {
    fn is_safe(&self) -> bool;
}

pub mod part1 {
    use super::*;
    use crate::Day;

    impl<const STACK_VEC_CAPACITY: usize> super::IsSafe<1> for Report<STACK_VEC_CAPACITY> {
        fn is_safe(&self) -> bool {
            self.all_adjacent(|levels| Self::are_adjacent_in_range(levels))
            && (
                self.all_adjacent(Self::monotonic_cmp(Ordering::Less))
                || self.all_adjacent(Self::monotonic_cmp(Ordering::Greater))
            )
        }
    }

    pub fn solve() {
        let day = Day::new(2, 1);

        let result = day.input()
            .lines()
            .map(|line| line
                .parse::<Report>()
                .unwrap()
            )
            .filter(Report::is_safe_by::<1>)
            .count();
        day.report(result);
    }
}

pub mod part2 {
    use super::*;
    use crate::Day;

    impl<const STACK_VEC_CAPACITY: usize> super::IsSafe<2> for Report<STACK_VEC_CAPACITY> {
        fn is_safe(&self) -> bool {
            (0..self.levels.len())
                .into_iter()
                .map(|selector| self.levels.iter()
                    .map(Clone::clone)
                    .enumerate()
                    .filter_map(|(index, levels)| if index != selector {
                        Some(levels)
                    } else {
                        None
                    })
                    .collect::<Self>()
                )
                .any(|rep| rep.is_safe_by::<1>())
        }
    }

    pub fn solve() {
        let day = Day::new(2, 2);

        let result = day.input()
            .lines()
            .map(|line| line
                .parse::<Report>()
                .unwrap()
            )
            .filter(Report::is_safe_by::<2>)
            .count();
        day.report(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() { assert_eq!("7 6 4 2 1".parse::<Report>().unwrap().is_safe_by::<2>(), true) }
    
    #[test]
    fn test_2() { assert_eq!("1 2 7 8 9".parse::<Report>().unwrap().is_safe_by::<2>(), false) }
    
    #[test]
    fn test_3() { assert_eq!("9 7 6 2 1".parse::<Report>().unwrap().is_safe_by::<2>(), false) }
    
    #[test]
    fn test_4() { assert_eq!("1 3 2 4 5".parse::<Report>().unwrap().is_safe_by::<2>(), true) }
    
    #[test]
    fn test_5() { assert_eq!("8 6 4 4 1".parse::<Report>().unwrap().is_safe_by::<2>(), true) }
    
    #[test]
    fn test_6() { assert_eq!("1 3 6 7 9".parse::<Report>().unwrap().is_safe_by::<2>(), true) }
    
    #[test]
    fn test_7() { assert_eq!("1 5 6 7 9".parse::<Report>().unwrap().is_safe_by::<2>(), true) }
}