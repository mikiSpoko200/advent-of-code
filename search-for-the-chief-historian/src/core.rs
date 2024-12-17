use std::{io::{self}, path::PathBuf};
use eyre::{eyre, Result};
use ureq::{AgentBuilder, Cookie};

pub mod ext {

    pub trait Split {
        fn split_exact<const N: usize>(&self) -> Option<[&Self; N]>;
    }

    impl<T> Split for [T] {
        fn split_exact<const N: usize>(&self) -> Option<[&Self; N]> {
            self.try_into().ok()
        }
    }
}

pub trait Solution<const DAY: usize> {
    type Result;

    fn solve() -> Self::Result {
        Self::try_solve().unwrap()
    }

    fn try_solve() -> eyre::Result<Self::Result>;
}

macro_rules! iter {
    ($($head:expr),+ $(,)?) => {
        [$($head),+].into_iter()
    };
}

pub mod functional {
    pub fn delay<T>(mut predicate: impl for<'a> FnMut(&'a T) -> bool, expected: bool) -> impl for<'a> FnMut(&'a T) -> bool {
        let mut found = false;
        move |x| {
            if found {
                return expected;
            }
            let result = predicate(x);
            if result == expected {
                // this must be the first time condition returns true
                // we should yield i, and then no more
                found = true;
            }
            !expected
        }
    }
}

pub struct Config {
    input_cache: PathBuf,
}

pub struct InputManager {
    agent: ureq::Agent,
    config: Config,
}

#[derive(Debug)]
#[derive(thiserror::Error)]
pub struct LoadingReport<T> {
    #[source]
    report: eyre::Report,
    data: Option<T>,
}

impl<T> From<LoadingReport<T>> for Result<T, eyre::Report> {
    fn from(value: LoadingReport<T>) -> Self {
        value.data.ok_or(value.report)
    }
}

impl<V> FromIterator<io::Result<V>> for LoadingReport<V> {
    fn from_iter<T: IntoIterator<Item = io::Result<V>>>(iter: T) -> Self {
        iter.into_iter()
            .take_while(functional::delay(Result::is_err, true))
            .fold(Self { report: eyre!("encountered multiple errors"), data: None }, |mut report, err| {
                match err {
                    Ok(data) => { report.data = Some(data); },
                    Err(err) => { report.report = report.report.wrap_err(err); },
                };
                report
            })
    }
}

type Loader = fn(&InputManager) -> io::Result<String>;

impl InputManager {
    const AUTH_COOKIE: usize = include_str!("../secret/cookie.txt");

    fn create_agent() -> ureq::Agent {
        let cookies = Cookie::new("session", Self::AUTH_COOKIE);

        AgentBuilder::new()
            .cookie_store()
            .build()
    }

    pub fn new() -> Self {

        Self {
            agent: todo!(),
            config: todo!(),
        }
    }

    // TODO: chain of resposibility + error accumulation
    pub fn input(&self) -> eyre::Result<String> {
        Self::chain_of_command()
            .map(|loader| loader(self))
            .collect::<LoadingReport<String>>()
            .into()
    }

    fn fetch(&self) -> io::Result<String> {
        self.agent.get(path)
    }

    fn read_form_disc(&self) -> io::Result<String> {
        todo!()
    }

    fn chain_of_command() -> impl Iterator<Item=Loader> {
        iter! [
            Self::read_form_disc,
            Self::fetch,
        ]
    }
}
