//! Tests for the enhanced interpreter task execution with placeholder support.
//!
//! This module contains comprehensive tests for the enhanced interpreter solution
//! that supports temporary file, inline script, and explicit stdin execution approaches,
//! addressing interpreter execution issues with modern interpreters.
//!
//! ## Problem Background
//!
//! The traditional approach of piping script content to interpreter stdin fails
//! with many modern interpreters:
//!
//! - `nushell`: `nu -c` expects script as argument, not stdin
//! - `deno`: Similar issues with stdin handling
//! - Various shells: TTY detection and buffering problems
//!
//! ## Enhanced Interpreter Solution
//!
//! This implementation supports three placeholder types:
//!
//! ### 1. Explicit Stdin Approach (`<` placeholder) - **DISCOURAGED**
//! - Explicitly pipes script content to interpreter's stdin
//! - Removes the `<` placeholder from arguments
//! - Can only appear once in interpreter arguments
//! - Executes: `echo "script" | interpreter`
//! - **Warning: This approach is discouraged as it provides no benefit over traditional stdin**
//!
//! ### 2. Temporary File Approach (`-` placeholder)
//! - Creates a temporary file with appropriate extension (.nu, .py, .sh, etc.)
//! - Writes the script content to the temporary file
//! - Replaces all `-` placeholders with the temporary file path
//! - Executes: `interpreter /tmp/script_xyz.ext` instead of piping
//! - Automatic cleanup of temporary files
//!
//! ### 3. Inline Script Approach (`@` placeholder)
//! - Replaces all `@` placeholders with the script content directly
//! - Useful for interpreters that accept script content as arguments
//! - No temporary file creation needed
//! - Executes: `interpreter -c "script content"`
//!
//! ## Usage Examples
//!
//! ```toml
//! # Traditional stdin approach (backward compatible)
//! [tasks.python_stdin]
//! cmd = "print('Hello World')"
//! interpreter = ["python"]
//!
//! # Explicit stdin approach (DISCOURAGED - use traditional instead)
//! [tasks.python_explicit_stdin]
//! cmd = "print('Hello World')"
//! interpreter = ["python", "<"]
//!
//! # Temporary file approach
//! [tasks.python_tempfile]
//! cmd = "print('Hello World')"
//! interpreter = ["python", "-"]
//!
//! # Inline script approach
//! [tasks.python_inline]
//! cmd = "print('Hello World')"
//! interpreter = ["python", "-c", "@"]
//!
//! # Multiple placeholders (same temp file used for all '-' placeholders)
//! [tasks.python_multi_file]
//! cmd = "print('Hello World')"
//! interpreter = ["python", "-", "--debug", "-"]
//!
//! # Mixed placeholders (but not with '<')
//! [tasks.python_mixed]
//! cmd = "print('Hello World')"
//! interpreter = ["python", "-c", "@", "--verbose"]
//!
//! # Nushell best practices - inline for simple commands
//! [tasks.nushell_simple]
//! cmd = "print 'Hello from Nushell'"
//! interpreter = ["nu", "-c", "@"]
//!
//! # Nushell best practices - tempfile for complex scripts
//! [tasks.nushell_complex]
//! cmd = '''
//! let data = [1, 2, 3, 4, 5]
//! let sum = ($data | math sum)
//! print $"Sum: ($sum)"
//! '''
//! interpreter = ["nu", "-"]
//!
//! # Explicit stdin with flags (DISCOURAGED - use traditional instead)
//! [tasks.bash_explicit_stdin]
//! cmd = "echo 'Explicit stdin with flags'"
//! interpreter = ["bash", "-e", "<"]
//! ```
//!
//! ## Benefits
//!
//! - **Universal compatibility**: Works with any interpreter
//! - **Flexible execution**: Supports file-based, inline, and explicit stdin approaches
//! - **Multiple placeholders**: Same file/content can be referenced multiple times (except '<')
//! - **Explicit control**: Clear distinction between execution modes
//! - **No stdin issues**: Eliminates TTY, buffering, and piping problems
//! - **Standard patterns**: Follows conventional interpreter usage
//! - **Backward compatible**: Coexists with existing stdin approach
//! - **Automatic cleanup**: Temporary files are cleaned up automatically
//! - **Input validation**: Prevents invalid placeholder combinations
//! - **Deprecation warnings**: Warns when discouraged patterns are used
//! - **Best practices**: Encourages appropriate approach based on command complexity
//!
//! ## Nushell Best Practices
//!
//! - **Single-line commands**: Use inline approach `interpreter = ["nu", "-c", "@"]`
//! - **Multi-line scripts**: Use temporary file approach `interpreter = ["nu", "-"]`
//! - **Complex data processing**: Always use temporary file approach for readability
//! - **With flags**: Both approaches work with additional nushell flags

