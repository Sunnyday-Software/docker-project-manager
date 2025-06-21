use regex::{Captures, Regex};
use std::collections::HashMap;
use std::path::Path;
use std::{env, io};

use crate::file_ops::{compute_dir_md5, read_env_file};
use crate::model::*;
use crate::utils::get_user_ids;

/// Carica in modo opzionale un file .env, ritornando una mappa vuota se il file non esiste.
///
/// # Arguments
/// * `path` - Percorso del file .env da leggere
///
/// # Returns
/// * `io::Result<HashMap<String, String>>` - HashMap contenente le variabili d'ambiente lette dal file,
///   o HashMap vuoto se il file non esiste
pub fn try_read_env_file(path: &str) -> io::Result<HashMap<String, String>> {
  if Path::new(path).exists() {
    read_env_file(path)
  } else {
    Ok(HashMap::new())
  }
}

/// Espande le variabili d'ambiente contenute nei valori di un HashMap.
///
/// # Arguments
/// * `input` - HashMap contenente le variabili d'ambiente da espandere
///
/// # Returns
/// * `HashMap<String, String>` - HashMap con le variabili d'ambiente espanse
///
/// # Note
/// - Le variabili d'ambiente sono nel formato ${NOME_VARIABILE}
/// - Se una variabile d'ambiente non è definita, la coppia chiave-valore viene omessa
/// - Solo le variabili con tutti i riferimenti risolti vengono incluse nel risultato
pub fn expand_env_vars(input: &HashMap<String, String>) -> HashMap<String, String> {
  let re = Regex::new(ENV_VAR_PATTERN).unwrap();
  let mut expanded_map = HashMap::new();

  for (key, value) in input {
    let mut has_missing_variable = false;

    let expanded_value = re.replace_all(value, |caps: &Captures| {
      let var_name = &caps[1];
      match env::var(var_name) {
        Ok(env_value) => env_value,
        Err(_) => {
          has_missing_variable = true;
          String::new() // Anche se non effettivamente utilizzato, necessario per il closure
        }
      }
    });

    // Solo se tutte le variabili trovate avevano valori validi, inserisco la variabile
    if !has_missing_variable {
      expanded_map.insert(key.clone(), expanded_value.to_string());
    }
    // Altrimenti: variabile omessa secondo le specifiche
  }

  expanded_map
}

/// Combina le variabili d'ambiente da diversi file .env in un unico HashMap.
///
/// # Arguments
/// * `input_env_file` - Percorso del file .env di input specificato dall'utente
/// * `verbose` - Flag per abilitare l'output verboso
///
/// # Returns
/// * `io::Result<HashMap<String, String>>` - HashMap contenente tutte le variabili d'ambiente combinate
///
/// # Note
/// - Legge variabili da .env (se presente)
/// - Legge variabili da .env.local (se presente), che sovrascrivono quelle di .env
/// - Legge variabili dal file di input specificato (se diverso dai precedenti), che sovrascrivono le precedenti
/// - Verifica la presenza di variabili obbligatorie e fornisce valori di default se necessario
/// - Espande le variabili d'ambiente nei valori
pub fn combine_env_files(
  input_env_file: &str,
  verbose: bool,
) -> io::Result<HashMap<String, String>> {
  use crate::model::{MSG_COMBINING_ENV_FILES, MSG_READING_ENV_FILE, MSG_ENV_FILE_NOT_FOUND};

  if verbose {
    println!("{}", MSG_COMBINING_ENV_FILES);
  }

  // legge variabili da .env, se presente
  if verbose {
    if Path::new(ENV_FILE).exists() {
      println!("{}", MSG_READING_ENV_FILE.replace("{}", ENV_FILE));
    } else {
      println!("{}", MSG_ENV_FILE_NOT_FOUND.replace("{}", ENV_FILE));
    }
  }
  let mut combined_env = expand_env_vars(&try_read_env_file(ENV_FILE)?);

  // Controlla se il file .env contiene variabili che andrebbero da un'altra parte
  if combined_env.contains_key(ENV_DOCKER_HOST_MAP) {
    println!("{}", WARNING_DOCKER_HOST_MAP_IN_ENV);
  }

  if !combined_env.contains_key(ENV_PROJECT_NAME) {
    println!("{}", WARNING_PROJECT_NAME_MISSING);
    combined_env.insert(ENV_PROJECT_NAME.to_string(), DEFAULT_PROJECT_NAME.to_string());
  }

  // legge variabili da .env.local, se presente, sovrascrivendo quelle precedenti
  if Path::new(ENV_LOCAL_FILE).exists() {
    if verbose {
      println!("{}", MSG_READING_ENV_FILE.replace("{}", ENV_LOCAL_FILE));
    }
    let local_env = expand_env_vars(&try_read_env_file(ENV_LOCAL_FILE)?);
    for (k, v) in local_env {
      combined_env.insert(k, v);
    }
  } else if verbose {
    println!("{}", MSG_ENV_FILE_NOT_FOUND.replace("{}", ENV_LOCAL_FILE));
  }

  // legge variabili dal file di input specificato, se presente e diverso da .env o .env.local
  if input_env_file != ENV_FILE
    && input_env_file != ENV_LOCAL_FILE
    && Path::new(input_env_file).exists()
  {
    if verbose {
      println!("{}", MSG_READING_ENV_FILE.replace("{}", input_env_file));
    }
    let input_env = expand_env_vars(&try_read_env_file(input_env_file)?);
    for (k, v) in input_env {
      combined_env.insert(k, v);
    }
  } else if verbose && input_env_file != ENV_FILE && input_env_file != ENV_LOCAL_FILE {
    println!("{}", MSG_ENV_FILE_NOT_FOUND.replace("{}", input_env_file));
  }

  Ok(combined_env)
}

