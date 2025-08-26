# Task Completion Checklist

## Before Committing Code

### 1. Code Quality
- [ ] Run `pixi run lint` to ensure all formatting and linting passes
- [ ] Run `pixi run test-all-fast` for fast test validation
- [ ] Run `pixi run cargo-clippy` if working on Rust code
- [ ] Run `pixi run typecheck-python` if working on Python code

### 2. Documentation
- [ ] Document new Rust code with docstrings
- [ ] Update CLI docs if changing command interface (`pixi run generate-cli-docs`)
- [ ] Update README.md or docs/ if interface changes are made
- [ ] Commit generated CLI documentation files

### 3. Testing
- [ ] Write tests for new functionality
- [ ] Run relevant test suites:
  - `pixi run test-fast` for unit tests
  - `pixi run test-integration-fast` for integration tests (requires debug build)
  - `pixi run test-specific-test <name>` for targeted testing
- [ ] Update snapshots if needed: `pixi run snapshot-update <test_name>`

### 4. Git Hooks and CI Preparation
- [ ] Install pre-commit hooks: `pixi run pre-commit-install`
- [ ] Ensure all automated checks pass
- [ ] Write conventional commit messages: `<type>[scope]: <description>`

## Build Verification
- [ ] Verify debug build: `pixi run build-debug`
- [ ] Verify release build: `pixi run build-release` (if needed)
- [ ] Test installation: `pixi run install` (Unix only)

## Breaking Changes
- [ ] Mark potential breaking changes with `BREAK:` comments
- [ ] Update relevant documentation
- [ ] Consider backward compatibility implications

## Performance Considerations
- [ ] Use `fs-err` instead of `std::fs` for file operations
- [ ] Follow clippy recommendations for performance
- [ ] Consider async/await patterns for I/O operations

## Final Checks
- [ ] All files formatted and linted
- [ ] All tests passing
- [ ] Documentation updated
- [ ] Commit message follows conventional format
- [ ] Ready for pull request review