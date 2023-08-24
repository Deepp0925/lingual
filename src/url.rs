use crate::{token, Errors, Lang};

/// FIXME: Why does https return 'invalid url, scheme is not http'?
pub(crate) const BASE_URL: &str = "http://translate.googleapis.com";
pub(crate) const SINGLE_TRANSLATE_URL: &str = "/translate_a/single";

pub fn generate_url<S: AsRef<str>>(text: S, src: Lang, target: Lang) -> Result<String, Errors> {
    let token = token::generate_token(text.as_ref())?;

    Ok(format!(
        "{}{}?client=t&sl={}&tl={}&hl={}&dt=t&ie=UTF-8&oe=UTF-8&otf=1&ssel=0&tsel=0&kc=7&tk={}&q={}",
        BASE_URL,
        SINGLE_TRANSLATE_URL,
        src,
        target,
        target,
        token,
        text.as_ref()
    ))

    // Url::parse_with_params(
    //     format!("{}{}", BASE_URL, SINGLE_TRANSLATE_URL).as_str(),
    //     &[
    //         ("client", "t"),
    //         ("sl", src.to_string().as_str()),
    //         ("tl", target.to_string().as_str()),
    //         ("hl", target.to_string().as_str()),
    //         ("dt", "t"),
    //         ("ie", "UTF-8"),
    //         ("oe", "UTF-8"),
    //         ("otf", "1"),
    //         ("ssel", "0"),
    //         ("tsel", "0"),
    //         ("kc", "7"),
    //         ("tk", token.as_str()),
    //         ("q", text.as_ref()),
    //     ],
    // )
    // .map_err(|_| Errors::UrlParseErr)
}
