# Pixi Codebase Architecture

## Project Structure

### Top-Level Organization
- `crates/` - Rust workspace with modular crates
- `docs/` - Documentation source files
- `tests/` - Integration and Python tests
- `examples/` - Usage examples
- `scripts/` - Development and release scripts

### Core Crates Architecture

#### Main Entry Points
- `crates/pixi/` - Main binary crate, thin wrapper around pixi_cli
- `crates/pixi_cli/` - Primary CLI implementation and command handling

#### Core Libraries
- `crates/pixi_core/` - Core workspace and environment management
- `crates/pixi_manifest/` - Manifest file parsing and handling
- `crates/pixi_config/` - Configuration management
- `crates/pixi_task/` - Task execution system

#### Specialized Components
- `crates/pixi_global/` - Global package installation (like pipx/condax)
- `crates/pixi_build_*/` - Build system integration crates
- `crates/pixi_spec*/` - Package specification handling
- `crates/pixi_uv_conversions/` - Integration with uv Python package manager
- `crates/pypi_*/` - PyPI ecosystem integration

#### Utilities
- `crates/pixi_utils/` - Shared utilities
- `crates/pixi_progress/` - Progress reporting
- `crates/pixi_reporters/` - Output formatting and reporting
- `crates/fancy_display/` - Pretty printing utilities
- `crates/barrier_cell/` - Synchronization primitives

### CLI Command Structure
Commands are organized in `crates/pixi_cli/src/`:
- Individual command modules: `add.rs`, `run.rs`, `install.rs`, etc.
- Subcommand groups: `global/`, `workspace/`
- Shared functionality: `shared/`, `has_specs.rs`

### Dependencies
- Built on **rattler** ecosystem (conda package management)
- Heavy use of **tokio** for async operations
- **clap** for CLI parsing
- **serde** for serialization
- Integration with **uv** for Python package management

### Build Configuration
- Workspace-based Cargo setup with shared dependencies
- Multiple build profiles: debug, release, dist, profiling
- Feature flags for different functionality (self_update, native-tls, etc.)
- Cross-compilation support for multiple platforms