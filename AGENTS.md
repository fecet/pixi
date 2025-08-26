- You MUST NOT manually edit `CHANGELOG.md` outside the release workflow; it is generated.

## Tooling

- PREFER to use `pixi run` to execute tasks instead of running executables coming from your system.

## Testing

- PREFER inline snapshots over substring assertions when testing rendered output.
- New tests MUST follow local test conventions. Check existing coverage before adding tests.
- PREFER integration tests for user-visible behavior; use unit tests for isolated logic.
- PREFER running specific tests during development; run relevant broader checks before finishing.

## Dependencies

- You MUST NOT work around limitations or bugs in rattler or rattler-build. Fix them upstream instead.

## Linting

- `pixi run typos` MUST pass. Suppress deliberate misspellings with `# spellchecker:off` / `# spellchecker:on` or their `//` equivalents instead of adding them to `extend-words`.
- You MUST NOT bypass pre-commit hooks with `--no-verify`.

## Rust

- Use enums with variant data to represent mutually exclusive states instead of contradictory flags or dependent `Option` fields.

## Pull requests

- Pull requests MUST follow [CONTRIBUTING.md](CONTRIBUTING.md) and the [pull request template](.github/pull_request_template.md), including the human-written description requirement.

## Error messages

- Authored diagnostic messages MUST start with a lowercase letter and end without a trailing dot. This does not apply to quoted upstream errors.

## Project Structure & Module Organization
- Rust workspace in `crates/` (primary binary: `crates/pixi`).
- Tests: Rust integration tests in `crates/pixi/tests/integration_rust/`; `tests/integration_python/` and `tests/wheel_tests/` plus data/scripts in `tests/`.
- Docs in `docs/` (MkDocs), schemas in `schema/`, examples in `examples/`, helper scripts in `scripts/`.
- Config highlights: `pixi.toml` (tasks/envs), `lefthook.yaml` (git hooks), `clippy.toml`, `pyproject.toml` (ruff, pytest), `.dprint.jsonc`.

## Build, Test, and Development Commands
Each task below lives in exactly one environment of `pixi.toml`, so `pixi run <task>` picks it.
- Setup: `pixi install` (creates the dev env).
- Build (debug/release): `pixi run build-debug` / `pixi run build-release`.
- Rust tests (fast/all): `pixi run test-fast` / `pixi run test-all-fast`.
- Python integration tests: `pixi run test-integration-fast`.
- Lint/format (all fast): `pixi run lint-fast`.
- Install git hooks: `pixi run pre-commit-install`.
- Docs local server: `pixi run docs`.

## Coding Style & Naming Conventions
- Rust: toolchain pinned in `rust-toolchain`; run `cargo fmt` and `cargo clippy` (no warnings). Naming: crates/modules `snake_case`, types `UpperCamelCase`, consts `SCREAMING_SNAKE_CASE`.
- Python: `ruff format` + `ruff check --fix`; 4‑space indent; modules `snake_case`, classes `CapWords`, functions/vars `snake_case`. Type-check with `pixi run typecheck-python`.
- Shell: format via `shfmt --indent=4`. TOML/YAML via `tombi`/`dprint`.

## Testing Guidelines
- Place Rust integration tests under `crates/pixi/tests/integration_rust/` (use `nextest`).
- Place Python tests under `tests/integration_python/`; name files `test_*.py`. Use markers `slow`/`extra_slow` as needed (see `pyproject.toml`).
- Useful commands: run a single test `pixi run test-specific-test -- test_substring=<pattern>`.

## Commit & Pull Request Guidelines
- Follow Conventional Commits (enforced by `cliff.toml`). Examples:
  - `feat(cli): add task to build docs`
  - `fix(resolver): handle url normalization`
- PRs should include: clear description, linked issues, tests or docs updates, and passing CI. Run `lint-fast` and `test-all-fast` locally before requesting review.

## Security & Configuration Tips
- Never commit secrets; prefer env variables/files ignored by git.
- Cross‑platform builds use `pixi` tasks; avoid system‑specific paths.
