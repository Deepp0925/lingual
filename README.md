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
