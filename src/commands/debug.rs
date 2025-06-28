use crate::{Command, Value};
use crate::context::Context;
use crate::emoji::*;
/// Debug command - prints the current state of the program
pub struct DebugCommand;

impl Command for DebugCommand {
  fn execute(
    &self,
    args: Vec<Value>,
    ctx: &mut Context,
  ) -> Result<Value, String> {
    // Check if we have arguments to set debugPrint variable
    if !args.is_empty() {
      if args.len() != 1 {
        return Err("{EmojiCatalog::} debug command accepts either no arguments or exactly one argument (true/false)".to_string());
      }

      let arg = &args[0];
      match arg {
        Value::Str(s) => match s.to_lowercase().as_str() {
          "true" => {
            ctx.set_debug_print(true);
            println!("🐛 Debug printing enabled");
            return Ok(Value::Str("Debug printing enabled".to_string()));
          }
          "false" => {
            ctx.set_debug_print(false);
            println!("🐛 Debug printing disabled");
            return Ok(Value::Str("Debug printing disabled".to_string()));
          }
          _ => {
            return Err(
              "debug command argument must be 'true' or 'false'".to_string(),
            );
          }
        },
        _ => {
          return Err(
            "debug command argument must be a string ('true' or 'false')"
              .to_string(),
          );
        }
      }
    }

    // Original behavior: print session variables - delegated to context
    let output = ctx.print_debug_info();

    print!("{}", output);
    Ok(Value::Str(output))
  }

  fn name(&self) -> &'static str {
    "debug"
  }

  fn description(&self) -> &'static str {
    "Print current program state or set debug printing true/false"
  }

  fn syntax(&self) -> &'static str {
    "(debug) or (debug \"true\"|\"false\")"
  }

  fn examples(&self) -> &'static str {
    "  (debug)          ; Print session variables\n  (debug \"true\")    ; Enable debug printing\n  (debug \"false\")   ; Disable debug printing"
  }
}