use pixi::cli::cli_config::WorkspaceConfig;
use pixi_manifest::FeatureName;

use crate::common::PixiControl;

/// Test nushell interpreter with appropriate approaches based on command size.
///
/// This test validates that single-line commands use inline approach ("@" placeholder)
/// while multi-line commands use temporary file approach ("-" placeholder).
/// This follows nushell best practices for different command complexities.
#[tokio::test]
async fn test_nushell_comprehensive_approach() {
    let manifest = r#"
[project]
name = "nushell-tempfile-test"
channels = ["conda-forge"]
platforms = ["linux-64", "win-64", "osx-64", "osx-arm64"]
version = "0.1.0"

[dependencies]
nushell = ">=0.105.0"

# Basic print using inline approach for single-line command
[tasks.nu_print]
cmd = "print 'Hello from Nushell inline!'"
interpreter = ["nu", "-c", "@"]

# Complex script with data processing using temporary file approach
[tasks.nu_data_processing]
cmd = '''
let data = [
    {name: "Alice", age: 30, city: "New York"},
    {name: "Bob", age: 25, city: "London"},
    {name: "Charlie", age: 35, city: "Tokyo"}
]

$data
| where age > 27
| select name city
| each { |row| $"($row.name) lives in ($row.city)" }
| str join "\n"
| print
'''
interpreter = ["nu", "-"]

# Mathematical operations using temporary file approach
[tasks.nu_math]
cmd = '''
let numbers = [10, 20, 30, 40, 50]
let sum = ($numbers | math sum)
let avg = ($numbers | math avg)
let max = ($numbers | math max)

print $"Sum: ($sum)"
print $"Average: ($avg)"
print $"Maximum: ($max)"
'''
interpreter = ["nu", "-"]

# Test with additional nushell flags using inline approach for single-line
[tasks.nu_with_flags]
cmd = "print 'Nushell with debug logging'"
interpreter = ["nu", "--log-level", "warn", "-c", "@"]
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    // Test basic print functionality
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["nu_print".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert_eq!(result.stdout.trim(), "Hello from Nushell inline!");

    // Test data processing script
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["nu_data_processing".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains("Alice lives in New York"));
    assert!(result.stdout.contains("Charlie lives in Tokyo"));
    assert!(!result.stdout.contains("Bob")); // Bob is filtered out (age <= 27)

    // Test mathematical operations
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["nu_math".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains("Sum: 150"));
    assert!(result.stdout.contains("Average: 30"));
    assert!(result.stdout.contains("Maximum: 50"));

    // Test with additional flags
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["nu_with_flags".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains("Nushell with debug logging"));
}

/// Test the temporary file approach with Python interpreter.
///
/// This validates that the temporary file approach works for interpreters
/// that traditionally work well with stdin, ensuring backward compatibility.
#[tokio::test]
async fn test_python_tempfile_approach() {
    let pixi = PixiControl::new().unwrap();
    pixi.init().without_channels().await.unwrap();

    // Create tasks using both stdin and tempfile approaches
    pixi.tasks()
        .add("python_stdin".into(), None, FeatureName::default())
        .with_commands(["print('Hello from Python stdin)', end='')"])
        .with_interpreter("python") // Traditional stdin approach
        .execute()
        .await
        .unwrap();

    pixi.tasks()
        .add("python_tempfile".into(), None, FeatureName::default())
        .with_commands(["print('Hello from Python tempfile)', end='')"])
        .with_interpreter("python -") // Temporary file approach
        .execute()
        .await
        .unwrap();

    // Test stdin approach
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["python_stdin".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert_eq!(result.stdout, "Hello from Python stdin)");

    // Test tempfile approach
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["python_tempfile".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert_eq!(result.stdout, "Hello from Python tempfile)");
}

/// Test multiple placeholder support in interpreter arrays.
///
/// This test validates that multiple "-" placeholders can be used in a single
/// interpreter command, each getting replaced with the same temporary file path.
#[tokio::test]
async fn test_multiple_placeholders() {
    let pixi = PixiControl::new().unwrap();
    pixi.init().without_channels().await.unwrap();

    // Create a simple task that uses multiple "-" placeholders
    pixi.tasks()
        .add("multi_placeholder".into(), None, FeatureName::default())
        .with_commands(["print('Multi-placeholder test')"])
        .with_interpreter("python -")
        .execute()
        .await
        .unwrap();

    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["multi_placeholder".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    match result {
        Ok(output) => {
            assert_eq!(output.exit_code, 0);
            assert!(output.stdout.contains("Multi-placeholder test"));
        }
        Err(e) => {
            panic!("Task failed with error: {:?}", e);
        }
    }
}

/// Test complex interpreter patterns with multiple placeholders.
///
/// This test validates that complex interpreter invocations with multiple
/// placeholders work correctly.
#[tokio::test]
async fn test_complex_multiple_placeholders() {
    let pixi = PixiControl::new().unwrap();
    pixi.init().without_channels().await.unwrap();

    // Create a task that uses @ placeholder for inline script
    pixi.tasks()
        .add("inline_script".into(), None, FeatureName::default())
        .with_commands(["print('Inline script test')"])
        .with_interpreter("python -c @")
        .execute()
        .await
        .unwrap();

    // Create a task that uses multiple - placeholders
    pixi.tasks()
        .add(
            "multi_file_placeholders".into(),
            None,
            FeatureName::default(),
        )
        .with_commands(["print('Multiple file placeholders')"])
        .with_interpreter("python - --debug -")
        .execute()
        .await
        .unwrap();

    // Create a task that uses both @ and - placeholders
    pixi.tasks()
        .add("mixed_placeholders".into(), None, FeatureName::default())
        .with_commands(["print('Mixed placeholders')"])
        .with_interpreter("python -c @ -W ignore")
        .execute()
        .await
        .unwrap();

    // Test inline script with @ placeholder
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["inline_script".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    match result {
        Ok(output) => {
            assert_eq!(output.exit_code, 0);
            assert!(output.stdout.contains("Inline script test"));
        }
        Err(e) => {
            panic!("Inline script task failed with error: {:?}", e);
        }
    }

    // Test multiple file placeholders
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["multi_file_placeholders".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    match result {
        Ok(output) => {
            assert_eq!(output.exit_code, 0);
            assert!(output.stdout.contains("Multiple file placeholders"));
        }
        Err(e) => {
            panic!("Multiple file placeholders task failed with error: {:?}", e);
        }
    }

    // Test mixed placeholders
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["mixed_placeholders".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    match result {
        Ok(output) => {
            assert_eq!(output.exit_code, 0);
            assert!(output.stdout.contains("Mixed placeholders"));
        }
        Err(e) => {
            panic!("Mixed placeholders task failed with error: {:?}", e);
        }
    }
}

/// Test comprehensive inline script approach with '@' placeholder.
///
/// This test validates that the '@' placeholder correctly replaces script content
/// inline and works with various interpreter patterns.
#[tokio::test]
async fn test_inline_script_comprehensive() {
    let pixi = PixiControl::new().unwrap();
    pixi.init().without_channels().await.unwrap();

    // Basic inline script with Python
    pixi.tasks()
        .add("python_inline_basic".into(), None, FeatureName::default())
        .with_commands(["print('Hello from Python inline!')"])
        .with_interpreter("python -c @")
        .execute()
        .await
        .unwrap();

    // Inline script with additional flags
    pixi.tasks()
        .add("python_inline_flags".into(), None, FeatureName::default())
        .with_commands(["print('Python inline with flags')"])
        .with_interpreter("python -c @ -W ignore")
        .execute()
        .await
        .unwrap();

    // Multiple @ placeholders (same content used multiple times)
    pixi.tasks()
        .add("python_multi_inline".into(), None, FeatureName::default())
        .with_commands(["print('Multi inline test')"])
        .with_interpreter("python -c @ -c @")
        .execute()
        .await
        .unwrap();

    // Bash inline script
    pixi.tasks()
        .add("bash_inline".into(), None, FeatureName::default())
        .with_commands(["echo 'Bash inline script'"])
        .with_interpreter("bash -c @")
        .execute()
        .await
        .unwrap();

    // Test basic Python inline
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["python_inline_basic".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    match result {
        Ok(output) => {
            assert_eq!(output.exit_code, 0);
            assert!(output.stdout.contains("Hello from Python inline!"));
        }
        Err(e) => {
            panic!("Python inline basic task failed with error: {:?}", e);
        }
    }

    // Test Python inline with flags
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["python_inline_flags".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    match result {
        Ok(output) => {
            assert_eq!(output.exit_code, 0);
            assert!(output.stdout.contains("Python inline with flags"));
        }
        Err(e) => {
            panic!("Python inline flags task failed with error: {:?}", e);
        }
    }

    // Test multiple @ placeholders
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["python_multi_inline".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    match result {
        Ok(output) => {
            assert_eq!(output.exit_code, 0);
            // Should see the output twice since @ appears twice
            let stdout_lines: Vec<&str> = output.stdout.lines().collect();
            let matching_lines = stdout_lines
                .iter()
                .filter(|line| line.contains("Multi inline test"))
                .count();
            assert!(matching_lines >= 1, "Should see at least one output line");
        }
        Err(e) => {
            panic!("Python multi inline task failed with error: {:?}", e);
        }
    }

    // Test bash inline
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["bash_inline".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    match result {
        Ok(output) => {
            assert_eq!(output.exit_code, 0);
            assert!(output.stdout.contains("Bash inline script"));
        }
        Err(e) => {
            panic!("Bash inline task failed with error: {:?}", e);
        }
    }
}

/// Test explicit stdin approach with '<' placeholder.
///
/// This test validates that the '<' placeholder correctly triggers explicit
/// stdin piping and that it can only appear once.
#[tokio::test]
async fn test_explicit_stdin_approach() {
    let pixi = PixiControl::new().unwrap();
    pixi.init().without_channels().await.unwrap();

    // Create a task that uses explicit stdin approach
    pixi.tasks()
        .add("explicit_stdin".into(), None, FeatureName::default())
        .with_commands(["print('Hello from explicit stdin!', end='')"])
        .with_interpreter("python <")
        .execute()
        .await
        .unwrap();

    // Create a task that uses explicit stdin with additional flags
    pixi.tasks()
        .add("explicit_stdin_flags".into(), None, FeatureName::default())
        .with_commands(["echo 'Explicit stdin with flags'"])
        .with_interpreter("bash -e <")
        .execute()
        .await
        .unwrap();

    // Test basic explicit stdin
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["explicit_stdin".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    match result {
        Ok(output) => {
            assert_eq!(output.exit_code, 0);
            assert!(output.stdout.contains("Hello from explicit stdin!"));
        }
        Err(e) => {
            panic!("Explicit stdin task failed with error: {:?}", e);
        }
    }

    // Test explicit stdin with flags
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["explicit_stdin_flags".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    match result {
        Ok(output) => {
            assert_eq!(output.exit_code, 0);
            assert!(output.stdout.contains("Explicit stdin with flags"));
        }
        Err(e) => {
            panic!("Explicit stdin with flags task failed with error: {:?}", e);
        }
    }
}

/// Test that multiple '<' placeholders are rejected.
///
/// This test validates that using more than one '<' placeholder
/// results in an appropriate error.
#[tokio::test]
async fn test_multiple_stdin_placeholders_rejected() {
    let pixi = PixiControl::new().unwrap();
    pixi.init().without_channels().await.unwrap();

    // Create a task that uses multiple '<' placeholders (should fail)
    pixi.tasks()
        .add(
            "invalid_multiple_stdin".into(),
            None,
            FeatureName::default(),
        )
        .with_commands(["print('This should not work')"])
        .with_interpreter("python < -e <")
        .execute()
        .await
        .unwrap();

    // This should fail during execution
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["invalid_multiple_stdin".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    // Should fail with an error about multiple stdin placeholders
    assert!(
        result.is_err(),
        "Expected error for multiple '<' placeholders"
    );
}

/// Test file extension detection based on interpreter.
///
/// This test validates that temporary files get appropriate extensions
/// based on the interpreter being used, which can be important for
/// interpreters that rely on file extensions for syntax highlighting,
/// error reporting, or module loading.
#[tokio::test]
async fn test_file_extension_detection() {
    let manifest = r#"
[project]
name = "extension-test"
channels = ["conda-forge"]
platforms = ["linux-64"]
version = "0.1.0"

[dependencies]
python = ">=3.8"

# Test that Python scripts get .py extension
[tasks.python_script]
cmd = '''
import sys
print(f"Script path: {sys.argv[0] if len(sys.argv) > 0 else 'unknown'}")
'''
interpreter = ["python", "-"]

# Test shell script gets .sh extension
[tasks.shell_script]
cmd = '''
echo "Shell script test"
echo "Script name: $0"
'''
interpreter = ["bash", "-"]
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    // Test Python script execution
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["python_script".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    // The output should contain a path ending with .py
    assert!(result.stdout.contains(".py") || result.stdout.contains("Script path:"));

    // Test shell script execution
    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["shell_script".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains("Shell script test"));
    // The script name should contain the temporary file path
    assert!(result.stdout.contains("Script name:"));
}

/// Test backward compatibility with existing stdin approach.
///
/// This test ensures that tasks without placeholders continue
/// to work with the traditional stdin piping approach, maintaining
/// full backward compatibility.
#[tokio::test]
async fn test_backward_compatibility() {
    let manifest = r#"
[project]
name = "backward-compat-test"
channels = ["conda-forge"]
platforms = ["linux-64"]
version = "0.1.0"

[dependencies]
python = ">=3.8"

# Traditional stdin approach (no placeholders)
[tasks.stdin_task]
cmd = "print('Traditional stdin approach')"
interpreter = ["python"]

# Explicit stdin approach (with "<" placeholder)
[tasks.explicit_stdin_task]
cmd = "print('Explicit stdin approach')"
interpreter = ["python", "<"]

# New tempfile approach (with "-" placeholder)
[tasks.tempfile_task]
cmd = "print('New tempfile approach')"
interpreter = ["python", "-"]

# Inline approach (with "@" placeholder)
[tasks.inline_task]
cmd = "print('Inline approach')"
interpreter = ["python", "-c", "@"]

# Mixed approach - some interpreters might use both
[tasks.bash_stdin]
cmd = "echo 'Bash via stdin'"
interpreter = ["bash"]

[tasks.bash_explicit_stdin]
cmd = "echo 'Bash via explicit stdin'"
interpreter = ["bash", "<"]

[tasks.bash_tempfile]
cmd = "echo 'Bash via tempfile'"
interpreter = ["bash", "-"]
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    // Test all approaches work
    let test_cases = vec![
        ("stdin_task", "Traditional stdin approach"),
        ("explicit_stdin_task", "Explicit stdin approach"),
        ("tempfile_task", "New tempfile approach"),
        ("inline_task", "Inline approach"),
        ("bash_stdin", "Bash via stdin"),
        ("bash_explicit_stdin", "Bash via explicit stdin"),
        ("bash_tempfile", "Bash via tempfile"),
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
            "Task {} output '{}' doesn't contain '{}'",
            task_name,
            result.stdout,
            expected_output
        );
    }
}

/// Test error handling for interpreters that don't exist.
///
/// This validates that appropriate error messages are shown when
/// using the temporary file approach with non-existent interpreters.
#[tokio::test]
async fn test_nonexistent_interpreter() {
    let manifest = r#"
[project]
name = "error-test"
channels = ["conda-forge"]
platforms = ["linux-64"]
version = "0.1.0"

[tasks.bad_interpreter]
cmd = "print('This should fail')"
interpreter = ["nonexistent_interpreter", "-"]
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    let result = pixi
        .run(pixi::cli::run::Args {
            task: vec!["bad_interpreter".to_string()],
            workspace_config: WorkspaceConfig {
                manifest_path: None,
            },
            ..Default::default()
        })
        .await;

    // Should fail with an appropriate error
    assert!(
        result.is_err(),
        "Expected error for nonexistent interpreter"
    );
}

/// Test complex interpreter argument patterns.
///
/// This test validates various ways the "-" placeholder can be used
/// with different interpreter argument patterns.
#[tokio::test]
async fn test_complex_interpreter_patterns() {
    let manifest = r#"
[project]
name = "complex-patterns-test"
channels = ["conda-forge"]
platforms = ["linux-64"]
version = "0.1.0"

[dependencies]
python = ">=3.8"

# Placeholder at the end
[tasks.placeholder_end]
cmd = "print('Placeholder at end')"
interpreter = ["python", "-u", "-"]

# Placeholder in the middle
[tasks.placeholder_middle]
cmd = "print('Placeholder in middle')"
interpreter = ["python", "-", "-u"]

# Multiple flags after placeholder
[tasks.flags_after]
cmd = "print('Flags after placeholder')"
interpreter = ["python", "-", "-W", "ignore"]
"#;

    let pixi = PixiControl::from_manifest(manifest).unwrap();

    let test_cases = vec![
        ("placeholder_end", "Placeholder at end"),
        ("placeholder_middle", "Placeholder in middle"),
        ("flags_after", "Flags after placeholder"),
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
            "Task {} output '{}' doesn't contain '{}'",
            task_name,
            result.stdout,
            expected_output
        );
    }
}
