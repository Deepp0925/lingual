use crate::{errors::ErrorsResult, token, Errors, Lang};
use reqwest::Url;

/// FIXME: Why does https return 'invalid url, scheme is not http'?
pub(crate) const BASE_URL: &str = "http://translate.googleapis.com";
pub(crate) const SINGLE_TRANSLATE_URL: &str = "/translate_a/single";

pub fn generate_url<S: AsRef<str>>(text: S, src: Lang, target: Lang) -> ErrorsResult<Url> {
    let token = token::generate_token(&text)?;
    Url::parse_with_params(
        format!("{}{}", BASE_URL, SINGLE_TRANSLATE_URL).as_str(),
        &[
            ("client", "t"),
            ("sl", src.as_ref()),
            ("tl", target.as_ref()),
            ("hl", target.as_ref()),
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
    .map_err(|_| Errors::UrlParseErr)
}
