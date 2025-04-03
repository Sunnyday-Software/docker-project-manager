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

---

## 📦 Project Structure

```sh
your-project-root/
├── dev/docker/         # Directory scanned for docker images checksums
├── .env                # Base environment file (default variables)
├── .env.local          # Optional overrides for your local settings
└── dpm                 # Docker Project Manager
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

### 2. Automated Docker Environment Management

Upon running, the utility automatically:

- Scans directories inside `dev/docker`, calculates their MD5 checksums, and
  injects them as environment variables.
  Every directory is a docker image.
- Merges `.env` and `.env.local` variables, providing flexibility and clear
  priority handling.
- Ensures necessary Docker socket mapping is correctly done depending on your
  OS (Linux/macOS/Windows).

---

### 3. Generated Docker Environment

Once executed, the utility generates a configuration file named `.env.docker`,
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

- Checks the existence and merges `.env` → `.env.local` → `.env.docker`.
- Computes MD5 checksums for directories under `dev/docker`.
- Detects Docker socket and ensures correct mapping.
- Executes Docker Compose commands transparently with pre-set correct
  environment.

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


Use Conventional Commit Specification for your commits
[https://www.conventionalcommits.org/en/v1.0.0/](Conventional Commits)