use colored::{ColoredString, Colorize};


pub trait Color {
    fn color(input: &str) -> ColoredString;
}

macro_rules! make_color {
    ($color:ident, $func:ident) => {
        pub struct $color;
        impl Color for $color {
            fn color(input: &str) -> ColoredString {
                <&str as colored::Colorize>::$func(input)
            }
        }
    };
}

make_color!(Black, black);
make_color!(Blue, blue);
make_color!(Green, green);
make_color!(Red, red);
make_color!(Cyan, cyan);
make_color!(Magenta, magenta);
make_color!(Yellow, yellow);
make_color!(White, white);

make_color!(BrightBlack, bright_black);
make_color!(BrightRed, bright_red);
make_color!(BrightGreen, bright_green);
make_color!(BrightYellow, bright_yellow);
make_color!(BrightBlue, bright_blue);
make_color!(BrightMagenta, bright_magenta);
make_color!(BrightCyan, bright_cyan);
make_color!(BrightWhite, bright_white);

pub struct TrueColor<const R: u8, const G: u8, const B: u8>;

impl<const R: u8, const G: u8, const B: u8> Color for TrueColor<R, G, B> {
    fn color(input: &str) -> ColoredString {
        input.truecolor(R, G, B)
    }
}
