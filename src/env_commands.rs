use crate::core::{Command, ExecutionContext, MSG_EXECUTING_OPERATION};

/// Write environment command implementation
#[derive(Debug, Clone)]
pub struct WriteEnvCommand;

impl Command for WriteEnvCommand {
  fn execute(
    &self,
    context: &mut ExecutionContext,
  ) -> Result<(), Box<dyn std::error::Error>> {
    if context.verbose {
      println!("{}", MSG_EXECUTING_OPERATION.replace("{}", self.name()));
    }

    let existing_env_vars = context
      .existing_env_vars
      .as_ref()
      .ok_or("Environment variables not initialized")?;
    let output_env = context
      .output_env
      .as_ref()
      .ok_or("Output environment file not specified")?;

    crate::file_ops::write_env_file(
      output_env,
      existing_env_vars,
    )?;
    Ok(())
  }

  fn name(&self) -> &'static str {
    "write_env"
  }

  fn display(&self) -> String {
    "write_env".to_string()
  }

  fn command_name() -> &'static str {
    "write-env"
  }

  fn try_parse(
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Option<Result<Box<dyn Command>, String>> {
    if command != "write-env" {
      return None;
    }

    // Expect output <file>
    if let Some(next_arg) = args.next() {
      if next_arg == "output" {
        if let Some(_output_file) = args.next() {
          Some(Ok(Box::new(WriteEnvCommand)))
        } else {
          Some(Err("write-env output requires a filename".to_string()))
        }
      } else {
        Some(Err("write-env step requires output <file>".to_string()))
      }
    } else {
      Some(Err("write-env step requires output <file>".to_string()))
    }
  }
}
