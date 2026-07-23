use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{ArgGroup, Args as ClapArgs, Parser, Subcommand};

use crate::models::{DEFAULT_MODEL, MODELS};
use crate::translation::{Inference, TranslationRequest, translate};

const DEFAULT_MAX_DOCUMENT_BYTES: u64 = 10 * 1024 * 1024;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    /// Model to use
    #[arg(short='m', long, value_parser = MODELS.keys().collect::<Vec<_>>(), default_value = DEFAULT_MODEL, global = true)]
    pub model: String,

    /// Path to a local GGUF model
    #[arg(long, default_value = "", global = true)]
    pub model_file: String,

    /// Use CPU only
    #[arg(long, global = true)]
    pub cpu: bool,

    /// Enable verbose logging
    #[arg(short = 'v', long, global = true)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Translate text or a .txt document
    Translate(TranslateArgs),
}

#[derive(ClapArgs, Debug)]
#[command(group(
    ArgGroup::new("translation_input")
        .required(true)
        .multiple(false)
        .args(["text", "stdin", "input"])
))]
pub struct TranslateArgs {
    /// Source language code, or auto
    #[arg(long)]
    pub source: String,

    /// Target language code
    #[arg(long)]
    pub target: String,

    /// Text to translate
    #[arg(long)]
    pub text: Option<String>,

    /// Read text from standard input
    #[arg(long)]
    pub stdin: bool,

    /// Read a UTF-8 .txt document
    #[arg(long, requires = "output")]
    pub input: Option<PathBuf>,

    /// Write a translated .txt document without overwriting an existing file
    #[arg(long, requires = "input")]
    pub output: Option<PathBuf>,

    /// Maximum document input size in bytes
    #[arg(long, default_value_t = DEFAULT_MAX_DOCUMENT_BYTES)]
    pub max_input_bytes: u64,
}

