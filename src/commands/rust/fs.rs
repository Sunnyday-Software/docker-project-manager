use crate::utils::debug_log;
use crate::{CommandRegistry, Value, tags};
use std::fs;

/// Register filesystem commands
pub fn register_fs_commands(registry: &mut CommandRegistry) {
  // rust-fs-read-to-string command
  registry.register_closure_with_help_and_tag(
        "rust-fs-read-to-string",
        "Read the entire contents of a file into a string",
        "(rust-fs-read-to-string path)",
        "  (rust-fs-read-to-string \"config.txt\")  ; Read file contents as string\n  (rust-fs-read-to-string \"/etc/hosts\")  ; Read system file",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-fs", "executing rust-fs-read-to-string command");

            if args.len() != 1 {
                return Err("rust-fs-read-to-string expects exactly one argument (file path)".to_string());
            }

            let file_path = match &args[0] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-fs-read-to-string file path must be a string".to_string()),
            };

            debug_log(ctx, "rust-fs", &format!("reading file contents from: {}", file_path));
            match fs::read_to_string(&file_path) {
                Ok(contents) => {
                    debug_log(ctx, "rust-fs", &format!("successfully read {} bytes from file", contents.len()));
                    Ok(Value::Str(contents))
                },
                Err(e) => Err(format!("Failed to read file '{}': {}", file_path, e)),
            }
        },
    );

  // rust-fs-write command
  registry.register_closure_with_help_and_tag(
        "rust-fs-write",
        "Write a string to a file, creating the file if it doesn't exist",
        "(rust-fs-write path content)",
        "  (rust-fs-write \"output.txt\" \"Hello, World!\")  ; Write string to file\n  (rust-fs-write \"config.json\" \"{\\\"key\\\": \\\"value\\\"}\")  ; Write JSON content",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-fs", "executing rust-fs-write command");

            if args.len() != 2 {
                return Err("rust-fs-write expects exactly two arguments (file path and content)".to_string());
            }

            let file_path = match &args[0] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-fs-write file path must be a string".to_string()),
            };

            let content = match &args[1] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-fs-write content must be a string".to_string()),
            };

            debug_log(ctx, "rust-fs", &format!("writing {} bytes to file: {}", content.len(), file_path));
            match fs::write(&file_path, &content) {
                Ok(()) => {
                    debug_log(ctx, "rust-fs", &format!("successfully wrote to file: {}", file_path));
                    Ok(Value::Str(format!("Successfully wrote {} bytes to '{}'", content.len(), file_path)))
                },
                Err(e) => Err(format!("Failed to write to file '{}': {}", file_path, e)),
            }
        },
    );

  // rust-fs-create-dir command
  registry.register_closure_with_help_and_tag(
        "rust-fs-create-dir",
        "Create a new directory",
        "(rust-fs-create-dir path)",
        "  (rust-fs-create-dir \"new_folder\")  ; Create directory\n  (rust-fs-create-dir \"/tmp/test_dir\")  ; Create directory with absolute path",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-fs", "executing rust-fs-create-dir command");

            if args.len() != 1 {
                return Err("rust-fs-create-dir expects exactly one argument (directory path)".to_string());
            }

            let dir_path = match &args[0] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-fs-create-dir directory path must be a string".to_string()),
            };

            debug_log(ctx, "rust-fs", &format!("creating directory: {}", dir_path));
            match fs::create_dir(&dir_path) {
                Ok(()) => {
                    debug_log(ctx, "rust-fs", &format!("successfully created directory: {}", dir_path));
                    Ok(Value::Str(format!("Successfully created directory '{}'", dir_path)))
                },
                Err(e) => Err(format!("Failed to create directory '{}': {}", dir_path, e)),
            }
        },
    );

  // rust-fs-remove-file command
  registry.register_closure_with_help_and_tag(
        "rust-fs-remove-file",
        "Remove a file from the filesystem",
        "(rust-fs-remove-file path)",
        "  (rust-fs-remove-file \"temp.txt\")  ; Remove file\n  (rust-fs-remove-file \"/tmp/old_file.log\")  ; Remove file with absolute path",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-fs", "executing rust-fs-remove-file command");

            if args.len() != 1 {
                return Err("rust-fs-remove-file expects exactly one argument (file path)".to_string());
            }

            let file_path = match &args[0] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-fs-remove-file file path must be a string".to_string()),
            };

            debug_log(ctx, "rust-fs", &format!("removing file: {}", file_path));
            match fs::remove_file(&file_path) {
                Ok(()) => {
                    debug_log(ctx, "rust-fs", &format!("successfully removed file: {}", file_path));
                    Ok(Value::Str(format!("Successfully removed file '{}'", file_path)))
                },
                Err(e) => Err(format!("Failed to remove file '{}': {}", file_path, e)),
            }
        },
    );

  // rust-fs-copy command
  registry.register_closure_with_help_and_tag(
        "rust-fs-copy",
        "Copy a file from source to destination",
        "(rust-fs-copy source destination)",
        "  (rust-fs-copy \"source.txt\" \"backup.txt\")  ; Copy file\n  (rust-fs-copy \"/etc/config\" \"/tmp/config.bak\")  ; Copy with absolute paths",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-fs", "executing rust-fs-copy command");

            if args.len() != 2 {
                return Err("rust-fs-copy expects exactly two arguments (source and destination paths)".to_string());
            }

            let source_path = match &args[0] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-fs-copy source path must be a string".to_string()),
            };

            let dest_path = match &args[1] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-fs-copy destination path must be a string".to_string()),
            };

            debug_log(ctx, "rust-fs", &format!("copying file from '{}' to '{}'", source_path, dest_path));
            match fs::copy(&source_path, &dest_path) {
                Ok(bytes_copied) => {
                    debug_log(ctx, "rust-fs", &format!("successfully copied {} bytes", bytes_copied));
                    Ok(Value::Str(format!("Successfully copied {} bytes from '{}' to '{}'", bytes_copied, source_path, dest_path)))
                },
                Err(e) => Err(format!("Failed to copy from '{}' to '{}': {}", source_path, dest_path, e)),
            }
        },
    );
}
