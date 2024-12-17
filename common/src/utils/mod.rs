
pub trait ParseWithShared<S>: Sized {
    fn parse(s: &str, shared: &mut S) -> Self;
}

pub mod parse {
    use std::str::FromStr;

    pub fn whitespace_separeted<P: FromStr>(input: &str) -> impl Iterator<Item=P> + '_ {
        input.trim()
            .trim_end()
            .split_whitespace()
            .filter_map(|element| P::from_str(element).ok())
    }
}
