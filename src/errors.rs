pub type TranslationResult<T> = Result<T, Errors>;

#[derive(Debug)]
pub enum Errors {
    /// An error occurred while generating the token which is normally
    /// caused by an parsing error of an integer.
    ParseIntErr,
    /// An error occurred while sending the request to the server.
    HttpErr(String),
    /// An error occurred while parsing the url.
    UrlParseErr,
    /// An error occurred while parsing the json.
    /// This error is normally caused by an invalid json.
    JsonParseErr(String),
}
