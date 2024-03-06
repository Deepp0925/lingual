use crate::{Lang, Translation, TranslationResult};

const URL: &str = "https://api-free.deepl.com/v2/translate";

/// Translate a text from a source language to a target language.
/// # Arguments
/// * `text` - The text to translate.
/// * `src` - The source language. Optional Value, Defaults to 'Auto'.
/// * `target` - The target language. Optional Value, Defaults to 'En'.
/// # Returns
/// * `Translation` - The translated text.
pub async fn deepl_free_translate<'a>(
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
