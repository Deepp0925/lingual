use std::ops::Add;

use crate::language::Langs;

#[derive(Debug, Clone, PartialEq)]
pub struct Translation {
    pub(crate) text: String,
    pub(crate) src: String,
    pub(crate) src_lang: Langs,
    pub(crate) target_lang: Langs,
}

impl Translation {
    /// Get the translated text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the source text.
    pub fn src(&self) -> &str {
        &self.src
    }

    /// Get the source language.
    pub fn src_lang(&self) -> Langs {
        self.src_lang
    }

    /// Get the target language.
    pub fn target_lang(&self) -> Langs {
        self.target_lang
    }
}

impl AsRef<str> for Translation {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

impl Add for Translation {
    type Output = String;

    fn add(self, rhs: Self) -> Self::Output {
        self.text + &rhs.text
    }
}
