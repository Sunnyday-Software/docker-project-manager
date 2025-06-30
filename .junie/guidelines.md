# Docker Project Manager (DPM) - Development Guidelines

## Project Overview

DPM is a Docker Project Manager written in Rust that provides a Lisp-based command interface for managing Docker
environments and project configurations. The project uses a custom Lisp interpreter to execute commands and manage
Docker project workflows.

## Build/Configuration Instructions

### Prerequisites

- Rust 1.85.0 or later (specified in Cargo.toml)
- Platform-specific build tools (automatically handled by build scripts)

### Cross-Platform Building

The project includes comprehensive build scripts for all major platforms:

#### Linux

```bash
chmod +x build-linux.sh
./build-linux.sh
```

- Builds both musl (static) and system-specific binaries
- Automatically detects Linux distribution
- Creates binaries in `build/` directory with appropriate naming

#### macOS

```bash
chmod +x build-macos.sh
./build-macos.sh
```

- Builds for both Intel (x86_64) and ARM64 (Apple Silicon)
- Creates universal binaries for macOS compatibility

#### Windows

```cmd
build-windows.bat
```

- Builds for Windows x86_64 with GNU toolchain
- Handles Windows-specific binary naming (.exe)

### Dependencies

Key dependencies (from Cargo.toml):

- `walkdir` (2.5.0) - Directory traversal
- `md-5` (0.10.6) - Hash generation for version management
- `regex` (1.11.1) - Pattern matching
- `dirs` (6.0.0) - Cross-platform directory operations
- `lexpr` (0.2.7) - Lisp expression parsing
- `emojis-rs` (0.1.3) - Emoji support for enhanced output
- `uzers` (0.12.1) - Unix user operations (Unix only)

## Testing Information

### Running Tests

```bash
cargo test
```

### Test Structure

Tests are organized in two main locations:

1. **Module-level tests**: Located in `src/lisp_interpreter.rs` under `#[cfg(test)]`
2. **Separate test modules**: Can be created as separate files (e.g., `src/test_example.rs`)

### Adding New Tests

#### For Lisp Commands

```rust
#[cfg(test)]
mod tests {
    use crate::lisp_interpreter::*;
    use crate::commands::*;

    #[test]
    fn test_your_command() {
        let mut registry = CommandRegistry::new();
        registry.register(YourCommand);
        let mut ctx = Context::new(registry);

        let result = evaluate_string("(your-command args)", &mut ctx).unwrap();
        assert_eq!(result, Value::Expected(expected_value));
    }
}
```

#### Test Module Registration

When creating new test files, add them to `src/main.rs`:

```rust
mod your_test_module;
```

### Example Test Execution

The project includes working examples:

- `test_multiply_command`: Tests arithmetic operations
- `test_concat_command`: Tests string concatenation
- `test_print_command`: Tests output formatting

## Code Style and Development Practices

### Formatting Configuration (rustfmt.toml)

- **Line width**: 80 characters
- **Indentation**: 2 spaces (no hard tabs)
- **Style edition**: 2024
- **Import organization**: Enabled (`reorder_imports = true`)
- **Line endings**: Unix style

### Apply Formatting

```bash
cargo fmt
```

### Linting and Security (deny.toml)

The project uses `cargo-deny` for:

- **Security advisories**: Monitors for known vulnerabilities
- **License compliance**: Allows specific licenses including GPL-3.0
- **Dependency management**: Warns about unknown registries/sources

### Run Security Checks

```bash
cargo deny check
```

### Allowed Licenses

- Apache-2.0, MIT, BSD variants
- GPL-3.0 (project license)
- MPL-2.0, ISC, Unlicense
- Unicode licenses, Zlib, WTFPL

## Project Architecture

### Core Components

#### 1. Lisp Interpreter (`src/lisp_interpreter.rs`)

- **Value enum**: Represents Lisp values (Int, Str, List, Nil)
- **Command trait**: Interface for all commands
- **CommandRegistry**: Manages command registration and lookup
- **Context**: Execution environment with variable storage
- **Evaluation engine**: Parses and executes Lisp expressions

##### Rust Standard Library Commands (`src/commands/rust/`)

The project includes a comprehensive set of commands that provide access to Rust's standard library functionality:

**Environment Commands (`env.rs`)**:

- `rust-env-current-dir`: Get current working directory
- `rust-env-current-exe`: Get path of current executable
- `rust-env-home-dir`: Get user's home directory
- `rust-env-var`: Get environment variable value
- `rust-env-vars`: List all environment variables

**Path Commands (`path.rs`)**:

