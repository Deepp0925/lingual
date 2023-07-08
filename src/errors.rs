#[derive(Debug)]
pub enum TranslationErrors {
    /// An error occurred while generating the token which is normally
    /// caused by an parsing error of an integer.
    ParseIntErr,
    HttpErr(reqwest::Error),
    UrlParseErr,
}
