use lingual::*;
use std::str::FromStr;

#[test]
fn is_accurate_lang_test() {
    assert!(Lang::Auto.is_accurate_lang());
    assert!(Lang::En.is_accurate_lang());
    assert!(Lang::Fr.is_accurate_lang());
    assert!(Lang::De.is_accurate_lang());
    assert!(Lang::Es.is_accurate_lang());
    assert!(Lang::It.is_accurate_lang());
    assert!(Lang::Pt.is_accurate_lang());
    assert!(Lang::Ru.is_accurate_lang());
    assert!(Lang::ZhCn.is_accurate_lang());
    assert!(Lang::Ja.is_accurate_lang());
    assert!(Lang::Ko.is_accurate_lang());
    assert!(!Lang::ZhTw.is_accurate_lang());
}

#[test]
fn test_lang_enums() {
    assert_eq!(Lang::En.to_string(), "en");
    assert_eq!(Lang::Es.to_string(), "es");
    assert_eq!(Lang::ZhCn.to_string(), "zh-cn");
    assert_eq!(Lang::ZhTw.to_string(), "zh-tw");

    assert_eq!(Lang::from_str("en"), Ok(Lang::En));
    assert_eq!(Lang::from_str("es"), Ok(Lang::Es));
    assert_eq!(Lang::from_str("zh-cn"), Ok(Lang::ZhCn));
    assert_eq!(Lang::from_str("zh-tw"), Ok(Lang::ZhTw));
    assert_eq!(Lang::from_str("mni-mtei"), Ok(Lang::MniMtei));
}

#[cfg(feature = "sqlx")]
#[test]
fn test_welds_sqlite() {
    #[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, welds::WeldsModel)]
    pub struct Model {
        /// THe primary key
        #[welds(primary_key)]
        pub id: u32,
        /// the source language
        pub lang: Lang,
        /// this is the key used to identify the translation
        /// this has to be unique, but it is not a primary key
        /// because the primary key is the id
        pub key: String,
        /// this is the source text
        pub text: String,
    }

    Model::where_col(|model| model.lang.equal(Lang::En));
}

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
