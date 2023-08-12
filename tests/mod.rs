use lingual::*;

const HELLO_WORLD_STR: &str = "Hello World";
const EXPECTED_HELLO_WORLD_STR: &str = "Hola Mundo";

const SPECIAL_CHAR_STR: &str =
    "Acá intentaré responder si es tarde para aprender a programar y que profesión debes estudiar para ser un ingeniero world class.";
const EXPECTED_SPECIAL_CHAR_STR: &str =
    "Here I will try to answer if it is too late to learn to program and what profession you should study to be a world class engineer.";

#[cfg(feature = "non-blocking")]
#[tokio::test]
async fn test_translate() {
    let translation = non_blocking::translate(HELLO_WORLD_STR, None, Some(Lang::Es))
        .await
        .unwrap();
    assert_eq!(EXPECTED_HELLO_WORLD_STR, translation.text());

    let translation = non_blocking::translate(SPECIAL_CHAR_STR, Some(Lang::Es), None)
        .await
        .unwrap();

    assert_eq!(
        EXPECTED_SPECIAL_CHAR_STR.to_lowercase(),
        translation.text().to_lowercase()
    );
}

#[cfg(feature = "blocking")]
#[test]
fn test_translate_blocking() {
    println!("testing blockng");
    let translation = blocking::translate(HELLO_WORLD_STR, None, Some(Lang::Es)).unwrap();
    assert_eq!(EXPECTED_HELLO_WORLD_STR, translation.text());

    let translation = blocking::translate(SPECIAL_CHAR_STR, Some(Lang::Es), None).unwrap();

    assert_eq!(
        EXPECTED_SPECIAL_CHAR_STR.to_lowercase(),
        translation.text().to_lowercase()
    );
}
