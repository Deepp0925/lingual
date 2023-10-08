use crate::langs::Lang;

/// Encapsulates a translated text and its source and target languages.
#[derive(Debug, Clone, PartialEq)]
pub struct Translation {
    pub text: String,
    pub src: String,
    pub src_lang: Lang,
    pub target_lang: Lang,
}

impl AsRef<str> for Translation {
    fn as_ref(&self) -> &str {
        &self.text
    }
}
