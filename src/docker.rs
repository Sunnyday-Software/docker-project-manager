use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use crate::file_ops::{read_env_file, write_env_file};
use crate::model::*;
use crate::utils::{get_home_directory, socket_exists};

/// Manages the versioning of a Docker component based on its MD5 hash.
///
/// # Arguments
/// * `docker_dir` - Path to the Docker component directory
/// * `current_md5` - Current MD5 hash of the component
/// * `versions_dir` - Directory where to save versioning files
/// * `verbose` - Flag per abilitare l'output verboso
///
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Ok if the operation is completed successfully, Err otherwise
///
/// # Note
/// - Compares the current MD5 with the one stored in the versioning file
/// - If the MD5 has changed, increments the PATCH version and updates the file
/// - Uses semantic versioning format (MAJOR.MINOR.PATCH)
/// - Creates the destination directory if it doesn't exist
pub fn process_docker_version(
  docker_dir: &str,
  current_md5: &str,
  versions_dir: &str,
  verbose: bool,
) -> Result<(), Box<dyn Error>> {
  let docker_dir_path = Path::new(docker_dir);
  let version_file_name = match docker_dir_path.file_name() {
    Some(name) => format!("{}.txt", name.to_string_lossy()),
    None => return Err(ERROR_CANNOT_DETERMINE_DOCKER_DIR.into()),
  };

  let version_file_path = PathBuf::from(versions_dir).join(version_file_name);

  // Leggiamo variabili precedenti, o creiamo nuove se file inesistente
  let mut env_vars = if version_file_path.exists() {
    read_env_file(version_file_path.to_str().unwrap())?
  } else {
    HashMap::new()
  };

  // Controlliamo MD5 esistente
  let stored_md5 = env_vars.get(VERSION_KEY_MD5).unwrap_or(&String::new()).clone();

  if stored_md5 == current_md5 {
    if verbose {
      println!(
        "{} aggiornato, nessun avanzamento di versione necessario.",
        docker_dir
      );
    }
    return Ok(());
  }

  // MD5 diversi: aggiorniamo la versione PATCH
  let major = env_vars
    .get(VERSION_KEY_MAJOR)
    .and_then(|v| v.parse::<u32>().ok())
    .unwrap_or(0);

  let minor = env_vars
    .get(VERSION_KEY_MINOR)
    .and_then(|v| v.parse::<u32>().ok())
    .unwrap_or(0);

  let patch = env_vars
    .get(VERSION_KEY_PATCH)
    .and_then(|v| v.parse::<u32>().ok())
    .unwrap_or(0)
    + 1; // incrementiamo patch

  // Aggiorniamo hashmap
  env_vars.insert(VERSION_KEY_MD5.into(), current_md5.to_string());
  env_vars.insert(VERSION_KEY_MAJOR.into(), major.to_string());
  env_vars.insert(VERSION_KEY_MINOR.into(), minor.to_string());
  env_vars.insert(VERSION_KEY_PATCH.into(), patch.to_string());
  env_vars.insert(
    VERSION_KEY_FULL.into(),
    format!("{}.{}.{}", major, minor, patch),
  );

  // Prepariamo eventualmente la cartella destinazione
  fs::create_dir_all(&versions_dir)?;

  // Scriviamo file aggiornato
  write_env_file(version_file_path.to_str().unwrap(), &env_vars)?;

  if verbose {
    println!(
      "{} aggiornato e avanzamento versione effettuato: {}.{}.{}",
      docker_dir, major, minor, patch
    );
  }

  Ok(())
}

/// Esegue un comando Docker con le variabili d'ambiente e le configurazioni appropriate.
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
    let docker_socket = format!("{}:{}", DOCKER_SOCKET_PATH, DOCKER_SOCKET_PATH);
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
      let home_directory = get_home_directory().ok_or(
        ERROR_CANNOT_DETERMINE_HOME,
      )?;
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