/// Crea una mappa delle directory Docker, calcola gli hash MD5 e prepara le variabili d'ambiente.
///
/// # Arguments
/// * `docker_dev_path` - Percorso della directory dev/docker
/// * `host_project_path_str` - Percorso del progetto host
/// * `verbose` - Flag per abilitare l'output verboso
///
/// # Returns
/// * `Result<(HashMap<String, String>, HashMap<String, String>, HashMap<String, String>), Box<dyn std::error::Error>>` - Tupla contenente:
///   - Mappa delle directory e delle rispettive variabili d'ambiente
///   - Variabili d'ambiente da passare al comando Docker
///   - Valori MD5 calcolati per ogni directory
///
/// # Note
/// - Scansiona tutte le sottodirectory in dev/docker
/// - Calcola l'hash MD5 per ogni sottodirectory
/// - Aggiunge variabili d'ambiente specifiche del sistema (UID, GID, ecc.)
/// - Prepara tutte le variabili necessarie per l'esecuzione dei comandi Docker
pub fn create_dir_env_map_and_calculate_md5(
  docker_dev_path: &Path,
  host_project_path_str: &str,
  verbose: bool,
) -> Result<
  (
    HashMap<String, String>,
    HashMap<String, String>,
    HashMap<String, String>,
  ),
  Box<dyn std::error::Error>,
> {
  use crate::model::{MSG_SCANNING_DOCKER_DIRS, MSG_DOCKER_FOLDER_MAPPING, MSG_CALCULATING_MD5, MSG_MD5_CALCULATED};

  if verbose {
    println!("{}", MSG_SCANNING_DOCKER_DIRS);
  }

  // Mappa delle directory e delle rispettive variabili d'ambiente
  let mut dir_env_map = HashMap::new();

  for entry in docker_dev_path.read_dir()? {
    if let Ok(entry) = entry {
      if entry.file_type()?.is_dir() {
        if let Some(subdir_name) = entry.file_name().to_str() {
          let env_var = format!("MD5_{}", subdir_name.to_uppercase());
          let dir_path = entry.path().to_str().unwrap().to_string();
          dir_env_map.insert(env_var.clone(), dir_path.clone());
          if verbose {
            println!("{}", MSG_DOCKER_FOLDER_MAPPING.replace("{}", &env_var).replace("{}", &dir_path));
          }
        }
      }
    }
  }

  // HashMap per conservare le variabili d'ambiente da passare al comando Docker
  let mut env_vars = HashMap::new();

  // Calcola gli MD5 e prepara le variabili d'ambiente
  let mut md5_values = HashMap::new();
  for (env_var, dir_path) in &dir_env_map {
    if verbose {
      println!("{}", MSG_CALCULATING_MD5.replace("{}", dir_path));
    }
    let md5_value = compute_dir_md5(dir_path)?;
    if verbose {
      println!("{}", MSG_MD5_CALCULATED.replace("{}", dir_path).replace("{}", &md5_value));
    }
    env_vars.insert(env_var.to_string(), md5_value.clone());
    md5_values.insert(dir_path.clone(), md5_value.clone());
  }

  // Aggiungi HOST_PROJECT_PATH alle variabili d'ambiente
  env_vars.insert(
    "HOST_PROJECT_PATH".to_string(),
    host_project_path_str.to_string(),
  );

  // Aggiungi UID e GID se su Linux/Mac
  if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
    let (n_uid, n_gid, user_name) = get_user_ids();
    let uid = n_uid.to_string();
    let gid = n_gid.to_string();

    env_vars.insert("HOST_UID".to_string(), uid);
    env_vars.insert("HOST_GID".to_string(), gid);
    env_vars.insert("HOST_USER".to_string(), user_name.to_string());
  }

  Ok((dir_env_map, env_vars, md5_values))
}
