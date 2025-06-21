# Docker Project Manager

[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/Compatible%20with-Docker-blue?logo=docker&logoColor=white)](https://www.docker.com/)

A lightweight Rust utility designed to simplify Docker integration within your
development workflow, leveraging dynamic
environment management, cross-platform compatibility, and automated build
triggers.

---

## 🚀 Features Overview

- ✅ **Automatic `.env` & `.env.local` Management:**  
  Easy, structured loading and merging of environment variables for different
  environments and host-specific overrides.

- ✅ **Directory-based MD5 Checksums:**  
  Detect directory changes through MD5 checksums, automating Docker rebuild
  triggers upon configuration changes.

- ✅ **Cross-platform Compatibility:**  
  Automatically handles Docker socket (`docker.sock`) mapping for Linux, macOS,
  and Windows.

- ✅ **Dynamic UID & GID Management (Unix-based):**  
  Automatically fetches current UID/GID, ensuring file permission consistency
  across host and container.

- ✅ **Semantic Versioning:**  
  Tracks changes to Docker components using MD5 checksums and automatically 
  updates version numbers (MAJOR.MINOR.PATCH) when changes are detected.

- ✅ **Command-line Interface:**  
  Flexible CLI with options for reading/writing environment files, updating 
  versions, and executing Docker commands with the appropriate environment.

---

## 📦 Project Structure

```sh
your-project-root/
├── dev/docker/           # Directory scanned for docker images checksums
├── dev/docker_versions/  # Directory for storing component version information
├── .env                  # Base environment file (default variables)
├── .env.local            # Optional overrides for your local settings
├── .env.docker           # Generated environment file for Docker Compose
└── dpm                   # Docker Project Manager executable
```

---

## 🛠️ Usage Instructions

### 1. Setup Your Environment Files

Prepare your default `.env` file in the project root:

```bash
PROJECT_NAME=my-awesome-project
#DOCKER_HOST_MAP will be handled automatically; add here only if necessary
```

Optional local overrides can be set in `.env.local`:

```bash
DOCKER_HOST_MAP=/var/run/docker.sock:/var/run/docker.sock
```

> **Recommended practice:**  
> Only use `.env.local` to store sensitive data or settings specific to the
> local environment.

### 2. Command-Line Interface

The Docker Project Manager provides a flexible command-line interface with the following options:

```bash
dpm [OPTIONS] [-- DOCKER_ARGS...]
```

Options:
- `-i, --input-env <FILE>` - File .env to read (default: .env.docker)
- `-o, --output-env <FILE>` - File .env to write (required with --write-env)
- `-w, --write-env` - Enable writing of the .env file
- `-u, --update-versions` - Enable updating component versions
- `-r, --run` - Enable execution of Docker command
- `[DOCKER_ARGS...]` - Additional arguments to pass to Docker command

Examples:
```bash
# Generate .env.docker file and update versions
dpm -i .env.dev -o .env.docker -w -u

# Generate .env.docker file, update versions, and run Docker build
dpm -i .env.dev -o .env.docker -w -u -r build test

# Just run Docker command using existing .env.docker
dpm -i .env.docker -r compose up -d
```

### 3. Automated Docker Environment Management

Upon running with the `-w` option, the utility automatically:

- Scans directories inside `dev/docker`, calculates their MD5 checksums, and
  injects them as environment variables.
  Every directory is a docker image.
- Merges `.env` and `.env.local` variables, providing flexibility and clear
  priority handling.
- Ensures necessary Docker socket mapping is correctly done depending on your
  OS (Linux/macOS/Windows).

### 4. Version Management

When the `-u` option is used, the utility:

- Compares the current MD5 checksums with previously stored values
- If changes are detected, increments the PATCH version number
- Stores the updated version information in the `dev/docker_versions` directory
- Each component maintains its own version history

### 5. Generated Docker Environment

When executed with the `-w` option, the utility generates a configuration file (specified by `-o`),
used internally by Docker Compose:

Example `.env.docker`:

```bash
PROJECT_NAME=my-awesome-project
HOST_PROJECT_PATH=/home/username/my-awesome-project
MD5_MYCONFIGDIR=f3e45fe9
HOST_UID=1000
HOST_GID=1000
DOCKER_HOST_MAP=/var/run/docker.sock:/var/run/docker.sock
```

---

## ⚙️ Under the Hood (What Happens Specifically?)

Here's the execution sequence for clarity:

1. **Environment File Processing**:
   - Reads and merges variables from `.env` → `.env.local` → input file (specified by `-i`).
   - Expands any environment variables in the values.

2. **MD5 Checksum Calculation**:
   - Scans all directories under `dev/docker`.
   - Calculates MD5 checksums for each directory by hashing all files.
   - Creates environment variables in the format `MD5_DIRECTORYNAME=checksum`.

3. **System Configuration Detection**:
   - Detects the operating system and configures platform-specific settings.
   - On Unix systems, fetches current UID/GID and username.
   - Detects Docker socket location based on the platform.

4. **Version Management** (when `-u` is specified):
   - Compares current MD5 checksums with previously stored values.
   - If changes are detected, increments the PATCH version number.
   - Updates version files in the `dev/docker_versions` directory.

5. **Environment File Generation** (when `-w` is specified):
   - Combines all variables from previous steps.
   - Writes the complete environment to the output file (specified by `-o`).

6. **Docker Command Execution** (when `-r` is specified):
   - Sets up the Docker command with all environment variables.
   - Configures Docker socket mapping based on the platform.
   - Executes the specified Docker command with all arguments.

---

## 📖 Prerequisites

- Docker & Docker Compose installed

---

## 🌐 Supported Platforms

- Linux
- macOS
- Windows

---

## 🤝 Contributions & Feedback

Feel free to contribute or raise issues via our GitHub Repository.


Use Conventional Commit Specification for your commits:
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
