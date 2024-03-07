mod backoff;

use crate::{Lang, Translation, TranslationResult};

const FREE_URL: &str = "https://api-free.deepl.com/v2/translate";
const PRO_URL: &str = "https://api.deepl.com/v2/translate";

/// Translate a text from a source language to a target language.
/// # Arguments
/// * `is_free` - Whether to use the free or pro version of the API.
/// * `api_key` - The API key to use.
/// * `text` - The text to translate.
/// * `src` - The source language. Optional Value, Defaults to 'Auto'.
/// * `target` - The target language. Optional Value, Defaults to 'En'.
/// # Returns
/// * `Translation` - The translated text.
pub async fn deepl_translate<'a>(
    is_free: bool,
    api_key: &str,
    text: &'a str,
    src: &'a Lang,
    target: &'a Lang,
) -> TranslationResult<Translation<'a>> {
    todo!()
    // let url = url::generate_url(text, src, target)?;
    // let req = CLIENT
    //     .get(url)
    //     .send()
    //     .await
    //     .map_err(|e| TranslationError::HttpErr(e.to_string()))?;
    // let translated = req.json::<Value>().await;

    // trans_from_value(translated, text, src, target)
}
