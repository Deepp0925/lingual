use crate::{langs::OptionLangExt, url::generate_url, Errors, Lang, Translation};
use gloo_net::http::Request;

/// Translate a text from a source language to a target language.
/// sync version available with feature "blocking"
/// # Arguments
/// * `text` - The text to translate.
/// * `src` - The source language. Optional Value, Defaults to 'Auto'.
/// * `target` - The target language. Optional Value, Defaults to 'En'.
/// # Returns
/// * `Translation` - The translated text.
pub async fn translate<S: AsRef<str>>(
    text: S,
    src: Option<Lang>,
    target: Option<Lang>,
) -> Result<Translation, Errors> {
    let src = src.unwrap_or_default_src();
    let target = target.unwrap_or_default_trgt();
    let url = generate_url(text.as_ref(), src, target)?;

    let req = Request::get(url.as_str())
        .send()
        .await
        .map_err(|e| Errors::HttpErr(e.to_string()))?;
    let translated = &req
        .json::<serde_json::Value>()
        .await
        .map_err(|err| Errors::JsonParseErr(err.to_string()))?[0][0][0];
    let translated = translated
        .as_str()
        .ok_or(Errors::JsonParseErr("Error Parsing String".to_owned()))?
        .to_string();

    Ok(Translation {
        text: translated,
        src: text.as_ref().to_string(),
        src_lang: src,
        target_lang: target,
    })
}
