// model.rs - Re-exports for backward compatibility

// Re-export everything from the new modular structure
pub use crate::core::*;

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
  fn execute(
    &self,
    context: &mut ExecutionContext,
  ) -> Result<(), Box<dyn std::error::Error>>;

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
  fn execute(
    &self,
    context: &mut ExecutionContext,
  ) -> Result<(), Box<dyn std::error::Error>> {
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
  fn execute(
    &self,
    context: &mut ExecutionContext,
  ) -> Result<(), Box<dyn std::error::Error>> {
    if context.verbose {
      println!("{}", MSG_EXECUTING_OPERATION.replace("{}", self.name()));
    }

    let existing_env_vars = context
      .existing_env_vars
      .as_ref()
      .ok_or("Environment variables not initialized")?;
    let output_env = context
      .output_env
      .as_ref()
      .ok_or("Output environment file not specified")?;

    crate::file_ops::write_env_to_file(
      output_env,
      existing_env_vars,
      context.verbose,
    )?;
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
  fn execute(
    &self,
    context: &mut ExecutionContext,
  ) -> Result<(), Box<dyn std::error::Error>> {
    if context.verbose {
      println!("{}", MSG_EXECUTING_OPERATION.replace("{}", self.name()));
    }

    let md5_values = context
      .md5_values
      .as_ref()
      .ok_or("MD5 values not calculated")?;

    crate::utils::update_versions(
      md5_values,
      context.config.versions_folder(),
      context.verbose,
    )?;
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
  fn execute(
    &self,
    context: &mut ExecutionContext,
  ) -> Result<(), Box<dyn std::error::Error>> {
    if context.verbose {
      println!("{}", MSG_EXECUTING_OPERATION.replace("{}", self.name()));
    }

    let mut env_vars = context
      .env_vars
      .as_ref()
      .ok_or("Environment variables not initialized")?
      .clone();
    let existing_env_vars = context
      .existing_env_vars
      .as_ref()
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

    crate::docker::execute_docker_command(
      &env_vars,
      existing_env_vars,
      &context.args,
      context.verbose,
    )?;
    Ok(())
  }

  fn name(&self) -> &'static str {
    "run"
  }

  fn display(&self) -> String {
    "run".to_string()
  }
}

/// Clean command implementation
#[derive(Debug, Clone)]
pub struct CleanCommand {
  pub force: bool,
}

impl CleanCommand {
  pub fn new(force: bool) -> Self {
    Self { force }
  }
}

impl Command for CleanCommand {
  fn execute(
    &self,
    context: &mut ExecutionContext,
  ) -> Result<(), Box<dyn std::error::Error>> {
    if context.verbose {
      println!("{}", MSG_EXECUTING_OPERATION.replace("{}", self.name()));
    }

    // TODO: Implement clean functionality
    println!("Clean command executed with force: {}", self.force);
    Ok(())
  }

  fn name(&self) -> &'static str {
    "clean"
  }

  fn display(&self) -> String {
    if self.force {
      "clean --force".to_string()
    } else {
      "clean".to_string()
    }
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
      Operation::Config { key, value } => {
        Box::new(ConfigCommand::new(key.clone(), value.clone()))
      }
      Operation::WriteEnv => Box::new(WriteEnvCommand),
      Operation::UpdateVersions => Box::new(UpdateVersionsCommand),
      Operation::Run => Box::new(RunCommand),
    }
  }
}

/// Trait for parsing steps from command line arguments
pub trait StepParser: std::fmt::Debug {
  /// Returns the command name this parser handles
  fn command_name(&self) -> &'static str;

  /// Attempts to parse a step from the given command and arguments
  /// Returns Some(Result) if this parser can handle the command, None otherwise
  fn try_parse(
    &self,
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Option<Result<Step, String>>;
}

/// Registry for step parsers
pub struct StepRegistry {
  parsers: Vec<Box<dyn StepParser>>,
}

impl StepRegistry {
  /// Creates a new step registry with default parsers
  pub fn new() -> Self {
    let mut registry = Self {
      parsers: Vec::new(),
    };

    // Register default step parsers
    registry.register(Box::new(CleanStepParser));
    registry.register(Box::new(ConfigStepParser));
    registry.register(Box::new(WriteEnvStepParser));
    registry.register(Box::new(UpdateVersionsStepParser));
    registry.register(Box::new(RunStepParser));

    registry
  }

  /// Registers a new step parser
  pub fn register(&mut self, parser: Box<dyn StepParser>) {
    self.parsers.push(parser);
  }

  /// Attempts to parse a step using registered parsers
  pub fn parse_step(
    &self,
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Result<Step, String> {
    for parser in &self.parsers {
      if let Some(result) = parser.try_parse(command, args) {
        return result;
      }
    }
    Err(format!("Unknown command: '{}'", command))
  }
}

/// Individual step parsers
#[derive(Debug)]
pub struct CleanStepParser;

impl StepParser for CleanStepParser {
  fn command_name(&self) -> &'static str {
    "clean"
  }

  fn try_parse(
    &self,
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Option<Result<Step, String>> {
    if command != "clean" {
      return None;
    }

    let mut force = false;
    // Check if next argument is force
    if let Some(next_arg) = args.peek() {
      if next_arg == "force" {
        force = true;
        args.next(); // consume force
      }
    }
    Some(Ok(Step::Clean { force }))
  }
}

#[derive(Debug)]
pub struct ConfigStepParser;

impl StepParser for ConfigStepParser {
  fn command_name(&self) -> &'static str {
    "config"
  }

