mod errors;
mod langs;
mod translation;
mod translator;
pub use errors::{TranslationError, TranslationResult};
pub use langs::*;
pub use translation::Translation;
pub use translator::*;

#[macro_use]
mod cfg;
