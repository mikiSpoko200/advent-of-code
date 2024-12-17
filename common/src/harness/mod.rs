pub mod color;
mod getter;
pub mod unit;

use std::{
    fmt::{Display, Formatter},
    marker::PhantomData, path::PathBuf,
};

use crate::harness::unit::time::Convert;

use self::{
    color::{Color, Cyan},
    unit::{
        prefix::{Prefix, Micro},
        time::{Second, Time},
    }, getter::Get,
};

pub struct Assert<const B: bool>;
pub trait True {}
pub trait False {}

impl True for Assert<true> {}
impl False for Assert<true> {}

pub trait Solution<'input, const PART: usize>
where
    Assert<{ PART > 0 }>: True,
    Assert<{ PART < 3 }>: True,
{
    type Output: std::fmt::Display;

    fn process(input: &'input str) -> Self;

    fn solve(&self) -> Self::Output;
}

#[derive(Clone, Copy)]
pub struct Unimplemented;

impl Display for Unimplemented {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("unimplemented")
    }
}

impl<'input, const PART: usize> Solution<'input, PART> for Unimplemented
where
    Assert<{ PART > 0 }>: True,
    Assert<{ PART < 3 }>: True,
{
    type Output = Self;

    fn process(_: &'input str) -> Self {
        Self
    }

    fn solve(&self) -> Self::Output {
        Self
    }
}

pub struct Day<const N: usize, S1, S2>
where
    Assert<{ N > 0 }>: True,
    Assert<{ N < 25 }>: True,
    S1: Solution<1>,
    S2: Solution<2>,
{
    input: String,
    _first_phantom: PhantomData<S1>,
    _second_phantom: PhantomData<S2>,
}

impl<const N: usize, S1, S2> Default for Day<N, S1, S2>
where
    Assert<{ N > 0 }>: True,
    Assert<{ N < 25 }>: True,
    S1: Solution<1>,
    S2: Solution<2>,
{
    fn default() -> Self {
        Self { 
            input: std::fs::read_to_string(Self::input()).expect(&format!("input file for day {} exists", N)),
            _first_phantom: PhantomData,
            _second_phantom: PhantomData,
        }
    }
}

impl<const N: usize, S1, S2> Day<N, S1, S2>
where
    Assert<{ N > 0 }>: True,
    Assert<{ N < 25 }>: True,
    S1: Solution<1>,
    S2: Solution<2>,
{
    #[inline(always)]
    fn input() -> PathBuf
    {
        format!("input/day-{}.txt", N).into()
    }

    /// Benchmark all solutions.
    #[allow(unused)]
    #[inline(always)]
    pub fn benchmark(&self) {
        println!("{}", self.time::<1>());
        println!("{}", self.time::<2>());
    }

    /// Time solution `PART` and return measurements.
    #[allow(unused)]
    #[inline(always)]
    pub fn time<const PART: usize>(
        &self,
    ) -> Benchmark<<<Self as getter::Get<PART>>::Solution as Solution<PART>>::Output, N, PART>
    where
        Assert<{ PART > 0 }>: True,
        Assert<{ PART < 3 }>: True,
        Self: Get<PART>,
    {
        let start = std::time::Instant::now();
        let output = Get::<PART>::get(self).solve();
        let end = std::time::Instant::now();

        Benchmark {
            duration: end - start,
            output: output,
            _config_phantom: PhantomData,
        }
    }
}

pub trait Measurement {}

// Make trait that encompasses color configuration;
pub trait Configuration<M: Measurement> {
    type Prefix: Prefix + Default;
    type Color: Color;
}

pub struct TimeConfiguration<P: Prefix + Default = Micro, C: Color = Cyan>(PhantomData<P>, PhantomData<C>);

impl<P, C> Configuration<Time> for TimeConfiguration<P, C>
where
    P: Prefix + Default,
    C: Color,
{
    type Prefix = P;
    type Color = C;
}

pub struct Benchmark<T, const DAY: usize, const TASK: usize, C=TimeConfiguration>
where
    T: std::fmt::Display,
    C: Configuration<Time>,
{
    duration: std::time::Duration,
    output: T,
    _config_phantom: PhantomData<C>,
}

impl<T, CO, const DAY: usize, const TASK: usize> Benchmark<T, DAY, TASK, CO>
where
    T: std::fmt::Display,
    CO: Configuration<Time>,
{
    #[allow(unused)]
    #[inline(always)]
    pub fn configuration<P: Prefix + Default, C: Color>(self) -> Benchmark<T, DAY, TASK, TimeConfiguration<P, C>> {
        Benchmark {
            _config_phantom: PhantomData,
            duration: self.duration,
            output: self.output,
        }
    }
}

impl<T, C, const DAY: usize, const TASK: usize> std::fmt::Display for Benchmark<T, DAY, TASK, C>
where
    T: std::fmt::Display,
    C: Configuration<Time>,
    Second<C::Prefix>: unit::time::Convert,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Benchmark :: Day {} :: Task {}", DAY, TASK)?;
        let time = Second::<C::Prefix>::new(self.duration);
        let formatted_time = format!("{}", time.convert());
        writeln!(f, "  Time: {} {}", C::Color::color(&formatted_time), Second::<C::Prefix>::default())?;
        writeln!(f, "  Output: {}", self.output)
    }
}
