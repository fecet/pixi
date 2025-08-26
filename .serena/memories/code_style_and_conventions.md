# Code Style and Conventions

## Rust Code Style
- **Formatter**: `cargo fmt` with standard Rust formatting
- **Linter**: `cargo clippy` with strict warnings (`-- -D warnings`)
- **Edition**: Rust 2024 edition
- **Toolchain**: Rust >=1.86.0

### Clippy Configuration
- Custom disallowed methods: Standard `std::fs` functions are banned (use `fs-err` instead)
- Interior mutability exceptions for workspace types
- Allow unwrap in tests
- Strict clippy rules enforced in CI

### Error Handling
- Use `fs-err` instead of `std::fs` for better error messages
- Use `miette` for user-facing error reporting with fancy formatting
- Use `anyhow` for internal error handling

### Documentation
- Document new code using Rust docstrings
- CLI reference is auto-generated from code documentation
- Edit docs under `src/cli` instead of generated Markdown files

## Python Code Style
- **Formatter**: `ruff format` with 100 character line length
- **Linter**: `ruff check` with automatic fixes
- **Type Checker**: `mypy` for type checking
- **Target Version**: Python 3.12+
- **Quote Style**: Double quotes
- **Indent Style**: Spaces

## Other Languages
- **Shell**: Format with `shfmt` (4-space indent, simplify, binary-next-line)
- **TOML**: Format with `taplo`
- **YAML**: Format with `dprint`
- **Action Files**: Lint with `actionlint`

## Git Conventions
- **Commit Messages**: Use conventional commits format
  - `<type>[optional scope]: <description>`
  - Example: `feat(exec): add xxx option`
- **Breaking Changes**: Mark with `BREAK:` comments in code for future changes

## UI and Output Style
- Use `console::style` for colored output
- Color conventions:
  - `style("environment").magenta()` - Environment names
  - `style("feature").cyan()` - Feature names  
  - `style("task").blue()` - Task names
- Store common styles in `consts` module or use `.fancy_display()` methods

## Code Organization
- Modular crate architecture with clear separation of concerns
- Prefer editing existing files over creating new ones
- Use workspace dependencies for shared crates
- Follow existing patterns for new components

## Testing Conventions
- Unit tests with `cargo test` and `cargo nextest`
- Integration tests in `tests/` directory
- Python tests use `pytest` with `inline-snapshot` for snapshot testing
- Use `pixi run test-specific-test` for targeted test execution