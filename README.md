# Lingual

Provides google translation api for Rust.

## Example

The crate uses reqwest to make http calls and by default uses `async/await` syntax.
Support for `wasm` is included and will be enabled implicitly if when the target_arch is `wasm32`.

```rs
use lingual::{translate, Langs, Translator}
let translator = Translator::default();
let translation = translator.translate("Hello World", Lang::Auto, Langs::Es).await.unwrap();
assert_eq!("Hola Mundo", translation.text());
```

If you prefer to use regular `sync` version, simply include `blocking` feature in your `Cargo.toml` file.
It will be the same code as above with the exception of `await` keyword.

```toml
lingual = {version = "...", features = ["blocking"]}
```

## Features

- `blocking` - uses blocking/sync api for fetching the translations.

## Roadmap

- [x] Support for async/await syntax
- [x] Support for blocking api
- [x] Support for wasm
- [] Support for DeepL api (Will be worked on later)
  - [] blocking
  - [] async/await
