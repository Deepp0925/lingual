use super::url;
use crate::errors::{TranslationError, TranslationResult};
use crate::translation::Translation;
use crate::{cfg_gen_blocking, langs::*, CLIENT};

use reqwest::Error;
use serde_json::Value;

cfg_gen_blocking! {
        /// Translate a text from a source language to a target language.
        /// # Arguments
        /// * `text` - The text to translate.
        /// * `src` - The source language. Optional Value, Defaults to 'Auto'.
        /// * `target` - The target language. Optional Value, Defaults to 'En'.
        /// # Returns
        /// * `Translation` - The translated text.
        pub async fn google_translate<'a>(
            text: &'a str,
            src: &'a Lang,
            target: &'a Lang,
        ) -> TranslationResult<Translation<'a>> {
            let url = url::generate_url(text, src, target)?;
            let req = CLIENT
                .get(url)
                .send()
                .await
                .map_err(|e| TranslationError::HttpErr(e.to_string()))?;
            let translated = req.json::<Value>().await;

            trans_from_value(translated, text, src, target)
        }
}

fn trans_from_value<'a>(
    value: Result<Value, Error>,
    text: &'a str,
    src: &'a Lang,
    target: &'a Lang,
) -> TranslationResult<Translation<'a>> {
    let mut translated = value.map_err(|err| TranslationError::JsonParseErr(err.to_string()))?;

    for _ in 0..3 {
        let v = translated
            .as_array_mut()
            .ok_or_else(|| {
                TranslationError::JsonParseErr(
                    "Expected a JSON array, but got something else.".to_string(),
                )
            })?
            .drain(0..1)
            .next()
            .ok_or_else(|| {
                TranslationError::JsonParseErr(
                    "Expected a JSON array, but got something else.".to_string(),
                )
            })?;
        translated = v;
    }

    let translated = match translated {
        Value::String(s) => s,
        _ => {
            return Err(TranslationError::JsonParseErr(
                "Expected a JSON string, but got something else.".to_string(),
            ))
        }
    };

    Ok(Translation {
        text: translated,
        src: text,
        src_lang: src,
        target_lang: target,
    })
}
