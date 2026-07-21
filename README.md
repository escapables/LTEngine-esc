# LTEngine

LTEngine-esc is a portable, offline-first Linux document translator powered by local GGUF language models and [llama.cpp](https://github.com/ggml-org/llama.cpp). The primary validated workflow is Swedish-to-English translation, while other language pairs remain supported.

The application now translates text or stdin directly from the CLI without a listener. The inherited [LibreTranslate](https://github.com/LibreTranslate/LibreTranslate)-compatible HTTP server remains temporarily for document parity, then will be removed with the bundled browser UI; see [the project specification](docs/PROJECT_SPEC.md) for the product contract and current-versus-target distinction.

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

```bash
./target/release/ltengine
```

With no subcommand, this starts the temporary local server at `http://127.0.0.1:5050`.

Translate Swedish text directly to English:

```bash
./target/release/ltengine translate --source sv --target en --text 'Hej världen!' --model-file ./models/model.gguf
```

Translate stdin with automatic source detection:

```bash
printf 'Hej världen!\n' | ./target/release/ltengine translate --source auto --target en --stdin --model-file ./models/model.gguf
```

Exactly one of `--text` or `--stdin` is required. The translation is the only stdout output; model status and errors use stderr.

To run different LLM models:

```bash
./target/release/ltengine -m gemma3-4b [--model-file /path/to/model.gguf]
```

For offline operation, stage the GGUF model before disconnecting and pass its local path:

```bash
./target/release/ltengine --model-file ./models/model.gguf
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

### Simple

Request:

```javascript
const res = await fetch("http://127.0.0.1:5050/translate", {
  method: "POST",
  body: JSON.stringify({
    q: "Hello!",
    source: "en",
    target: "es",
  }),
  headers: { "Content-Type": "application/json" },
});

console.log(await res.json());
```

Response:

```javascript
{
    "translatedText": "¡Hola!"
}
```

List of language codes: http://127.0.0.1:5050/languages

### Auto Detect Language

Request:

```javascript
const res = await fetch("http://127.0.0.1:5050/translate", {
  method: "POST",
  body: JSON.stringify({
    q: "Ciao!",
    source: "auto",
    target: "en",
  }),
  headers: { "Content-Type": "application/json" },
});

console.log(await res.json());
```

Response:

```javascript
{
    "detectedLanguage": {
        "confidence": 83,
        "language": "it"
    },
    "translatedText": "Bye!"
}
```

## Language Bindings

You can use the LTEngine API using the following bindings:

- Rust: <https://github.com/DefunctLizard/libretranslate-rs>
- Node.js: <https://github.com/franciscop/translate>
- TypeScript: <https://github.com/tderflinger/libretranslate-ts>
- .Net: <https://github.com/sigaloid/LibreTranslate.Net>
- Go: <https://github.com/SnakeSel/libretranslate>
- Python: <https://github.com/argosopentech/LibreTranslate-py>
- PHP: <https://github.com/jefs42/libretranslate>
- C++: <https://github.com/argosopentech/LibreTranslate-cpp>
- Swift: <https://github.com/wacumov/libretranslate>
- Unix: <https://github.com/argosopentech/LibreTranslate-sh>
- Shell: <https://github.com/Hayao0819/Hayao-Tools/tree/master/libretranslate-sh>
- Java: <https://github.com/suuft/libretranslate-java>
- Ruby: <https://github.com/noesya/libretranslate>
- R: <https://github.com/myanesp/libretranslateR>

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
