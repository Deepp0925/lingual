pub type TranslationResult<T> = Result<T, TranslationError>;

#[derive(Debug)]
pub enum TranslationError {
    /// An error occurred while generating the token which is normally
    /// caused by an parsing error of an integer.
    ParseIntErr(String),
    /// An error occurred while sending the request to the server.
    HttpErr(String),
    /// An error occurred while parsing the url.
    UrlParseErr(String),
    /// An error occurred while parsing the json.
    /// This error is normally caused by an invalid json.
    JsonParseErr(String),
}
