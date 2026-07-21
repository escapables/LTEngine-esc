use anyhow::Result;

use crate::languages::{detect_lang, get_language_from_code};
use crate::prompt::PromptBuilder;

pub trait Inference {
    fn run_prompt(&self, system: String, user: String) -> Result<String>;
}

pub struct TranslationRequest<'a> {
    pub text: &'a str,
    pub source: &'a str,
    pub target: &'a str,
    pub format: &'a str,
}

#[derive(Debug)]
pub struct DetectedLanguage {
    pub code: &'static str,
    pub confidence: i32,
}

#[derive(Debug)]
pub struct Translation {
    pub text: String,
    pub detected_language: Option<DetectedLanguage>,
}

#[derive(Debug)]
pub enum TranslationError {
    InvalidFormat(String),
    UnsupportedLanguage(String),
    Inference(anyhow::Error),
}

impl std::fmt::Display for TranslationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat(format) => write!(formatter, "Invalid format: {format}"),
            Self::UnsupportedLanguage(language) => {
                write!(formatter, "{language} is not supported")
            }
            Self::Inference(error) => write!(formatter, "Translation failed: {error}"),
        }
    }
}

impl std::error::Error for TranslationError {}

pub fn translate(
    inference: &impl Inference,
    request: TranslationRequest<'_>,
) -> Result<Translation, TranslationError> {
    if !matches!(request.format, "text" | "html") {
        return Err(TranslationError::InvalidFormat(request.format.to_string()));
    }

    let source_language = if request.source == "auto" {
        "auto"
    } else {
        get_language_from_code(request.source)
            .ok_or_else(|| TranslationError::UnsupportedLanguage(request.source.to_string()))?
            .name
    };
    let target_language = get_language_from_code(request.target)
        .ok_or_else(|| TranslationError::UnsupportedLanguage(request.target.to_string()))?
        .name;

    let translated_text = if request.source == request.target {
        request.text.to_string()
    } else {
        let mut prompt_builder = PromptBuilder::new();
        prompt_builder
            .set_format(request.format)
            .set_source_language(source_language)
            .set_target_language(target_language);
        let source_text = request.text.to_string();
        let prompt = prompt_builder.build(&source_text);
        inference
            .run_prompt(prompt.system, prompt.user)
            .map_err(TranslationError::Inference)?
    };

    let detected_language = (request.source == "auto").then(|| {
        let detected = detect_lang(request.text);
        DetectedLanguage {
            code: detected.language.code,
            confidence: detected.confidence,
        }
    });

    Ok(Translation {
        text: improve_formatting(request.text, &translated_text),
        detected_language,
    })
}

