use strum::{EnumCount, IntoEnumIterator};

mod accurate;
pub use accurate::AccurateLang;

mod all;
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

    pub fn iter_() -> impl Iterator<Item = Self> {
        Lang::iter()
    }

    pub fn accurate_iter_() -> impl Iterator<Item = AccurateLang> {
        AccurateLang::iter()
    }

    ///checks if the provided lang can be converted to an accurate lang
    pub fn is_accurate_lang(&self) -> bool {
        AccurateLang::is_accurate_lang(self)
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
