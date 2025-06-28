use crate::{Command, Value, evaluate};
use crate::context::Context;

/// Pipe command - executes a pipeline of commands
pub struct PipeCommand;

impl Command for PipeCommand {
    fn execute(&self, args: Vec<Value>, ctx: &mut Context) -> Result<Value, String> {
        if args.is_empty() {
            return Ok(Value::Nil);
        }

        let mut result = args[0].clone();

        for i in 1..args.len() {
            // Each subsequent argument should be a command to execute with the previous result
            match &args[i] {
                Value::List(cmd_list) => {
                    if cmd_list.is_empty() {
                        continue;
                    }

                    // Convert back to lexpr format for evaluation
                    let lexpr_value = args[i].to_lexpr();
                    result = evaluate(&lexpr_value, ctx)?;
                }
                _ => {
                    return Err("Pipe arguments must be command lists".to_string());
                }
            }
        }

        Ok(result)
    }

    fn name(&self) -> &'static str {
        "pipe"
    }

    fn description(&self) -> &'static str {
        "Execute a pipeline of commands, passing results between them"
    }

    fn syntax(&self) -> &'static str {
        "(pipe command1 command2 ...)"
    }

    fn examples(&self) -> &'static str {
        "  (pipe (sum 1 2 3) (print \"Result:\"))"
    }
}
