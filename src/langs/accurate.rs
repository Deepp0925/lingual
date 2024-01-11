#![allow(dead_code)]
use strum::{AsRefStr, EnumCount, EnumIter, EnumString};

use super::Lang;
/// The language codes supported by the API.
/// It is also possible to use strings like "en" or "fr" instead of the enum variants but it is not recommended
/// because it is not checked at compile time, therefore it is eliminated by default features.
/// To enable this feature, add `strings` to the features list of the crate.
/// get how many variants are there in the enum at compile time.
/// This is list has languages that have high translation accuracy and
/// are also part of DeepL's supported languages.
/// this list includes:
/// - english
/// - french
/// - german
/// - spanish,
/// - italian,
/// - portuguese,
/// - russian,
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
    Default,
    EnumCount,
    EnumIter,
    EnumString,
    AsRefStr,
    serde::Serialize,
    serde::Deserialize,
)]
#[strum(serialize_all = "kebab-case")]
pub enum AccurateLang {
    #[default]
    Auto,
    En,
    Fr,
    De,
    Es,
    It,
    Pt,
    Ru,
    ZhCn,
    Ja,
    Ko,
}

impl AccurateLang {
    pub const fn len() -> usize {
        AccurateLang::COUNT
    }

    /// this is used to map the lang varient to the string(full) representation of the language
    /// for example: `Lang::En` => "English", `Lang::Fr` => "French", `Lang::Auto` => "Auto"
    pub fn fullname(&self) -> &str {
        let lang: &Lang = self.into();
        lang.fullname()
    }

    pub fn is_accurate_lang(lang: &Lang) -> bool {
        let accurate_lang: &AccurateLang = lang.into();
        // if accurate_lang is auto and so is lang then return true
        if accurate_lang == &AccurateLang::Auto {
            // check if lang is also auto
            if lang == &Lang::Auto {
                return true;
            }

            return false;
        }

        true
    }
}

impl From<&AccurateLang> for &Lang {
    fn from(lang: &AccurateLang) -> Self {
        match lang {
            AccurateLang::Auto => &Lang::Auto,
            AccurateLang::En => &Lang::En,
            AccurateLang::Fr => &Lang::Fr,
            AccurateLang::De => &Lang::De,
            AccurateLang::Es => &Lang::Es,
            AccurateLang::It => &Lang::It,
            AccurateLang::Pt => &Lang::Pt,
            AccurateLang::Ru => &Lang::Ru,
            AccurateLang::ZhCn => &Lang::ZhCn,
            AccurateLang::Ja => &Lang::Ja,
            AccurateLang::Ko => &Lang::Ko,
        }
    }
}

impl From<AccurateLang> for Lang {
    fn from(lang: AccurateLang) -> Self {
        lang.into()
    }
}

impl From<&Lang> for &AccurateLang {
    fn from(lang: &Lang) -> Self {
        match lang {
            Lang::Auto => &AccurateLang::Auto,
            Lang::En => &AccurateLang::En,
            Lang::Fr => &AccurateLang::Fr,
            Lang::De => &AccurateLang::De,
            Lang::Es => &AccurateLang::Es,
            Lang::It => &AccurateLang::It,
            Lang::Pt => &AccurateLang::Pt,
            Lang::Ru => &AccurateLang::Ru,
            Lang::ZhCn => &AccurateLang::ZhCn,
            Lang::Ja => &AccurateLang::Ja,
            Lang::Ko => &AccurateLang::Ko,
            _ => &AccurateLang::Auto,
        }
    }
}

impl From<Lang> for AccurateLang {
    fn from(lang: Lang) -> Self {
        match lang {
            Lang::Auto => AccurateLang::Auto,
            Lang::En => AccurateLang::En,
            Lang::Fr => AccurateLang::Fr,
            Lang::De => AccurateLang::De,
            Lang::Es => AccurateLang::Es,
            Lang::It => AccurateLang::It,
            Lang::Pt => AccurateLang::Pt,
            Lang::Ru => AccurateLang::Ru,
            Lang::ZhCn => AccurateLang::ZhCn,
            Lang::Ja => AccurateLang::Ja,
            Lang::Ko => AccurateLang::Ko,
            _ => AccurateLang::Auto,
        }
    }
}
