use crate::{Command, Value, Context};

/// Print command - prints its arguments
pub struct PrintCommand;

impl Command for PrintCommand {
    fn execute(&self, args: Vec<Value>, _ctx: &mut Context) -> Result<Value, String> {
        let output = args.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", output);
        Ok(Value::Str(output))
    }

    fn name(&self) -> &'static str {
        "print"
    }

    fn description(&self) -> &'static str {
        "Print arguments to stdout"
    }

    fn syntax(&self) -> &'static str {
        "(print arg1 arg2 ...)"
    }

    fn examples(&self) -> &'static str {
        "  (print \"Hello World\")\n  (print \"Sum is:\" (sum 1 2 3))"
    }
}
