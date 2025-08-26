# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Pixi is a cross-platform, multi-language package manager and workflow tool built on the conda ecosystem. It's written in Rust and provides a Cargo-like experience for managing dependencies across multiple languages (Python, C++, R, etc.).

## Development Commands

### Building
- `pixi run build-debug` - Fast debug build
- `pixi run build-release` - Optimized release build
- `pixi run install` - Install pixi binary locally (Unix only)

### Testing
- `pixi run test` - Run all fast tests
- `pixi run test-all-fast` - Fast unit and integration tests
- `pixi run test-integration-fast` - Integration tests (needs debug build)
- `pixi run test-specific-test -- <substring>` - Run specific tests

### Code Quality
- `pixi run lint` - Run all formatters and linters
- `pixi run cargo-clippy` - Rust linting with strict warnings
- `pixi run cargo-fmt` - Rust formatting
- `pixi run ruff-lint` - Python linting
- `pixi run typecheck-python` - Python type checking with mypy

### Git Hooks
- `pixi run pre-commit-install` - Install all git hooks
- `pixi run pre-commit-install-minimal` - Install minimal hooks (no pre-push)

## Architecture

### Crate Structure
The project uses a modular Rust workspace:

- **`crates/pixi/`** - Main binary crate
- **`crates/pixi_cli/`** - CLI implementation and commands
- **`crates/pixi_core/`** - Core workspace/environment management
- **`crates/pixi_manifest/`** - Manifest parsing
- **`crates/pixi_global/`** - Global package installation
- **`crates/pixi_build_*/`** - Build system integration
- **`crates/pixi_*_spec/`** - Package specification handling

### Command Organization
CLI commands are in `crates/pixi_cli/src/` with individual modules for each command (add.rs, run.rs, etc.) and subcommand groups in subdirectories.

## Code Style

### Rust
- Use `cargo fmt` with standard formatting
- Use `fs-err` instead of `std::fs` (disallowed by clippy)
- Document with Rust docstrings
- Follow conventional commits: `<type>[scope]: <description>`

### Python
- Format with `ruff format` (100 char line length)
- Lint with `ruff check`
- Type check with `mypy`

### UI Colors
- Environment names: `style("env").magenta()`
- Feature names: `style("feature").cyan()`
- Task names: `style("task").blue()`

## Development Workflow

1. Make changes following existing patterns
2. Run `pixi run lint` for formatting/linting
3. Run `pixi run test-all-fast` for testing
4. Update docs if changing CLI interface: `pixi run generate-cli-docs`
5. Commit with conventional commit messages

## Dependencies

Built on the **rattler** ecosystem for conda package management, with heavy use of tokio for async operations and integration with **uv** for Python packages.