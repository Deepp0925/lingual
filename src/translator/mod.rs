mod deepl;
mod google;
use crate::{
    cfg_blocking, cfg_gen_blocking, cfg_non_blocking, Lang, Translation, TranslationResult,
};
use google::*;
use once_cell::sync::Lazy;

cfg_non_blocking! {
    pub static CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);
}

cfg_blocking! {
    pub static CLIENT: Lazy<reqwest::blocking::Client> = Lazy::new(reqwest::blocking::Client::new);
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Translator {
    #[default]
    /// Uses googles free translation api
    GoogleFree,
}

impl Translator {
    cfg_gen_blocking! {
        pub async fn translate<'a>(
            &'a self,
            text: &'a str,
            from: &'a Lang,
            to: &'a Lang,
        ) -> TranslationResult<Translation<'a>> {
            match self {
                Translator::GoogleFree => google_translate(text, from, to).await,
            }
        }
    }
}
