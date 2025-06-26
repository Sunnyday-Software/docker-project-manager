use crate::core::{Command, ExecutionContext, MSG_EXECUTING_OPERATION, MSG_CONFIG_PARSING};

/// Configuration command implementation
#[derive(Debug, Clone)]
pub struct ConfigCommand {
  pub key: String,
  pub value: String,
}

impl ConfigCommand {
  pub fn new(key: String, value: String) -> Self {
    Self { key, value }
  }
}

impl Command for ConfigCommand {
  fn execute(
    &self,
    context: &mut ExecutionContext,
  ) -> Result<(), Box<dyn std::error::Error>> {
    if context.verbose {
      println!("{}", MSG_EXECUTING_OPERATION.replace("{}", self.name()));
      println!("{}", MSG_CONFIG_PARSING);
    }

    if let Err(e) = context.config.set(&self.key, &self.value) {
      eprintln!("Configuration error: {}", e);
      return Err(e.into());
    }

    if context.verbose {
      println!("Configuration set: {} = {}", self.key, self.value);
    }

    Ok(())
  }

  fn name(&self) -> &'static str {
    "cfg"
  }

  fn display(&self) -> String {
    format!("cfg({}={})", self.key, self.value)
  }

  fn is_config_command(&self) -> bool {
    true
  }

  fn command_name() -> &'static str {
    "config"
  }

  fn try_parse(
    command: &str,
    args: &mut std::iter::Peekable<std::vec::IntoIter<String>>,
  ) -> Option<Result<Box<dyn Command>, String>> {
    if command != "config" {
      return None;
    }

    // Expect key=value format
    if let Some(config_arg) = args.next() {
      if let Some((key, value)) = config_arg.split_once('=') {
        Some(Ok(Box::new(ConfigCommand::new(
          key.to_string(),
          value.to_string(),
        ))))
      } else {
        Some(Err(format!(
          "Invalid config format: '{}'. Expected key=value",
          config_arg
        )))
      }
    } else {
      Some(Err("Config step requires key=value argument".to_string()))
    }
  }
}

