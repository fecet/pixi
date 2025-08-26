# Pixi Project Overview

## Purpose
Pixi is a cross-platform, multi-language package manager and workflow tool built on the conda ecosystem. It provides developers with an exceptional experience similar to popular package managers like `cargo` or `yarn`, but for any language. It supports multiple languages including Python, C++, and R using Conda packages.

## Tech Stack
- **Primary Language**: Rust (100% written in Rust)
- **Built on**: rattler library (conda ecosystem foundation)
- **Supported Platforms**: Linux, Windows, macOS (including Apple Silicon)
- **Supported Languages**: Python, C++, R, and more via conda packages

## Key Features
- Multi-language package management via conda ecosystem
- Always includes up-to-date lock files
- Clean Cargo-like command-line interface
- Per-project and system-wide tool installation
- Cross-platform compatibility

## Development Status
- Ready for production
- Active development with regular releases
- Built using Pixi itself (self-hosting)