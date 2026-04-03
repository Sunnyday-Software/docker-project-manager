# Contributing to Docker Project Manager

First off, thank you for considering contributing to Docker Project Manager!

We welcome contributions from the community. This document outlines the process
for contributing and the guidelines to follow.

## Ways to Contribute

There are many ways to contribute:

- **Report Bugs**: Create an issue with detailed reproduction steps
- **Suggest Features**: Open an issue describing the feature you'd like to see
- **Write Documentation**: Improve our docs or add new content
- **Code Contributions**: Fix bugs, implement features, or add tests
- **Review Pull Requests**: Help review others' contributions

## Development Process

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes following our coding standards
4. Write tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## Coding Standards

We use strict coding standards to maintain code quality:

- **Line width**: 80 characters
- **Indentation**: 2 spaces (no tabs)
- **Style**: Rust 2024 edition
- **Formatting**: Run `cargo fmt` before committing
- **Linting**: Run `cargo clippy` and fix warnings

## Commit Message Format

We follow Conventional Commits specification. See [commit-message-guidelines.md](./commit-message-guidelines.md)
for details.

```
# Good examples
git commit -m "feat: add Docker container management commands"
git commit -m "fix: resolve path resolution issue on Windows"
git commit -m "docs: update installation instructions"
```

## Testing

All new code should include tests. Run tests with:

```bash
cargo test
```

## Security Checks

Before submitting, run security checks:

```bash
cargo deny check
```

## Pull Request Process

1. Ensure all tests and checks pass
2. Update documentation if needed
3. Follow commit message format
4. Request review from maintainers

## Code of Conduct

Please note that this project is released with a Contributor Code of Conduct.
By participating in this project you agree to abide by its terms.

See [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) for details.

## Getting Help

- Open an issue for bugs or feature requests
- Join our community discussions
- Check the documentation

Thank you for contributing!