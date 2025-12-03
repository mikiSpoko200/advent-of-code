use crate::TaskInfo;

pub type Output = u64;

pub trait FromLines<'a>: FromIterator<Self::Line> {
    type Line: 'a;

    fn parse_line(line: &'a str) -> Self::Line;

    fn parse(lines: impl Iterator<Item = &'a str>) -> Self {
        lines.map(Self::parse_line).collect::<Self>()
    }
}

pub trait ITask {
    const INFO: TaskInfo;

    fn solve(self, input: &str) -> Output;
}

pub struct Task<const DAY: usize, const PART: usize>;
