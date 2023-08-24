use strum::{EnumCount, IntoEnumIterator};

#[cfg(feature = "accurate")]
mod accurate;
#[cfg(feature = "accurate")]
pub use accurate::Lang;

#[cfg(not(feature = "accurate"))]
mod all;
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

    pub fn iter_() -> impl Iterator<Item = Self> {
        Lang::iter()
    }

    pub fn can_be_target_lang(&self) -> bool {
        !matches!(self, Lang::Auto)
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

#[cfg(feature = "sea-orm")]
use sea_orm::{entity::prelude::*, sea_query::Alias};
#[cfg(feature = "sea-orm")]
use std::str::FromStr;
#[cfg(feature = "sea-orm")]
impl ActiveEnum for Lang {
    // The macro attribute `rs_type` is being pasted here
    type Value = String;

    type ValueVec = Vec<Self::Value>;

    // By default, the name of Rust enum in camel case if `enum_name` was not provided explicitly
    fn name() -> DynIden {
        SeaRc::new(Alias::new("Lang"))
    }

    // Map Rust enum variants to corresponding `num_value` or `string_value`
    fn to_value(&self) -> Self::Value {
        self.to_string()
    }

    // Map `num_value` or `string_value` to corresponding Rust enum variants
    fn try_from_value(v: &Self::Value) -> Result<Self, DbErr> {
        Self::from_str(v).map_err(|_| DbErr::Type(format!("Invalid Lang: {}", v)))
    }

    // The macro attribute `db_type` is being pasted here
    fn db_type() -> ColumnDef {
        let variants: Vec<DynIden> = Lang::iter()
            .map(|l| SeaRc::new(Alias::new(l.to_string())))
            .collect();
        ColumnType::Enum {
            name: Self::name(),
            variants,
        }
        .def()
    }
}
