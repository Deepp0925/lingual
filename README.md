# Lingual

Provides google translation api for Rust.

## Example

The crate uses reqwest to make http calls and by default uses `async/await` syntax.

```rs
use lingual::{Translator, Langs}
let translator = Translator::new();
let translation = translator
    .translate("Hello World", None, Some(Langs::Es))
    .await
    .unwrap();
assert_eq!("Hola Mundo", translation.text());
```

If you prefer to use regular `sync` version, simply include `blocking` feature in your `Cargo.toml` file.

```toml
lingual = {version = "1.0.0", features = ["blocking"]}
```

```rs
use lingual::{Translator, Langs}
let translator = Translator::new();
let translation = translator
    .translate("Hello World", None, Some(Langs::Es))
    .unwrap();
assert_eq!("Hola Mundo", translation.text());
```

## Features

- `wasm` - allows the crate to be used in wasm environment - only supports async/await syntax.
- `sea-orm` - support for conversion between db types and rust types from sea-orm.
- `blocking` - uses blocking api (non async/await syntax) for fetching the translations.
- `non-blocking` - default - use async/await syntax
- `accurate` - limits the number langs supported to the ones with higher level accuracy

## Roadmap

- [x] Support for async/await syntax
- [x] Support for blocking api
- [x] Support for wasm
- [] Support for DeepL api (Will be worked on later)
  - [] blocking
  - [] async/await
