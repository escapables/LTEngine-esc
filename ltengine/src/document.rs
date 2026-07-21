use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::translation::{Inference, TranslationRequest, translate};

pub fn translate_document(
    input: &Path,
    output: &Path,
    max_input_bytes: u64,
    source: &str,
    target: &str,
    inference: &impl Inference,
) -> Result<()> {
    require_txt(input, "Input")?;
    require_txt(output, "Output")?;

    let input_path = fs::canonicalize(input)
        .with_context(|| format!("Failed to resolve input document {}", input.display()))?;
    if !input_path.is_file() {
        bail!("Input document is not a regular file: {}", input.display());
    }

    if output.exists() {
        let output_path = fs::canonicalize(output)
            .with_context(|| format!("Failed to resolve output path {}", output.display()))?;
        if input_path == output_path {
            bail!("Input and output paths refer to the same file");
        }
        bail!("Output path already exists: {}", output.display());
    }

    let mut bytes = Vec::new();
    File::open(&input_path)
        .with_context(|| format!("Failed to open input document {}", input.display()))?
        .take(max_input_bytes.saturating_add(1))
        .read_to_end(&mut bytes)
        .with_context(|| format!("Failed to read input document {}", input.display()))?;
    if bytes.len() as u64 > max_input_bytes {
        bail!(
            "Input document exceeds the {max_input_bytes}-byte limit: {}",
            input.display()
        );
    }

    let text = String::from_utf8(bytes).with_context(|| {
        format!(
            "Input document must contain valid UTF-8: {}",
            input.display()
        )
    })?;
    if text.trim().is_empty() {
        bail!("Input document is empty: {}", input.display());
    }

    let content_start = text.len() - text.trim_start_matches(char::is_whitespace).len();
    let content_end = text.trim_end_matches(char::is_whitespace).len();
    let leading_whitespace = &text[..content_start];
    let content = &text[content_start..content_end];
    let trailing_whitespace = &text[content_end..];
    let translated = translate(
        inference,
        TranslationRequest {
            text: content,
            source,
            target,
            format: "text",
        },
    )?;

    let output_text = leading_whitespace.to_string() + &translated.text + trailing_whitespace;

    let mut output_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)
        .with_context(|| format!("Failed to create output document {}", output.display()))?;
    if let Err(error) = output_file.write_all(output_text.as_bytes()) {
        return Err(error).with_context(|| {
            format!(
                "Failed to write output document {}; partial output may remain",
                output.display()
            )
        });
    }

    Ok(())
}

