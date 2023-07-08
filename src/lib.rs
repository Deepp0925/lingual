pub use errors::TranslationErrors;
pub use language::Langs;
pub use translation::Translation;
mod errors;
mod language;
mod token;
mod translation;
use reqwest::{blocking::get, Url};
use serde_json::Value;

const BASE_URL: &str = "https://translate.googleapis.com";
const SINGLE_TRANSLATE_URL: &str = "/translate_a/single";

fn generate_url<S: AsRef<str>>(
    text: S,
    src: Langs,
    target: Langs,
) -> Result<Url, TranslationErrors> {
    let token = token::generate_token(text.as_ref())?;

    Url::parse_with_params(
        format!("{}{}", BASE_URL, SINGLE_TRANSLATE_URL).as_str(),
        &[
            ("client", "t"),
            ("sl", src.to_string().as_str()),
            ("tl", target.to_string().as_str()),
            ("hl", target.to_string().as_str()),
            ("dt", "t"),
            ("ie", "UTF-8"),
            ("oe", "UTF-8"),
            ("otf", "1"),
            ("ssel", "0"),
            ("tsel", "0"),
            ("kc", "7"),
            ("tk", token.as_str()),
            ("q", text.as_ref()),
        ],
    )
    .map_err(|_| TranslationErrors::UrlParseErr)
}

/// Translate a text from a source language to a target language.
/// # Arguments
/// * `text` - The text to translate.
/// * `src` - The source language. Optional Value, Defaults to 'Auto'.
/// * `target` - The target language. Optional Value, Defaults to 'En'.
/// # Returns
/// * `Translation` - The translated text.
pub fn translate<S: AsRef<str>>(
    text: S,
    src: Option<Langs>,
    target: Option<Langs>,
) -> Result<Translation, TranslationErrors> {
    let src = src.unwrap_or(Langs::Auto);
    let target = target.unwrap_or(Langs::En);
    let url = generate_url(text.as_ref(), src, target)?;

    let req = get(url).map_err(TranslationErrors::HttpErr)?;

    let translated = &req.json::<Value>().map_err(TranslationErrors::HttpErr)?[0][0][0];

    Ok(Translation {
        text: translated.as_str().unwrap().to_string(),
        src: text.as_ref().to_string(),
        src_lang: src,
        target_lang: target,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        let translation = translate("Hello World", None, Some(Langs::Es)).unwrap();
        println!("{:?}", translation);
    }
}
