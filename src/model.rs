// model.rs - Global constants and variables for Docker Project Manager

/// Constants for file paths
pub const DEFAULT_INPUT_ENV: &str = ".env.docker";
pub const ENV_FILE: &str = ".env";
pub const ENV_LOCAL_FILE: &str = ".env.local";

/// Keys and default values for configurable variables
pub const DOCKER_DEV_PATH_KEY: &str = "DOCKER_DEV_PATH";
pub const DOCKER_DEV_PATH_DEFAULT_VALUE: &str = "./dev/docker";
pub const VERSIONS_FOLDER_KEY: &str = "VERSIONS_FOLDER";
pub const VERSIONS_FOLDER_DEFAULT_VALUE: &str = "dev/docker_versions";

/// Structure for dynamic runtime configuration
#[derive(Debug, Clone)]
pub struct Config {
  variables: std::collections::HashMap<String, String>,
}

impl Default for Config {
  fn default() -> Self {
    let mut variables = std::collections::HashMap::new();
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
  pub fn get_all_variables(
    &self,
  ) -> &std::collections::HashMap<String, String> {
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

/// Constants for Docker
pub const DOCKER_SOCKET_PATH: &str = "/var/run/docker.sock";
pub const DOCKER_DESKTOP_SOCKET_SUFFIX: &str = "/.docker/desktop/docker.sock";
pub const DOCKER_SOCKET_SUFFIX: &str = "/docker.sock";

/// Docker command arguments
pub const DOCKER_COMPOSE_ARGS: &[&str] =
  &["compose", "run", "--rm", "--no-deps"];
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
  pub env_vars: Option<std::collections::HashMap<String, String>>,
  pub existing_env_vars: Option<std::collections::HashMap<String, String>>,
  pub md5_values: Option<std::collections::HashMap<String, String>>,
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

/// Command trait following the Command pattern
pub trait Command: std::fmt::Debug {
  /// Executes the command with the given context
  fn execute(&self, context: &mut ExecutionContext) -> Result<(), Box<dyn std::error::Error>>;

  /// Returns the name of the command for logging purposes
  fn name(&self) -> &'static str;

  /// Returns a display string for the command including parameters
  fn display(&self) -> String;

  /// Returns true if this is a configuration command that should be executed first
  fn is_config_command(&self) -> bool {
    false
  }
}

/// Configuration command implementation
#[derive(Debug, Clone)]
pub struct ConfigCommand {
  pub key: String,
  pub value: String,
}

impl ConfigCommand {
  pub fn new(key: String, value: String) -> Self {
    Self { key, value }
  }
}

impl Command for ConfigCommand {
  fn execute(&self, context: &mut ExecutionContext) -> Result<(), Box<dyn std::error::Error>> {
    if context.verbose {
      println!("{}", MSG_EXECUTING_OPERATION.replace("{}", self.name()));
      println!("{}", MSG_CONFIG_PARSING);
    }

    if let Err(e) = context.config.set(&self.key, &self.value) {
      eprintln!("Configuration error: {}", e);
      return Err(e.into());
    }

    if context.verbose {
      println!("Configuration set: {} = {}", self.key, self.value);
    }

    Ok(())
  }

  fn name(&self) -> &'static str {
    "cfg"
  }

  fn display(&self) -> String {
    format!("cfg({}={})", self.key, self.value)
  }

  fn is_config_command(&self) -> bool {
    true
  }
}

/// Write environment command implementation
#[derive(Debug, Clone)]
pub struct WriteEnvCommand;

impl Command for WriteEnvCommand {
  fn execute(&self, context: &mut ExecutionContext) -> Result<(), Box<dyn std::error::Error>> {
    if context.verbose {
      println!("{}", MSG_EXECUTING_OPERATION.replace("{}", self.name()));
    }

    let existing_env_vars = context.existing_env_vars.as_ref()
      .ok_or("Environment variables not initialized")?;
    let output_env = context.output_env.as_ref()
      .ok_or("Output environment file not specified")?;

    crate::file_ops::write_env_to_file(output_env, existing_env_vars, context.verbose)?;
    Ok(())
  }

  fn name(&self) -> &'static str {
    "write_env"
  }

  fn display(&self) -> String {
    "write_env".to_string()
  }
}

/// Update versions command implementation
#[derive(Debug, Clone)]
pub struct UpdateVersionsCommand;

impl Command for UpdateVersionsCommand {
  fn execute(&self, context: &mut ExecutionContext) -> Result<(), Box<dyn std::error::Error>> {
    if context.verbose {
      println!("{}", MSG_EXECUTING_OPERATION.replace("{}", self.name()));
    }

    let md5_values = context.md5_values.as_ref()
      .ok_or("MD5 values not calculated")?;

    crate::utils::update_versions(md5_values, context.config.versions_folder(), context.verbose)?;
    Ok(())
  }

  fn name(&self) -> &'static str {
    "update_versions"
  }

  fn display(&self) -> String {
    "update_versions".to_string()
  }
}

/// Run command implementation
#[derive(Debug, Clone)]
pub struct RunCommand;

