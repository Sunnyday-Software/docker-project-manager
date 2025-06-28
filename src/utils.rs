use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::env;

#[cfg(windows)]
use dirs::home_dir;
#[cfg(unix)]
use uzers::get_current_username;
#[cfg(unix)]
use uzers::os::unix::UserExt;
#[cfg(unix)]
use uzers::{get_current_gid, get_current_uid, get_user_by_uid};

use crate::docker::process_docker_version;
use crate::model::*;
use crate::context::Context;

#[cfg(unix)]
/// Ottiene l'ID utente, l'ID gruppo e il nome utente corrente sui sistemi Unix.
///
/// # Returns
/// Una tupla contenente:
/// - UID (User ID) dell'utente corrente
/// - GID (Group ID) dell'utente corrente
/// - Nome utente come stringa, o stringa vuota in caso di errore
pub fn get_user_ids() -> (u32, u32, String) {
  (
    get_current_uid(),
    get_current_gid(),
    get_current_username()
      .and_then(|os_str| os_str.into_string().ok())
      .unwrap_or_else(|| "".to_string()),
  )
}

#[cfg(windows)]
/// Ottiene l'ID utente, l'ID gruppo e il nome utente corrente sui sistemi
/// Windows.
///
/// # Returns
/// Una tupla contenente:
/// - UID (User ID) impostato a 0 su Windows
/// - GID (Group ID) impostato a 0 su Windows
/// - Nome utente dalla variabile d'ambiente USERNAME, o stringa vuota in caso
///   di errore
pub fn get_user_ids() -> (u32, u32, String) {
  let username = env::var("USERNAME").unwrap_or_else(|_| "".to_string());
  (0, 0, username)
}

/// Verifica se un socket Unix esiste nel percorso specificato.
///
/// # Arguments
/// * `path` - Percorso del socket da verificare
///
/// # Returns
/// * `bool` - `true` se il socket esiste, `false` altrimenti
pub fn socket_exists(path: &str) -> bool {
  Path::new(path).exists()
}

#[cfg(unix)]
/// Ottiene il percorso della home directory dell'utente corrente sui sistemi Unix.
///
/// # Returns
/// * `Option<PathBuf>` - Percorso della home directory se disponibile, `None` altrimenti
pub fn get_home_directory() -> Option<PathBuf> {
  let (uid, _, _) = get_user_ids();
  get_user_by_uid(uid)
    .and_then(|user| PathBuf::from(user.home_dir()).to_str().map(PathBuf::from))
}

#[cfg(windows)]
/// Ottiene il percorso della home directory dell'utente corrente sui sistemi Windows.
///
/// # Returns
/// * `Option<PathBuf>` - Attualmente restituisce sempre `None` su Windows
pub fn get_home_directory() -> Option<PathBuf> {
  return dirs::home_dir();
}

/// Configura i percorsi del progetto necessari per l'esecuzione.
///
/// # Arguments
/// * `docker_dev_path_str` - Percorso della directory dev/docker come stringa
/// * `verbose` - Flag per abilitare l'output verboso
///
/// # Returns
/// * `Result<(String, PathBuf), Box<dyn std::error::Error>>` - Tupla contenente:
///   - Il percorso del progetto host come stringa
///   - Il percorso della directory dev/docker come PathBuf
///
/// # Errors
/// Restituisce un errore se:
/// - Non è possibile determinare la directory corrente
/// - Il percorso non è valido o non può essere convertito in stringa
/// - La directory dev/docker non esiste o non è valida
pub fn setup_project_paths(docker_dev_path_str: &str, verbose: bool) -> Result<(String, PathBuf), Box<dyn std::error::Error>>
{
  // Percorso del progetto host
  let host_project_path = env::current_dir()?;
  let host_project_path_str =
    host_project_path.to_str().ok_or(ERROR_INVALID_PATH)?;

  if verbose {
    println!("RUST Project builder");
    println!("* host_project_path: {}", host_project_path_str);
  }

  let docker_dev_path = Path::new(docker_dev_path_str);

  if verbose {
    println!("* docker_dev_path: {}", docker_dev_path.display());
    println!("");
  }
  // Verifica che il percorso sia una directory
  if !docker_dev_path.is_dir() {
    eprintln!(
      "Errore: '{}' non è una directory valida o non esiste.",
      docker_dev_path.display()
    );
    return Err(ERROR_INVALID_DIRECTORY.into());
  }

  Ok((
    host_project_path_str.to_string(),
    docker_dev_path.to_path_buf(),
  ))
}

/// Aggiorna le versioni di tutti i componenti Docker basandosi sui loro hash MD5.
///
/// # Arguments
/// * `md5_values` - HashMap contenente i percorsi delle directory e i relativi hash MD5
/// * `versions_folder` - Directory dove salvare i file di versioning
/// * `verbose` - Flag per abilitare l'output verboso
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok se tutte le operazioni sono completate con successo, Err altrimenti
///
/// # Note
/// - Itera su tutti i componenti e aggiorna le loro versioni se necessario
/// - Utilizza `process_docker_version` per gestire il versioning di ogni singolo componente
pub fn update_versions(
  md5_values: &HashMap<String, String>,
  versions_folder: &str,
  verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  if verbose {
    println!("{}", MSG_UPDATING_VERSIONS);
  }
  for (dir_path, md5_value) in md5_values {
    process_docker_version(dir_path, md5_value, versions_folder, verbose)?;
  }
  Ok(())
}

/// Prints a debug message if debug_print is enabled in the context.
///
/// # Arguments
/// * `ctx` - The execution context containing the debug_print flag
/// * `module_name` - The name of the module/command issuing the debug message
/// * `description` - Description of what is being done
///
/// # Format
/// The debug message is printed in the format: "module-name: description"
pub fn debug_log(ctx: &Context, module_name: &str, description: &str) {
  if ctx.get_debug_print() {
    println!("{}: {}", module_name, description);
  }
}
