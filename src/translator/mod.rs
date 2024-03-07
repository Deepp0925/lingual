mod deepl;
mod google;
use crate::{
    cfg_blocking, cfg_gen_blocking, cfg_non_blocking, Lang, Translation, TranslationResult,
};
use deepl::deepl_translate;
use google::google_translate;
use once_cell::sync::Lazy;

cfg_non_blocking! {
    pub static CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);
}

cfg_blocking! {
    pub static CLIENT: Lazy<reqwest::blocking::Client> = Lazy::new(reqwest::blocking::Client::new);
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Translator {
    #[default]
    /// Uses googles free translation api
    GoogleFree,
    /// Uses deepl's Pro translation api, requires a pro api key
    DeeplPro(String),
    /// Uses deepl's free translation api, requires an api key
    DeeplFree(String),
}

impl Translator {
    /// returns the translator from the given string
    /// # Arguments
    /// * `s` - The string to parse - should be "google", "deepl"
    /// * `api_key` - The api key to use for the translator, if any
    /// # Returns
    /// * `Option<Translator>` - The translator from the given string
    pub fn from_str(s: &str, api_key: Option<String>) -> Option<Translator> {
        match (s, api_key) {
            ("google", None) => Some(Translator::GoogleFree),
            ("deepl", Some(api_key)) => {
                // if the api key ends with ":fx" then it is a free api key
                // this outlined in the deepl api documentation[https://developers.deepl.com/docs/getting-started/auth]
                if api_key.ends_with(":fx") {
                    Some(Translator::DeeplFree(api_key))
                } else {
                    Some(Translator::DeeplPro(api_key))
                }
            }
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
                Translator::DeeplPro(api_key) => deepl_translate(false, api_key, text, from, to).await,
                Translator::DeeplFree(api_key) => deepl_translate(true, api_key, text, from, to).await,
            }
        }
    }
}
