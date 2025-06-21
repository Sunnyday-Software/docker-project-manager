/// CLI information
pub const CLI_AUTHOR: &str = "Diego Cattelan";
pub const CLI_ABOUT: &str =
  "Docker Project Manager - Automated Docker pipeline management";

/// CLI help texts for arguments
pub const CLI_INPUT_ENV_HELP: &str = "Input .env file to read";
pub const CLI_INPUT_ENV_LONG_HELP: &str = "Specifies the .env file to use as base for configurations.\n\
This file should contain the main environment variables of the project.\n\
If a .env.local file exists, its variables will override those of this file.\n\
The content will be integrated with variables calculated automatically by the tool.\n\n\
Examples:\n\
  -i .env.development  # For development environment\n\
  -i .env.production   # For production environment\n\
  -i .env.testing      # For automated testing";

pub const CLI_OUTPUT_ENV_HELP: &str =
  "Output .env file to write (required with --write-env)";
pub const CLI_OUTPUT_ENV_LONG_HELP: &str = "Specifies the destination file for processed configurations.\n\
This file will contain the complete combination of:\n\
• Variables from the specified input file\n\
• Variables from .env.local (if present, with precedence)\n\
• Variables calculated automatically (MD5 hashes, paths, etc.)\n\
• Variables derived from the current project\n\n\
WARNING: The existing file will be overwritten without confirmation.\n\n\
Examples:\n\
  -o .env.docker       # Standard file for Docker Compose\n\
  -o .env.final        # Final file for deployment\n\
  -o .env.generated    # Automatically generated file";

pub const CLI_WRITE_ENV_HELP: &str = "Enable .env file writing";
pub const CLI_WRITE_ENV_LONG_HELP: &str = "Activates the output .env file generation phase.\n\
The file will contain all variables processed by the pipeline:\n\
• Variables read from input files\n\
• MD5 hashes calculated for each Docker directory\n\
• Project paths and configurations\n\
• System variables (UID, GID, etc.)\n\n\
IMPORTANT: Requires --output-env to be specified.\n\n\
Example: -w -o .env.docker";

pub const CLI_UPDATE_VERSIONS_HELP: &str = "Enable version updates";
pub const CLI_UPDATE_VERSIONS_LONG_HELP: &str = "Activates the automatic versioning system based on MD5 hashes.\n\
For each directory in dev/docker:\n\
• Calculates the current MD5 hash\n\
• Compares with the stored hash\n\
• If different, increments the PATCH version\n\
• Saves the new hash and version\n\n\
Versions follow the semantic format MAJOR.MINOR.PATCH.\n\
Versioning files are saved in dev/docker_versions/.\n\n\
Example: -u";

pub const CLI_RUN_HELP: &str = "Enable Docker command execution";
pub const CLI_RUN_LONG_HELP: &str = "Activates Docker command execution with processed configurations.\n\
The executed command will be: docker compose run --rm --no-deps make make [args...]\n\n\
Automatic configurations:\n\
• Appropriate Docker socket mapping for the system\n\
• All calculated environment variables\n\
• Support for custom DOCKER_HOST_MAP\n\
• Cross-platform management (Linux, macOS, Windows)\n\n\
Additional arguments are passed to the Docker command.\n\n\
Examples:\n\
  -r build test        # docker ... make make build test\n\
  -r compose up -d     # docker ... make make compose up -d";

pub const CLI_CFG_HELP: &str = "Custom configuration (format: key=value)";
pub const CLI_CFG_LONG_HELP: &str = "Allows customizing configuration variables at runtime.\n\
Format: --cfg VARIABLE=VALUE\n\n\
Supported variables:\n\
• DOCKER_DEV_PATH: Path to the directory containing Docker files\n\
  Default: './dev/docker'\n\
  Example: --cfg DOCKER_DEV_PATH=./custom/docker\n\n\
• VERSIONS_FOLDER: Directory to save versioning files\n\
  Default: 'dev/docker_versions'\n\
  Example: --cfg VERSIONS_FOLDER=custom/versions\n\n\
Usage examples:\n\
  dpm --cfg DOCKER_DEV_PATH=./custom/docker -w -u\n\
  dpm --cfg VERSIONS_FOLDER=./versions --cfg DOCKER_DEV_PATH=./docker -r build\n\
  dpm --cfg DOCKER_DEV_PATH=/opt/project/docker -w -o .env.custom";

pub const CLI_VERBOSE_HELP: &str = "Enable verbose output";

pub const CLI_ARGS_HELP: &str = "Additional arguments for the Docker command";
pub const CLI_ARGS_LONG_HELP: &str = "Additional arguments that will be passed to the Docker command.\n\
These arguments are added after 'make make' in the final command.\n\n\
Usage examples:\n\
  dpm -r build --no-cache service1    # docker ... build --no-cache service1\n\
  dpm -r compose up -d --force-recreate # docker ... compose up -d --force-recreate\n\
  dpm -r run --rm test-container bash  # docker ... run --rm test-container bash\n\n\
TIP:\n\
Use -- to clearly separate dpm flags from Docker arguments:\n\
  dpm -w -u -r -- compose up -d";
