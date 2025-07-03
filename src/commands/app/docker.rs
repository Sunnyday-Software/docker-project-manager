use crate::file_ops::read_env_file;
use crate::model::*;
use crate::utils::debug_log;
use crate::utils::{get_home_directory, socket_exists};
use crate::{CommandRegistry, Context, Value, tags};
use std::collections::HashMap;
use std::env;
use std::process::Command;

/// Configuration structure for Docker commands
/// Allows dynamic configuration of Docker command behavior through Lisp functions
#[derive(Debug, Clone)]
pub struct DockerCommandConfig {
  /// Arguments for Docker Compose (default: ["-f", "docker-compose.yml"])
  pub compose_args: Vec<String>,
  /// Arguments for make command (default: ["make", "make"])
  pub make_args: Vec<String>,
  /// Custom socket path (default: None for auto-detect)
  pub socket_path: Option<String>,
  /// Environment variables to set
  pub env_vars: HashMap<String, String>,
  /// Commands to execute before Docker command
  pub pre_commands: Vec<Vec<String>>,
  /// Commands to execute after Docker command
  pub post_commands: Vec<Vec<String>>,
}

impl Default for DockerCommandConfig {
  fn default() -> Self {
    Self {
      compose_args: DOCKER_COMPOSE_ARGS.iter().map(|s| s.to_string()).collect(),
      make_args: DOCKER_MAKE_ARGS.iter().map(|s| s.to_string()).collect(),
      socket_path: None,
      env_vars: HashMap::new(),
      pre_commands: Vec::new(),
      post_commands: Vec::new(),
    }
  }
}

/// Builds Docker configuration from Context variables
/// Extracts configuration from Lisp variables set by configuration commands
fn build_docker_config(ctx: &Context) -> DockerCommandConfig {
  let mut config = DockerCommandConfig::default();

  // Extract compose_args from context
  if let Some(value) = ctx.get_variable("docker_compose_args") {
    match value {
      Value::List(args) => {
        config.compose_args = args.iter()
          .filter_map(|v| match v {
            Value::Str(s) => Some(s.clone()),
            _ => None,
          })
          .collect();
      },
      Value::Nil => {
        // Keep default values when explicitly set to nil
      },
      _ => {
        // Invalid type, keep defaults
      }
    }
  }

  // Extract make_args from context
  if let Some(value) = ctx.get_variable("docker_make_args") {
    match value {
      Value::List(args) => {
        config.make_args = args.iter()
          .filter_map(|v| match v {
            Value::Str(s) => Some(s.clone()),
            _ => None,
          })
          .collect();
      },
      Value::Nil => {
        // Keep default values when explicitly set to nil
      },
      _ => {
        // Invalid type, keep defaults
      }
    }
  }

  // Extract socket_path from context
  if let Some(value) = ctx.get_variable("docker_socket_path") {
    match value {
      Value::Str(path) => {
        config.socket_path = Some(path.clone());
      },
      Value::Nil => {
        // Keep default (None) when explicitly set to nil
        config.socket_path = None;
      },
      _ => {
        // Invalid type, keep defaults
      }
    }
  }

  // Extract pre_commands from context
  if let Some(value) = ctx.get_variable("docker_pre_hooks") {
    match value {
      Value::List(pre_hooks) => {
        config.pre_commands = pre_hooks.iter()
          .filter_map(|v| match v {
            Value::List(cmd_args) => {
              let cmd: Vec<String> = cmd_args.iter()
                .filter_map(|arg| match arg {
                  Value::Str(s) => Some(s.clone()),
                  _ => None,
                })
                .collect();
              if !cmd.is_empty() { Some(cmd) } else { None }
            },
            _ => None,
          })
          .collect();
      },
      Value::Nil => {
        // Keep default (empty) when explicitly set to nil
        config.pre_commands = Vec::new();
      },
      _ => {
        // Invalid type, keep defaults
      }
    }
  }

  // Extract post_commands from context
  if let Some(value) = ctx.get_variable("docker_post_hooks") {
    match value {
      Value::List(post_hooks) => {
        config.post_commands = post_hooks.iter()
          .filter_map(|v| match v {
            Value::List(cmd_args) => {
              let cmd: Vec<String> = cmd_args.iter()
                .filter_map(|arg| match arg {
                  Value::Str(s) => Some(s.clone()),
                  _ => None,
                })
                .collect();
              if !cmd.is_empty() { Some(cmd) } else { None }
            },
            _ => None,
          })
          .collect();
      },
      Value::Nil => {
        // Keep default (empty) when explicitly set to nil
        config.post_commands = Vec::new();
      },
      _ => {
        // Invalid type, keep defaults
      }
    }
  }

  config
}

