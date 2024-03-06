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
    assert_eq!(Lang::En.as_ref(), "en");
    assert_eq!(Lang::Es.as_ref(), "es");
    assert_eq!(Lang::ZhCn.as_ref(), "zh-cn");
    assert_eq!(Lang::ZhTw.as_ref(), "zh-tw");

    assert_eq!(Lang::from_str("en"), Ok(Lang::En));
    assert_eq!(Lang::from_str("es"), Ok(Lang::Es));
    assert_eq!(Lang::from_str("zh-cn"), Ok(Lang::ZhCn));
    assert_eq!(Lang::from_str("zh-tw"), Ok(Lang::ZhTw));
    assert_eq!(Lang::from_str("mni-mtei"), Ok(Lang::MniMtei));
}

const HELLO_WORLD_STR: &str = "Hello World";
const EXPECTED_HELLO_WORLD_STR: &str = "Hola Mundo";

const SPECIAL_CHAR_STR: &str =
    "Acá intentaré responder si es tarde para aprender a programar y que profesión debes estudiar para ser un ingeniero world class.";
const EXPECTED_SPECIAL_CHAR_STR: &str =
    "Here I will try to answer if it is too late to learn to program and what profession you should study to be a world class engineer.";

#[cfg_attr(not(feature = "blocking"), tokio::test)]
#[cfg_attr(feature = "blocking", remove_async_await::remove_async_await, test)]
async fn test_translate() {
    let translator = Translator::default();
    let translation = translator
        .translate(HELLO_WORLD_STR, &Lang::Auto, &Lang::Es)
        .await
        .unwrap();
    assert_eq!(EXPECTED_HELLO_WORLD_STR, translation.text);

    let translation = translator
        .translate(SPECIAL_CHAR_STR, &Lang::Es, &Lang::En)
        .await
        .unwrap();

    assert_eq!(
        EXPECTED_SPECIAL_CHAR_STR.to_lowercase(),
        translation.text.to_lowercase()
    );

    let egs = [
        "Kitty set - velvet#0888  ",
        "マテリアルカラーの変更はInspectorのMaterialsの所へ好きなカラーのマテリアルをドラッグ&ドロップして",
        "薄荷 VRChat向けアバター #Hakka3D",
        "💗概要",
        "本作品はモデリング&他",
        "FBX/textures package + Unity package",
        "🌱こちらのワールドで試着できます",
        "水瀬 VRChat向けアバター #Minase3D",
        "【NO.37 moon&sun】ver1.00",
        "Shoes- Bobster#8539 ",
        "・ウィンドウ下部のBuild & Publish for Windowsボタンを押す"
    ];

    let mut err_count = 0;
    for eg in egs.iter() {
        let translation = translator.translate(eg.trim(), &Lang::Ja, &Lang::En).await;
        match translation {
            Ok(translation) => {
                println!("{} -> {}", eg, translation.text);
            }
            Err(err) => {
                println!("{} -> {:#?}", eg, err);
                err_count += 1;
            }
        }
    }

    assert_eq!(err_count, 2);
}

#[cfg_attr(not(feature = "blocking"), tokio::test)]
#[cfg_attr(feature = "blocking", remove_async_await::remove_async_await, test)]
async fn trial() {
    let translator = Translator::default();
    let translation = translator
        .translate("Hola Mundo", &Lang::Auto, &Lang::EnUS)
        .await
        .unwrap();
    println!("{:?}", translation);
    assert_eq!("Hello World", translation.text);
}
