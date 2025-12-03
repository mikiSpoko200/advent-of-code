pub mod common;

use std::{
    collections::HashMap,
    fs,
    io::{self},
    path::Path,
    str::FromStr,
};

use common::*;

pub mod day1;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskInfo {
    day: u8,
    part: u8,
}

impl FromStr for TaskInfo {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let Some((day, part)) = input.split_once("-") else {
            anyhow::bail!("missing '-'")
        };

        let Ok(day @ 1..=12) = day.parse() else {
            anyhow::bail!("invalid `day` encoding {day}")
        };

        let Ok(part @ (1 | 2)) = part.parse() else {
            anyhow::bail!("invalid `part` encoding {part}")
        };

        Ok(Self { day, part })
    }
}

impl TaskInfo {
    pub fn new(day: u8, part: u8) -> Self {
        Self { day, part }
    }
}

pub struct TaskInputDb {
    db: HashMap<TaskInfo, String>,
}

impl TaskInputDb {
    pub fn load_from(dir: &Path) -> io::Result<Self> {
        let mut db = HashMap::new();

        for file in fs::read_dir(dir)? {
            if let Ok(file) = file
                && let Ok(file_name) = file.file_name().into_string()
                && let Some(encoding) = file_name.strip_suffix(".txt")
                && let Ok(task_info) = encoding.parse()
            {
                db.insert(task_info, fs::read_to_string(file.path())?);
            }
        }

        Ok(Self { db })
    }
}

fn main() -> anyhow::Result<()> {
    let TaskInputDb { db } = TaskInputDb::load_from(Path::new("input"))?;

    println!("{}", day1::part1(&db[&TaskInfo::new(1, 1)])?);

    Ok(())
}
