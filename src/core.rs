use std::collections::HashMap;
use std::error::Error;

/// Constants for file paths
pub const DEFAULT_INPUT_ENV: &str = ".env.docker";
pub const ENV_FILE: &str = ".env";
pub const ENV_LOCAL_FILE: &str = ".env.local";

/// Keys and default values for configurable variables
pub const DOCKER_DEV_PATH_KEY: &str = "DOCKER_DEV_PATH";
pub const DOCKER_DEV_PATH_DEFAULT_VALUE: &str = "./dev/docker";
pub const VERSIONS_FOLDER_KEY: &str = "VERSIONS_FOLDER";
pub const VERSIONS_FOLDER_DEFAULT_VALUE: &str = "dev/docker_versions";

/// Constants for Docker
pub const DOCKER_SOCKET_PATH: &str = "/var/run/docker.sock";
pub const DOCKER_DESKTOP_SOCKET_SUFFIX: &str = "/.docker/desktop/docker.sock";
pub const DOCKER_SOCKET_SUFFIX: &str = "/docker.sock";

/// Docker command arguments
pub const DOCKER_COMPOSE_ARGS: &[&str] =
  &["compose", "run", "--rm", "--no-deps", "-T"];
pub const DOCKER_MAKE_ARGS: &[&str] = &["make", "make"];

/// Environment variable names
pub const ENV_DOCKER_HOST_MAP: &str = "DOCKER_HOST_MAP";
pub const ENV_DOCKER_ENV_KEYS: &str = "DOCKER_ENV_KEYS";
pub const ENV_PROJECT_NAME: &str = "PROJECT_NAME";
pub const ENV_HOST_PROJECT_PATH: &str = "HOST_PROJECT_PATH";
pub const ENV_HOST_UID: &str = "HOST_UID";
pub const ENV_HOST_GID: &str = "HOST_GID";
pub const ENV_HOST_USER: &str = "HOST_USER";

/// Keys for versioning
pub const VERSION_KEY_MD5: &str = "md5";
pub const VERSION_KEY_MAJOR: &str = "v_major";
pub const VERSION_KEY_MINOR: &str = "v_minor";
pub const VERSION_KEY_PATCH: &str = "v_patch";
pub const VERSION_KEY_FULL: &str = "v_full_version";

/// Default values
pub const DEFAULT_PROJECT_NAME: &str = "NoName";

/// Prefixes and patterns
pub const MD5_PREFIX: &str = "MD5_";
pub const ENV_VAR_PATTERN: &str = r"\$\{(\w+)\}";

/// Special characters
pub const COMMENT_CHAR: char = '#';

/// Lengths
pub const MD5_SHORT_LENGTH: usize = 8;

/// Error messages
pub const ERROR_INVALID_PATH: &str = "Invalid path";
pub const ERROR_INVALID_DIRECTORY: &str = "Invalid directory";
pub const ERROR_MISSING_PARAMETERS: &str = "Missing parameters";
pub const ERROR_WRITE_ENV_REQUIRES_OUTPUT: &str =
  "Error: when --write-env is active, --output-env must be specified";
pub const ERROR_CANNOT_DETERMINE_HOME: &str =
  "Unable to determine current user's home directory";
pub const ERROR_CANNOT_DETERMINE_DOCKER_DIR: &str =
  "Cannot determine docker directory name";

/// Informational messages
pub const MSG_RUST_PROJECT_BUILDER: &str = "RUST Project builder";
pub const MSG_HOST_PROJECT_PATH: &str = "* host_project_path: {}";
pub const MSG_DOCKER_DEV_PATH: &str = "* docker_dev_path: {}";
pub const MSG_INPUT_ENV_SELECTED: &str = "Input .env file selected: {}";
pub const MSG_OUTPUT_ENV_SELECTED: &str = "Output .env file selected: {}";
pub const MSG_ADDITIONAL_ARGS: &str = "Additional arguments received: {:?}";
pub const MSG_UPDATING_VERSIONS: &str = "Updating versions in progress...";
pub const MSG_WRITING_ENV_FILE: &str = "Writing .env file in progress...";
pub const MSG_DOCKER_FOLDER_MAPPING: &str = "* docker folder {} -> {}";
pub const MSG_ENV_KEY_VALUE: &str = "* env key: {} = {}";
pub const MSG_DOCKER_SOCKET_MAPPING: &str = "Docker Socket mapping: {}";
pub const MSG_USING_DOCKER_HOST_MAP: &str =
  "Using DOCKER_HOST_MAP from .env file: {}";
pub const MSG_EXECUTING_COMMAND: &str = "Executing command: {:?}";
pub const MSG_VERSION_NO_UPDATE: &str =
  "{} updated, no version advancement needed.";
pub const MSG_VERSION_UPDATED: &str =
  "{} updated and version advancement performed: {}.{}.{}";
pub const MSG_DOCKER_COMMAND_FAILED: &str = "Docker command failed";

