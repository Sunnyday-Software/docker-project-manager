use md5::{Digest, Md5};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

/// Legge un file .env e restituisce le variabili d'ambiente come HashMap.
///
/// # Arguments
/// * `path` - Percorso del file .env da leggere
///
/// # Returns
/// * `io::Result<HashMap<String, String>>` - HashMap contenente le variabili d'ambiente lette dal file,
///   o HashMap vuoto se il file non esiste
///
/// # Note
/// - Le righe che iniziano con '#' vengono considerate commenti e ignorate
/// - Le righe vuote vengono ignorate
/// - Le variabili devono essere nel formato KEY=VALUE
pub fn read_env_file(path: &str) -> io::Result<HashMap<String, String>> {
  let mut env_map = HashMap::new();
  if Path::new(path).exists() {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
      let line = line?;
      // Salta le righe di commento o vuote
      if line.starts_with('#') || line.trim().is_empty() {
        continue;
      }
      if let Some((key, value)) = line.split_once('=') {
        env_map.insert(key.to_string(), value.to_string());
      }
    }
  }
  Ok(env_map)
}

/// Scrive le variabili d'ambiente in un file .env.
///
/// # Arguments
/// * `path` - Percorso del file .env da scrivere
/// * `env_map` - HashMap contenente le variabili d'ambiente da scrivere
///
/// # Returns
/// * `io::Result<()>` - Ok se la scrittura è avvenuta con successo, Err altrimenti
///
/// # Note
/// - Le variabili vengono scritte in ordine alfabetico per chiave
/// - Il file viene creato se non esiste, altrimenti viene sovrascritto
pub fn write_env_file(
  path: &str,
  env_map: &HashMap<String, String>,
) -> io::Result<()> {
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)?;
  let mut keys: Vec<_> = env_map.keys().collect();
  keys.sort(); // Ordina le chiavi alfabeticamente

  for key in keys {
    if let Some(value) = env_map.get(key) {
      writeln!(file, "{}={}", key, value)?;
    }
  }

  Ok(())
}

/// Scrive le variabili d'ambiente in un file .env, con messaggio di log.
///
/// # Arguments
/// * `env_file` - Percorso del file .env da scrivere
/// * `env_vars` - HashMap contenente le variabili d'ambiente da scrivere
/// * `verbose` - Flag per abilitare l'output verboso
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok se la scrittura è avvenuta con successo, Err altrimenti
pub fn write_env_to_file(
  env_file: &str,
  env_vars: &HashMap<String, String>,
  verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  use crate::model::MSG_WRITING_ENV_FILE;
  if verbose {
    println!("{}", MSG_WRITING_ENV_FILE);
  }
  write_env_file(env_file, env_vars)?;
  if verbose {
    println!("Environment file written successfully: {}", env_file);
  }
  Ok(())
}

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
pub fn compute_dir_md5(dir: &str) -> Result<String, Box<dyn std::error::Error>> {
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
