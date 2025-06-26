use crate::core::{Command, ExecutionContext, MSG_EXECUTING_OPERATION, MSG_ENV_VAR_ADDED};

/// Run command implementation
#[derive(Debug, Clone)]
pub struct RunCommand;

impl Command for RunCommand {
  fn execute(
    &self,
    context: &mut ExecutionContext,
  ) -> Result<(), Box<dyn std::error::Error>> {
    if context.verbose {
      println!("{}", MSG_EXECUTING_OPERATION.replace("{}", self.name()));
    }

    let mut env_vars = context
      .env_vars
      .as_ref()
      .ok_or("Environment variables not initialized")?
      .clone();
    let existing_env_vars = context
      .existing_env_vars
      .as_ref()
      .ok_or("Existing environment variables not initialized")?;

    // Missing environment variables present in .env are added before each run
    for (key, value) in existing_env_vars.clone() {
      if !env_vars.contains_key(&key) {
        if context.verbose {
          println!(
            "{}",
            MSG_ENV_VAR_ADDED.replace("{}", &key).replace("{}", &value)
          );
        }
        env_vars.insert(key, value);
      }
    }

    crate::docker::execute_docker_command(
      &env_vars,
      existing_env_vars,
      &context.args,
      context.verbose,
    )?;
    Ok(())
  }

  fn name(&self) -> &'static str {
    "run"
  }

  fn display(&self) -> String {
    "run".to_string()
  }

  fn command_name() -> &'static str {
    "run"
  }

  fn try_parse(
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Option<Result<Box<dyn Command>, String>> {
    if command != "run" {
      return None;
    }

    // Collect arguments until we find another known command
    let _run_args: Vec<String> = Vec::new();
    while let Some(next_arg) = args.peek() {
      // Check if next argument is a known command
      if matches!(
        next_arg.as_str(),
        "clean" | "config" | "write-env" | "update-versions" | "run"
      ) {
        break;
      }
      args.next();
    }

    Some(Ok(Box::new(RunCommand)))
  }
}

