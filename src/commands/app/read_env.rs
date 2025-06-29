use crate::commands::app::get_var::register_get_var_command;
use crate::commands::app::set_var::register_set_var_command;
use crate::commands::app::write_env::register_write_env_command;
use crate::commands::app::version_check::register_version_check_command;
use crate::commands::app::docker::register_docker_command;
use crate::utils::debug_log;
use crate::{CommandRegistry, Context, Value, tags};
use regex::Regex;
use std::fs;

/// Register app commands
pub fn register_app_commands(registry: &mut CommandRegistry) {
  // Register the set-var command
  register_set_var_command(registry);

  // Register the get-var command
  register_get_var_command(registry);

  // Register the write-env command
  register_write_env_command(registry);

  // Register the version-check command
  register_version_check_command(registry);

  // Register the docker command
  register_docker_command(registry);

  // Register the read-env command
  registry.register_closure_with_help_and_tag(
    "read-env",
    "Read environment variables from a file and store them in the context",
    "(read-env path)",
    "  (read-env \"config.env\")     ; Read from config.env relative to basedir\n  (read-env \"../shared.env\")  ; Read from parent directory",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "read-env", "executing read-env command");

      if args.len() != 1 {
        return Err("read-env expects exactly one argument (path)".to_string());
      }

      let path_arg = match &args[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("read-env path must be a string".to_string()),
      };

      debug_log(ctx, "read-env", &format!("processing path argument: {}", path_arg));

      // Resolve path relative to basedir
      let basedir = ctx.get_basedir();
      let file_path = basedir.join(&path_arg);

      debug_log(ctx, "read-env", &format!("resolved file path: {}", file_path.display()));

      // Check if file exists
      if !file_path.exists() {
        return Err(format!("File does not exist: {}", file_path.display()));
      }

      // Read file contents
      let contents = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read file {}: {}", file_path.display(), e)),
      };

      debug_log(ctx, "read-env", "file read successfully, processing lines");

      let mut variables_loaded = 0;
      let mut lines_processed = 0;

      // Process each line
      for (line_num, line) in contents.lines().enumerate() {
        lines_processed += 1;
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
          debug_log(ctx, "read-env", &format!("skipping line {}: empty or comment", line_num + 1));
          continue;
        }

        // Parse key=value format
        if let Some(eq_pos) = trimmed.find('=') {
          let key = trimmed[..eq_pos].trim().to_string();
          let value = trimmed[eq_pos + 1..].trim().to_string();

          if key.is_empty() {
            debug_log(ctx, "read-env", &format!("skipping line {}: empty key", line_num + 1));
            continue;
          }

          debug_log(ctx, "read-env", &format!("found variable: {} = {}", key, value));

          // Interpolate variables in the value
          let interpolated_value = match interpolate_variables(&value, ctx) {
            Ok(val) => val,
            Err(e) => return Err(format!("Error interpolating variable '{}': {}", key, e)),
          };

          debug_log(ctx, "read-env", &format!("interpolated value: {} = {}", key, interpolated_value));

          // Store in context
          ctx.set_variable(key, Value::Str(interpolated_value));
          variables_loaded += 1;
        } else {
          debug_log(ctx, "read-env", &format!("skipping line {}: no '=' found", line_num + 1));
        }
      }

      let result_msg = format!(
        "Loaded {} variables from {} (processed {} lines)",
        variables_loaded,
        file_path.display(),
        lines_processed
      );

      debug_log(ctx, "read-env", &format!("completed: {}", result_msg));
      Ok(Value::Str(result_msg))
    },
  );
}

/// Interpolate variables in a string value
/// Supports ${key} format with single-pass resolution
fn interpolate_variables(value: &str, ctx: &Context) -> Result<String, String> {
  let var_regex = Regex::new(r"\$\{([^}]+)\}").unwrap();
  let mut result = String::new();
  let mut last_end = 0;

  for cap in var_regex.captures_iter(value) {
    let full_match = cap.get(0).unwrap();
    let var_name = cap.get(1).unwrap().as_str();

    // Add text before the match
    result.push_str(&value[last_end..full_match.start()]);

    // Look up variable value
    let replacement = if let Some(ctx_value) = ctx.get_variable(var_name) {
      ctx_value.to_string()
    } else if let Ok(env_value) = std::env::var(var_name) {
      env_value
    } else {
      // Variable not found, leave as is
      full_match.as_str().to_string()
    };

    result.push_str(&replacement);
    last_end = full_match.end();
  }

  // Add remaining text
  result.push_str(&value[last_end..]);

  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lisp_interpreter::CommandRegistry;
  use std::path::PathBuf;

  #[test]
  fn test_interpolate_variables_simple() {
    let mut registry = CommandRegistry::new();
    let mut ctx = Context::new(registry);
    ctx.set_variable("NAME".to_string(), Value::Str("test".to_string()));

    let result = interpolate_variables("Hello ${NAME}!", &ctx).unwrap();
    assert_eq!(result, "Hello test!");
  }

  #[test]
  fn test_interpolate_variables_nested() {
    let mut registry = CommandRegistry::new();
    let mut ctx = Context::new(registry);
    ctx.set_variable("PREFIX".to_string(), Value::Str("app".to_string()));
    ctx.set_variable(
      "SUFFIX".to_string(),
      Value::Str("${PREFIX}_config".to_string()),
    );

    // With single-pass interpolation, nested variables are not resolved
    let result = interpolate_variables("File: ${SUFFIX}.json", &ctx).unwrap();
    assert_eq!(result, "File: ${PREFIX}_config.json");
  }

  #[test]
  fn test_interpolate_variables_single_pass() {
    let mut registry = CommandRegistry::new();
    let mut ctx = Context::new(registry);
    ctx.set_variable("A".to_string(), Value::Str("${B}".to_string()));
    ctx.set_variable("B".to_string(), Value::Str("value_b".to_string()));

    // With single-pass interpolation, A resolves to "${B}" (not "value_b")
    let result = interpolate_variables("${A}", &ctx).unwrap();
    assert_eq!(result, "${B}");
  }
}