/// Configuration and operation messages
pub const MSG_CONFIG_PARSING: &str = "Parsing configuration parameters...";
pub const MSG_CONFIG_SET: &str = "Configuration set: {} = {}";
pub const MSG_OPERATIONS_DETECTED: &str = "Operations detected: {:?}";
pub const MSG_COMBINING_ENV_FILES: &str = "Combining environment files...";
pub const MSG_READING_ENV_FILE: &str = "Reading environment file: {}";
pub const MSG_ENV_FILE_NOT_FOUND: &str = "Environment file not found: {}";
pub const MSG_SCANNING_DOCKER_DIRS: &str = "Scanning Docker directories...";
pub const MSG_CALCULATING_MD5: &str = "Calculating MD5 for directory: {}";
pub const MSG_MD5_CALCULATED: &str = "MD5 calculated for {}: {}";
pub const MSG_ENV_VAR_ADDED: &str = "Environment variable added: {} = {}";
pub const MSG_ENV_VAR_UPDATED: &str = "Environment variable updated: {} = {}";
pub const MSG_EXECUTING_OPERATION: &str = "Executing operation: {}";

/// Warnings
pub const WARNING_DOCKER_HOST_MAP_IN_ENV: &str = "Warning: The 'DOCKER_HOST_MAP' variable is present in .env. It is recommended to move it to .env.local.";
pub const WARNING_PROJECT_NAME_MISSING: &str =
  "ERROR: The 'PROJECT_NAME' variable is not present in .env.";

/// Structure for dynamic runtime configuration
#[derive(Debug, Clone)]
pub struct Config {
  variables: HashMap<String, String>,
}

impl Default for Config {
  fn default() -> Self {
    let mut variables = HashMap::new();
    variables.insert(
      DOCKER_DEV_PATH_KEY.to_string(),
      DOCKER_DEV_PATH_DEFAULT_VALUE.to_string(),
    );
    variables.insert(
      VERSIONS_FOLDER_KEY.to_string(),
      VERSIONS_FOLDER_DEFAULT_VALUE.to_string(),
    );

    Self { variables }
  }
}

impl Config {
  /// Creates a new configuration with default values
  pub fn new() -> Self {
    Self::default()
  }

  /// Updates the configuration with a key-value pair
  pub fn set(&mut self, key: &str, value: &str) -> Result<(), String> {
    if self.variables.contains_key(key) {
      self.variables.insert(key.to_string(), value.to_string());
      Ok(())
    } else {
      Err(format!("Unknown configuration variable: {}", key))
    }
  }

  /// Gets the value of a configuration variable
  pub fn get(&self, key: &str) -> Option<&String> {
    self.variables.get(key)
  }

  /// Gets the path of the dev/docker directory
  pub fn docker_dev_path(&self) -> &String {
    self.variables.get(DOCKER_DEV_PATH_KEY).unwrap()
  }

  /// Gets the path of the directory for versioning files
  pub fn versions_folder(&self) -> &String {
    self.variables.get(VERSIONS_FOLDER_KEY).unwrap()
  }

  /// Gets all configured variables
  pub fn get_all_variables(&self) -> &HashMap<String, String> {
    &self.variables
  }

  /// Adds a new configuration variable with its default value
  pub fn add_variable(&mut self, key: &str, default_value: &str) {
    if !self.variables.contains_key(key) {
      self
        .variables
        .insert(key.to_string(), default_value.to_string());
    }
  }
}

/// Context structure that holds all data needed for command execution
#[derive(Debug)]
pub struct ExecutionContext {
  pub config: Config,
  pub input_env: String,
  pub output_env: Option<String>,
  pub args: Vec<String>,
  pub verbose: bool,
  pub host_project_path: Option<String>,
  pub docker_dev_path: Option<String>,
  pub env_vars: Option<HashMap<String, String>>,
  pub existing_env_vars: Option<HashMap<String, String>>,
  pub md5_values: Option<HashMap<String, String>>,
}

impl ExecutionContext {
  pub fn new(
    config: Config,
    input_env: String,
    output_env: Option<String>,
    args: Vec<String>,
    verbose: bool,
  ) -> Self {
    Self {
      config,
      input_env,
      output_env,
      args,
      verbose,
      host_project_path: None,
      docker_dev_path: None,
      env_vars: None,
      existing_env_vars: None,
      md5_values: None,
    }
  }
}

/// Trait that defines the interface for all commands
pub trait Command: std::fmt::Debug {
  /// Execute the command with the given context
  fn execute(
    &self,
    context: &mut ExecutionContext,
  ) -> Result<(), Box<dyn Error>>;

  /// Get the name of the command
  fn name(&self) -> &'static str;

  /// Get a display representation of the command
  fn display(&self) -> String;

  /// Check if this is a configuration command
  fn is_config_command(&self) -> bool {
    false
  }

  /// Get the command name this parser handles
  fn command_name() -> &'static str
  where
    Self: Sized;

  /// Try to parse a command from the given arguments
  /// Returns Some(Result) if this command can handle the parsing, None otherwise
  fn try_parse(
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Option<Result<Box<dyn Command>, String>>
  where
    Self: Sized;
}

/// Registry for managing command parsers
#[derive(Debug)]
pub struct CommandRegistry {
  parsers: Vec<
    fn(
      &str,
      &mut std::iter::Peekable<std::vec::IntoIter<String>>,
    ) -> Option<Result<Box<dyn Command>, String>>,
  >,
}

impl CommandRegistry {
  pub fn new() -> Self {
    Self {
      parsers: Vec::new(),
    }
  }

  pub fn register<T: Command + 'static>(&mut self) {
    self.parsers.push(T::try_parse);
  }

  pub fn parse_command(
    &self,
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Result<Box<dyn Command>, String> {
    for parser in &self.parsers {
      if let Some(result) = parser(command, args) {
        return result;
      }
    }
    Err(format!("Unknown command: {}", command))
  }
}
