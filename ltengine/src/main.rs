use clap::Parser;

mod cli;
mod document;
mod languages;
mod llm;
mod models;
mod prompt;
mod translation;

use cli::{Args, Command};
use models::load_model;

fn main() {
    let args = Args::parse();
    let Command::Translate(command) = &args.command;

    let model_path = load_model(&args.model, &args.model_file).unwrap_or_else(|error| {
        eprintln!("Failed to load model: {error}");
        std::process::exit(1);
    });
    eprintln!("Loading model: {}", model_path.display());

    let llm = llm::LLM::new(model_path, args.cpu, args.verbose).unwrap_or_else(|error| {
        eprintln!("Failed to initialize LLM: {error}");
        std::process::exit(1);
    });
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    cli::run_translate(command, &llm, stdin.lock(), stdout.lock()).unwrap_or_else(|error| {
        eprintln!("Error: {error:#}");
        std::process::exit(1);
    });
}
