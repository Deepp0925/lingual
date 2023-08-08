use lingual::*;

#[cfg(feature = "non-blocking")]
#[tokio::test]
async fn test_translate() {
    let translation = non_blocking::translate("Hello World", None, Some(Lang::Es))
        .await
        .unwrap();
    assert_eq!("Hola Mundo", translation.text());
}

#[cfg(feature = "blocking")]
#[test]
fn test_translate_blocking() {
    let translation = blocking::translate("Hello World", None, Some(Lang::Es)).unwrap();
    assert_eq!("Hola Mundo", translation.text());
}
