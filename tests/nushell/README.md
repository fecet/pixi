# Nushell Interpreter Tests

This directory contains comprehensive tests for the Nushell interpreter functionality in Pixi tasks.

## Overview

These tests demonstrate and validate the interpreter feature that allows Pixi tasks to be executed using Nushell (`nu`) instead of the default shell. The tests cover various Nushell features and capabilities to ensure the interpreter integration works correctly.

## Test Structure

The tests are organized into several categories:

### Basic Operations
- **basic_echo_demo**: Simple echo command
- **basic_print_demo**: Basic print statement
- **string_ops_demo**: String manipulation and processing

### Data Processing
- **list_processing_demo**: Working with lists and arrays
- **table_demo**: Creating and manipulating tables
- **json_processing_demo**: JSON data parsing and filtering
- **csv_demo**: CSV data processing

### Advanced Features
- **function_demo**: Custom function definitions
- **conditional_demo**: Conditional logic and branching
- **loop_demo**: Iteration and loops
- **pipeline_demo**: Complex data pipelines
- **structured_data_demo**: Working with structured data

### System Integration
- **env_demo**: Environment variable handling
- **system_info_demo**: System information queries
- **file_operations_demo**: File system operations
- **utilities_demo**: Network and file system utilities

### Configuration and Error Handling
- **config_demo**: Nushell configuration examples
- **error_demo**: Error handling patterns
- **complex_script_demo**: Multi-line complex scripts

### Task Dependencies
- **setup_data_demo**: Data preparation task
- **process_data_demo**: Data processing (depends on setup)
- **cleanup_demo**: Cleanup operations (depends on process)

### Parameterized Tasks
- **parameterized_task_demo**: Tasks with arguments
- **working_dir_task_demo**: Tasks with custom working directories
- **env_task_demo**: Tasks with environment variables

## Running the Tests

To run individual tests:

```bash
# From the pixi root directory
cd tests/nushell
pixi run basic_print_demo
```

To run all tests:

```bash
# List all available tasks
pixi info

# Run specific test categories
pixi run table_demo
pixi run advanced_features_demo
pixi run cleanup_demo  # This will run the entire dependency chain
```

## Interpreter Configuration

All tasks in this test suite use the Nushell interpreter with the `-c` flag:

```toml
[tasks.example_task]
cmd = "print 'Hello from Nushell!'"
interpreter = ["nu", "-c"]
```

Some tasks use additional Nushell flags:

```toml
[tasks.config_demo]
cmd = "version | print"
interpreter = ["nu", "-c", "--no-config-file"]
```

## Requirements

- Nushell >= 0.95
- Pixi with interpreter support (>= 0.50.0)

## Testing Interpreter Implementation

These tests validate that the Pixi interpreter implementation correctly:

1. **Handles `-c` flag**: Passes script content as command-line arguments for `nu -c`
2. **Manages stdout/stderr**: Properly captures and displays output
3. **Supports dependencies**: Task dependency chains work with interpreters
4. **Handles errors**: Error messages are displayed when tasks fail
5. **Environment integration**: Environment variables and working directories work correctly
6. **Parameterization**: Task arguments are properly substituted

## Key Features Tested

- **Data manipulation**: Tables, lists, JSON, CSV processing
- **String operations**: Interpolation, formatting, manipulation
- **Control flow**: Conditionals, loops, error handling
- **System integration**: File operations, environment access
- **Custom functions**: User-defined commands and functions
- **Pipeline operations**: Complex data transformation pipelines
- **Configuration**: Nushell-specific flags and options

## Notes

- The interpreter implementation detects the `-c` flag and passes script content as arguments rather than via stdin
- All tasks are designed to be self-contained and can run independently (except for dependency chains)
- The test suite covers both simple and complex Nushell scenarios to ensure robust interpreter support
- Schema validation is included to ensure TOML format compliance

## Implementation Summary

This nushell test suite was successfully migrated from `temp_test/` to `tests/nushell/` and validates the enhanced interpreter functionality in Pixi v0.50.0.

### Key Accomplishments

1. **Enhanced Interpreter Implementation**: Successfully implemented support for both stdin-based interpreters (python, bash) and argument-based interpreters (nu -c, bash -c)

2. **Error Handling Improvements**: Fixed stdout/stderr display for failed tasks so users can see detailed error information

3. **Comprehensive Test Coverage**: Created 30+ test tasks covering all major Nushell features including:
   - Data processing and manipulation
   - Custom functions and control flow
   - System integration and file operations
   - Task dependencies and parameterization

4. **Integration Testing**: Added Rust integration tests that validate the interpreter functionality programmatically

5. **Documentation**: Comprehensive README with usage examples and test descriptions

### Technical Details

- **Interpreter Format**: Uses `["nu", "-c"]` for proper Nushell script execution
- **Version**: Tested with Nushell v0.105.1 from conda-forge
- **Platform Support**: Works across linux-64, win-64, osx-64, osx-arm64
- **Schema Compliance**: All tasks validate against the Pixi schema

The nushell interpreter integration is now fully functional and provides a robust foundation for shell-agnostic task execution in Pixi! 🚀