- `rust-path-join`: Join path components
- `rust-path-parent`: Get parent directory
- `rust-path-filename`: Get filename component
- `rust-path-extension`: Get file extension
- `rust-path-exists`: Check if path exists
- `rust-path-is-dir`: Check if path is directory
- `rust-path-is-file`: Check if path is file

**Filesystem Commands (`fs.rs`)**:

- `rust-fs-read-to-string`: Read file contents as string
- `rust-fs-write`: Write string to file
- `rust-fs-create-dir`: Create directory
- `rust-fs-remove-file`: Remove file
- `rust-fs-copy`: Copy file

**Process Commands (`process.rs`)**:

- `rust-process-command`: Execute system command
- `rust-process-output`: Execute command and capture output

#### 3. Docker Integration (`src/docker.rs`)

- Docker version management
- Environment variable handling
- Command execution with Docker context

#### 4. Configuration Management (`src/core.rs`)

- Project configuration handling
- Environment variable management
- Alternative command system (traditional CLI)

### Adding New Commands

#### Struct-based Commands

```rust
pub struct YourCommand;

impl Command for YourCommand {
    fn execute(&self, args: Vec<Value>, ctx: &mut Context) -> Result<Value, String> {
        // Implementation
    }

    fn name(&self) -> &'static str { "your-command" }
    fn description(&self) -> &'static str { "Description" }
    // ... other required methods
}
```

#### Closure-based Commands

Commands can also be registered using closures with the `register_closure` method. The closure should accept a vector of
Values and a mutable Context reference, returning a Result with a Value or error String.

#### Enhanced Command Registration

The project also supports enhanced command registration using `register_closure_with_help_and_tag` which provides
comprehensive documentation, syntax examples, and categorization tags. This method is used extensively in the Rust
standard library commands to ensure consistent help documentation and proper command categorization.

### Module Organization

- `src/main.rs`: Entry point and command registration
- `src/lisp_interpreter.rs`: Core interpreter functionality
- `src/commands/`: Individual command implementations
    - `src/commands/rust/`: Rust standard library commands
        - `mod.rs`: Module exports and registration
        - `env.rs`: Environment variable commands
        - `path.rs`: Path manipulation commands
        - `fs.rs`: Filesystem operation commands
        - `process.rs`: Process execution commands
- `src/docker.rs`: Docker-specific operations
- `src/core.rs`: Configuration and alternative CLI
- `src/utils.rs`: Utility functions
- `src/file_ops.rs`: File system operations

## Development Workflow

### 1. Setup

```bash
git clone <repository>
cd docker-project-manager
cargo build
```

### 2. Development

- Follow rustfmt configuration (2-space indentation, 80-char lines)
- Add tests for new functionality
- Run `cargo test` before committing
- Use `cargo clippy` for additional linting

### 3. Testing New Features

- Create tests in appropriate modules
- Test both success and error cases
- Verify Lisp expression parsing and evaluation

### 4. Building for Release

Use platform-specific build scripts for distribution:

- Linux: `./build-linux.sh`
- macOS: `./build-macos.sh`
- Windows: `build-windows.bat`

## Debugging Tips

### Common Issues

1. **Import warnings**: The project has many unused imports - use `cargo fix` to clean up
2. **Command registration**: Ensure new commands are registered in `register_builtin_commands()`
3. **Lisp syntax**: Use proper S-expression syntax: `(command arg1 arg2)`

### Debug Output

Use the `DebugCommand` for runtime debugging:

```lisp
(debug "Debug message" variable)
```

### Rust Standard Library Command Examples

The new Rust commands provide powerful system integration capabilities:

```lisp
; Environment operations
(print (rust-env-current-dir))                    ; Get current directory
(print (rust-env-var "PATH"))                     ; Get environment variable

; Path operations  
(print (rust-path-join "/home" "user" "docs"))    ; Join path components
(print (rust-path-exists "."))                    ; Check if path exists

; File operations
(rust-fs-write "test.txt" "Hello World")          ; Write to file
(print (rust-fs-read-to-string "test.txt"))       ; Read file contents

; Process execution
(print (rust-process-output "echo" "Hello"))      ; Execute command with output
```

### Verbose Mode

Many functions support verbose output for troubleshooting Docker operations.

## Notes for Advanced Developers

- The project uses both a Lisp-based command system and a traditional CLI system
- Docker integration focuses on environment management and version control
- The Lisp interpreter is custom-built using the `lexpr` crate for parsing
- Cross-platform compatibility is a key design consideration
- Security and license compliance are enforced through `cargo-deny`