/// Executes a generic command with arguments
fn execute_command(command: &str, args: &[String], ctx: &Context) -> Result<(), String> {
  debug_log(ctx, "docker", &format!("executing command: {} {:?}", command, args));

  let mut cmd = Command::new(command);
  cmd.current_dir(ctx.get_basedir());
  cmd.args(args);

  match cmd.status() {
    Ok(status) => {
      if status.success() {
        Ok(())
      } else {
        Err(format!("Command failed with exit code: {:?}", status.code()))
      }
    },
    Err(e) => Err(format!("Failed to execute command: {}", e)),
  }
}

/// Executes Docker command with the provided configuration
fn execute_docker_command_with_config(
  ctx: &Context,
  config: &DockerCommandConfig,
  env_vars: &HashMap<String, String>,
  existing_env_vars: &HashMap<String, String>,
  args: &[String],
  verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  // Execute pre-commands
  for pre_cmd in &config.pre_commands {
    if !pre_cmd.is_empty() {
      let cmd_name = &pre_cmd[0];
      let cmd_args = &pre_cmd[1..];
      if let Err(e) = execute_command(cmd_name, cmd_args, ctx) {
        debug_log(ctx, "docker", &format!("pre-command failed: {}", e));
        return Err(e.into());
      }
    }
  }

  // Prepare Docker command
  let mut command = Command::new("docker");
  command.current_dir(ctx.get_basedir());

  // Use configured compose args or fallback to defaults
  if config.compose_args.is_empty() {
    command.args(DOCKER_COMPOSE_ARGS);
  } else {
    command.args(&config.compose_args);
  }

  // Handle socket mapping (adapted for cross-platform compatibility)
  if cfg!(target_os = "windows") {
    // On Windows, Docker socket is handled differently or omitted
    let socket_path = config.socket_path.as_deref().unwrap_or(DOCKER_SOCKET_PATH);
    let docker_socket = format!("{}:{}", socket_path, DOCKER_SOCKET_PATH);
    command.args(&["-v", &docker_socket]);
    if verbose {
      println!("Docker Socket mapping: {}", docker_socket);
    }
  } else {
    // Check if DOCKER_HOST exists in .env file
    if let Some(docker_host_map) = existing_env_vars.get(ENV_DOCKER_HOST_MAP) {
      if verbose {
        println!("Using DOCKER_HOST_MAP from .env file: {}", docker_host_map);
      }
      command.args(&["-v", &*docker_host_map]);
    } else {
      // If not exists, find the first available socket
      let socket_path = if let Some(custom_path) = &config.socket_path {
        custom_path.clone()
      } else {
        let home_directory = get_home_directory().ok_or(ERROR_CANNOT_DETERMINE_HOME)?;
        if socket_exists(DOCKER_SOCKET_PATH) {
          DOCKER_SOCKET_PATH.to_string()
        } else if socket_exists(&format!(
          "{}{}",
          home_directory.to_str().unwrap(),
          DOCKER_DESKTOP_SOCKET_SUFFIX
        )) {
          format!(
            "{}{}",
            home_directory.to_str().unwrap(),
            DOCKER_DESKTOP_SOCKET_SUFFIX
          )
        } else if let Ok(xdg_runtime_dir) = env::var("XDG_RUNTIME_DIR") {
          format!("{}{}", xdg_runtime_dir, DOCKER_SOCKET_SUFFIX)
        } else {
          DOCKER_SOCKET_PATH.to_string()
        }
      };

      // Volume mapping
      let docker_socket = format!("{}:{}", socket_path, DOCKER_SOCKET_PATH);
      command.args(&["-v", &*docker_socket]);
      if verbose {
        println!("Docker Socket mapping: {}", docker_socket);
      }
    };
  }

  // Set environment variables in the process environment
  for (key, value) in env_vars {
    command.env(key, value);
    if verbose {
      println!("* env key: {} = {}", key, value);
    }
  }

  // Pass only environment variable names to Docker
  for key in env_vars.keys() {
    command.args(&["-e", key]);
  }

  // Create concatenated string of all keys
  let concatenated_keys = env_vars.keys().cloned().collect::<Vec<_>>().join(";");
  command.env(ENV_DOCKER_ENV_KEYS, concatenated_keys);
  command.args(&["-e", ENV_DOCKER_ENV_KEYS]);

  // Specify service and command to execute
  if config.make_args.is_empty() {
    command.args(DOCKER_MAKE_ARGS);
  } else {
    command.args(&config.make_args);
  }

  // Add any additional arguments passed to the program
  command.args(args);

  // Print complete command (for debugging)
  if verbose {
    println!("Executing command: {:?}", command);
  }

  // Execute Docker command
  let status = command.status()?;

  if !status.success() {
    eprintln!("{}", MSG_DOCKER_COMMAND_FAILED);
    return Err("Docker command failed".into());
  }

  // Execute post-commands
  for post_cmd in &config.post_commands {
    if !post_cmd.is_empty() {
      let cmd_name = &post_cmd[0];
      let cmd_args = &post_cmd[1..];
      if let Err(e) = execute_command(cmd_name, cmd_args, ctx) {
        debug_log(ctx, "docker", &format!("post-command failed: {}", e));
        // Post-command failures are logged but don't fail the main operation
      }
    }
  }

  Ok(())
}

