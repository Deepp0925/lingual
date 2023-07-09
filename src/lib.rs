pub use errors::Errors;
pub use language::Langs;
pub use translation::Translation;
mod errors;
mod language;
mod token;
mod translation;
use reqwest::Url;

const BASE_URL: &str = "http://translate.googleapis.com";
const SINGLE_TRANSLATE_URL: &str = "/translate_a/single";

fn generate_url<S: AsRef<str>>(text: S, src: Langs, target: Langs) -> Result<Url, Errors> {
    let token = token::generate_token(text.as_ref())?;

    Url::parse_with_params(
        format!("{}{}", BASE_URL, SINGLE_TRANSLATE_URL).as_str(),
        &[
            ("client", "t"),
            ("sl", src.to_string().as_str()),
            ("tl", target.to_string().as_str()),
            ("hl", target.to_string().as_str()),
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

enum Client {
    #[cfg(feature = "blocking")]
    Blocking(reqwest::blocking::Client),
    Async(reqwest::Client),
}

pub struct Translator {
    client: Client,
}

impl Default for Translator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "blocking")]
impl Translator {
    /// Create a new Translator instance
    pub fn new() -> Self {
        Self {
            client: Client::Blocking(reqwest::blocking::Client::new()),
        }
    }

    /// Translate a text from a source language to a target language.
    /// # Arguments
    /// * `text` - The text to translate.
    /// * `src` - The source language. Optional Value, Defaults to 'Auto'.
    /// * `target` - The target language. Optional Value, Defaults to 'En'.
    /// # Returns
    /// * `Translation` - The translated text.
    pub fn translate<S: AsRef<str>>(
        &self,
        text: S,
        src: Option<Langs>,
        target: Option<Langs>,
    ) -> Result<Translation, Errors> {
        let src = src.unwrap_or(Langs::Auto);
        let target = target.unwrap_or(Langs::En);
        let url = generate_url(text.as_ref(), src, target)?;

        if let Client::Blocking(client) = &self.client {
            let req = client.get(url).send().map_err(Errors::HttpErr)?;
            let translated = &req.json::<serde_json::Value>().map_err(Errors::HttpErr)?[0][0][0];
            let translated = translated.as_str().ok_or(Errors::JsonParseErr)?.to_string();

            return Ok(Translation {
                text: translated,
                src: text.as_ref().to_string(),
                src_lang: src,
                target_lang: target,
            });
        }

        unreachable!()
    }
}

#[cfg(all(feature = "default", not(feature = "blocking")))]
impl Translator {
    /// Create a new Translator instance
    pub fn new() -> Self {
        Self {
            client: Client::Async(reqwest::Client::new()),
        }
    }

    /// Translate a text from a source language to a target language.
    /// # Arguments
    /// * `text` - The text to translate.
    /// * `src` - The source language. Optional Value, Defaults to 'Auto'.
    /// * `target` - The target language. Optional Value, Defaults to 'En'.
    /// # Returns
    /// * `Translation` - The translated text.
    pub async fn translate<S: AsRef<str>>(
        &self,
        text: S,
        src: Option<Langs>,
        target: Option<Langs>,
    ) -> Result<Translation, Errors> {
        let src = src.unwrap_or(Langs::Auto);
        let target = target.unwrap_or(Langs::En);
        let url = generate_url(text.as_ref(), src, target)?;

        let Client::Async(client) = &self.client;

        let req = client.get(url).send().await.map_err(Errors::HttpErr)?;
        let translated = &req
            .json::<serde_json::Value>()
            .await
            .map_err(Errors::HttpErr)?[0][0][0];
        let translated = translated.as_str().ok_or(Errors::JsonParseErr)?.to_string();

        Ok(Translation {
            text: translated,
            src: text.as_ref().to_string(),
            src_lang: src,
            target_lang: target,
        })
    }
}