  fn try_parse(
    &self,
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Option<Result<Step, String>> {
    if command != "config" {
      return None;
    }

    // Expect key=value format
    if let Some(config_arg) = args.next() {
      if let Some((key, value)) = config_arg.split_once('=') {
        Some(Ok(Step::Config {
          key: key.to_string(),
          value: value.to_string(),
        }))
      } else {
        Some(Err(format!(
          "Invalid config format: '{}'. Expected key=value",
          config_arg
        )))
      }
    } else {
      Some(Err("Config step requires key=value argument".to_string()))
    }
  }
}

#[derive(Debug)]
pub struct WriteEnvStepParser;

impl StepParser for WriteEnvStepParser {
  fn command_name(&self) -> &'static str {
    "write-env"
  }

  fn try_parse(
    &self,
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Option<Result<Step, String>> {
    if command != "write-env" {
      return None;
    }

    // Expect output <file>
    if let Some(next_arg) = args.next() {
      if next_arg == "output" {
        if let Some(output_file) = args.next() {
          Some(Ok(Step::WriteEnv {
            output: output_file,
          }))
        } else {
          Some(Err("write-env output requires a filename".to_string()))
        }
      } else {
        Some(Err("write-env step requires output <file>".to_string()))
      }
    } else {
      Some(Err("write-env step requires output <file>".to_string()))
    }
  }
}

#[derive(Debug)]
pub struct UpdateVersionsStepParser;

impl StepParser for UpdateVersionsStepParser {
  fn command_name(&self) -> &'static str {
    "update-versions"
  }

  fn try_parse(
    &self,
    command: &str,
    _args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Option<Result<Step, String>> {
    if command != "update-versions" {
      return None;
    }
    Some(Ok(Step::UpdateVersions))
  }
}

#[derive(Debug)]
pub struct RunStepParser;

impl StepParser for RunStepParser {
  fn command_name(&self) -> &'static str {
    "run"
  }

  fn try_parse(
    &self,
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Option<Result<Step, String>> {
    if command != "run" {
      return None;
    }

    // Collect arguments until we find another known command
    let mut run_args = Vec::new();
    while let Some(next_arg) = args.peek() {
      // Check if next argument is a known command
      if matches!(
        next_arg.as_str(),
        "clean" | "config" | "write-env" | "update-versions" | "run"
      ) {
        break;
      }
      run_args.push(args.next().unwrap());
    }
    Some(Ok(Step::Run { args: run_args }))
  }
}

/// Pipeline step types that can be executed in sequence
#[derive(Debug, Clone)]
pub enum Step {
  /// Clean temporary files
  Clean { force: bool },
  /// Set configuration variable
  Config { key: String, value: String },
  /// Write environment file
  WriteEnv { output: String },
  /// Update component versions
  UpdateVersions,
  /// Execute Docker command
  Run { args: Vec<String> },
}

impl Step {
  /// Returns the name of the step for logging purposes
  pub fn name(&self) -> &'static str {
    match self {
      Step::Clean { .. } => "clean",
      Step::Config { .. } => "config",
      Step::WriteEnv { .. } => "write-env",
      Step::UpdateVersions => "update-versions",
      Step::Run { .. } => "run",
    }
  }

  /// Parses a command and its parameters from the argument iterator
  /// Returns the parsed Step using individual step parsers
  pub fn parse_from_args(
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Result<Step, String> {
    let registry = StepRegistry::new();
    registry.parse_step(command, args)
  }

  /// Returns a display string for the step including parameters
  pub fn display(&self) -> String {
    match self {
      Step::Clean { force } => {
        if *force {
          "clean --force".to_string()
        } else {
          "clean".to_string()
        }
      }
      Step::Config { key, value } => format!("config {}={}", key, value),
      Step::WriteEnv { output } => format!("write-env --output {}", output),
      Step::UpdateVersions => "update-versions".to_string(),
      Step::Run { args } => {
        if args.is_empty() {
          "run".to_string()
        } else {
          format!("run {}", args.join(" "))
        }
      }
    }
  }

  /// Converts the step to a command object
  pub fn to_command(&self) -> Box<dyn Command> {
    match self {
      Step::Clean { .. } => {
        // For now, Clean step doesn't have a corresponding command
        // We'll implement this as needed
        Box::new(ConfigCommand::new("_clean".to_string(), "true".to_string()))
      }
      Step::Config { key, value } => {
        Box::new(ConfigCommand::new(key.clone(), value.clone()))
      }
      Step::WriteEnv { .. } => Box::new(WriteEnvCommand),
      Step::UpdateVersions => Box::new(UpdateVersionsCommand),
      Step::Run { .. } => Box::new(RunCommand),
    }
  }
}

/// Parses a pipeline of arguments into a vector of commands using the command registry
/// Each argument can be a command or a command attribute
/// The first element found is always a command
/// When creating the pipeline, each command can have multiple attributes
/// Parameters after the command are passed to the command itself which can consume or not consume some of the attributes
pub fn parse_pipeline_with_registry(
  args: Vec<String>,
) -> Result<Vec<Box<dyn crate::core::Command>>, String> {
  let registry = CommandRegistry::new();
  let mut commands = Vec::new();
  let mut iter = args.into_iter().peekable();

  while let Some(command) = iter.next() {
    // The first element (and any subsequent element that's not consumed by a previous command) is treated as a command
    let parsed_command = registry.parse_command(&command, &mut iter)?;
    commands.push(parsed_command);
  }

  if commands.is_empty() {
    return Err("No valid commands found in pipeline".to_string());
  }

  Ok(commands)
}
