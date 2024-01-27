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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Translator {
    #[default]
    /// Uses googles free translation api
    GoogleFree,
}

impl Translator {
    /// returns the translator from the given string
    /// # Arguments
    /// * `s` - The string to parse
    /// * `api_key` - The api key to use for the translator, if any
    /// # Returns
    /// * `Option<Translator>` - The translator from the given string
    pub fn from_str(s: &str, api_key: Option<String>) -> Option<Translator> {
        match (s, api_key) {
            ("google", None) => Some(Translator::GoogleFree),
            _ => None,
        }
    }

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
