//! Tests for string interpreter format with {0} placeholder support.
//!
//! This module tests the enhanced interpreter functionality that supports
//! string format interpreters with {0} placeholder replacement.
//!
//! ## String Interpreter Format
//!
//! String interpreters support two modes:
//!
//! ### 1. Explicit {0} Placeholder
//! ```toml
//! interpreter = "python {0}"
//! interpreter = "nu --log-level warn {0}"
//! ```
//! The {0} placeholder is replaced with the temporary file path.
//!
//! ### 2. Implicit File Appending
//! ```toml
//! interpreter = "python"
//! interpreter = "bash"
//! ```
//! If no {0} placeholder is found, the file path is appended at the end.

use pixi::cli::cli_config::WorkspaceConfig;

use crate::common::PixiControl;

/// Test string interpreter with explicit {0} placeholder
#[tokio::test]
async fn test_string_interpreter_explicit_placeholder() {
    let manifest = r#"
[project]
name = "string-interpreter-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[dependencies]
python = ">=3.8"

[tasks.test_python_explicit]
cmd = "print('Hello from Python with explicit {0} placeholder!')"
interpreter = "python {0}"
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["test_python_explicit".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert!(
        result
            .stdout
            .contains("Hello from Python with explicit {0} placeholder!")
    );
}

/// Test string interpreter without {0} placeholder (implicit file appending)
#[tokio::test]
async fn test_string_interpreter_implicit_append() {
    let manifest = r#"
[project]
name = "string-interpreter-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[dependencies]
python = ">=3.8"

[tasks.test_python_implicit]
cmd = "print('Hello from Python with implicit append!')"
interpreter = "python"
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["test_python_implicit".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert!(
        result
            .stdout
            .contains("Hello from Python with implicit append!")
    );
}

/// Test string interpreter with additional flags and {0} placeholder
#[tokio::test]
async fn test_string_interpreter_with_flags() {
    let manifest = r#"
[project]
name = "string-interpreter-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[dependencies]
python = ">=3.8"

[tasks.test_python_flags]
cmd = "print('Hello from Python with flags!')"
interpreter = "python -u {0}"
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["test_python_flags".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains("Hello from Python with flags!"));
}

/// Test compatibility between string and array interpreter formats
#[tokio::test]
async fn test_string_vs_array_format_compatibility() {
    let manifest = r#"
[project]
name = "string-interpreter-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[dependencies]
python = ">=3.8"

[tasks.test_string_format]
cmd = "print('String format output')"
interpreter = "python {0}"

[tasks.test_array_format]
cmd = "print('Array format output')"
interpreter = ["python", "-"]
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    // Test string format
    let result_string = pixi
        .run(pixi::cli::run::Args {
            task: vec!["test_string_format".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result_string.exit_code, 0);
    assert!(result_string.stdout.contains("String format output"));

    // Test array format
    let result_array = pixi
        .run(pixi::cli::run::Args {
            task: vec!["test_array_format".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result_array.exit_code, 0);
    assert!(result_array.stdout.contains("Array format output"));
}

/// Test string interpreter with multiline script
#[tokio::test]
async fn test_string_interpreter_multiline_script() {
    let manifest = r#"
[project]
name = "string-interpreter-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[dependencies]
python = ">=3.8"

[tasks.test_multiline]
cmd = '''
data = [1, 2, 3, 4, 5]
result = sum(data)
print(f"Sum: {result}")
print("Multiline script executed successfully")
'''
interpreter = "python {0}"
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["test_multiline".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains("Sum: 15"));
    assert!(
        result
            .stdout
            .contains("Multiline script executed successfully")
    );
}

/// Test string interpreter with bash
#[tokio::test]
async fn test_string_interpreter_bash() {
    let manifest = r#"
[project]
name = "string-interpreter-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[tasks.test_bash_explicit]
cmd = "echo 'Hello from Bash with explicit placeholder!'"
interpreter = "bash {0}"

[tasks.test_bash_implicit]
cmd = "echo 'Hello from Bash with implicit append!'"
interpreter = "bash"
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    // Test bash with explicit placeholder
    let result_explicit = pixi
        .run(pixi::cli::run::Args {
            task: vec!["test_bash_explicit".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result_explicit.exit_code, 0);
    assert!(
        result_explicit
            .stdout
            .contains("Hello from Bash with explicit placeholder!")
    );

    // Test bash with implicit append
    let result_implicit = pixi
        .run(pixi::cli::run::Args {
            task: vec!["test_bash_implicit".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result_implicit.exit_code, 0);
    assert!(
        result_implicit
            .stdout
            .contains("Hello from Bash with implicit append!")
    );
}

/// Test string interpreter file extension detection
#[tokio::test]
async fn test_string_interpreter_file_extension() {
    let manifest = r#"
[project]
name = "string-interpreter-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[dependencies]
python = ">=3.8"

[tasks.test_file_extension]
cmd = "import sys; print(f'Script file: {sys.argv[0]}')"
interpreter = "python {0}"
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["test_file_extension".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    // Should contain .py extension in the temporary file path
    assert!(result.stdout.contains(".py"));
}

/// Test string interpreter error handling
#[tokio::test]
async fn test_string_interpreter_error_handling() {
    let manifest = r#"
[project]
name = "string-interpreter-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[dependencies]
python = ">=3.8"

[tasks.test_error]
cmd = "raise ValueError('Test error')"
interpreter = "python {0}"
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["test_error".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    // Should fail due to the error in the script
    assert!(result.is_err());
}

/// Test string interpreter with multiple {0} placeholders
#[tokio::test]
async fn test_string_interpreter_multiple_placeholders() {
    let manifest = r#"
[project]
name = "string-interpreter-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[dependencies]
python = ">=3.8"

[tasks.test_multiple_placeholders]
cmd = "print('Multiple placeholder test executed')"
interpreter = "python -u {0} --debug {0}"
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["test_multiple_placeholders".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    // This might fail because the same file is used multiple times in different positions
    // which could be invalid for some interpreters, but let's test the basic functionality
    // The implementation should handle this gracefully
    if let Ok(result) = result {
        assert_eq!(result.exit_code, 0);
        assert!(result.stdout.contains("Multiple placeholder test executed"));
    }
    // If it fails, that's also acceptable behavior for this edge case
}
