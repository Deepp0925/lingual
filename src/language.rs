// pub type LanguageMap = phf::Map<&'static str, Lang>;
// /// Returns the language code corresponding to the given string.
// /// how many
// pub const LANGUAGES: LanguageMap = phf::phf_map! {
//     "auto" => Lang::Auto,
//     "af" => Lang::Af,
//     "sq" => Lang::Sq,
//     "am" => Lang::Am,
//     "ar" => Lang::Ar,
//     "hy" => Lang::Hy,
//     "as" => Lang::As,
//     "ay" => Lang::Ay,
//     "az" => Lang::Az,
//     "bm" => Lang::Bm,
//     "eu" => Lang::Eu,
//     "be" => Lang::Be,
//     "bn" => Lang::Bn,
//     "bho" => Lang::Bho,
//     "bs" => Lang::Bs,
//     "bg" => Lang::Bg,
//     "ca" => Lang::Ca,
//     "ceb" => Lang::Ceb,
//     "zh-cn" => Lang::ZhCn,
//     "zh-tw" => Lang::ZhTw,
//     "mni-mtei" => Lang::MniMtei,
//     "co" => Lang::Co,
//     "hr" => Lang::Hr,
//     "cs" => Lang::Cs,
//     "da" => Lang::Da,
//     "dv" => Lang::Dv,
//     "doi" => Lang::Doi,
//     "nl" => Lang::Nl,
//     "en" => Lang::En,
//     "eo" => Lang::Eo,
//     "et" => Lang::Et,
//     "ee" => Lang::Ee,
//     "tl" => Lang::Tl,
//     "fi" => Lang::Fi,
//     "fr" => Lang::Fr,
//     "fy" => Lang::Fy,
//     "gl" => Lang::Gl,
//     "ka" => Lang::Ka,
//     "de" => Lang::De,
//     "el" => Lang::El,
//     "gn" => Lang::Gn,
//     "gu" => Lang::Gu,
//     "ht" => Lang::Ht,
//     "ha" => Lang::Ha,
//     "haw" => Lang::Haw,
//     "iw" => Lang::Iw,
//     "hi" => Lang::Hi,
//     "hmn" => Lang::Hmn,
//     "hu" => Lang::Hu,
//     "is" => Lang::Is,
//     "ig" => Lang::Ig,
//     "id" => Lang::Id,
//     "ga" => Lang::Ga,
//     "it" => Lang::It,
//     "ja" => Lang::Ja,
//     "kn" => Lang::Kn,
//     "kk" => Lang::Kk,
//     "km" => Lang::Km,
//     "rw" => Lang::Rw,
//     "ky" => Lang::Ky,
//     "ko" => Lang::Ko,
//     "ku" => Lang::Ku,
//     "lo" => Lang::Lo,
//     "la" => Lang::La,
//     "lv" => Lang::Lv,
//     "lt" => Lang::Lt,
//     "lb" => Lang::Lb,
//     "mk" => Lang::Mk,
//     "mg" => Lang::Mg,
//     "ms" => Lang::Ms,
//     "ml" => Lang::Ml,
//     "mt" => Lang::Mt,
//     "mi" => Lang::Mi,
//     "mr" => Lang::Mr,
//     "mn" => Lang::Mn,
//     "my" => Lang::My,
//     "ne" => Lang::Ne,
//     "no" => Lang::No,
//     "ny" => Lang::Ny,
//     "or" => Lang::Or,
//     "ps" => Lang::Ps,
//     "fa" => Lang::Fa,
//     "pl" => Lang::Pl,
//     "pt" => Lang::Pt,
//     "pa" => Lang::Pa,
//     "qu" => Lang::Qu,
//     "ro" => Lang::Ro,
//     "ru" => Lang::Ru,
//     "sm" => Lang::Sm,
//     "gd" => Lang::Gd,
//     "sr" => Lang::Sr,
//     "sn" => Lang::Sn,
//     "sd" => Lang::Sd,
//     "si" => Lang::Si,
//     "sk" => Lang::Sk,
//     "sl" => Lang::Sl,
//     "so" => Lang::So,
//     "st" => Lang::St,
//     "es" => Lang::Es,
//     "su" => Lang::Su,
//     "sw" => Lang::Sw,
//     "sv" => Lang::Sv,
//     "tg" => Lang::Tg,
//     "ta" => Lang::Ta,
//     "tt" => Lang::Tt,
//     "te" => Lang::Te,
//     "th" => Lang::Th,
//     "ti" => Lang::Ti,
//     "tr" => Lang::Tr,
//     "tk" => Lang::Tk,
//     "ug" => Lang::Ug,
//     "uk" => Lang::Uk,
//     "ur" => Lang::Ur,
//     "uz" => Lang::Uz,
//     "vi" => Lang::Vi,
//     "cy" => Lang::Cy,
//     "xh" => Lang::Xh,
//     "yi" => Lang::Yi,
//     "yo" => Lang::Yo,
//     "zu" => Lang::Zu,
// };

