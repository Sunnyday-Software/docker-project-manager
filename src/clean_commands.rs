use crate::core::{Command, ExecutionContext, MSG_EXECUTING_OPERATION};

/// Clean command implementation
#[derive(Debug, Clone)]
pub struct CleanCommand {
  pub force: bool,
}

impl CleanCommand {
  pub fn new(force: bool) -> Self {
    Self { force }
  }
}

impl Command for CleanCommand {
  fn execute(
    &self,
    context: &mut ExecutionContext,
  ) -> Result<(), Box<dyn std::error::Error>> {
    if context.verbose {
      println!("{}", MSG_EXECUTING_OPERATION.replace("{}", self.name()));
    }

    // TODO: Implement clean functionality
    println!("Clean command executed with force: {}", self.force);
    Ok(())
  }

  fn name(&self) -> &'static str {
    "clean"
  }

  fn display(&self) -> String {
    if self.force {
      "clean --force".to_string()
    } else {
      "clean".to_string()
    }
  }

  fn command_name() -> &'static str {
    "clean"
  }

  fn try_parse(
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Option<Result<Box<dyn Command>, String>> {
    if command != "clean" {
      return None;
    }

    let mut force = false;
    // Check if next argument is force
    if let Some(next_arg) = args.peek() {
      if next_arg == "force" {
        force = true;
        args.next(); // consume force
      }
    }

    Some(Ok(Box::new(CleanCommand::new(force))))
  }
}

