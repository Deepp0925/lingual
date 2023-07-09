use lingual::*;

#[tokio::test]
async fn test_translate() {
    let translator = Translator::new();
    let translation = translator
        .translate("Hello World", None, Some(Langs::Es))
        .await
        .unwrap();
    assert_eq!("Hola Mundo", translation.text());
}

#[test]
#[cfg(feature = "blocking")]
fn test_translate_blocking() {
    let translator = Translator::new();
    let translation = translator
        .translate("Hello World", None, Some(Langs::Es))
        .unwrap();
    assert_eq!("Hola Mundo", translation.text());
}
