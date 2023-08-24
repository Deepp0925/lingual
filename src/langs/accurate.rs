#![allow(dead_code)]
use strum::{EnumCount, EnumIter, EnumString};
/// The language codes supported by the API.
/// It is also possible to use strings like "en" or "fr" instead of the enum variants but it is not recommended
/// because it is not checked at compile time, therefore it is eliminated by default features.
/// To enable this feature, add `strings` to the features list of the crate.
/// get how many variants are there in the enum at compile time.
/// This is list has languages that have high translation accuracy and only available
/// if enable the `accurate` feature.
/// this list includes:
/// - english
/// - french
/// - german
/// - spanish,
/// - italian,
/// - portuguese,
/// - russian,
/// - arabic,
/// - chinese,
/// - japanese,
/// - korean
#[derive(
    Debug,
    PartialEq,
    Clone,
    Copy,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumCount,
    EnumIter,
    EnumString,
    strum::Display,
    serde::Serialize,
    serde::Deserialize,
)]
#[strum(serialize_all = "kebab-case")]
pub enum Lang {
    Auto,
    En,
    Fr,
    De,
    Es,
    It,
    Pt,
    Ru,
    Ar,
    ZhCn,
    ZhTw,
    Ja,
    Ko,
}

impl Lang {
    /// this is used to map the lang varient to the string(full) representation of the language
    /// for example: `Lang::En` => "English", `Lang::Fr` => "French", `Lang::Auto` => "Auto"
    pub fn fullname(&self) -> &str {
        match self {
            Lang::Auto => "Auto",
            Lang::En => "English",
            Lang::Fr => "French",
            Lang::De => "German",
            Lang::Es => "Spanish",
            Lang::It => "Italian",
            Lang::Pt => "Portuguese",
            Lang::Ru => "Russian",
            Lang::Ar => "Arabic",
            Lang::ZhCn => "Chinese Simplified",
            Lang::ZhTw => "Chinese Traditional",
            Lang::Ja => "Japanese",
            Lang::Ko => "Korean",
        }
    }
}
