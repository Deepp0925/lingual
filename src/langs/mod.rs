mod accurate;
mod all;
use strum::{EnumCount, IntoEnumIterator};

#[cfg(feature = "accurate")]
pub use accurate::Lang;
#[cfg(not(feature = "accurate"))]
pub use all::Lang;

pub trait OptionLangExt {
    /// returns the language if it is set, otherwise returns default src: `Lang::Auto`
    fn unwrap_or_default_src(&self) -> Lang;
    /// returns the language if it is set, otherwise returns default target: `Lang::En`
    fn unwrap_or_default_trgt(&self) -> Lang;
}

impl Lang {
    pub const fn len() -> usize {
        Lang::COUNT
    }

    pub fn iter_() -> impl Iterator<Item = Lang> {
        Lang::iter()
    }
}

impl OptionLangExt for Option<Lang> {
    /// returns the language if it is set, otherwise returns default src: `Lang::Auto`
    fn unwrap_or_default_src(&self) -> Lang {
        match self {
            None => Lang::Auto,
            Some(l) => *l,
        }
    }
    /// returns the language if it is set, otherwise returns default target: `Lang::En`
    fn unwrap_or_default_trgt(&self) -> Lang {
        match self {
            None => Lang::En,
            Some(l) => *l,
        }
    }
}
