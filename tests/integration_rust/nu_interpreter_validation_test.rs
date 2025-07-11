//! Tests to verify that Nushell interpreter with -c option requires @ placeholder.
//!
//! This test ensures that when using the `nu` interpreter with the `-c` option
//! in array format, it must be followed by "@" to work correctly. This is a
//! specific requirement for Nushell where the `-c` flag expects the script
//! content to follow immediately as an argument.

use pixi::cli::cli_config::WorkspaceConfig;

use crate::common::PixiControl;

/// Test that nu with -c option works correctly with @ placeholder
#[tokio::test]
async fn test_nu_with_c_option_requires_at_placeholder() {
    let manifest = r#"
[project]
name = "nu-c-validation-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[dependencies]
nushell = ">=0.105.0"

# Correct usage: nu -c @ (should work)
[tasks.nu_c_correct]
cmd = "print 'Hello from nu -c with @ placeholder'"
interpreter = ["nu", "-c", "@"]

# Correct usage with additional flags: nu --log-level warn -c @
[tasks.nu_c_with_flags_correct]
cmd = "print 'Hello from nu -c with flags and @ placeholder'"
interpreter = ["nu", "--log-level", "warn", "-c", "@"]

# Alternative correct approach: use temporary file with -
[tasks.nu_file_correct]
cmd = "print 'Hello from nu with file approach'"
interpreter = ["nu", "-"]

# Test string format (also correct)
[tasks.nu_string_correct]
cmd = "print 'Hello from nu string format'"
interpreter = "nu {0}"
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    // Test correct usage: nu -c @
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["nu_c_correct".to_string()],
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
            .contains("Hello from nu -c with @ placeholder")
    );

    // Test correct usage with flags: nu --log-level warn -c @
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["nu_c_with_flags_correct".to_string()],
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
            .contains("Hello from nu -c with flags and @ placeholder")
    );

    // Test alternative correct approach: nu -
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["nu_file_correct".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains("Hello from nu with file approach"));

    // Test string format (also correct)
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["nu_string_correct".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains("Hello from nu string format"));
}