fn improve_formatting(source: &str, translation: &str) -> String {
    let translation = translation.trim();

    if source.is_empty() {
        return String::new();
    }

    if translation.is_empty() {
        return source.to_string();
    }

    let source_last = source.chars().next_back().expect("source is not empty");
    let translation_last = translation
        .chars()
        .next_back()
        .expect("translation is not empty");
    let mut result = translation.to_string();

    const PUNCTUATION: [char; 6] = ['!', '?', '.', ',', ';', '。'];
    if PUNCTUATION.contains(&source_last) {
        if source_last != translation_last {
            if PUNCTUATION.contains(&translation_last) {
                result.pop();
            }
            result.push(source_last);
        }
    } else if PUNCTUATION.contains(&translation_last) {
        result.pop();
    }

    if source.chars().all(char::is_lowercase) {
        result = result.to_lowercase();
    }

    if source.chars().all(char::is_uppercase) {
        result = result.to_uppercase();
    }

    if let (Some(source_first), Some(result_first)) = (source.chars().next(), result.chars().next())
    {
        if source_first.is_lowercase() && result_first.is_uppercase() {
            result.replace_range(
                0..result_first.len_utf8(),
                &result_first.to_lowercase().to_string(),
            );
        } else if source_first.is_uppercase() && result_first.is_lowercase() {
            result.replace_range(
                0..result_first.len_utf8(),
                &result_first.to_uppercase().to_string(),
            );
        }
    }

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};

    use anyhow::{Result, anyhow};

    use super::{Inference, TranslationError, TranslationRequest, translate};

    #[derive(Default)]
    struct ControlledInference {
        calls: Cell<usize>,
        prompts: RefCell<Vec<(String, String)>>,
        response: RefCell<Option<Result<String>>>,
    }

    impl ControlledInference {
        fn returning(response: &str) -> Self {
            Self {
                response: RefCell::new(Some(Ok(response.to_string()))),
                ..Self::default()
            }
        }

        fn failing(message: &str) -> Self {
            Self {
                response: RefCell::new(Some(Err(anyhow!(message.to_string())))),
                ..Self::default()
            }
        }
    }

    impl Inference for ControlledInference {
        fn run_prompt(&self, system: String, user: String) -> Result<String> {
            self.calls.set(self.calls.get() + 1);
            self.prompts.borrow_mut().push((system, user));
            self.response
                .borrow_mut()
                .take()
                .expect("controlled response must be configured")
        }
    }

    #[test]
    fn translates_swedish_to_english_through_controlled_inference() {
        let inference = ControlledInference::returning("Hello world.");

        let output = translate(
            &inference,
            TranslationRequest {
                text: "Hej världen!",
                source: "sv",
                target: "en",
                format: "text",
            },
        )
        .expect("translation should succeed");

        assert_eq!(output.text, "Hello world!");
        assert!(output.detected_language.is_none());
        let prompts = inference.prompts.borrow();
        assert!(prompts[0].1.contains("from Swedish to English"));
        assert!(prompts[0].1.contains("Hej världen!"));
    }

    #[test]
    fn preserves_general_language_selection() {
        let inference = ControlledInference::returning("Bonjour.");

        translate(
            &inference,
            TranslationRequest {
                text: "Hola.",
                source: "es",
                target: "fr",
                format: "text",
            },
        )
        .expect("translation should succeed");

        assert!(
            inference.prompts.borrow()[0]
                .1
                .contains("from Spanish to French")
        );
    }

    #[test]
    fn source_equal_to_target_skips_inference() {
        let inference = ControlledInference::default();

        let output = translate(
            &inference,
            TranslationRequest {
                text: "Samma text.",
                source: "sv",
                target: "sv",
                format: "text",
            },
        )
        .expect("identity translation should succeed");

        assert_eq!(output.text, "Samma text.");
        assert_eq!(inference.calls.get(), 0);
    }

    #[test]
    fn auto_source_returns_detected_language() {
        let inference = ControlledInference::returning("This is a translated sentence.");

        let output = translate(
            &inference,
            TranslationRequest {
                text: "Det här är en svensk mening som ska identifieras korrekt.",
                source: "auto",
                target: "en",
                format: "text",
            },
        )
        .expect("translation should succeed");

        let detected = output
            .detected_language
            .expect("auto source should include detection metadata");
        assert_eq!(detected.code, "sv");
    }

    #[test]
    fn returns_inference_errors_instead_of_source_text() {
        let inference = ControlledInference::failing("controlled failure");

        let error = translate(
            &inference,
            TranslationRequest {
                text: "Hej.",
                source: "sv",
                target: "en",
                format: "text",
            },
        )
        .expect_err("inference failure must be returned");

        assert!(matches!(error, TranslationError::Inference(_)));
        assert!(error.to_string().contains("controlled failure"));
    }

    #[test]
    fn rejects_unsupported_languages_before_inference() {
        let inference = ControlledInference::default();

        let error = translate(
            &inference,
            TranslationRequest {
                text: "Hej.",
                source: "unsupported",
                target: "en",
                format: "text",
            },
        )
        .expect_err("unsupported source must fail");

        assert!(matches!(error, TranslationError::UnsupportedLanguage(_)));
        assert_eq!(inference.calls.get(), 0);
    }

    #[test]
    fn rejects_unsupported_formats_before_inference() {
        let inference = ControlledInference::default();

        let error = translate(
            &inference,
            TranslationRequest {
                text: "Hej.",
                source: "sv",
                target: "en",
                format: "markdown",
            },
        )
        .expect_err("unsupported format must fail");

        assert!(matches!(error, TranslationError::InvalidFormat(_)));
        assert_eq!(inference.calls.get(), 0);
    }
}
