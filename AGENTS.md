# Repository Guidelines

## Project Structure & Module Organization
- Rust workspace in `crates/` (primary binary: `crates/pixi`).
- Tests in `tests/`: `integration_rust/`, `integration_python/`, and `wheel_tests/` plus data/scripts.
- Docs in `docs/` (MkDocs), schemas in `schema/`, examples in `examples/`, helper scripts in `scripts/`.
- Config highlights: `pixi.toml` (tasks/envs), `lefthook.yaml` (git hooks), `clippy.toml`, `.ruff.toml`, `mypy.ini`, `.dprint.jsonc`, `tbump.toml`.

## Build, Test, and Development Commands
- Setup: `pixi install` (creates the dev env).
- Build (debug/release): `pixi run build-debug` / `pixi run build-release`.
- Rust tests (fast/all): `pixi run test-fast` / `pixi run test-all-fast`.
- Python integration tests: `pixi run --environment=pytest test-integration-fast`.
- Lint/format (all fast): `pixi run --environment=lint lint-fast`.
- Install git hooks: `pixi run --environment=lint pre-commit-install`.
- Docs local server: `pixi run --environment=docs docs`.

## Coding Style & Naming Conventions
- Rust: toolchain `1.86.0`; run `cargo fmt` and `cargo clippy` (no warnings). Naming: crates/modules `snake_case`, types `UpperCamelCase`, consts `SCREAMING_SNAKE_CASE`.
- Python: `ruff format` + `ruff check --fix`; 4‑space indent; modules `snake_case`, classes `CapWords`, functions/vars `snake_case`. Type-check with `mypy` (strict for `tests/` and `scripts/`).
- Shell: format via `shfmt --indent=4`. TOML/YAML via `taplo`/`dprint`.

## Testing Guidelines
- Place Rust integration tests under `tests/integration_rust/` (use `nextest`).
- Place Python tests under `tests/integration_python/`; name files `test_*.py`. Use markers `slow`/`extra_slow` as needed (see `pytest.ini`).
- Useful commands: run a single test `pixi run --environment=pytest test-specific-test -- test_substring=<pattern>`.

## Commit & Pull Request Guidelines
- Follow Conventional Commits (enforced by `cliff.toml`). Examples:
  - `feat(cli): add task to build docs`
  - `fix(resolver): handle url normalization`
- PRs should include: clear description, linked issues, tests or docs updates, and passing CI. Run `lint-fast` and `test-all-fast` locally before requesting review.

## Security & Configuration Tips
- Never commit secrets; prefer env variables/files ignored by git.
- Cross‑platform builds use `pixi` tasks; avoid system‑specific paths.
- Version bumps done via `pixi run bump` (maintainers only); changelog via `pixi run --environment=docs bump-changelog`.

