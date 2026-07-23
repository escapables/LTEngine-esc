use once_cell::sync::Lazy;
use std::collections::HashMap;

const LANGS: &[(&str, &str, &str)] = &[
    ("en", "", "English"),
    ("sq", "", "Albanian"),
    ("ar", "", "Arabic"),
    ("az", "", "Azerbaijani"),
    ("eu", "", "Basque"),
    ("bn", "", "Bengali"),
    ("bg", "", "Bulgarian"),
    ("ca", "", "Catalan"),
    ("zh", "zh-Hans", "Chinese"),
    ("zt", "zh-Hant", "Chinese (traditional)"),
    ("cs", "", "Czech"),
    ("da", "", "Danish"),
    ("nl", "", "Dutch"),
    ("eo", "", "Esperanto"),
    ("et", "", "Estonian"),
    ("fi", "", "Finnish"),
    ("fr", "", "French"),
    ("gl", "", "Galician"),
    ("de", "", "German"),
    ("el", "", "Greek"),
    ("he", "", "Hebrew"),
    ("hi", "", "Hindi"),
    ("hu", "", "Hungarian"),
    ("id", "", "Indonesian"),
    ("ga", "", "Irish"),
    ("it", "", "Italian"),
    ("ja", "", "Japanese"),
    ("ko", "", "Korean"),
    ("lv", "", "Latvian"),
    ("lt", "", "Lithuanian"),
    ("ms", "", "Malay"),
    ("nb", "", "Norwegian"),
    ("fa", "", "Persian"),
    ("pl", "", "Polish"),
    ("pt", "", "Portuguese"),
    ("pb", "pt-BR", "Portuguese (Brazil)"),
    ("ro", "", "Romanian"),
    ("ru", "", "Russian"),
    ("sr", "", "Serbian"),
    ("sk", "", "Slovak"),
    ("sl", "", "Slovenian"),
    ("es", "", "Spanish"),
    ("sv", "", "Swedish"),
    ("tl", "", "Tagalog"),
    ("th", "", "Thai"),
    ("tr", "", "Turkish"),
    ("uk", "", "Ukrainian"),
    ("ur", "", "Urdu"),
    ("vi", "", "Vietnamese"),
];

pub struct Language {
    pub name: &'static str,
    pub internal_code: &'static str,
}

pub static LANGUAGES: Lazy<Vec<Language>> = Lazy::new(|| {
    LANGS
        .iter()
        .map(|&(code, _, name)| Language {
            name,
            internal_code: code,
        })
        .collect()
});

static LANGUAGES_MAP: Lazy<HashMap<&'static str, &'static Language>> = Lazy::new(|| {
    LANGUAGES
        .iter()
        .map(|lang| (lang.internal_code, lang))
        .collect()
});

static CODE_TO_INTERNAL_CODE_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    LANGS
        .iter()
        .map(|&(code, alias, _)| (if alias.is_empty() { code } else { alias }, code))
        .collect()
});

pub fn get_language_from_code(code: &str) -> Option<&'static Language> {
    let internal_code = CODE_TO_INTERNAL_CODE_MAP.get(code).unwrap_or(&code);
    LANGUAGES_MAP.get(internal_code).map(|v| &**v)
}
