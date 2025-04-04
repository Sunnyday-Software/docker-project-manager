use std::collections::HashMap;
use std::{env, io};
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;
use md5::{Md5, Digest};


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

fn combine_env_files() -> io::Result<HashMap<String, String>> {
    // legge variabili da .env, se presente
    let mut combined_env = try_read_env_file(".env")?;


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
        let local_env = try_read_env_file(".env.local")?;
        for (k, v) in local_env {
            combined_env.insert(k, v);
        }
    }

    Ok(combined_env)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

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
        env_vars.insert(env_var.to_string(), md5_value);
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
    write_env_file(".env.docker", &existing_env_vars)?;

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


/*        if docker_socket_path != "/var/run/docker.sock" {
            env_vars.insert("DOCKER_HOST".to_string(), format!("unix://{}", docker_socket_path));
        }*/
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
    let args: Vec<String> = env::args().skip(1).collect();
    command.args(&args);

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
