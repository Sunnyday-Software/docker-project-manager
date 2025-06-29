use crate::utils::debug_log;
use crate::{CommandRegistry, Value, tags};
use std::env;

/// Register environment commands
pub fn register_env_commands(registry: &mut CommandRegistry) {
  // rust-env-current-dir command
  registry.register_closure_with_help_and_tag(
    "rust-env-current-dir",
    "Get the current working directory",
    "(rust-env-current-dir)",
    "  (rust-env-current-dir)  ; Returns current working directory path",
    &tags::RUST,
    |args, ctx| {
      debug_log(ctx, "rust-env", "executing rust-env-current-dir command");

      if !args.is_empty() {
        return Err("rust-env-current-dir expects no arguments".to_string());
      }

      debug_log(ctx, "rust-env", "getting current working directory");
      match env::current_dir() {
        Ok(path) => {
          debug_log(
            ctx,
            "rust-env",
            &format!("current directory retrieved: {}", path.display()),
          );
          Ok(Value::Str(path.to_string_lossy().to_string()))
        }
        Err(e) => Err(format!("Failed to get current directory: {}", e)),
      }
    },
  );

  // rust-env-current-exe command
  registry.register_closure_with_help_and_tag(
    "rust-env-current-exe",
    "Get the path of the current executable",
    "(rust-env-current-exe)",
    "  (rust-env-current-exe)  ; Returns path to current executable",
    &tags::RUST,
    |args, ctx| {
      debug_log(ctx, "rust-env", "executing rust-env-current-exe command");

      if !args.is_empty() {
        return Err("rust-env-current-exe expects no arguments".to_string());
      }

      debug_log(ctx, "rust-env", "getting current executable path");
      match env::current_exe() {
        Ok(path) => {
          debug_log(
            ctx,
            "rust-env",
            &format!("executable path retrieved: {}", path.display()),
          );
          Ok(Value::Str(path.to_string_lossy().to_string()))
        }
        Err(e) => Err(format!("Failed to get current executable path: {}", e)),
      }
    },
  );

  // rust-env-home-dir command
  registry.register_closure_with_help_and_tag(
    "rust-env-home-dir",
    "Get the user's home directory",
    "(rust-env-home-dir)",
    "  (rust-env-home-dir)  ; Returns user's home directory path",
    &tags::RUST,
    |args, ctx| {
      debug_log(ctx, "rust-env", "executing rust-env-home-dir command");

      if !args.is_empty() {
        return Err("rust-env-home-dir expects no arguments".to_string());
      }

      debug_log(ctx, "rust-env", "getting user's home directory");
      match dirs::home_dir() {
        Some(path) => {
          debug_log(
            ctx,
            "rust-env",
            &format!("home directory retrieved: {}", path.display()),
          );
          Ok(Value::Str(path.to_string_lossy().to_string()))
        }
        None => Err("Failed to get home directory".to_string()),
      }
    },
  );

  // rust-env-var command
  registry.register_closure_with_help_and_tag(
        "rust-env-var",
        "Get the value of an environment variable",
        "(rust-env-var name)",
        "  (rust-env-var \"PATH\")  ; Get PATH environment variable\n  (rust-env-var \"HOME\")  ; Get HOME environment variable",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-env", "executing rust-env-var command");

            if args.len() != 1 {
                return Err("rust-env-var expects exactly one argument (variable name)".to_string());
            }

            let var_name = match &args[0] {
                Value::Str(s) => s.clone(),
                _ => return Err("rust-env-var variable name must be a string".to_string()),
            };

            debug_log(ctx, "rust-env", &format!("getting environment variable: {}", var_name));
            match env::var(&var_name) {
                Ok(value) => {
                    debug_log(ctx, "rust-env", &format!("environment variable '{}' retrieved successfully", var_name));
                    Ok(Value::Str(value))
                },
                Err(env::VarError::NotPresent) => {
                    debug_log(ctx, "rust-env", &format!("environment variable '{}' not found", var_name));
                    Ok(Value::Nil)
                },
                Err(env::VarError::NotUnicode(_)) => Err(format!("Environment variable '{}' contains invalid Unicode", var_name)),
            }
        },
    );

  // rust-env-vars command
  registry.register_closure_with_help_and_tag(
    "rust-env-vars",
    "Get all environment variables as a list of (name value) pairs",
    "(rust-env-vars)",
    "  (rust-env-vars)  ; Returns list of all environment variables",
    &tags::RUST,
    |args, ctx| {
      debug_log(ctx, "rust-env", "executing rust-env-vars command");

      if !args.is_empty() {
        return Err("rust-env-vars expects no arguments".to_string());
      }

      debug_log(ctx, "rust-env", "collecting all environment variables");
      let mut vars = Vec::new();
      let mut count = 0;
      for (key, value) in env::vars() {
        let pair = vec![Value::Str(key), Value::Str(value)];
        vars.push(Value::List(pair));
        count += 1;
      }

      debug_log(
        ctx,
        "rust-env",
        &format!("collected {} environment variables", count),
      );
      Ok(Value::List(vars))
    },
  );
}
