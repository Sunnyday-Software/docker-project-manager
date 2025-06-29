use crate::model::*;
use crate::utils::debug_log;
use crate::utils::{get_home_directory, socket_exists};
use crate::{CommandRegistry, Context, Value, tags};
use std::collections::HashMap;
use std::env;
use std::process::Command;

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

      //debug_log(ctx, "docker", &format!("docker args: {:?}", docker_args));

      // Get environment variables from context
      let env_vars = HashMap::new();

      // Collect all string variables from context as environment variables
      /*for (key, value) in &ctx.variables {
        if let Value::Str(val) = value {
          env_vars.insert(key.clone(), val.clone());
        }
      }*/

      //debug_log(ctx, "docker", &format!("collected {} environment variables", env_vars.len()));

      // Read existing environment variables from .env files if they exist
      let existing_env_vars = HashMap::new();
      /*let basedir = ctx.get_basedir();
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
      }*/

      // Execute the docker command
      match execute_docker_command_internal(ctx, &env_vars, &existing_env_vars, &docker_args, ctx.get_debug_print()) {
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
  //let concatenated_keys =
  //env_vars.keys().cloned().collect::<Vec<_>>().join(";");
  // command.env(ENV_DOCKER_ENV_KEYS, concatenated_keys);
  //command.args(&["-e", ENV_DOCKER_ENV_KEYS]);

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
}
