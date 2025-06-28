use crate::{CommandRegistry, Context, Value, tags};
use crate::utils::debug_log;
use std::path::Path;

/// Register path commands
pub fn register_path_commands(registry: &mut CommandRegistry) {
    // rust-path-join command
    registry.register_closure_with_help_and_tag(
        "rust-path-join",
        "Join path components together",
        "(rust-path-join base component1 component2 ...)",
        "  (rust-path-join \"/home\" \"user\" \"documents\")  ; Returns /home/user/documents\n  (rust-path-join \"..\" \"project\" \"src\")  ; Returns ../project/src",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-path", "executing rust-path-join command");

            if args.is_empty() {
                return Err("rust-path-join expects at least one argument".to_string());
            }

            let mut path_components = Vec::new();
            for arg in &args {
                match arg {
                    Value::Str(s) => path_components.push(s.clone()),
                    _ => return Err("rust-path-join all arguments must be strings".to_string()),
                }
            }

            debug_log(ctx, "rust-path", &format!("joining {} path components", path_components.len()));
            let mut result_path = Path::new(&path_components[0]).to_path_buf();
            for component in &path_components[1..] {
                result_path = result_path.join(component);
            }

            debug_log(ctx, "rust-path", &format!("path joined successfully: {}", result_path.display()));
            Ok(Value::Str(result_path.to_string_lossy().to_string()))
        },
    );

    // rust-path-parent command
    registry.register_closure_with_help_and_tag(
        "rust-path-parent",
        "Get the parent directory of a path",
        "(rust-path-parent path)",
        "  (rust-path-parent \"/home/user/file.txt\")  ; Returns /home/user\n  (rust-path-parent \"../project/src\")  ; Returns ../project",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-path", "executing rust-path-parent command");

            if args.len() != 1 {
                return Err("rust-path-parent expects exactly one argument (path)".to_string());
            }

            let path_str = match &args[0] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-path-parent path must be a string".to_string()),
            };

            debug_log(ctx, "rust-path", &format!("getting parent directory of: {}", path_str));
            let path = Path::new(&path_str);
            match path.parent() {
                Some(parent) => {
                    debug_log(ctx, "rust-path", &format!("parent directory found: {}", parent.display()));
                    Ok(Value::Str(parent.to_string_lossy().to_string()))
                },
                None => {
                    debug_log(ctx, "rust-path", "no parent directory found");
                    Ok(Value::Nil)
                },
            }
        },
    );

    // rust-path-filename command
    registry.register_closure_with_help_and_tag(
        "rust-path-filename",
        "Get the filename component of a path",
        "(rust-path-filename path)",
        "  (rust-path-filename \"/home/user/file.txt\")  ; Returns file.txt\n  (rust-path-filename \"../project/src/main.rs\")  ; Returns main.rs",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-path", "executing rust-path-filename command");

            if args.len() != 1 {
                return Err("rust-path-filename expects exactly one argument (path)".to_string());
            }

            let path_str = match &args[0] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-path-filename path must be a string".to_string()),
            };

            debug_log(ctx, "rust-path", &format!("extracting filename from: {}", path_str));
            let path = Path::new(&path_str);
            match path.file_name() {
                Some(filename) => {
                    debug_log(ctx, "rust-path", &format!("filename extracted: {}", filename.to_string_lossy()));
                    Ok(Value::Str(filename.to_string_lossy().to_string()))
                },
                None => {
                    debug_log(ctx, "rust-path", "no filename found");
                    Ok(Value::Nil)
                },
            }
        },
    );

    // rust-path-extension command
    registry.register_closure_with_help_and_tag(
        "rust-path-extension",
        "Get the file extension of a path",
        "(rust-path-extension path)",
        "  (rust-path-extension \"file.txt\")  ; Returns txt\n  (rust-path-extension \"archive.tar.gz\")  ; Returns gz",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-path", "executing rust-path-extension command");

            if args.len() != 1 {
                return Err("rust-path-extension expects exactly one argument (path)".to_string());
            }

            let path_str = match &args[0] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-path-extension path must be a string".to_string()),
            };

            debug_log(ctx, "rust-path", &format!("extracting extension from: {}", path_str));
            let path = Path::new(&path_str);
            match path.extension() {
                Some(ext) => {
                    debug_log(ctx, "rust-path", &format!("extension extracted: {}", ext.to_string_lossy()));
                    Ok(Value::Str(ext.to_string_lossy().to_string()))
                },
                None => {
                    debug_log(ctx, "rust-path", "no extension found");
                    Ok(Value::Nil)
                },
            }
        },
    );

    // rust-path-exists command
    registry.register_closure_with_help_and_tag(
        "rust-path-exists",
        "Check if a path exists",
        "(rust-path-exists path)",
        "  (rust-path-exists \"/home/user\")  ; Returns true if path exists\n  (rust-path-exists \"nonexistent.txt\")  ; Returns false",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-path", "executing rust-path-exists command");

            if args.len() != 1 {
                return Err("rust-path-exists expects exactly one argument (path)".to_string());
            }

            let path_str = match &args[0] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-path-exists path must be a string".to_string()),
            };

            debug_log(ctx, "rust-path", &format!("checking if path exists: {}", path_str));
            let path = Path::new(&path_str);
            let exists = path.exists();
            debug_log(ctx, "rust-path", &format!("path exists: {}", exists));
            Ok(Value::Bool(exists))
        },
    );

    // rust-path-is-dir command
    registry.register_closure_with_help_and_tag(
        "rust-path-is-dir",
        "Check if a path is a directory",
        "(rust-path-is-dir path)",
        "  (rust-path-is-dir \"/home/user\")  ; Returns true if path is a directory\n  (rust-path-is-dir \"file.txt\")  ; Returns false",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-path", "executing rust-path-is-dir command");

            if args.len() != 1 {
                return Err("rust-path-is-dir expects exactly one argument (path)".to_string());
            }

            let path_str = match &args[0] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-path-is-dir path must be a string".to_string()),
            };

            debug_log(ctx, "rust-path", &format!("checking if path is directory: {}", path_str));
            let path = Path::new(&path_str);
            let is_dir = path.is_dir();
            debug_log(ctx, "rust-path", &format!("path is directory: {}", is_dir));
            Ok(Value::Bool(is_dir))
        },
    );

    // rust-path-is-file command
    registry.register_closure_with_help_and_tag(
        "rust-path-is-file",
        "Check if a path is a file",
        "(rust-path-is-file path)",
        "  (rust-path-is-file \"file.txt\")  ; Returns true if path is a file\n  (rust-path-is-file \"/home/user\")  ; Returns false",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-path", "executing rust-path-is-file command");

            if args.len() != 1 {
                return Err("rust-path-is-file expects exactly one argument (path)".to_string());
            }

            let path_str = match &args[0] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-path-is-file path must be a string".to_string()),
            };

            debug_log(ctx, "rust-path", &format!("checking if path is file: {}", path_str));
            let path = Path::new(&path_str);
            let is_file = path.is_file();
            debug_log(ctx, "rust-path", &format!("path is file: {}", is_file));
            Ok(Value::Bool(is_file))
        },
    );
}
