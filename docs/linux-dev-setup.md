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
git clone https://github.com/LibreTranslate/LTEngine --recursive
cd LTEngine
cargo build [--features cuda,vulkan,metal] --release
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

## Troubleshooting

- **Build fails with linker errors**: Ensure clang and a C++ compiler are installed.
- **CUDA build fails**: Check CUDA toolkit installation and `nvcc` is on PATH.
- **Model download is slow**: The Gemma3 models are several GB. Use a local file with `--model-file` if you have it.
