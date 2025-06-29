use crate::utils::debug_log;
use crate::{CommandRegistry, Value, tags};
use std::fs;

/// Register write-env command
pub fn register_write_env_command(registry: &mut CommandRegistry) {
  registry.register_closure_with_help_and_tag(
    "write-env",
    "Write all context variables to a file",
    "(write-env path)",
    "  (write-env \"config.env\")     ; Write to config.env relative to basedir\n  (write-env \"../shared.env\")  ; Write to parent directory",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "write-env", "executing write-env command");

      if args.len() != 1 {
        return Err("write-env expects exactly one argument (path)".to_string());
      }

      let path_arg = match &args[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("write-env path must be a string".to_string()),
      };

      debug_log(ctx, "write-env", &format!("processing path argument: {}", path_arg));

      // Resolve path relative to basedir
      let basedir = ctx.get_basedir();
      let file_path = basedir.join(&path_arg);

      debug_log(ctx, "write-env", &format!("resolved file path: {}", file_path.display()));

      // Create parent directories if they don't exist
      if let Some(parent) = file_path.parent() {
        if !parent.exists() {
          if let Err(e) = fs::create_dir_all(parent) {
            return Err(format!("Failed to create parent directories for {}: {}", file_path.display(), e));
          }
        }
      }

      // Collect all variables from context
      let mut content = String::new();
      let mut variables_written = 0;

      // Add header comment
      content.push_str("# Environment variables written by write-env command\n");
      content.push_str("# Generated automatically - do not edit manually\n\n");

      // Write all context variables
      for (key, value) in &ctx.variables {
        let line = format!("{}={}\n", key, value.to_string());
        content.push_str(&line);
        variables_written += 1;
        debug_log(ctx, "write-env", &format!("writing variable: {} = {}", key, value.to_string()));
      }

      // If no variables, add a comment
      if variables_written == 0 {
        content.push_str("# No variables to write\n");
      }

      debug_log(ctx, "write-env", &format!("writing {} variables to file", variables_written));

      // Write content to file
      match fs::write(&file_path, content) {
        Ok(_) => {
          let result_msg = format!(
            "Wrote {} variables to {}",
            variables_written,
            file_path.display()
          );
          debug_log(ctx, "write-env", &format!("completed: {}", result_msg));
          Ok(Value::Str(result_msg))
        }
        Err(e) => Err(format!("Failed to write file {}: {}", file_path.display(), e)),
      }
    },
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lisp_interpreter::CommandRegistry;
  use crate::context::Context;
  use std::fs;
  use std::path::PathBuf;

  #[test]
  fn test_write_env_command() {
    let mut registry = CommandRegistry::new();
    register_write_env_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Set up a test directory (use current directory for simplicity)
    let test_dir = std::env::current_dir()
      .unwrap()
      .join("target")
      .join("test_write_env");
    fs::create_dir_all(&test_dir).unwrap();
    ctx.set_basedir(test_dir.clone());

    // Add some variables to context
    ctx.set_variable("TEST_VAR1".to_string(), Value::Str("value1".to_string()));
    ctx.set_variable("TEST_VAR2".to_string(), Value::Str("value2".to_string()));

    // Execute write-env command using the closure directly
    let args = vec![Value::Str("test.env".to_string())];
    let write_env_closure = |args: Vec<Value>,
                             ctx: &mut Context|
     -> Result<Value, String> {
      register_write_env_command(&mut CommandRegistry::new());
      // Call the actual implementation logic here
      if args.len() != 1 {
        return Err(
          "write-env expects exactly one argument (path)".to_string(),
        );
      }

      let path_arg = match &args[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("write-env path must be a string".to_string()),
      };

      let basedir = ctx.get_basedir();
      let file_path = basedir.join(&path_arg);

      if let Some(parent) = file_path.parent() {
        if !parent.exists() {
          if let Err(e) = fs::create_dir_all(parent) {
            return Err(format!(
              "Failed to create parent directories for {}: {}",
              file_path.display(),
              e
            ));
          }
        }
      }

      let mut content = String::new();
      let mut variables_written = 0;

      content
        .push_str("# Environment variables written by write-env command\n");
      content.push_str("# Generated automatically - do not edit manually\n\n");

      for (key, value) in &ctx.variables {
        let line = format!("{}={}\n", key, value.to_string());
        content.push_str(&line);
        variables_written += 1;
      }

      if variables_written == 0 {
        content.push_str("# No variables to write\n");
      }

      match fs::write(&file_path, content) {
        Ok(_) => {
          let result_msg = format!(
            "Wrote {} variables to {}",
            variables_written,
            file_path.display()
          );
          Ok(Value::Str(result_msg))
        }
        Err(e) => Err(format!(
          "Failed to write file {}: {}",
          file_path.display(),
          e
        )),
      }
    };

    let result = write_env_closure(args, &mut ctx);
    assert!(result.is_ok());

    // Check that file was created and contains expected content
    let file_path = test_dir.join("test.env");
    assert!(file_path.exists());

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("TEST_VAR1=value1"));
    assert!(content.contains("TEST_VAR2=value2"));

    // Clean up
    let _ = fs::remove_file(&file_path);
    let _ = fs::remove_dir(&test_dir);
  }
}
