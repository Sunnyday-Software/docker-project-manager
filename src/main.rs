use std::collections::HashMap;
use std::{env, fs, io};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;
use md5::{Md5, Digest};
use clap::Parser;
use regex::{Regex, Captures};



#[cfg(unix)]
use uzers::os::unix::UserExt;
#[cfg(unix)]
use uzers::{get_current_uid, get_current_gid, get_user_by_uid};

#[cfg(unix)]
fn get_user_ids() -> (u32, u32) {
    (get_current_uid(), get_current_gid())
}

#[cfg(windows)]
fn get_user_ids() -> (u32, u32) {
    (0, 0)
}


/// Struttura per gestire gli argomenti passati da riga di comando
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// File .env da utilizzare
    #[arg(short, long, default_value = ".env.docker")]
    env: String,

    #[arg(short, long, default_value = "false")]
    skip_env_write:bool,

    #[arg(short, long, default_value = "false")]
    update_versions:bool,

    /// Tutti i parametri aggiuntivi che saranno passati direttamente al comando finale
    #[arg(trailing_var_arg = true)]
    args: Vec<String>,
}


fn read_env_file(path: &str) -> io::Result<HashMap<String, String>> {
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

fn write_env_file(path: &str, env_map: &HashMap<String, String>) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
    let mut keys: Vec<_> = env_map.keys().collect();
    keys.sort(); // Ordina le chiavi alfabeticamente

    for key in keys {
        if let Some(value) = env_map.get(key) {
            writeln!(file, "{}={}", key, value)?;
        }
    }

    Ok(())
}

