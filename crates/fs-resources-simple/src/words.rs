//! The words of the window: `MessageFormat` 2 files in German and English (ADR 0057), picked by the language of the session.
//!
//! Pattern: Factory over embedded `.mf2` files; a language without a file comes in English.

use fs_components_mf2::{Mf2Texts, Source};
use fs_components_words::Locale;

/// The two language files; `fs_components_mf2::check_pairs(TEXTS)` must be empty (a test asserts it).
pub const TEXTS: &[Source] = &[
    Source {
        language: "en",
        text: include_str!("texts/en.mf2"),
    },
    Source {
        language: "de",
        text: include_str!("texts/de.mf2"),
    },
];

/// The words in `locale`.
#[must_use]
pub fn words(locale: &Locale) -> Mf2Texts {
    Mf2Texts::new(TEXTS, locale)
}

/// The language of this session: `LC_ALL`, `LC_MESSAGES`, `LANG` in this order, else English.
#[must_use]
pub fn session_locale() -> Locale {
    ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .filter_map(|name| std::env::var(name).ok())
        .find_map(|value| Locale::from_posix(&value))
        .unwrap_or_else(Locale::english)
}

#[cfg(test)]
mod tests {
    use super::{TEXTS, words};
    use fs_components_mf2::check_pairs;
    use fs_components_words::Locale;

    #[test]
    fn both_languages_have_the_same_ids_and_variables() {
        assert!(check_pairs(TEXTS).is_empty(), "{:?}", check_pairs(TEXTS));
    }

    #[test]
    fn german_and_english_say_different_things() {
        let en = words(&Locale::english()).plain("soon");
        let de = words(&Locale::parse("de").unwrap()).plain("soon");
        assert_eq!(en, "coming soon");
        assert_eq!(de, "kommt noch");
    }
}