/// Test that demonstrates why nu -c without @ would be problematic
#[tokio::test]
async fn test_nu_c_without_at_placeholder_explanation() {
    // This test documents why "nu -c" without "@" would not work correctly.
    // When using "nu -c", Nushell expects the script content to be provided
    // as the next argument immediately after "-c".
    //
    // Incorrect patterns that would not work properly:
    // - ["nu", "-c", "-"]     // "-" is not valid after -c
    // - ["nu", "-c", "<"]     // "<" is not valid after -c
    // - ["nu", "-c"]          // Missing script content argument
    //
    // The "@" placeholder specifically tells the system to pass the script
    // content as an inline argument, which is exactly what "-c" expects.
    //
    // For demonstration, we'll test a manifest that uses the wrong approach
    // and verify it fails as expected.

    let manifest_with_incorrect_usage = r#"
[project]
name = "nu-c-incorrect-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[dependencies]
nushell = ">=0.105.0"

# This would be incorrect: nu -c with - instead of @
[tasks.nu_c_incorrect]
cmd = "print 'This should not work correctly'"
interpreter = ["nu", "-c", "-"]
"#;

    let pixi = PixiControl::from_manifest(manifest_with_incorrect_usage).unwrap();

    // This should fail or behave unexpectedly because "-" is not appropriate after "-c"
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["nu_c_incorrect".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    // The result should either fail or not work as expected
    // because "-c" expects the script content as an argument, not from stdin/file
    if let Ok(result) = result {
        // If it somehow succeeds, it likely didn't execute the intended command
        // The output should not contain our expected text
        assert_ne!(result.exit_code, 0);
    }
    // If it fails (which is more likely), that's the expected behavior
}

/// Test that the system can detect and handle problematic nu -c combinations
#[tokio::test]
async fn test_nu_c_with_problematic_placeholders() {
    // Test cases that should work but might be suboptimal or cause warnings
    let manifest_edge_cases = r#"
[project]
name = "nu-c-edge-cases-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[dependencies]
nushell = ">=0.105.0"

# Edge case: nu -c with stdin placeholder (should not work properly)
[tasks.nu_c_with_stdin]
cmd = "print 'This uses stdin placeholder incorrectly'"
interpreter = ["nu", "-c", "<"]

# Correct alternative for comparison
[tasks.nu_c_correct_alternative]
cmd = "print 'This uses @ placeholder correctly'"
interpreter = ["nu", "-c", "@"]
"#;

    let pixi = PixiControl::from_manifest(manifest_edge_cases).unwrap();

    // Test the correct alternative first to establish baseline
    let result_correct = pixi
        .run(pixi::cli::run::Args {
            task: vec!["nu_c_correct_alternative".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result_correct.exit_code, 0);
    assert!(
        result_correct
            .stdout
            .contains("This uses @ placeholder correctly")
    );

    // Test the problematic case - this should either fail or produce warnings
    let result_problematic = pixi
        .run(pixi::cli::run::Args {
            task: vec!["nu_c_with_stdin".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    // The result should indicate the problem
    if let Ok(result) = result_problematic {
        // If it somehow succeeds, it likely didn't work as intended
        // Check if there are warnings in stderr or if output is unexpected
        assert!(
            result.exit_code != 0
                || result.stderr.contains("Warning")
                || !result
                    .stdout
                    .contains("This uses stdin placeholder incorrectly"),
            "Expected failure or warning for nu -c with stdin placeholder"
        );
    }
    // If it fails, that's expected behavior for this incorrect usage
}

/// Test comprehensive nu interpreter patterns and best practices
#[tokio::test]
async fn test_nu_interpreter_comprehensive_patterns() {
    let manifest = r#"
[project]
name = "nu-patterns-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[dependencies]
nushell = ">=0.105.0"

# Pattern 1: Simple commands - use inline approach with @
[tasks.nu_simple_inline]
cmd = "print 'Simple inline command'"
interpreter = ["nu", "-c", "@"]

# Pattern 2: Complex scripts - use temporary file approach with -
[tasks.nu_complex_script]
cmd = '''
let data = [
    {name: "Alice", score: 95},
    {name: "Bob", score: 87},
    {name: "Charlie", score: 92}
]

$data
| where score > 90
| get name
| str join ", "
| $"High scorers: ($in)"
'''
interpreter = ["nu", "-"]

# Pattern 3: With logging flags - maintain @ after -c
[tasks.nu_with_logging]
cmd = "print 'Command with logging enabled'"
interpreter = ["nu", "--log-level", "debug", "-c", "@"]

# Pattern 4: String format for flexibility
[tasks.nu_string_format]
cmd = "print 'Using string format'"
interpreter = "nu {0}"

# Pattern 5: String format with additional flags
[tasks.nu_string_with_flags]
cmd = "print 'String format with flags'"
interpreter = "nu --log-level warn {0}"
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    // Test all patterns work correctly
    let test_cases = vec![
        ("nu_simple_inline", "Simple inline command"),
        ("nu_complex_script", "High scorers: Alice, Charlie"),
        ("nu_with_logging", "Command with logging enabled"),
        ("nu_string_format", "Using string format"),
        ("nu_string_with_flags", "String format with flags"),
    ];

    for (task_name, expected_output) in test_cases {
        let result = pixi
            .run(pixi::cli::run::Args {
                task: vec![task_name.to_string()],
                workspace_config: WorkspaceConfig {
                    manifest_path: None,
                },
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(result.exit_code, 0, "Task {} failed", task_name);
        assert!(
            result.stdout.contains(expected_output),
            "Task {} output '{}' does not contain expected '{}'",
            task_name,
            result.stdout,
            expected_output
        );
    }
}