pub fn run_translate(
    args: &TranslateArgs,
    inference: &impl Inference,
    mut stdin: impl Read,
    mut stdout: impl Write,
) -> Result<()> {
    if let (Some(input), Some(output)) = (&args.input, &args.output) {
        return crate::document::translate_document(
            input,
            output,
            args.max_input_bytes,
            &args.source,
            &args.target,
            inference,
        );
    }

    let text = if let Some(text) = &args.text {
        text.clone()
    } else {
        let mut text = String::new();
        stdin
            .read_to_string(&mut text)
            .context("Failed to read translation text from stdin")?;
        text.trim_end_matches(['\r', '\n']).to_string()
    };

    if text.trim().is_empty() {
        bail!("Translation input is empty");
    }

    let output = translate(
        inference,
        TranslationRequest {
            text: &text,
            source: &args.source,
            target: &args.target,
            format: "text",
        },
    )?;

    writeln!(stdout, "{}", output.text).context("Failed to write translated text to stdout")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::io::Cursor;

    use anyhow::{Result, anyhow};
    use clap::Parser;

    use super::{Args, Command, TranslateArgs, run_translate};
    use crate::translation::Inference;

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

    fn translate_args(args: &Args) -> &TranslateArgs {
        match &args.command {
            Command::Translate(args) => args,
        }
    }

    #[test]
    fn translates_swedish_text_to_stdout() {
        let args = Args::try_parse_from([
            "ltengine",
            "translate",
            "--source",
            "sv",
            "--target",
            "en",
            "--text",
            "Hej världen!",
            "--model",
            "gemma3-4b",
        ])
        .expect("CLI arguments should parse");
        let inference = ControlledInference::returning("Hello world.");
        let mut stdout = Vec::new();

        run_translate(
            translate_args(&args),
            &inference,
            Cursor::new(Vec::<u8>::new()),
            &mut stdout,
        )
        .expect("CLI translation should succeed");

        assert_eq!(args.model, "gemma3-4b");
        assert_eq!(String::from_utf8(stdout).unwrap(), "Hello world!\n");
    }

    #[test]
    fn translates_another_language_pair() {
        let args = Args::try_parse_from([
            "ltengine",
            "translate",
            "--source",
            "es",
            "--target",
            "fr",
            "--text",
            "Hola.",
        ])
        .expect("CLI arguments should parse");
        let inference = ControlledInference::returning("Bonjour.");
        let mut stdout = Vec::new();

        run_translate(
            translate_args(&args),
            &inference,
            Cursor::new(Vec::<u8>::new()),
            &mut stdout,
        )
        .expect("CLI translation should succeed");

        assert_eq!(String::from_utf8(stdout).unwrap(), "Bonjour.\n");
    }

    #[test]
    fn translates_auto_source_stdin() {
        let args = Args::try_parse_from([
            "ltengine",
            "translate",
            "--source",
            "auto",
            "--target",
            "en",
            "--stdin",
        ])
        .expect("CLI arguments should parse");
        let inference = ControlledInference::returning("This is Swedish.");
        let mut stdout = Vec::new();

        run_translate(
            translate_args(&args),
            &inference,
            Cursor::new("Det här är svenska.\n"),
            &mut stdout,
        )
        .expect("stdin translation should succeed");

        assert_eq!(String::from_utf8(stdout).unwrap(), "This is Swedish.\n");
    }

    #[test]
    fn source_equal_to_target_skips_inference() {
        let args = Args::try_parse_from([
            "ltengine",
            "translate",
            "--source",
            "sv",
            "--target",
            "sv",
            "--text",
            "Samma text.",
        ])
        .expect("CLI arguments should parse");
        let inference = ControlledInference::unused();
        let mut stdout = Vec::new();

        run_translate(
            translate_args(&args),
            &inference,
            Cursor::new(Vec::<u8>::new()),
            &mut stdout,
        )
        .expect("identity translation should succeed");

        assert_eq!(String::from_utf8(stdout).unwrap(), "Samma text.\n");
        assert_eq!(inference.calls.get(), 0);
    }

    #[test]
    fn rejects_missing_or_ambiguous_input() {
        let missing =
            Args::try_parse_from(["ltengine", "translate", "--source", "sv", "--target", "en"]);
        let ambiguous = Args::try_parse_from([
            "ltengine",
            "translate",
            "--source",
            "sv",
            "--target",
            "en",
            "--text",
            "Hej.",
            "--stdin",
        ]);

        assert!(missing.is_err());
        assert!(ambiguous.is_err());
    }

    #[test]
    fn requires_translation_subcommand() {
        assert!(Args::try_parse_from(["ltengine"]).is_err());
    }

    #[test]
    fn rejects_removed_server_flags() {
        for (flag, value) in [
            ("--host", "127.0.0.1"),
            ("--port", "5050"),
            ("--char-limit", "5000"),
            ("--api-key", "secret"),
        ] {
            let parsed = Args::try_parse_from([
                "ltengine",
                flag,
                value,
                "translate",
                "--source",
                "sv",
                "--target",
                "en",
                "--text",
                "Hej.",
            ]);

            assert!(parsed.is_err(), "{flag} must not remain in the CLI");
        }
    }

    #[test]
    fn requires_document_input_and_output_together() {
        let missing_output = Args::try_parse_from([
            "ltengine",
            "translate",
            "--source",
            "sv",
            "--target",
            "en",
            "--input",
            "source.txt",
        ]);
        let missing_input = Args::try_parse_from([
            "ltengine",
            "translate",
            "--source",
            "sv",
            "--target",
            "en",
            "--output",
            "translated.txt",
        ]);
        let mixed_modes = Args::try_parse_from([
            "ltengine",
            "translate",
            "--source",
            "sv",
            "--target",
            "en",
            "--text",
            "Hej.",
            "--input",
            "source.txt",
            "--output",
            "translated.txt",
        ]);

        assert!(missing_output.is_err());
        assert!(missing_input.is_err());
        assert!(mixed_modes.is_err());
    }

    #[test]
    fn returns_actionable_inference_errors() {
        let args = Args::try_parse_from([
            "ltengine",
            "translate",
            "--source",
            "sv",
            "--target",
            "en",
            "--text",
            "Hej.",
        ])
        .expect("CLI arguments should parse");
        let inference = ControlledInference {
            calls: Cell::new(0),
            response: RefCell::new(Some(Err(anyhow!("controlled failure")))),
        };

        let error = run_translate(
            translate_args(&args),
            &inference,
            Cursor::new(Vec::<u8>::new()),
            Vec::new(),
        )
        .expect_err("inference failure must be returned");

        assert!(error.to_string().contains("controlled failure"));
    }

    #[test]
    fn rejects_empty_stdin_before_inference() {
        let args = Args::try_parse_from([
            "ltengine",
            "translate",
            "--source",
            "sv",
            "--target",
            "en",
            "--stdin",
        ])
        .expect("CLI arguments should parse");
        let inference = ControlledInference::unused();

        let error = run_translate(
            translate_args(&args),
            &inference,
            Cursor::new("\n"),
            Vec::new(),
        )
        .expect_err("empty stdin must fail");

        assert!(error.to_string().contains("input is empty"));
        assert_eq!(inference.calls.get(), 0);
    }
}
