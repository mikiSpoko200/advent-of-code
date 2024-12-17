
pub mod part1 {
    use crate::Day;

    pub(super) fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
        let mut first = Vec::new();
        let mut second = Vec::new();
        for line in input.lines() {
            let mut parsed = line.split_whitespace();
            first.push(parsed.next().expect("missing first number").parse().expect("first element is not a number"));
            second.push(parsed.next().expect("missing second number").parse().expect("second element is a number"));
        }
        (first, second)
    }
    
    pub fn solve() {
        let day = Day::new(1, 1);
        let (mut first, mut second) = parse(&day.input());
        first.sort();
        second.sort();
        let result: u32 = first.iter().zip(second).map(|(first, second)| first.abs_diff(second)).sum();
        day.report(result);
    }
}


pub mod part2 {
    use std::{collections::HashMap, ops::{Deref, Mul}};

    use crate::Day;

    use super::part1::parse;

    pub fn solve() {
        let day = Day::new(1, 2);

        let (first, second) = parse(&day.input());

        let mut counter = second
            .iter()
            .fold(
                HashMap::<u32, u32>::with_capacity(second.len()), 
                |mut counter, number| {
                    counter.entry(*number)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    counter
                }
            );

        let result: u32 = first.into_iter()
            .map(move |num| counter.entry(num)
                .or_default()
                .deref()
                .mul(num)
            ).sum();
        day.report(result);
    }
}