mod errors;
mod langs;
mod token;
mod translation;
mod url;
pub use errors::{Errors, ErrorsResult};
pub use langs::*;
use once_cell::sync::Lazy;
use reqwest::Error;
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
pub async fn translate<L: Into<Lang> + Copy>(
    text: &str,
    src: L,
    target: L,
) -> ErrorsResult<Translation> {
    let url = url::generate_url(text, src.into(), target.into())?;
    let req = CLIENT
        .get(url)
        .send()
        .await
        .map_err(|e| Errors::HttpErr(e.to_string()))?;
    let translated = req.json::<Value>().await;

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
pub fn translate<L: Into<Lang> + Copy>(text: &str, src: L, target: L) -> ErrorsResult<Translation> {
    let url = url::generate_url(text.as_ref(), src.into(), target.into())?;

    let req = CLIENT
        .get(url)
        .send()
        .map_err(|e| Errors::HttpErr(e.to_string()))?;
    let translated = req.json::<Value>();

    trans_from_value(translated, text, src, target)
}

fn trans_from_value<L: Into<Lang>>(
    value: Result<Value, Error>,
    text: &str,
    src: L,
    target: L,
) -> ErrorsResult<Translation> {
    let translated = value.map_err(|err| Errors::JsonParseErr(err.to_string()))?;

    Ok(Translation {
        text: translated,
        src: text,
        src_lang: src.into(),
        target_lang: target.into(),
    })
}
