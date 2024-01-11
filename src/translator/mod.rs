mod deepl;
mod google;

use google::*;

use crate::{Lang, Translation, TranslationResult};

#[derive(Debug, Clone, Copy, Default)]
pub enum Translator {
    #[default]
    /// Uses googles free translation api
    GoogleFree,
}

impl Translator {
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