/// Register docker command
pub fn register_docker_command(registry: &mut CommandRegistry) {
  registry.register_closure_with_help_and_tag(
    "docker",
    "Execute Docker commands with environment variables and configurations",
    "(docker [args...])",
    "  (docker \"run\" \"hello-world\")     ; Run a simple Docker container\n  (docker \"ps\" \"-a\")              ; List all containers\n  (docker \"build\" \".\" \"-t\" \"myapp\") ; Build an image",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "docker", "executing docker command");

      // Convert args to strings
      let mut docker_args = Vec::new();
      for arg in args {
        match arg {
          Value::Str(s) => docker_args.push(s),
          Value::Int(i) => docker_args.push(i.to_string()),
          _ => return Err("docker arguments must be strings or integers".to_string()),
        }
      }

      debug_log(ctx, "docker", &format!("docker args: {:?}", docker_args));

      // Get environment variables from context
      let mut env_vars = HashMap::new();

      // Collect all string variables from context as environment variables
      for (key, value) in &ctx.variables {
        if let Value::Str(val) = value {
          env_vars.insert(key.clone(), val.clone());
        }
      }

      //debug_log(ctx, "docker", &format!("collected {} environment variables", env_vars.len()));

      // Read existing environment variables from .env files if they exist
      let mut existing_env_vars = HashMap::new();
      let basedir = ctx.get_basedir();
      let env_file_path = basedir.join(".env");

      if env_file_path.exists() {
        match read_env_file(&env_file_path.to_string_lossy()) {
          Ok(vars) => {
            existing_env_vars.extend(vars);
            debug_log(ctx, "docker", &format!("loaded {} variables from .env file", existing_env_vars.len()));
          },
          Err(e) => {
            debug_log(ctx, "docker", &format!("warning: failed to read .env file: {}", e));
          }
        }
      }

      // Build configuration from context
      let config = build_docker_config(ctx);

      // Execute the docker command with configuration
      match execute_docker_command_with_config(ctx, &config, &env_vars, &existing_env_vars, &docker_args, ctx.get_debug_print()) {
        Ok(_) => {
          debug_log(ctx, "docker", "docker command executed successfully");
          Ok(Value::Str("Docker command executed successfully".to_string()))
        },
        Err(e) => {
          let error_msg = format!("Docker command failed: {}", e);
          debug_log(ctx, "docker", &error_msg);
          Err(error_msg)
        }
      }
    },
  );

  // Register docker-compose-args command
  registry.register_closure_with_help_and_tag(
    "docker-compose-args",
    "Configure Docker Compose arguments",
    "(docker-compose-args arg1 arg2 ...)",
    "  (docker-compose-args \"compose\" \"run\" \"--rm\")  ; Set compose arguments\n  (docker-compose-args \"-f\" \"custom-compose.yml\")   ; Use custom compose file",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "docker-compose-args", "configuring Docker Compose arguments");

      let mut compose_args = Vec::new();
      for arg in args {
        match arg {
          Value::Str(s) => compose_args.push(s),
          Value::Int(i) => compose_args.push(i.to_string()),
          _ => return Err("docker-compose-args arguments must be strings or integers".to_string()),
        }
      }

      let args_list = compose_args.into_iter().map(Value::Str).collect();
      ctx.set_variable("docker_compose_args".to_string(), Value::List(args_list));

      debug_log(ctx, "docker-compose-args", "Docker Compose arguments configured");
      Ok(Value::Str("Docker Compose arguments configured".to_string()))
    },
  );

  // Register docker-make-args command
  registry.register_closure_with_help_and_tag(
    "docker-make-args",
    "Configure Docker make arguments",
    "(docker-make-args arg1 arg2 ...)",
    "  (docker-make-args \"make\" \"build\")     ; Set make arguments\n  (docker-make-args \"npm\" \"run\" \"dev\")  ; Use npm instead of make",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "docker-make-args", "configuring Docker make arguments");

      let mut make_args = Vec::new();
      for arg in args {
        match arg {
          Value::Str(s) => make_args.push(s),
          Value::Int(i) => make_args.push(i.to_string()),
          _ => return Err("docker-make-args arguments must be strings or integers".to_string()),
        }
      }

      let args_list = make_args.into_iter().map(Value::Str).collect();
      ctx.set_variable("docker_make_args".to_string(), Value::List(args_list));

      debug_log(ctx, "docker-make-args", "Docker make arguments configured");
      Ok(Value::Str("Docker make arguments configured".to_string()))
    },
  );

  // Register docker-socket command
  registry.register_closure_with_help_and_tag(
    "docker-socket",
    "Set custom Docker socket path",
    "(docker-socket path)",
    "  (docker-socket \"/var/run/docker.sock\")           ; Set standard socket\n  (docker-socket \"/home/user/.docker/desktop/docker.sock\") ; Set custom socket",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "docker-socket", "configuring Docker socket path");

      if args.len() != 1 {
        return Err("docker-socket requires exactly one argument (socket path)".to_string());
      }

      match &args[0] {
        Value::Str(path) => {
          ctx.set_variable("docker_socket_path".to_string(), Value::Str(path.clone()));
          debug_log(ctx, "docker-socket", &format!("Docker socket path set to: {}", path));
          Ok(Value::Str(format!("Docker socket path set to: {}", path)))
        },
        _ => Err("docker-socket argument must be a string".to_string()),
      }
    },
  );

  // Register docker-pre command
  registry.register_closure_with_help_and_tag(
    "docker-pre",
    "Add pre-hook command to execute before Docker command",
    "(docker-pre command arg1 arg2 ...)",
    "  (docker-pre \"echo\" \"Starting Docker...\")  ; Add echo command\n  (docker-pre \"mkdir\" \"-p\" \"logs\")          ; Create logs directory",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "docker-pre", "adding Docker pre-hook command");

      if args.is_empty() {
        return Err("docker-pre requires at least one argument (command)".to_string());
      }

      let mut cmd_args = Vec::new();
      for arg in args {
        match arg {
          Value::Str(s) => cmd_args.push(Value::Str(s)),
          Value::Int(i) => cmd_args.push(Value::Str(i.to_string())),
          _ => return Err("docker-pre arguments must be strings or integers".to_string()),
        }
      }

      // Get existing pre-hooks or create new list
      let mut pre_hooks = match ctx.get_variable("docker_pre_hooks") {
        Some(Value::List(hooks)) => hooks.clone(),
        _ => Vec::new(),
      };

      pre_hooks.push(Value::List(cmd_args));
      ctx.set_variable("docker_pre_hooks".to_string(), Value::List(pre_hooks));

      debug_log(ctx, "docker-pre", "Docker pre-hook command added");
      Ok(Value::Str("Docker pre-hook command added".to_string()))
    },
  );

  // Register docker-post command
  registry.register_closure_with_help_and_tag(
    "docker-post",
    "Add post-hook command to execute after Docker command",
    "(docker-post command arg1 arg2 ...)",
    "  (docker-post \"echo\" \"Docker completed\")  ; Add echo command\n  (docker-post \"rm\" \"-rf\" \"temp\")          ; Clean up temp files",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "docker-post", "adding Docker post-hook command");

      if args.is_empty() {
        return Err("docker-post requires at least one argument (command)".to_string());
      }

      let mut cmd_args = Vec::new();
      for arg in args {
        match arg {
          Value::Str(s) => cmd_args.push(Value::Str(s)),
          Value::Int(i) => cmd_args.push(Value::Str(i.to_string())),
          _ => return Err("docker-post arguments must be strings or integers".to_string()),
        }
      }

      // Get existing post-hooks or create new list
      let mut post_hooks = match ctx.get_variable("docker_post_hooks") {
        Some(Value::List(hooks)) => hooks.clone(),
        _ => Vec::new(),
      };

      post_hooks.push(Value::List(cmd_args));
      ctx.set_variable("docker_post_hooks".to_string(), Value::List(post_hooks));

      debug_log(ctx, "docker-post", "Docker post-hook command added");
      Ok(Value::Str("Docker post-hook command added".to_string()))
    },
  );

  // Register docker-reset command
  registry.register_closure_with_help_and_tag(
    "docker-reset",
    "Reset Docker configuration to defaults",
    "(docker-reset)",
    "  (docker-reset)  ; Reset all Docker configuration to defaults",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "docker-reset", "resetting Docker configuration to defaults");

      if !args.is_empty() {
        return Err("docker-reset takes no arguments".to_string());
      }

      // Reset all Docker configuration variables to defaults
      ctx.set_variable("docker_compose_args".to_string(), Value::Nil);
      ctx.set_variable("docker_make_args".to_string(), Value::Nil);
      ctx.set_variable("docker_socket_path".to_string(), Value::Nil);
      ctx.set_variable("docker_pre_hooks".to_string(), Value::Nil);
      ctx.set_variable("docker_post_hooks".to_string(), Value::Nil);

      debug_log(ctx, "docker-reset", "Docker configuration reset to defaults");
      Ok(Value::Str("Docker configuration reset to defaults".to_string()))
    },
  );

  // Register docker-show-config command
  registry.register_closure_with_help_and_tag(
    "docker-show-config",
    "Show current Docker configuration",
    "(docker-show-config)",
    "  (docker-show-config)  ; Display current Docker configuration",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "docker-show-config", "showing Docker configuration");

      if !args.is_empty() {
        return Err("docker-show-config takes no arguments".to_string());
      }

      let config = build_docker_config(ctx);

      let mut output = String::new();
      output.push_str("=== Docker Configuration ===\n");
      output.push_str(&format!("Compose args: {:?}\n", config.compose_args));
      output.push_str(&format!("Make args: {:?}\n", config.make_args));
      output.push_str(&format!("Socket path: {:?}\n", config.socket_path));
      output.push_str(&format!("Pre-commands: {:?}\n", config.pre_commands));
      output.push_str(&format!("Post-commands: {:?}\n", config.post_commands));
      output.push_str("============================");

      println!("{}", output);
      Ok(Value::Str(output))
    },
  );
}

