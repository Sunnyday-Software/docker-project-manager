# Docker Project Manager (DPM)

[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/Compatible%20with-Docker-blue?logo=docker&logoColor=white)](https://www.docker.com/)

A Docker Project Manager written in Rust that provides a **Lisp-based command interface** for managing Docker environments and project configurations. The project uses a custom Lisp interpreter to execute commands and manage Docker project workflows.

---

## 🚀 Features Overview

- ✅ **Lisp-based Command Interface:**  
  Custom Lisp interpreter with S-expression syntax for powerful command composition and scripting capabilities.

- ✅ **Comprehensive Rust Standard Library Integration:**  
  Built-in commands for environment variables, filesystem operations, path manipulation, and process execution.

- ✅ **Docker Environment Management:**  
  Specialized commands for Docker integration, version management, and environment variable handling.

- ✅ **Cross-platform Compatibility:**  
  Supports Linux, macOS, and Windows with platform-specific optimizations and build scripts.

- ✅ **Extensible Command System:**  
  Easy-to-extend architecture for adding new commands through both struct-based and closure-based implementations.

- ✅ **Advanced Configuration Management:**  
  Project configuration handling with both Lisp-based and traditional CLI interfaces.

---

## 📦 Project Structure

```sh
docker-project-manager/
├── src/
│   ├── main.rs                    # Entry point and command registration
│   ├── lisp_interpreter.rs        # Core Lisp interpreter functionality
│   ├── commands/                  # Command implementations
│   │   ├── rust/                  # Rust standard library commands
│   │   │   ├── env.rs            # Environment variable commands
│   │   │   ├── fs.rs             # Filesystem operation commands
│   │   │   ├── path.rs           # Path manipulation commands
│   │   │   └── process.rs        # Process execution commands
│   │   ├── app/                  # Application-specific commands
│   │   └── core/                 # Core system commands
│   ├── docker.rs                 # Docker-specific operations
│   ├── core.rs                   # Configuration and alternative CLI
│   └── utils.rs                  # Utility functions
├── build-linux.sh                # Linux build script
├── build-macos.sh                # macOS build script
├── build-windows.bat             # Windows build script
├── Cargo.toml                    # Rust project configuration
├── rustfmt.toml                  # Code formatting configuration
├── deny.toml                     # Security and license compliance
└── dpm                           # Docker Project Manager executable
```

---

## 🛠️ Usage Instructions

### 1. Lisp Command Interface

DPM provides a powerful Lisp-based command interface using S-expression syntax. Commands are executed using parentheses notation:

```lisp
(command-name arg1 arg2 ...)
```

### 2. Running Commands

Start the DPM interpreter and execute Lisp commands:

```bash
./dpm
```

### 3. Available Command Categories

#### Environment Commands
```lisp
; Get current working directory
(rust-env-current-dir)

; Get environment variable value
(rust-env-var "PATH")

; List all environment variables
(rust-env-vars)

; Get user's home directory
(rust-env-home-dir)
```

#### Filesystem Commands
```lisp
; Read file contents as string
(rust-fs-read-to-string "config.txt")

; Write string to file
(rust-fs-write "output.txt" "Hello World")

; Create directory
(rust-fs-create-dir "new-folder")

; Check if path exists
(rust-path-exists ".")

; Copy file
(rust-fs-copy "source.txt" "destination.txt")
```

#### Path Manipulation Commands
```lisp
; Join path components
(rust-path-join "/home" "user" "documents")

; Get parent directory
(rust-path-parent "/home/user/file.txt")

; Get filename component
(rust-path-filename "/home/user/file.txt")

; Get file extension
(rust-path-extension "document.pdf")

; Check if path is directory
(rust-path-is-dir "/home/user")

; Check if path is file
(rust-path-is-file "/home/user/file.txt")
```

#### Process Execution Commands
```lisp
; Execute system command
(rust-process-command "ls" "-la")

; Execute command and capture output
(rust-process-output "echo" "Hello World")
```

### 4. Basic Lisp Operations

DPM supports standard Lisp operations for data manipulation:

```lisp
; Arithmetic operations
(+ 1 2 3)          ; Addition: 6
(* 4 5)            ; Multiplication: 20

; String operations
(concat "Hello" " " "World")  ; String concatenation

; Print output
(print "Debug message")

; Variable operations (if supported)
(debug "Debug info" variable-name)
```

---

## 🔧 Building and Installation

### Prerequisites

- Rust 1.88.0 or later (specified in Cargo.toml)
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

### Development Build
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

---

## ⚙️ Architecture Overview

### Core Components

1. **Lisp Interpreter** (`src/lisp_interpreter.rs`)
   - Custom Lisp expression parser using `lexpr` crate
   - Value system supporting Int, Str, List, and Nil types
   - Command registry and execution context
   - Variable storage and scope management

2. **Command System** (`src/commands/`)
   - Extensible command architecture with trait-based design
   - Rust standard library integration commands
   - Docker-specific operation commands
   - Application and core system commands

3. **Docker Integration** (`src/docker.rs`)
   - Docker version management
   - Environment variable handling
   - Command execution with Docker context

4. **Configuration Management** (`src/core.rs`)
   - Project configuration handling
   - Alternative traditional CLI interface
   - Cross-platform compatibility layer

### Key Dependencies

- `lexpr` (0.2.7) - Lisp expression parsing
- `walkdir` (2.5.0) - Directory traversal
- `md-5` (0.10.6) - Hash generation for version management
- `regex` (1.11.1) - Pattern matching
- `dirs` (6.0.0) - Cross-platform directory operations
- `emojis-rs` (0.1.3) - Enhanced output formatting
- `uzers` (0.12.1) - Unix user operations (Unix only)

---

## 📖 Prerequisites

- Rust 1.88.0 or later for building from source
- Docker & Docker Compose (optional, only needed for Docker-specific commands)

---

## 🌐 Supported Platforms

- Linux (with musl and system-specific builds)
- macOS (Intel x86_64 and ARM64 Apple Silicon)
- Windows (x86_64 with GNU toolchain)

---

## 🔧 Development Guidelines

### Code Style

The project follows strict formatting guidelines:
- **Line width**: 80 characters
- **Indentation**: 2 spaces (no hard tabs)
- **Style edition**: 2024
- **Import organization**: Enabled

Apply formatting:
```bash
cargo fmt
```

### Security and License Compliance

The project uses `cargo-deny` for security and license compliance:
```bash
cargo deny check
```

### Commit Message Conventions

This project enforces **Conventional Commits** specification for all commit messages to ensure consistent versioning and automated releases through semantic-release.

#### Required Format
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

#### Valid Commit Types
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools
- `perf`: A code change that improves performance
- `ci`: Changes to CI configuration files and scripts
- `build`: Changes that affect the build system or external dependencies
- `revert`: Reverts a previous commit

#### Examples
```bash
# Good commit messages
git commit -m "feat: add Docker container management commands"
git commit -m "fix: resolve path resolution issue on Windows"
git commit -m "docs: update installation instructions"
git commit -m "chore: update dependencies to latest versions"

# Breaking changes
git commit -m "feat!: remove deprecated CLI interface"
git commit -m "fix: correct API endpoint URL

BREAKING CHANGE: The API endpoint has changed from /v1 to /v2"
```

#### Local Validation
You can validate commit messages locally using:
```bash
# Install dependencies
npm install

# Validate the last commit
npm run commitlint-ci

# Validate commit message from file
npm run commitlint
```

#### Automated Validation
All commits and pull requests are automatically validated by GitHub Actions. Non-compliant commit messages will block:
- Direct pushes to main branch
- Pull request merges
- Release creation

For more information, see the [Conventional Commits specification](https://www.conventionalcommits.org/).

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
}
```

#### Closure-based Commands
Commands can also be registered using closures with `register_closure` or `register_closure_with_help_and_tag` for enhanced documentation.

---

## 🤝 Contributions & Feedback

Feel free to contribute or raise issues via our GitHub Repository.

### Development Workflow
1. Fork the repository
2. Create a feature branch
3. Follow the code style guidelines (`cargo fmt`)
4. Add tests for new functionality (`cargo test`)
5. Run security checks (`cargo clippy` and `cargo deny check`)
6. Submit a pull request

Use Conventional Commit Specification for your commits:
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
