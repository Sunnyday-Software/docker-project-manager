use md5::{Digest, Md5};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

/// Calcola l'hash MD5 di una directory, considerando tutti i file
/// in essa contenuti.
///
/// # Arguments
/// * `dir` - Percorso della directory di cui calcolare l'hash MD5
///
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - I primi 8 caratteri
/// dell'hash MD5 calcolato, o un errore se la directory non esiste o non
/// è valida
///
/// # Note
/// - L'hash viene calcolato considerando tutti i file nella directory e nelle
///   sottodirectory
/// - I file vengono ordinati alfabeticamente per garantire coerenza nel calcolo
/// - Per ogni file viene calcolato l'MD5, poi tutti gli MD5 vengono concatenati
/// - Viene calcolato l'MD5 della concatenazione e vengono restituiti
///   i primi 8 caratteri
pub fn compute_dir_md5(
  dir: &str,
) -> Result<String, Box<dyn std::error::Error>> {
  let path = Path::new(dir);

  // Verifica che il percorso sia una directory esistente
  if !path.is_dir() {
    eprintln!("Errore: '{}' non è una directory valida o non esiste.", dir);
    return Err("Directory non valida".into());
  }

  // Colleziona tutti i file nella directory, ricorsivamente
  let mut file_paths = Vec::new();
  for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
    if entry.file_type().is_file() {
      file_paths.push(entry.path().to_owned());
    }
  }

  // Ordina i percorsi dei file per garantire coerenza
  file_paths.sort();

  let mut md5_sums = Vec::new();

  // Calcola l'MD5 di ogni file
  for file_path in file_paths {
    let mut file = File::open(&file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let mut hasher = Md5::new();
    hasher.update(&contents);
    let result = hasher.finalize();

    md5_sums.push(format!("{:x}", result));
  }

  // Concatenazione di tutti gli MD5
  let concatenated_md5s = md5_sums.join("");

  // Calcola l'MD5 della concatenazione
  let mut final_hasher = Md5::new();
  final_hasher.update(concatenated_md5s.as_bytes());
  let final_result = final_hasher.finalize();
  let final_md5 = format!("{:x}", final_result);

  // Prende i primi 8 caratteri
  let md5_short = &final_md5[..8];

  Ok(md5_short.to_string())
}

/// Read environment variables from a .env file
///
/// # Arguments
/// * `path` - Path to the .env file to read
///
/// # Returns
/// * `io::Result<HashMap<String, String>>` - HashMap containing the environment variables
pub fn read_env_file(path: &str) -> io::Result<HashMap<String, String>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let mut env_vars = HashMap::new();

  for line in reader.lines() {
    let line = line?;
    let trimmed = line.trim();

    // Skip empty lines and comments
    if trimmed.is_empty() || trimmed.starts_with('#') {
      continue;
    }

    // Parse key=value format
    if let Some(eq_pos) = trimmed.find('=') {
      let key = trimmed[..eq_pos].trim().to_string();
      let value = trimmed[eq_pos + 1..].trim().to_string();

      if !key.is_empty() {
        env_vars.insert(key, value);
      }
    }
  }

  Ok(env_vars)
}

/// Write environment variables to a .env file
///
/// # Arguments
/// * `path` - Path to the .env file to write
/// * `env_vars` - HashMap containing the environment variables to write
///
/// # Returns
/// * `io::Result<()>` - Result indicating success or failure
pub fn write_env_file(path: &str, env_vars: &HashMap<String, String>) -> io::Result<()> {
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)?;

  // Collect and sort keys alphabetically
  let mut keys: Vec<&String> = env_vars.keys().collect();
  keys.sort();

  // Write entries in alphabetical order by key
  for key in keys {
    let value = &env_vars[key];
    writeln!(file, "{}={}", key, value)?;
  }

  Ok(())
}
