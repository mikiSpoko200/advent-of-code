use super::{Assert, True, Solution, Day};

/// Allows for retrival of specific Solution based on const index.
pub trait Get<const PART: usize> 
where
    Assert<{PART > 0}>: True,
    Assert<{PART < 3}>: True,
{
    type Solution: Solution<PART>;

    fn get(&self) -> &Self::Solution;
}

impl<const N: usize, S1, S2> Get<1> for Day<N, S1, S2>
where
    Assert<{N > 0}>: True,
    Assert<{N < 25}>: True,
    S1: Solution<1>,
    S2: Solution<2>,
{
    type Solution = S1;

    fn get(&self) -> &Self::Solution {
        &self.first
    }
}

impl<const N: usize, S1, S2> Get<2> for Day<N, S1, S2> 
where
    Assert<{N > 0}>: True,
    Assert<{N < 25}>: True,
    S1: Solution<1>,
    S2: Solution<2>,
{
    type Solution = S2;

    fn get(&self) -> &Self::Solution {
        &self.second
    }
}