/// Marker for metric prefixes.
pub trait Prefix: std::fmt::Display { }

#[derive(Default)]
pub struct _None;
impl Prefix for _None { }

#[derive(Default)]
pub struct Mega;
impl Prefix for Mega { }

#[derive(Default)]
pub struct Kilo;
impl Prefix for Kilo { }

#[derive(Default)]
pub struct Milli;
impl Prefix for Milli { }

#[derive(Default)]
pub struct Micro;
impl Prefix for Micro { }

#[derive(Default)]
pub struct Nano;
impl Prefix for Nano { }

macro_rules! const_display {
    ($t:ty, $val: literal) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str($val)
            }
        }
    };
}
const_display!(Mega, "M");
const_display!(Kilo, "k");
const_display!(_None, "");
const_display!(Milli, "m");
const_display!(Micro, "μ");
const_display!(Nano, "n");
