use crate::utils::debug_log;
use crate::{CommandRegistry, Value, tags};
use std::env;
use std::path::PathBuf;

/// Register basedir commands
pub fn register_basedir_commands(registry: &mut CommandRegistry) {
  // Command management commands
  registry.register_closure_with_help_and_tag(
    "basedir",
    "Set the base directory for subsequent operations",
    "(basedir path)",
    "  (basedir \"/home/user/project\")  ; Set absolute path\n  (basedir \"../project\")         ; Set relative path",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "basedir", "executing basedir command");

      if args.len() != 1 {
        return Err("basedir expects exactly one argument (path)".to_string());
      }

      let path_arg = match &args[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("basedir path must be a string".to_string()),
      };

      debug_log(ctx, "basedir", &format!("processing path argument: {}", path_arg));

      // Handle relative paths - make them relative to the current executable
      let base_path = if std::path::Path::new(&path_arg).is_absolute() {
        debug_log(ctx, "basedir", "path is absolute, using as-is");
        path_arg.clone()
      } else {
        debug_log(ctx, "basedir", "path is relative, resolving against executable directory");
        // Get the directory of the current executable
        match env::current_exe() {
          Ok(exe_path) => {
            if let Some(exe_dir) = exe_path.parent() {
              exe_dir.join(&path_arg).to_string_lossy().to_string()
            } else {
              path_arg.clone()
            }
          }
          Err(_) => {
            debug_log(ctx, "basedir", "fallback to current working directory");
            // Fallback to current working directory
            match env::current_dir() {
              Ok(cwd) => cwd.join(&path_arg).to_string_lossy().to_string(),
              Err(_) => path_arg.clone(),
            }
          }
        }
      };

      debug_log(ctx, "basedir", &format!("resolved base path: {}", base_path));

      // Verify the path exists
      if !std::path::Path::new(&base_path).exists() {
        return Err(format!("Path does not exist: {}", base_path));
      }

      debug_log(ctx, "basedir", "path exists, setting as base directory");
      // Store the base directory in the context
      ctx.set_basedir(PathBuf::from(&base_path));

      let result_msg = format!("Base directory set to: {}", base_path);
      debug_log(ctx, "basedir", "base directory successfully set");

      Ok(Value::Str(result_msg))
    },
  );

  // Register the get-basedir command
  registry.register_closure_with_help_and_tag(
    "get-basedir",
    "Get the current base directory from the context",
    "(get-basedir)",
    "  (get-basedir)                 ; Get the current base directory path",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "get-basedir", "executing get-basedir command");

      if !args.is_empty() {
        return Err("get-basedir expects no arguments".to_string());
      }

      let basedir = ctx.get_basedir();
      let basedir_str = basedir.to_string_lossy().to_string();

      debug_log(
        ctx,
        "get-basedir",
        &format!("returning basedir: {}", basedir_str),
      );

      Ok(Value::Str(basedir_str))
    },
  );

  // basedir-root command
  registry.register_closure_with_help_and_tag(
    "basedir-root",
    "Find and set base directory by searching up the filesystem for a target file/folder",
    "(basedir-root [target])",
    "  (basedir-root)           ; Search for .git folder (default)\n  (basedir-root \".git\")    ; Search for .git folder\n  (basedir-root \"package.json\") ; Search for package.json file\n  (basedir-root \"src\")      ; Search for src folder",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "basedir", "executing basedir-root command");

      // Default target is ".git"
      let target = if args.is_empty() {
        debug_log(ctx, "basedir", "using default target: .git");
        ".git".to_string()
      } else if args.len() == 1 {
        match &args[0] {
          Value::Str(s) => {
            debug_log(ctx, "basedir", &format!("using specified target: {}", s));
            s.clone()
          }
          _ => return Err("basedir-root target must be a string".to_string()),
        }
      } else {
        return Err("basedir-root expects at most one argument (target)".to_string());
      };

      // Start from current working directory
      let mut current_dir = match env::current_dir() {
        Ok(dir) => {
          debug_log(ctx, "basedir", &format!("starting search from: {}", dir.display()));
          dir
        }
        Err(e) => return Err(format!("Failed to get current directory: {}", e)),
      };

      // Search up the filesystem
      loop {
        let target_path = current_dir.join(&target);
        debug_log(ctx, "basedir", &format!("checking for target at: {}", target_path.display()));

        if target_path.exists() {
          debug_log(ctx, "basedir", &format!("target found at: {}", target_path.display()));
          // Found the target, update basedir
          ctx.set_basedir(current_dir.clone());

          let result_msg = format!(
            "Found '{}' at: {}\nBase directory set to: {}",
            target,
            target_path.display(),
            current_dir.display()
          );

          debug_log(ctx, "basedir", "base directory successfully set from root search");
          return Ok(Value::Str(result_msg));
        }

        // Move up one directory
        match current_dir.parent() {
          Some(parent) => {
            current_dir = parent.to_path_buf();
            debug_log(ctx, "basedir", &format!("moving up to parent directory: {}", current_dir.display()));
          }
          None => {
            debug_log(ctx, "basedir", "reached filesystem root, target not found");
            return Err(format!(
              "Target '{}' not found in any parent directory from current working directory",
              target
            ));
          }
        }
      }
    },
  );
}
