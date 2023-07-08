use std::ops::Add;

use serde_json::Value;

use crate::language::Langs;

#[derive(Debug, Clone, PartialEq)]
pub struct Translation {
    pub(crate) text: String,
    pub(crate) src: String,
    pub(crate) src_lang: Langs,
    pub(crate) target_lang: Langs,
}

impl Translation {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn src(&self) -> &str {
        &self.src
    }

    pub fn src_lang(&self) -> Langs {
        self.src_lang
    }

    pub fn target_lang(&self) -> Langs {
        self.target_lang
    }
}

impl ToString for Translation {
    fn to_string(&self) -> String {
        self.text.clone()
    }
}

impl Add for Translation {
    type Output = String;

    fn add(self, rhs: Self) -> Self::Output {
        self.text + &rhs.text
    }
}
