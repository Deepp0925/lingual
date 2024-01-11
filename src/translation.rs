use crate::langs::Lang;

/// Encapsulates a translated text and its source and target languages.
#[derive(Debug, Clone, PartialEq)]
pub struct Translation<'a> {
    pub text: String,
    pub src: &'a str,
    pub src_lang: &'a Lang,
    pub target_lang: &'a Lang,
}
