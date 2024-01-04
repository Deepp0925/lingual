use serde_json::Value;

use crate::{langs::Lang, Errors, ErrorsResult};

/// Encapsulates a translated text and its source and target languages.
#[derive(Debug, Clone, PartialEq)]
pub struct Translation<'a> {
    pub(crate) text: Value,
    pub src: &'a str,
    pub src_lang: Lang,
    pub target_lang: Lang,
}

impl Translation<'_> {
    /// Returns the translated text.
    pub fn text(&self) -> ErrorsResult<&str> {
        self.text[0][0][0]
            .as_str()
            .ok_or(Errors::JsonParseErr("Expected String".to_owned()))
    }
}
