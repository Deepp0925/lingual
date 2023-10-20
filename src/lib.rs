mod errors;
mod langs;
mod token;
mod translation;
mod url;
use errors::{Errors, ErrorsResult};
pub use langs::*;
use once_cell::sync::Lazy;
use serde_json::Value;
pub use translation::Translation;

#[cfg(any(target_arch = "wasm32", not(feature = "blocking")))]
static CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);
/// Translate a text from a source language to a target language.
/// # Arguments
/// * `text` - The text to translate.
/// * `src` - The source language. Optional Value, Defaults to 'Auto'.
/// * `target` - The target language. Optional Value, Defaults to 'En'.
/// # Returns
/// * `Translation` - The translated text.
#[cfg(any(target_arch = "wasm32", not(feature = "blocking")))]
pub async fn translate<S: AsRef<str>>(
    text: S,
    src: Lang,
    target: Lang,
) -> ErrorsResult<Translation> {
    let url = url::generate_url(text.as_ref(), src, target)?;
    let req = CLIENT
        .get(url)
        .send()
        .await
        .map_err(|e| Errors::HttpErr(e.to_string()))?;
    let translated = &req
        .json::<Value>()
        .await
        .map_err(|err| Errors::JsonParseErr(err.to_string()))?[0][0][0];

    trans_from_value(translated, text, src, target)
}

#[cfg(all(not(target_arch = "wasm32"), feature = "blocking"))]
static CLIENT: Lazy<reqwest::blocking::Client> = Lazy::new(reqwest::blocking::Client::new);
/// Translate a text from a source language to a target language.
/// # Arguments
/// * `text` - The text to translate.
/// * `src` - The source language. Optional Value, Defaults to 'Auto'.
/// * `target` - The target language. Optional Value, Defaults to 'En'.
/// # Returns
/// * `Translation` - The translated text.
#[cfg(all(not(target_arch = "wasm32"), feature = "blocking"))]
pub fn translate<S: AsRef<str>>(text: S, src: Lang, target: Lang) -> ErrorsResult<Translation> {
    let url = url::generate_url(text.as_ref(), src, target)?;

    let req = CLIENT
        .get(url)
        .send()
        .map_err(|e| Errors::HttpErr(e.to_string()))?;
    let translated = &req
        .json::<Value>()
        .map_err(|err| Errors::JsonParseErr(err.to_string()))?[0][0][0];

    trans_from_value(translated, text, src, target)
}

fn trans_from_value<S: AsRef<str>>(
    value: &Value,
    text: S,
    src: Lang,
    target: Lang,
) -> ErrorsResult<Translation> {
    let translated = value
        .as_str()
        .ok_or(Errors::JsonParseErr("Expected String".to_owned()))?
        .to_string();

    Ok(Translation {
        text: translated,
        src: text.as_ref().to_string(),
        src_lang: src,
        target_lang: target,
    })
}
