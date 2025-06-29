use std::collections::HashMap;
use std::env;
use std::process::Command;

use crate::file_ops::{read_env_file, write_env_file};
use crate::model::*;
use crate::utils::{get_home_directory, socket_exists};

/// Esegue un comando Docker con le variabili d'ambiente e le configurazioni appropriate.
///
/// # Deprecated
/// This function has been migrated to the new command system in `commands/app/docker.rs`.
/// Use the new `docker` command in the Lisp-based command system instead.
/// This function is kept for backward compatibility with the traditional CLI system.
///
/// # Arguments
/// * `env_vars` - HashMap contenente le variabili d'ambiente da passare al comando Docker
/// * `existing_env_vars` - HashMap contenente le variabili d'ambiente lette dai file .env
/// * `args` - Argomenti aggiuntivi da passare al comando Docker
/// * `verbose` - Flag per abilitare l'output verboso
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok se il comando è eseguito con successo, Err altrimenti
///
/// # Note
/// - Configura automaticamente il mapping del socket Docker in base al sistema operativo
/// - Passa tutte le variabili d'ambiente al comando Docker
/// - Supporta configurazioni personalizzate tramite DOCKER_HOST_MAP
/// - Termina il processo con codice 1 se il comando Docker fallisce
#[deprecated(
  since = "0.0.1",
  note = "Use the new `docker` command in commands/app instead"
)]
pub fn execute_docker_command(
  env_vars: &HashMap<String, String>,
  existing_env_vars: &HashMap<String, String>,
  args: &[String],
  verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  // Prepara il comando Docker
  let mut command = Command::new("docker");
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
    std::process::exit(1);
  }

  Ok(())
}

/// Processes Docker version information for a specific component
///
/// # Arguments
/// * `dir_path` - Path to the directory containing the Docker component
/// * `md5_value` - MD5 hash value for the component
/// * `versions_folder` - Directory where version files are stored
/// * `verbose` - Flag for verbose output
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, Err otherwise
pub fn process_docker_version(
  dir_path: &str,
  md5_value: &str,
  versions_folder: &str,
  verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  if verbose {
    println!("Processing Docker version for: {}", dir_path);
    println!("MD5: {}", md5_value);
    println!("Versions folder: {}", versions_folder);
  }

  // TODO: Implement actual version processing logic
  // This is a stub implementation to fix the build issue

  Ok(())
}
