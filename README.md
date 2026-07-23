# LTEngine

LTEngine-esc is a portable, offline-first Linux document translator powered by local GGUF language models and [llama.cpp](https://github.com/ggml-org/llama.cpp). The primary validated workflow is Swedish-to-English translation, while other language pairs remain supported.

The application translates text, stdin, or local `.txt` documents directly from the CLI. It has no HTTP server, browser UI, or loopback listener; see [the project specification](docs/PROJECT_SPEC.md) for the product contract.

![Translation](https://github.com/user-attachments/assets/37dd4e20-382b-459d-bcc1-5de3ed4b4c18)

The LLMs in LTEngine are much larger than the lightweight transformer models in [LibreTranslate](https://github.com/LibreTranslate/LibreTranslate). Thus memory usage and speed are traded off for quality of outputs, which for some languages has been reported as being [on par or better than DeepL](https://community.libretranslate.com/t/ltengine-llm-powered-local-machine-translation/1862/5).

It is possible to run LTEngine entirely on the CPU, but an accelerator will greatly improve performance. Supported accelerators currently include:

 * CUDA
 * Metal (macOS)
 * Vulkan

 The largest model (`gemma3-27b`) can fit on a single consumer RTX 3090 with 24G of VRAM.

> ⚠️ LTEngine is in active development. Check the [Roadmap](#roadmap) for current limitations.


## Requirements

 * [Rust](https://www.rust-lang.org/)
 * [clang](https://clang.llvm.org/)
 * [CMake](https://cmake.org/)
 * A C++ compiler (g++, MSVC) for building the llama.cpp bindings

## Build

```bash
git clone https://github.com/escapables/LTEngine-esc.git --recursive
cd LTEngine-esc
cargo build --release
```

## Run

Running without a subcommand prints an error and usage. Use the required `translate` subcommand:

Translate Swedish text directly to English:

```bash
./target/release/ltengine translate --source sv --target en --text 'Hej världen!' --model-file ./models/model.gguf
```

Translate stdin while delegating source-language recognition to the model:

```bash
printf 'Hej världen!\n' | ./target/release/ltengine translate --source auto --target en --stdin --model-file ./models/model.gguf
```

Translate a UTF-8 `.txt` document to a new path:

```bash
./target/release/ltengine translate --source sv --target en \
  --input ./documents/source.txt --output ./documents/translated.txt \
  --model-file ./models/model.gguf
```

Exactly one of `--text`, `--stdin`, or `--input` is required; document mode also requires `--output`. The default document limit is 10 MiB and can be changed with `--max-input-bytes`. Existing output files are never overwritten. Text/stdin translation is the only stdout output; document output goes to the selected path. Model status and errors use stderr.

To run different LLM models:

```bash
./target/release/ltengine translate --source sv --target en --text 'Hej' \
  -m gemma3-4b [--model-file /path/to/model.gguf]
```

For offline operation, stage the GGUF model before disconnecting and pass its local path:

```bash
./target/release/ltengine translate --source sv --target en \
  --input ./documents/source.txt --output ./documents/translated.txt \
  --model-file ./models/model.gguf
```

Inference remains local and makes no external translation API calls. Without `--model-file`, first use may download a model from Hugging Face.

## Models

LTEngine supports any GGUF language model supported by [llama.cpp](https://github.com/ggml-org/llama.cpp). You can pass a path to load a custom .gguf model using the `--model-file` parameter. Otherwise LTEngine will download one of the Gemma3 models based on the `-m` parameter: 

| Model      | RAM Usage | GPU Usage | Notes                               | Default            |
| ---------- | --------- | --------- | ----------------------------------- | ------------------ |
| gemma3-1b  | 1G        | 2G        | Good for testing, poor translations |                    |
| gemma3-4b  | 4G        | 4G        |                                     | :heavy_check_mark: |
| gemma3-12b | 8G        | 10G       |                                     |                    |
| gemma3-27b | 16G       | 18G       | Best translation quality, slowest   |                    |

Memory usage numbers are approximate.

## Roadmap

See [docs/ROADMAP.md](docs/ROADMAP.md) for the public roadmap. Maintainers use
[docs/PRIMARY_TODO.md](docs/PRIMARY_TODO.md) for milestone detail and
[docs/TODO.md](docs/TODO.md) for ready work.

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on pull requests, code style, and local quality gates.

## Credits

This work is largely possible thanks [llama-cpp-rs](https://github.com/utilityai/llama-cpp-rs) which provide the Rust bindings to [llama.cpp](https://github.com/ggml-org/llama.cpp).

## License

[GNU Affero General Public License v3](https://www.gnu.org/licenses/agpl-3.0.en.html)

## Trademark

See [Trademark Guidelines](https://github.com/LibreTranslate/LibreTranslate/blob/main/TRADEMARK.md)
