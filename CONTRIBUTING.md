---
summary: 'Contribution guidelines for LTEngine including quality gates and PR workflow.'
read_when:
  - Preparing your first contribution.
  - Opening a pull request.
  - Understanding project conventions.
---

# Contributing to LTEngine

Thank you for your interest in contributing! This guide covers how to contribute effectively.

## Quick Start

1. Fork and clone the repository:
```bash
git clone https://github.com/LibreTranslate/LTEngine --recursive
cd LTEngine
```

2. Build the project:
```bash
cargo build --release
```

3. Run tests:
```bash
cargo test
```

4. Make your changes and test locally.

## Pull Request Workflow

### Before Submitting

- [ ] Code compiles without warnings: `cargo build --release`
- [ ] Tests pass: `cargo test`
- [ ] Changes follow existing code style
- [ ] Documentation updated (if behavior changes)
- [ ] Commit messages are clear and descriptive

### Opening a PR

1. Create a feature branch:
```bash
git checkout -b feature/my-feature
```

2. Make commits with clear messages:
```bash
git commit -m "feat: add new translation endpoint"
```

3. Push and open PR:
```bash
git push origin feature/my-feature
```

4. In your PR description:
   - Explain what changed and why
   - Reference any related issues
   - Note any breaking changes

### PR Review Process

- All PRs require at least one review
- Address review feedback promptly
- CI must pass before merge
- Maintainers may squash commits on merge

## Local Quality Gates

Run these before pushing:

```bash
# Build verification
cargo build --release

# Test verification
cargo test

# Smoke test the binary
./target/release/ltengine --help
```

## Code Style

- Follow Rust idioms and `cargo fmt` conventions
- Add comments for complex logic
- Keep functions focused and small
- Use meaningful variable names

## Documentation

If your change affects user-facing behavior:

- Update `README.md` for API changes
- Update `docs/PORTABLE_APP.md` for runtime contract changes
- Add/update examples if applicable

## Getting Help

- Open an issue for bugs or feature requests
- Join discussions in existing issues
- Check `docs/` for architecture and setup information

## License

By contributing, you agree that your contributions will be licensed under the GNU Affero General Public License v3.