// impl std::str::FromStr for Lang {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(*LANGUAGES.get(s).unwrap_or(&Lang::Auto))
//     }
// }

// impl AsRef<Lang> for &str {
//     fn as_ref(&self) -> &Lang {
//         LANGUAGES.get(self).unwrap_or(&Lang::Auto)
//     }
// }

use strum::{EnumCount, EnumIter};

/// The language codes supported by the API.
/// It is also possible to use strings like "en" or "fr" instead of the enum variants but it is not recommended
/// because it is not checked at compile time, therefore it is eliminated by default features.
/// To enable this feature, add `strings` to the features list of the crate.
/// get how many variants are there in the enum at compile time.
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash, EnumCount, EnumIter)]
pub enum Lang {
    Auto,
    Af,
    Sq,
    Am,
    Ar,
    Hy,
    As,
    Ay,
    Az,
    Bm,
    Eu,
    Be,
    Bn,
    Bho,
    Bs,
    Bg,
    Ca,
    Ceb,
    ZhCn,
    ZhTw,
    Co,
    Hr,
    Cs,
    Da,
    Dv,
    Doi,
    Nl,
    En,
    Eo,
    Et,
    Ee,
    Tl,
    Fi,
    Fr,
    Fy,
    Gl,
    Ka,
    De,
    El,
    Gn,
    Gu,
    Ht,
    Ha,
    Haw,
    Iw,
    Hi,
    Hmn,
    Hu,
    Is,
    Ig,
    Ilo,
    Id,
    Ga,
    It,
    Ja,
    Jw,
    Kn,
    Kk,
    Km,
    Rw,
    Gom,
    Ko,
    Kri,
    Ku,
    Ckb,
    Ky,
    Lo,
    La,
    Lv,
    Lt,
    Lg,
    Lb,
    Mk,
    Mg,
    Mai,
    Ms,
    Ml,
    Mt,
    Mi,
    Mr,
    MniMtei,
    Lus,
    Mn,
    My,
    Ne,
    No,
    Ny,
    Or,
    Om,
    Ps,
    Fa,
    Pl,
    Pt,
    Pa,
    Qu,
    Ro,
    Ru,
    Sm,
    Sa,
    Gd,
    Nso,
    Sr,
    St,
    Sn,
    Sd,
    Si,
    Sk,
    Sl,
    So,
    Es,
    Su,
    Sw,
    Sv,
    // Tl,
    Tg,
    Ta,
    Tt,
    Te,
    Th,
    Ti,
    Ts,
    Tr,
    Tk,
    Ak,
    Uk,
    Ur,
    Ug,
    Uz,
    Vi,
    Cy,
    Xh,
    Yi,
    Yo,
    Zu,
}

pub trait OptionLangExt {
    /// returns the language if it is set, otherwise returns default src: `Lang::Auto`
    fn unwrap_or_default_src(&self) -> Lang;
    /// returns the language if it is set, otherwise returns default target: `Lang::En`
    fn unwrap_or_default_trgt(&self) -> Lang;
}

impl Lang {
    pub const fn len() -> usize {
        Lang::COUNT
    }
}

impl OptionLangExt for Option<Lang> {
    /// returns the language if it is set, otherwise returns default src: `Lang::Auto`
    fn unwrap_or_default_src(&self) -> Lang {
        match self {
            None => Lang::Auto,
            Some(l) => *l,
        }
    }
    /// returns the language if it is set, otherwise returns default target: `Lang::En`
    fn unwrap_or_default_trgt(&self) -> Lang {
        match self {
            None => Lang::En,
            Some(l) => *l,
        }
    }
}

impl ToString for Lang {
    fn to_string(&self) -> String {
        if self == &Lang::MniMtei {
            return "mni-mtei".to_string();
        } else if self == &Lang::ZhCn {
            return "zh-cn".to_string();
        } else if self == &Lang::ZhTw {
            return "zh-tw".to_string();
        }
        format!("{:?}", self).to_lowercase()
    }
}