impl Command for RunCommand {
  fn execute(&self, context: &mut ExecutionContext) -> Result<(), Box<dyn std::error::Error>> {
    if context.verbose {
      println!("{}", MSG_EXECUTING_OPERATION.replace("{}", self.name()));
    }

    let mut env_vars = context.env_vars.as_ref()
      .ok_or("Environment variables not initialized")?
      .clone();
    let existing_env_vars = context.existing_env_vars.as_ref()
      .ok_or("Existing environment variables not initialized")?;

    // Missing environment variables present in .env are added before each run
    for (key, value) in existing_env_vars.clone() {
      if !env_vars.contains_key(&key) {
        if context.verbose {
          println!(
            "{}",
            MSG_ENV_VAR_ADDED.replace("{}", &key).replace("{}", &value)
          );
        }
        env_vars.insert(key, value);
      }
    }

    crate::docker::execute_docker_command(&env_vars, existing_env_vars, &context.args, context.verbose)?;
    Ok(())
  }

  fn name(&self) -> &'static str {
    "run"
  }

  fn display(&self) -> String {
    "run".to_string()
  }
}

/// Operation types that can be performed by the application
#[derive(Debug, Clone)]
pub enum Operation {
  /// Configuration operation with key=value pair
  Config { key: String, value: String },
  /// Write environment variables to output file
  WriteEnv,
  /// Update version information
  UpdateVersions,
  /// Execute Docker command with arguments
  Run,
}

impl Operation {
  /// Returns the name of the operation for logging purposes
  pub fn name(&self) -> &'static str {
    match self {
      Operation::Config { .. } => "cfg",
      Operation::WriteEnv => "write_env",
      Operation::UpdateVersions => "update_versions",
      Operation::Run => "run",
    }
  }

  /// Returns a display string for the operation including parameters
  pub fn display(&self) -> String {
    match self {
      Operation::Config { key, value } => format!("cfg({}={})", key, value),
      Operation::WriteEnv => "write_env".to_string(),
      Operation::UpdateVersions => "update_versions".to_string(),
      Operation::Run => "run".to_string(),
    }
  }

  /// Converts the operation to a command object
  pub fn to_command(&self) -> Box<dyn Command> {
    match self {
      Operation::Config { key, value } => Box::new(ConfigCommand::new(key.clone(), value.clone())),
      Operation::WriteEnv => Box::new(WriteEnvCommand),
      Operation::UpdateVersions => Box::new(UpdateVersionsCommand),
      Operation::Run => Box::new(RunCommand),
    }
  }
}

/// Container for all parsed commands and execution context
#[derive(Debug)]
pub struct ExecutionPlan {
  /// List of commands to execute in order
  pub commands: Vec<Box<dyn Command>>,
  /// Input environment file path
  pub input_env: String,
  /// Output environment file path (optional)
  pub output_env: Option<String>,
  /// Additional arguments for Docker commands
  pub args: Vec<String>,
  /// Verbose output flag
  pub verbose: bool,
  /// Runtime configuration
  pub config: Config,
}

impl ExecutionPlan {
  /// Creates a new execution plan
  pub fn new(
    input_env: String,
    output_env: Option<String>,
    args: Vec<String>,
    verbose: bool,
  ) -> Self {
    Self {
      commands: Vec::new(),
      input_env,
      output_env,
      args,
      verbose,
      config: Config::new(),
    }
  }

  /// Adds a command to the execution plan
  pub fn add_command(&mut self, command: Box<dyn Command>) {
    self.commands.push(command);
  }

  /// Adds an operation to the execution plan (converts to command)
  pub fn add_operation(&mut self, operation: Operation) {
    self.commands.push(operation.to_command());
  }

  /// Returns true if the plan contains any commands
  pub fn has_operations(&self) -> bool {
    !self.commands.is_empty()
  }

  /// Returns a list of command names for display
  pub fn operation_names(&self) -> Vec<String> {
    self.commands.iter().map(|cmd| cmd.display()).collect()
  }

  /// Creates an execution context from the plan
  pub fn create_execution_context(&self) -> ExecutionContext {
    ExecutionContext::new(
      self.config.clone(),
      self.input_env.clone(),
      self.output_env.clone(),
      self.args.clone(),
      self.verbose,
    )
  }

  /// Executes all commands in the plan
  pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = self.create_execution_context();

    // First pass: Execute all configuration commands
    for command in &self.commands {
      if command.is_config_command() {
        command.execute(&mut context)?;
      }
    }

    // Setup project paths (now uses updated configuration)
    let (host_project_path_str, docker_dev_path) =
      crate::utils::setup_project_paths(context.config.docker_dev_path(), context.verbose)?;

    // Store paths in context
    context.host_project_path = Some(host_project_path_str.clone());
    context.docker_dev_path = Some(docker_dev_path.to_string_lossy().to_string());

    // Creation of directory map and MD5 calculation
    let (dir_env_map, mut env_vars, md5_values) =
      crate::env_ops::create_dir_env_map_and_calculate_md5(
        &docker_dev_path,
        &host_project_path_str,
        context.verbose,
      )?;

    // Reading .env, .env.local and specified input file, and updating variables
    let mut existing_env_vars = crate::env_ops::combine_env_files(&context.input_env, context.verbose)?;
    for (key, value) in &env_vars {
      existing_env_vars.insert(key.clone(), value.clone());
    }

    // Store environment variables and MD5 values in context
    context.env_vars = Some(env_vars);
    context.existing_env_vars = Some(existing_env_vars);
    context.md5_values = Some(md5_values);

    // Second pass: Execute non-configuration commands in the order they were specified
    for command in &self.commands {
      if !command.is_config_command() {
        command.execute(&mut context)?;
      }
    }

    Ok(())
  }
}

/// Formats
pub const FORMAT_HEX: &str = "{:x}";
pub const FORMAT_VERSION: &str = "{}.{}.{}";
pub const FORMAT_ENV_VAR: &str = "{}={}";
pub const FORMAT_TXT_EXTENSION: &str = "{}.txt";