/// Internal function to execute Docker commands with environment variables and configurations
/// This is the migrated functionality from the original execute_docker_command function
fn execute_docker_command_internal(
  ctx: &Context,
  env_vars: &HashMap<String, String>,
  existing_env_vars: &HashMap<String, String>,
  args: &[String],
  verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  // Prepara il comando Docker
  let mut command = Command::new("docker");
  command.current_dir(ctx.get_basedir());
  command.args(DOCKER_COMPOSE_ARGS);

  // Mapping dei volumi (adattato per compatibilità cross-platform)
  if cfg!(target_os = "windows") {
    // Su Windows, il socket Docker si gestisce diversamente o si omette
    let docker_socket =
      format!("{}:{}", DOCKER_SOCKET_PATH, DOCKER_SOCKET_PATH);
    command.args(&["-v", &docker_socket]);
    if verbose {
      println!("Docker Socket mapping: {}", docker_socket);
    }
  } else {
    // Controlla se esiste la variabile DOCKER_HOST nel file .env
    if let Some(docker_host_map) = existing_env_vars.get(ENV_DOCKER_HOST_MAP) {
      if verbose {
        println!(
          "Utilizzo DOCKER_HOST_MAP dal file .env: {}",
          docker_host_map
        );
      }
      command.args(&["-v", &*docker_host_map]);
    } else {
      // Se non esiste, trova il primo socket disponibile
      let home_directory =
        get_home_directory().ok_or(ERROR_CANNOT_DETERMINE_HOME)?;
      let docker_socket_path = if socket_exists(DOCKER_SOCKET_PATH) {
        DOCKER_SOCKET_PATH.to_string()
      } else if socket_exists(&format!(
        "{}{}",
        home_directory.to_str().unwrap(),
        DOCKER_DESKTOP_SOCKET_SUFFIX
      )) {
        format!(
          "{}{}",
          home_directory.to_str().unwrap(),
          DOCKER_DESKTOP_SOCKET_SUFFIX
        )
      } else if let Ok(xdg_runtime_dir) = env::var("XDG_RUNTIME_DIR") {
        format!("{}{}", xdg_runtime_dir, DOCKER_SOCKET_SUFFIX)
      } else {
        DOCKER_SOCKET_PATH.to_string()
      };
      // Mapping dei volumi
      let docker_socket =
        format!("{}:{}", docker_socket_path, DOCKER_SOCKET_PATH);
      command.args(&["-v", &*docker_socket]);
      if verbose {
        println!("Docker Socket mapping: {}", docker_socket);
      }
    };
  }

  // Imposta le variabili d'ambiente nell'ambiente del processo
  for (key, value) in env_vars {
    command.env(key, value);
    if verbose {
      println!("* env key: {} = {}", key, value);
    }
  }

  // Passa a Docker solo i nomi delle variabili d'ambiente
  for key in env_vars.keys() {
    command.args(&["-e", key]);
  }

  // Creazione della stringa concatenata di tutte le chiavi
  let concatenated_keys =
    env_vars.keys().cloned().collect::<Vec<_>>().join(";");
  command.env(ENV_DOCKER_ENV_KEYS, concatenated_keys);
  command.args(&["-e", ENV_DOCKER_ENV_KEYS]);

  // Specifica il servizio e il comando da eseguire
  command.args(DOCKER_MAKE_ARGS);

  // Aggiunge eventuali argomenti aggiuntivi passati al programma
  command.args(args);

  // Stampa del comando completo (per il debug)
  if verbose {
    println!("Eseguendo il comando: {:?}", command);
  }

  // Esegue il comando Docker
  let status = command.status()?;

  if !status.success() {
    eprintln!("{}", MSG_DOCKER_COMMAND_FAILED);
    return Err("Docker command failed".into());
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::context::Context;
  use crate::lisp_interpreter::CommandRegistry;

  #[test]
  fn test_docker_command_registration() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);

    // Check that the command is registered
    assert!(registry.get("docker").is_some());
  }

  #[test]
  fn test_docker_command_args_validation() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test with string arguments
    let args = vec![Value::Str("ps".to_string()), Value::Str("-a".to_string())];

    // Note: This test will fail if Docker is not available, but it tests argument validation
    let result = ctx.registry.get("docker").unwrap().execute(args, &mut ctx);

    // The command should at least validate arguments correctly
    // (actual execution may fail if Docker is not available)
    match result {
      Ok(_) => {} // Docker command succeeded
      Err(e) => {
        // Should not be an argument validation error
        assert!(!e.contains("arguments must be strings"));
      }
    }
  }

  #[test]
  fn test_docker_command_invalid_args() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test with invalid argument type
    let args = vec![Value::List(vec![Value::Str("invalid".to_string())])];

    let result = ctx.registry.get("docker").unwrap().execute(args, &mut ctx);

    assert!(result.is_err());
    assert!(
      result
        .unwrap_err()
        .contains("arguments must be strings or integers")
    );
  }

  #[test]
  fn test_build_docker_config_defaults() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let ctx = Context::new(registry);

    let config = build_docker_config(&ctx);

    // Test default values
    assert_eq!(config.compose_args, DOCKER_COMPOSE_ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>());
    assert_eq!(config.make_args, DOCKER_MAKE_ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>());
    assert_eq!(config.socket_path, None);
    assert!(config.pre_commands.is_empty());
    assert!(config.post_commands.is_empty());
  }

  #[test]
  fn test_docker_compose_args_command() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test setting compose args
    let args = vec![
      Value::Str("compose".to_string()),
      Value::Str("run".to_string()),
      Value::Str("--rm".to_string()),
    ];

    let result = ctx.registry.get("docker-compose-args").unwrap().execute(args, &mut ctx);
    assert!(result.is_ok());

    // Verify configuration was set
    let config = build_docker_config(&ctx);
    assert_eq!(config.compose_args, vec!["compose", "run", "--rm"]);
  }

  #[test]
  fn test_docker_make_args_command() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test setting make args
    let args = vec![
      Value::Str("npm".to_string()),
      Value::Str("run".to_string()),
      Value::Str("dev".to_string()),
    ];

    let result = ctx.registry.get("docker-make-args").unwrap().execute(args, &mut ctx);
    assert!(result.is_ok());

    // Verify configuration was set
    let config = build_docker_config(&ctx);
    assert_eq!(config.make_args, vec!["npm", "run", "dev"]);
  }

  #[test]
  fn test_docker_socket_command() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test setting socket path
    let args = vec![Value::Str("/custom/docker.sock".to_string())];

    let result = ctx.registry.get("docker-socket").unwrap().execute(args, &mut ctx);
    assert!(result.is_ok());

    // Verify configuration was set
    let config = build_docker_config(&ctx);
    assert_eq!(config.socket_path, Some("/custom/docker.sock".to_string()));
  }

  #[test]
  fn test_docker_socket_command_invalid_args() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test with no arguments
    let result = ctx.registry.get("docker-socket").unwrap().execute(vec![], &mut ctx);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("requires exactly one argument"));

    // Test with too many arguments
    let args = vec![
      Value::Str("/path1".to_string()),
      Value::Str("/path2".to_string()),
    ];
    let result = ctx.registry.get("docker-socket").unwrap().execute(args, &mut ctx);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("requires exactly one argument"));
  }

  #[test]
  fn test_docker_pre_command() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test adding pre-hook command
    let args = vec![
      Value::Str("echo".to_string()),
      Value::Str("Starting Docker...".to_string()),
    ];

    let result = ctx.registry.get("docker-pre").unwrap().execute(args, &mut ctx);
    assert!(result.is_ok());

    // Verify configuration was set
    let config = build_docker_config(&ctx);
    assert_eq!(config.pre_commands.len(), 1);
    assert_eq!(config.pre_commands[0], vec!["echo", "Starting Docker..."]);
  }

  #[test]
  fn test_docker_post_command() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test adding post-hook command
    let args = vec![
      Value::Str("echo".to_string()),
      Value::Str("Docker completed".to_string()),
    ];

    let result = ctx.registry.get("docker-post").unwrap().execute(args, &mut ctx);
    assert!(result.is_ok());

    // Verify configuration was set
    let config = build_docker_config(&ctx);
    assert_eq!(config.post_commands.len(), 1);
    assert_eq!(config.post_commands[0], vec!["echo", "Docker completed"]);
  }

  #[test]
  fn test_docker_pre_post_multiple_commands() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Add multiple pre-hook commands
    let args1 = vec![Value::Str("mkdir".to_string()), Value::Str("-p".to_string()), Value::Str("logs".to_string())];
    let result1 = ctx.registry.get("docker-pre").unwrap().execute(args1, &mut ctx);
    assert!(result1.is_ok());

    let args2 = vec![Value::Str("echo".to_string()), Value::Str("Starting...".to_string())];
    let result2 = ctx.registry.get("docker-pre").unwrap().execute(args2, &mut ctx);
    assert!(result2.is_ok());

    // Add multiple post-hook commands
    let args3 = vec![Value::Str("echo".to_string()), Value::Str("Completed".to_string())];
    let result3 = ctx.registry.get("docker-post").unwrap().execute(args3, &mut ctx);
    assert!(result3.is_ok());

    let args4 = vec![Value::Str("rm".to_string()), Value::Str("-rf".to_string()), Value::Str("temp".to_string())];
    let result4 = ctx.registry.get("docker-post").unwrap().execute(args4, &mut ctx);
    assert!(result4.is_ok());

    // Verify configuration
    let config = build_docker_config(&ctx);
    assert_eq!(config.pre_commands.len(), 2);
    assert_eq!(config.pre_commands[0], vec!["mkdir", "-p", "logs"]);
    assert_eq!(config.pre_commands[1], vec!["echo", "Starting..."]);
    assert_eq!(config.post_commands.len(), 2);
    assert_eq!(config.post_commands[0], vec!["echo", "Completed"]);
    assert_eq!(config.post_commands[1], vec!["rm", "-rf", "temp"]);
  }

  #[test]
  fn test_docker_reset_command() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Set some configuration
    let args = vec![Value::Str("custom".to_string()), Value::Str("compose".to_string())];
    ctx.registry.get("docker-compose-args").unwrap().execute(args, &mut ctx).unwrap();

    let args = vec![Value::Str("/custom/socket".to_string())];
    ctx.registry.get("docker-socket").unwrap().execute(args, &mut ctx).unwrap();

    // Verify configuration is set
    let config_before = build_docker_config(&ctx);
    assert_eq!(config_before.compose_args, vec!["custom", "compose"]);
    assert_eq!(config_before.socket_path, Some("/custom/socket".to_string()));

    // Reset configuration
    let result = ctx.registry.get("docker-reset").unwrap().execute(vec![], &mut ctx);
    assert!(result.is_ok());

    // Verify configuration is reset to defaults
    let config_after = build_docker_config(&ctx);
    assert_eq!(config_after.compose_args, DOCKER_COMPOSE_ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>());
    assert_eq!(config_after.socket_path, None);
  }

  #[test]
  fn test_docker_reset_command_invalid_args() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test with arguments (should fail)
    let args = vec![Value::Str("invalid".to_string())];
    let result = ctx.registry.get("docker-reset").unwrap().execute(args, &mut ctx);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("takes no arguments"));
  }

  #[test]
  fn test_docker_show_config_command() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test show config with defaults
    let result = ctx.registry.get("docker-show-config").unwrap().execute(vec![], &mut ctx);
    assert!(result.is_ok());

    if let Ok(Value::Str(output)) = result {
      assert!(output.contains("=== Docker Configuration ==="));
      assert!(output.contains("Compose args:"));
      assert!(output.contains("Make args:"));
      assert!(output.contains("Socket path:"));
    }
  }

  #[test]
  fn test_docker_show_config_command_invalid_args() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test with arguments (should fail)
    let args = vec![Value::Str("invalid".to_string())];
    let result = ctx.registry.get("docker-show-config").unwrap().execute(args, &mut ctx);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("takes no arguments"));
  }

  #[test]
  fn test_build_docker_config_with_nil_values() {
    let mut registry = CommandRegistry::new();
    register_docker_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Set some configuration
    ctx.set_variable("docker_compose_args".to_string(), Value::List(vec![Value::Str("custom".to_string())]));
    ctx.set_variable("docker_socket_path".to_string(), Value::Str("/custom".to_string()));

    // Verify custom configuration
    let config_custom = build_docker_config(&ctx);
    assert_eq!(config_custom.compose_args, vec!["custom"]);
    assert_eq!(config_custom.socket_path, Some("/custom".to_string()));

    // Set to nil (reset to defaults)
    ctx.set_variable("docker_compose_args".to_string(), Value::Nil);
    ctx.set_variable("docker_socket_path".to_string(), Value::Nil);

    // Verify defaults are restored
    let config_nil = build_docker_config(&ctx);
    assert_eq!(config_nil.compose_args, DOCKER_COMPOSE_ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>());
    assert_eq!(config_nil.socket_path, None);
  }
}
