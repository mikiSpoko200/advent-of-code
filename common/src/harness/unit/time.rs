use std::{time::Duration, marker::PhantomData, fmt::Display};

use super::{super::Measurement, prefix::{Prefix, _None, Milli, Micro, Nano}};


/// Marker for units of measurement.

pub struct Time;

impl Measurement for Time { }

pub trait Unit: Convert + Default + Display { }

pub trait Convert {
    type Output: std::fmt::Display;

    fn convert(&self) -> Self::Output;
}

#[derive(Default)]
pub struct Second<P: Prefix = _None>(Duration, PhantomData<P>);

impl<P: Prefix> Second<P> {
    pub fn new(duration: Duration) -> Self {
        Self(duration, Default::default())
    }
}

impl Unit for Second { }
impl Unit for Second<Milli> { }
impl Unit for Second<Micro> { }
impl Unit for Second<Nano> { }

impl Convert for Second {
    type Output = u64;

    fn convert(&self) -> Self::Output {
        self.0.as_secs()
    }
}

impl Convert for Second<Milli> {
    type Output = u128;

    fn convert(&self) -> Self::Output {
        self.0.as_millis()
    }
}

impl Convert for Second<Micro> {
    type Output = u128;

    fn convert(&self) -> Self::Output {
        self.0.as_micros()
    }
}

impl Convert for Second<Nano> {
    type Output = u128;

    fn convert(&self) -> Self::Output {
        self.0.as_nanos()
    }
}

impl<P: Prefix + Default + std::fmt::Display> std::fmt::Display for Second<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", P::default())
    }
}