fn require_txt(path: &Path, label: &str) -> Result<()> {
    let is_txt = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("txt"));
    if !is_txt {
        bail!("{label} document is unsupported. Only .txt files are accepted");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::fs;
    use std::path::{Path, PathBuf};

    use anyhow::{Result, anyhow};
    use uuid::Uuid;

    use super::translate_document;
    use crate::translation::Inference;

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!("ltengine-document-{}", Uuid::new_v4()));
            fs::create_dir(&path).expect("test directory should be created");
            Self(path)
        }

        fn path(&self, name: &str) -> PathBuf {
            self.0.join(name)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).expect("test directory should be removed");
        }
    }

    struct ControlledInference {
        calls: Cell<usize>,
        response: RefCell<Option<Result<String>>>,
    }

    impl ControlledInference {
        fn returning(response: &str) -> Self {
            Self {
                calls: Cell::new(0),
                response: RefCell::new(Some(Ok(response.to_string()))),
            }
        }

        fn unused() -> Self {
            Self {
                calls: Cell::new(0),
                response: RefCell::new(None),
            }
        }
    }

    impl Inference for ControlledInference {
        fn run_prompt(&self, _system: String, _user: String) -> Result<String> {
            self.calls.set(self.calls.get() + 1);
            self.response
                .borrow_mut()
                .take()
                .expect("controlled response must be configured")
        }
    }

    fn translate(
        input: &Path,
        output: &Path,
        max_input_bytes: u64,
        inference: &ControlledInference,
    ) -> Result<()> {
        translate_document(input, output, max_input_bytes, "sv", "en", inference)
    }

    #[test]
    fn translates_swedish_multiline_document_to_selected_path() {
        let directory = TestDirectory::new();
        let input = directory.path("source.txt");
        let output = directory.path("translated.txt");
        fs::write(&input, "Första stycket.\n\nAndra stycket.\n")
            .expect("fixture should be written");
        let inference = ControlledInference::returning("First paragraph.\n\nSecond paragraph.\n");

        translate(&input, &output, 10 * 1024 * 1024, &inference)
            .expect("document translation should succeed");

        assert_eq!(
            fs::read_to_string(output).expect("output should be readable"),
            "First paragraph.\n\nSecond paragraph.\n"
        );
        assert_eq!(inference.calls.get(), 1);
    }

    #[test]
    fn identity_translation_preserves_document_boundary_whitespace() {
        let directory = TestDirectory::new();
        let input = directory.path("source.txt");
        let output = directory.path("translated.txt");
        let source = "  Rubrik\n\nText  \n";
        fs::write(&input, source).expect("fixture should be written");
        let inference = ControlledInference::unused();

        translate_document(&input, &output, 100, "sv", "sv", &inference)
            .expect("identity document translation should succeed");

        assert_eq!(fs::read_to_string(output).unwrap(), source);
        assert_eq!(inference.calls.get(), 0);
    }

    #[test]
    fn rejects_document_over_configured_byte_limit() {
        let directory = TestDirectory::new();
        let input = directory.path("source.txt");
        let output = directory.path("translated.txt");
        fs::write(&input, "123456").expect("fixture should be written");
        let inference = ControlledInference::unused();

        let error =
            translate(&input, &output, 5, &inference).expect_err("oversized document must fail");

        assert!(error.to_string().contains("exceeds the 5-byte limit"));
        assert!(!output.exists());
        assert_eq!(inference.calls.get(), 0);
    }

    #[test]
    fn rejects_invalid_utf8_without_creating_output() {
        let directory = TestDirectory::new();
        let input = directory.path("source.txt");
        let output = directory.path("translated.txt");
        fs::write(&input, [0xff, 0xfe]).expect("fixture should be written");
        let inference = ControlledInference::unused();

        let error =
            translate(&input, &output, 10, &inference).expect_err("invalid UTF-8 must fail");

        assert!(error.to_string().contains("valid UTF-8"));
        assert!(!output.exists());
        assert_eq!(inference.calls.get(), 0);
    }

    #[test]
    fn refuses_to_overwrite_existing_output() {
        let directory = TestDirectory::new();
        let input = directory.path("source.txt");
        let output = directory.path("translated.txt");
        fs::write(&input, "Hej.").expect("fixture should be written");
        fs::write(&output, "keep me").expect("existing output should be written");
        let inference = ControlledInference::unused();

        let error = translate(&input, &output, 10, &inference)
            .expect_err("existing output must not be overwritten");

        assert!(error.to_string().contains("already exists"));
        assert_eq!(fs::read_to_string(output).unwrap(), "keep me");
        assert_eq!(inference.calls.get(), 0);
    }

    #[test]
    fn rejects_same_input_and_output_path() {
        let directory = TestDirectory::new();
        let input = directory.path("source.txt");
        fs::write(&input, "Hej.").expect("fixture should be written");
        let inference = ControlledInference::unused();

        let error = translate(&input, &input, 10, &inference)
            .expect_err("input path must not be used as output");

        assert!(error.to_string().contains("same file"));
        assert_eq!(fs::read_to_string(input).unwrap(), "Hej.");
        assert_eq!(inference.calls.get(), 0);
    }

    #[test]
    fn rejects_non_txt_input() {
        let directory = TestDirectory::new();
        let input = directory.path("source.md");
        let output = directory.path("translated.txt");
        fs::write(&input, "Hej.").expect("fixture should be written");
        let inference = ControlledInference::unused();

        let error =
            translate(&input, &output, 10, &inference).expect_err("unsupported input must fail");

        assert!(error.to_string().contains("Only .txt"));
        assert!(!output.exists());
        assert_eq!(inference.calls.get(), 0);
    }

    #[test]
    fn leaves_no_output_when_translation_fails() {
        let directory = TestDirectory::new();
        let input = directory.path("source.txt");
        let output = directory.path("translated.txt");
        fs::write(&input, "Hej.").expect("fixture should be written");
        let inference = ControlledInference {
            calls: Cell::new(0),
            response: RefCell::new(Some(Err(anyhow!("controlled failure")))),
        };

        let error = translate(&input, &output, 10, &inference)
            .expect_err("translation failure must be returned");

        assert!(error.to_string().contains("controlled failure"));
        assert!(!output.exists());
    }
}
