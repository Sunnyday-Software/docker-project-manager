use crate::clean_commands::CleanCommand;
use crate::config_commands::ConfigCommand;
use crate::core::{Command, Config, ExecutionContext, CommandRegistry};
use crate::env_commands::WriteEnvCommand;
use crate::run_commands::RunCommand;
use crate::version_commands::UpdateVersionsCommand;

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
  /// Returns the parsed Step and the number of arguments consumed
  pub fn parse_from_args(
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Result<Step, String> {
    match command {
      "clean" => {
        let mut force = false;
        // Check if next argument is force
        if let Some(next_arg) = args.peek() {
          if next_arg == "force" {
            force = true;
            args.next(); // consume force
          }
        }
        Ok(Step::Clean { force })
      }
      "config" => {
        // Expect key=value format
        if let Some(config_arg) = args.next() {
          if let Some((key, value)) = config_arg.split_once('=') {
            Ok(Step::Config {
              key: key.to_string(),
              value: value.to_string(),
            })
          } else {
            Err(format!(
              "Invalid config format: '{}'. Expected key=value",
              config_arg
            ))
          }
        } else {
          Err("Config step requires key=value argument".to_string())
        }
      }
      "write-env" => {
        // Expect output <file>
        if let Some(next_arg) = args.next() {
          if next_arg == "output" {
            if let Some(output_file) = args.next() {
              Ok(Step::WriteEnv {
                output: output_file,
              })
            } else {
              Err("write-env output requires a filename".to_string())
            }
          } else {
            Err("write-env step requires output <file>".to_string())
          }
        } else {
          Err("write-env step requires output <file>".to_string())
        }
      }
      "update-versions" => Ok(Step::UpdateVersions),
      "run" => {
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
        Ok(Step::Run { args: run_args })
      }
      _ => Err(format!("Unknown command: '{}'", command)),
    }
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
      Step::Clean { force } => Box::new(CleanCommand::new(*force)),
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
) -> Result<Vec<Box<dyn Command>>, String> {
  let mut registry = CommandRegistry::new();

  // Register all command types
  registry.register::<CleanCommand>();
  registry.register::<ConfigCommand>();
  registry.register::<WriteEnvCommand>();
  registry.register::<UpdateVersionsCommand>();
  registry.register::<RunCommand>();

  let mut commands = Vec::new();
  let mut iter = args.into_iter().peekable();

  while let Some(command) = iter.next() {
    // The first element (and any subsequent element that's not consumed by a previous command) is treated as a command
    let parsed_command = registry.parse_command(&command, &mut iter)?;
    commands.push(parsed_command);
  }

  if commands.is_empty() {
    return Err("No valid commands found".to_string());
  }

  Ok(commands)
}

/// Parses a pipeline of arguments into a vector of steps (legacy function for backward compatibility)
/// Each argument can be a command or a command attribute
/// The first element found is always a command
/// When creating the pipeline, each command can have multiple attributes
/// Parameters after the command are passed to the command itself which can consume or not consume some of the attributes
pub fn parse_pipeline(args: Vec<String>) -> Result<Vec<Step>, String> {
  let mut steps = Vec::new();
  let mut iter = args.into_iter().peekable();

  while let Some(command) = iter.next() {
    // The first element (and any subsequent element that's not consumed by a previous command) is treated as a command
    let step = Step::parse_from_args(&command, &mut iter)?;
    steps.push(step);
  }

  if steps.is_empty() {
    return Err("No valid steps found in pipeline".to_string());
  }

  Ok(steps)
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
      crate::utils::setup_project_paths(
        context.config.docker_dev_path(),
        context.verbose,
      )?;

    // Store paths in context
    context.host_project_path = Some(host_project_path_str.clone());
    context.docker_dev_path =
      Some(docker_dev_path.to_string_lossy().to_string());

    // Creation of directory map and MD5 calculation
    let (dir_env_map, mut env_vars, md5_values) =
      crate::env_ops::create_dir_env_map_and_calculate_md5(
        &docker_dev_path,
        &host_project_path_str,
        context.verbose,
      )?;

    // Reading .env, .env.local and specified input file, and updating variables
    let mut existing_env_vars =
      crate::env_ops::combine_env_files(&context.input_env, context.verbose)?;
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
