use crate::{CommandRegistry, Context, Value, tags};
use std::env;

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
      if args.len() != 1 {
        return Err("basedir expects exactly one argument (path)".to_string());
      }

      let path_arg = match &args[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("basedir path must be a string".to_string()),
      };

      // Handle relative paths - make them relative to the current executable
      let base_path = if std::path::Path::new(&path_arg).is_absolute() {
        path_arg.clone()
      } else {
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
            // Fallback to current working directory
            match env::current_dir() {
              Ok(cwd) => cwd.join(&path_arg).to_string_lossy().to_string(),
              Err(_) => path_arg.clone(),
            }
          }
        }
      };

      // Verify the path exists
      if !std::path::Path::new(&base_path).exists() {
        return Err(format!("Path does not exist: {}", base_path));
      }

      // Store the base directory in the context
      ctx.set_variable("basedir".to_string(), Value::Str(base_path.clone()));

      let result_msg = format!("Base directory set to: {}", base_path);
      println!("{}", result_msg);
      Ok(Value::Str(result_msg))
    },
  );
}