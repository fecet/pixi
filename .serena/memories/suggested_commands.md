# Suggested Commands for Pixi Development

## Build Commands
- `pixi run build-debug` - Build in debug mode (fast compilation)
- `pixi run build-release` - Build with optimizations for release
- `cargo build` - Direct cargo build (debug)
- `cargo build --release` - Direct cargo build (release)

## Testing Commands
- `pixi run test` - Run all fast tests (alias for test-all-fast)
- `pixi run test-all-fast` - Run fast unit and integration tests
- `pixi run test-all-slow` - Run all tests including slow ones
- `pixi run test-all-extra-slow` - Run all tests including extra slow ones
- `pixi run test-fast` - Run only fast unit tests with nextest
- `pixi run test-slow` - Run slow unit tests with online/integration features
- `pixi run test-integration-fast` - Run fast integration tests (requires debug build)
- `pixi run test-integration-slow` - Run integration tests (requires release build)
- `pixi run test-integration-extra-slow` - Run all integration tests (requires release build)
- `pixi run test-specific-test -- <test_substring>` - Run specific test by substring
- `cargo nextest run --workspace --all-targets` - Direct nextest execution
- `cargo test` - Standard cargo test

## Linting and Formatting
- `pixi run lint` - Run all linters and formatters (fast + slow)
- `pixi run lint-fast` - Run fast linters and formatters (no clippy)
- `pixi run lint-slow` - Run slow linters including clippy
- `pixi run pre-commit-install` - Install all git hooks (pre-commit + pre-push)
- `pixi run pre-commit-install-minimal` - Install minimal git hooks (pre-commit only)
- `pixi run cargo-fmt` - Format Rust code
- `pixi run cargo-clippy` - Run Rust linter
- `pixi run ruff-lint` - Lint Python code
- `pixi run ruff-format` - Format Python code
- `pixi run typecheck-python` - Type check Python code with mypy
- `pixi run toml-format` - Format TOML files
- `pixi run dprint-fmt` - Format YAML files
- `pixi run typos` - Check and fix typos

## Installation Commands
- `pixi run install` - Install pixi binary locally (Unix only)
- `pixi run install-as <name>` - Build and install with custom name
- `cargo install --locked --git https://github.com/prefix-dev/pixi.git pixi` - Install from source

## Development Utilities
- `pixi run generate-cli-docs` - Generate CLI documentation
- `pixi run insta-review` - Review insta test snapshots
- `pixi run snapshot-update <expression>` - Update Python test snapshots

## Environment Management
- `pixi shell` - Activate pixi development environment
- `pixi run <task>` - Run any defined task in pixi environment

## Git and Release Commands
- `pixi run bump` - Bump version using tbump
- `pixi run release` - Execute release process
- `pixi run bump-changelog` - Update changelog for release