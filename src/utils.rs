use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

#[cfg(windows)]
use dirs::home_dir;
#[cfg(unix)]
use uzers::get_current_username;
#[cfg(unix)]
use uzers::os::unix::UserExt;
#[cfg(unix)]
use uzers::{get_current_gid, get_current_uid, get_user_by_uid};

use crate::context::Context;
use crate::docker::process_docker_version;
use crate::model::*;

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