fn compute_dir_md5(dir: &str) -> Result<String, Box<dyn std::error::Error>> {
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

fn socket_exists(path: &str) -> bool {
    Path::new(path).exists()
}

#[cfg(unix)]
fn get_home_directory() -> Option<PathBuf> {
    let (uid, _) = get_user_ids();
    get_user_by_uid(uid).and_then(|user| PathBuf::from(user.home_dir()).to_str().map(PathBuf::from))
}

#[cfg(windows)]
fn get_home_directory() -> Option<PathBuf> {
    return Option::None
}


// Questa funzione carica in modo opzionale un file env,
// ritornando una mappa vuota in caso il file non esista o non contenga variabili
fn try_read_env_file(path: &str) -> io::Result<HashMap<String, String>> {
    if Path::new(path).exists() {
        read_env_file(path)
    } else {
        Ok(HashMap::new())
    }
}


fn expand_env_vars(input: &HashMap<String, String>) -> HashMap<String, String> {
    let re = Regex::new(r"\$\{(\w+)\}").unwrap();
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
                },
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


fn combine_env_files() -> io::Result<HashMap<String, String>> {
    // legge variabili da .env, se presente
    let mut combined_env = expand_env_vars(&try_read_env_file(".env")?);


    // Controlla se il file .env contiene variabili che andrebbero da un'altra parte
    if combined_env.contains_key("DOCKER_HOST_MAP") {
        println!("Avviso: La variabile 'DOCKER_HOST_MAP' è presente in .env. Si consiglia di spostarla in .env.local.");
    }

    if !combined_env.contains_key("PROJECT_NAME") {
        println!("ERROR: La variabile 'PROJECT_NAME' non è presente in .env.");
        combined_env.insert("PROJECT_NAME".to_string(), "NoName".to_string());
    }



    // legge variabili da .env.local, se presente, sovrascrivendo quelle precedenti
    if Path::new(".env.local").exists() {
        let local_env = expand_env_vars(&try_read_env_file(".env.local")?);
        for (k, v) in local_env {
            combined_env.insert(k, v);
        }
    }

    Ok(combined_env)
}


fn process_docker_version(
    docker_dir: &str,
    current_md5: &str,
    versions_dir: &str
) -> Result<(), Box<dyn Error>> {
    let docker_dir_path = Path::new(docker_dir);
    let version_file_name = match docker_dir_path.file_name() {
        Some(name) => format!("{}.txt", name.to_string_lossy()),
        None => return Err("Cannot determine docker directory name".into()),
    };

    let version_file_path = PathBuf::from(versions_dir).join(version_file_name);

    // Leggiamo variabili precedenti, o creiamo nuove se file inesistente
    let mut env_vars = if version_file_path.exists() {
        read_env_file(version_file_path.to_str().unwrap())?
    } else {
        HashMap::new()
    };

    // Controlliamo MD5 esistente
    let stored_md5 = env_vars.get("md5").unwrap_or(&String::new()).clone();

    if stored_md5 == current_md5 {
        println!("{} aggiornato, nessun avanzamento di versione necessario.",docker_dir);
        return Ok(());
    }

    // MD5 diversi: aggiorniamo la versione PATCH
    let major = env_vars
        .get("v_major")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);

    let minor = env_vars
        .get("v_minor")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);

    let patch = env_vars
        .get("v_patch")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0) + 1; // incrementiamo patch

    // Aggiorniamo hashmap
    env_vars.insert("md5".into(), current_md5.to_string());
    env_vars.insert("v_major".into(), major.to_string());
    env_vars.insert("v_minor".into(), minor.to_string());
    env_vars.insert("v_patch".into(), patch.to_string());
    env_vars.insert("v_full_version".into(), format!("{}.{}.{}",major,minor,patch));

    // Prepariamo eventualmente la cartella destinazione
    fs::create_dir_all(&versions_dir)?;

    // Scriviamo file aggiornato
    write_env_file(version_file_path.to_str().unwrap(), &env_vars)?;

    println!(
        "{} aggiornato e avanzamento versione effettuato: {}.{}.{}",docker_dir,
        major, minor, patch
    );

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let versions_folder = "dev/docker_versions";

    // Parsing dei parametri della linea di comando
    let cli = Cli::parse();

    println!("File .env selezionato: {}", cli.env);
    if !cli.args.is_empty() {
        println!("Argomenti aggiuntivi ricevuti: {:?}", cli.args);
    }


    // Percorso del progetto host
    let host_project_path = env::current_dir()?;
    let host_project_path_str = host_project_path.to_str().ok_or("Percorso non valido")?;
    println!("RUST Project builder");
    println!("* host_project_path: {}", host_project_path_str);

    let docker_dev_path = Path::new("./dev/docker");

    println!("* docker_dev_path: {}", docker_dev_path.display());
    println!("");
    // Verifica che il percorso sia una directory
    if !docker_dev_path.is_dir() {
        eprintln!("Errore: '{}' non è una directory valida o non esiste.", docker_dev_path.display());
        return Err("Directory non valida".into());
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
                    println!("* docker folder {} -> {}", env_var.clone(), dir_path.clone());
                }
            }
        }
    }

    // HashMap per conservare le variabili d'ambiente da passare al comando Docker
    let mut env_vars = HashMap::new();

    // Calcola gli MD5 e prepara le variabili d'ambiente
    for (env_var, dir_path) in &dir_env_map {
        let md5_value = compute_dir_md5(dir_path)?;
        env_vars.insert(env_var.to_string(), md5_value.clone());
        if cli.update_versions == true {
            process_docker_version(dir_path, &md5_value, versions_folder)?;
        }
    }

    // Aggiungi HOST_PROJECT_PATH alle variabili d'ambiente
    env_vars.insert(
        "HOST_PROJECT_PATH".to_string(),
        host_project_path_str.to_string(),
    );

    // Aggiungi UID e GID se su Linux/Mac
    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        let (n_uid, n_gid) = get_user_ids();
        let uid = n_uid.to_string();
        let gid = n_gid.to_string();

        env_vars.insert("HOST_UID".to_string(), uid);
        env_vars.insert("HOST_GID".to_string(), gid);
    }

    // Lettura del file .env e .env.local e aggiornamento delle variabili
    let mut existing_env_vars = combine_env_files()?;
    for (key, value) in &env_vars {
        existing_env_vars.insert(key.clone(), value.clone());
    }

    // Scrittura delle variabili aggiornate nel file .env
    if cli.skip_env_write == false {
        write_env_file(&cli.env, &existing_env_vars)?;
    }

    //le variabili ambientali mancanti e presenti in .env vengono aggiunte
    for (key, value) in existing_env_vars.clone() {
        if !env_vars.contains_key(&key) {
            env_vars.insert(key, value);
        }
    }
    // Prepara il comando Docker
    let mut command = Command::new("docker");
    command.args(&["compose", "run", "--rm", "--no-deps"]);

    // Mapping dei volumi (adattato per compatibilità cross-platform)
    if cfg!(target_os = "windows") {
        // Su Windows, il socket Docker si gestisce diversamente o si omette
        let docker_socket = "/var/run/docker.sock:/var/run/docker.sock";
        command.args(&["-v", docker_socket]);
        println!("Docker Socket mapping: {}", docker_socket);
    } else {

        // Controlla se esiste la variabile DOCKER_HOST nel file .env
        if let Some(docker_host_map) = existing_env_vars.get("DOCKER_HOST_MAP") {
            println!("Utilizzo DOCKER_HOST_MAP dal file .env: {}", docker_host_map);
            command.args(&["-v", &*docker_host_map]);
        } else {
            // Se non esiste, trova il primo socket disponibile
            let home_directory = get_home_directory().ok_or("Impossibile determinare la home directory dell'utente corrente")?;
            let docker_socket_path = if socket_exists("/var/run/docker.sock") {
                "/var/run/docker.sock".to_string()
            } else if socket_exists(&format!("{}/.docker/desktop/docker.sock", home_directory.to_str().unwrap())) {
                format!("{}/.docker/desktop/docker.sock", home_directory.to_str().unwrap())
            } else if let Ok(xdg_runtime_dir) = env::var("XDG_RUNTIME_DIR") {
                format!("{}/docker.sock", xdg_runtime_dir)
            } else {
                "/var/run/docker.sock".to_string()
            };
            // Mapping dei volumi
            let docker_socket = format!("{}:/var/run/docker.sock", docker_socket_path);
            command.args(&["-v", &*docker_socket]);
            println!("Docker Socket mapping: {}", docker_socket);
        };

    }

    // Imposta le variabili d'ambiente nell'ambiente del processo
    for (key, value) in &env_vars {
        command.env(key, value);
        println!("* env key: {} = {}", key, value);
    }

    // Passa a Docker solo i nomi delle variabili d'ambiente
    for key in env_vars.keys() {
        command.args(&["-e", key]);
    }

    // Creazione della stringa concatenata di tutte le chiavi
    let concatenated_keys = env_vars.keys().cloned().collect::<Vec<_>>().join(";");
    command.env("DOCKER_ENV_KEYS", concatenated_keys);
    command.args(&["-e", "DOCKER_ENV_KEYS"]);

    // Specifica il servizio e il comando da eseguire
    command.args(&["make", "make"]);

    // Aggiunge eventuali argomenti aggiuntivi passati al programma
    //let args: Vec<String> = env::args().skip(1).collect();
    command.args(&cli.args);

    // Stampa del comando completo (per il debug)
    println!("Eseguendo il comando: {:?}", command);

    // Esegue il comando Docker
    let status = command.status()?;

    if !status.success() {
        eprintln!("Il comando Docker non è riuscito");
        std::process::exit(1);
    }

    Ok(())
}
