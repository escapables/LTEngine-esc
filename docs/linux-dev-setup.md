---
summary: 'Development toolchain and build setup requirements for LTEngine.'
read_when:
  - Preparing local build environment.
  - Reproducing CI toolchain locally.
---

# Development Setup

Requirements:

- [Rust](https://www.rust-lang.org/) (latest stable)
- [clang](https://clang.llvm.org/)
- [CMake](https://cmake.org/) 3.12+
- A C++ compiler (g++, MSVC, or clang++) for building llama.cpp bindings
- Git

Optional for hardware acceleration:
- CUDA Toolkit (for NVIDIA GPUs)
- Vulkan SDK (for Vulkan support)

## Build

```bash
# Standard build
git clone https://github.com/LibreTranslate/LTEngine --recursive
cd LTEngine
cargo build --release

# With hardware acceleration (choose one)
cargo build --release --features cuda     # NVIDIA GPU
cargo build --release --features metal    # macOS
cargo build --release --features vulkan   # Vulkan-compatible GPU
```

## Run

```bash
./target/release/ltengine
```

To run with a specific model:

```bash
./target/release/ltengine -m gemma3-12b [--model-file /path/to/model.gguf]
```

## Development Tips

- Use `cargo run` for development builds (slower but faster compile).
- Use `--features cuda` if you have an NVIDIA GPU for much faster inference.
- First run will download the model from HuggingFace (several GB).
- Use `--model-file` to use a locally stored GGUF file instead.

## Expected First-Run Behavior

On first run, the server will:
1. Download the default model (~5-15GB depending on model size) from HuggingFace
2. Load the model into memory (this may take 30-60 seconds)
3. Start the HTTP server on `127.0.0.1:5050`

Expected output:
```
Loading model gemma-3-12b-it-q4_0.gguf...
Server running on http://127.0.0.1:5050
```

## Troubleshooting

- **Build fails with linker errors**: Ensure clang and a C++ compiler are installed.
  ```bash
  # Ubuntu/Debian
  sudo apt-get install clang build-essential cmake
  
  # macOS
  xcode-select --install
  ```

- **CUDA build fails**: Check CUDA toolkit installation and `nvcc` is on PATH.
  ```bash
  nvcc --version  # Should print version info
  ```

- **Model download is slow**: The Gemma3 models are several GB. Use a local file with `--model-file` if you have it.

- **"Out of memory" error**: Use a smaller model:
  ```bash
  ./target/release/ltengine -m gemma3-4b  # or gemma3-1b
  ```

- **Server won't start / port in use**: Check if another instance is running:
  ```bash
  lsof -i :5050
  kill $(lsof -t -i:5050)  # Stop existing process
  ```
