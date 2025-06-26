use crate::core::{Command, ExecutionContext, MSG_EXECUTING_OPERATION};

/// Update versions command implementation
#[derive(Debug, Clone)]
pub struct UpdateVersionsCommand;

impl Command for UpdateVersionsCommand {
  fn execute(
    &self,
    context: &mut ExecutionContext,
  ) -> Result<(), Box<dyn std::error::Error>> {
    if context.verbose {
      println!("{}", MSG_EXECUTING_OPERATION.replace("{}", self.name()));
    }

    let md5_values = context
      .md5_values
      .as_ref()
      .ok_or("MD5 values not calculated")?;

    crate::utils::update_versions(
      md5_values,
      context.config.versions_folder(),
      context.verbose,
    )?;
    Ok(())
  }

  fn name(&self) -> &'static str {
    "update_versions"
  }

  fn display(&self) -> String {
    "update_versions".to_string()
  }

  fn command_name() -> &'static str {
    "update-versions"
  }

  fn try_parse(
    command: &str,
    _args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Option<Result<Box<dyn Command>, String>> {
    if command != "update-versions" {
      return None;
    }

    Some(Ok(Box::new(UpdateVersionsCommand)))
  }
}

