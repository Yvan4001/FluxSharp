# Contributing to FluxSharp

Thank you for your interest in contributing to FluxSharp! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

- Be respectful and inclusive
- Focus on the code, not the person
- Help others learn and grow
- Report issues constructively

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Create a new branch** for your feature or fix
4. **Make your changes** following the coding standards
5. **Test thoroughly** before submitting
6. **Submit a pull request** with clear description

## Development Setup

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and setup
git clone https://github.com/Yvan4001/FluxSharp.git
cd FluxSharp

# Build the project
./build.sh
```

## Coding Standards

### Rust Code
- Follow [Rust naming conventions](https://rust-lang.org/api-guidelines/naming.html)
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Write tests for new functionality
- Add documentation comments with `///`

### FluxSharp Language (.fsh)
- Use PascalCase for class names
- Use camelCase for method and variable names
- Always use semicolons at end of statements
- Include proper type annotations
- Follow the language grammar defined in `flux_grammar.pest`

### Documentation
- Write clear, concise documentation
- Use proper English grammar
- Include code examples where applicable
- Update relevant .md files when making changes

## Commit Messages

Use clear, descriptive commit messages:

```
feat: Add new feature description
fix: Fix bug description
docs: Update documentation
refactor: Refactor code section
test: Add or update tests
chore: Update dependencies or configuration
```

## Pull Request Process

1. Update documentation if needed
2. Add tests for new features
3. Ensure all tests pass: `cargo test`
4. Ensure code compiles: `cargo build --release`
5. Write a clear PR description
6. Reference any related issues
7. Wait for review before merging

## Testing

Run the test suite:

```bash
# Build and test
./build.sh

# Run specific test
cargo test --test specific_test

# Run with output
cargo test -- --nocapture
```

## Reporting Issues

When reporting bugs, include:

- Clear title and description
- Steps to reproduce
- Expected behavior
- Actual behavior
- FluxSharp version and system info
- Code example or test case

## Feature Requests

For feature requests:

1. Check if already requested or implemented
2. Provide clear use case
3. Include example code if applicable
4. Explain why this feature is important

## Compiler Components

When working on the compiler:

- **Main compiler** (`flux_compiler/fluxc/src/main.rs`)
- **Error handling** (`flux_compiler/fluxc/src/error_handler.rs`)
- **Grammar** (`flux_compiler/fluxc/src/flux_grammar.pest`)
- **Runtime** (`flux_compiler/fluxc/runtime/`)
- **Security** (`flux_compiler/fluxc/src/advanced_security.rs`)
- **Bounds checking** (`flux_compiler/fluxc/src/bounds_checker.rs`)

## Documentation Updates

Documentation files are in the `docs/` directory:

- `docs/01-quickstart/` - Getting started guides
- `docs/02-language/` - Language features and syntax
- `docs/03-advanced/` - Advanced topics
- `docs/04-compiler/` - Compiler documentation
- `docs/05-reference/` - Reference materials

## License

By contributing to FluxSharp, you agree that your contributions will be licensed under the MIT License.

## Questions?

- Check existing documentation in `docs/`
- Search closed issues on GitHub
- Open a discussion for questions

Thank you for contributing to FluxSharp! 🚀

