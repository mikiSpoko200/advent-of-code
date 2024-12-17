mod core;
mod day1;
mod day2;
// mod day3;
mod day12;

pub struct Day {
    number: usize,
    part: usize,
}

impl Day {
    pub fn new(number: usize, part: usize) -> Self {
        Self {
            number,
            part,
        }
    }

    pub fn input(&self) -> eyre::Result<String> {
        std::fs::read_to_string(format!("inputs/input-{}-{}.txt", self.number, self.part))
    }

    pub fn report<T>(&self, data: T) where T: std::fmt::Display {
        println!("day {}-{}: {}", self.number, self.part, data);
    }
}

fn main() {
    day1::part1::solve();
    day1::part2::solve();
    day2::part1::solve();
    day2::part2::solve();
    // day3::part1::solve();
    // day3::part2::solve();
    day12::part1::solve();
}
