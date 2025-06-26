use crate::{Command, Value, Context, value_to_int};

/// Multiply command - multiplies two numbers
pub struct MultiplyCommand;

impl Command for MultiplyCommand {
    fn execute(&self, args: Vec<Value>, _ctx: &mut Context) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("multiply expects exactly 2 arguments".to_string());
        }

        let a = value_to_int(&args[0])?;
        let b = value_to_int(&args[1])?;
        Ok(Value::Int(a * b))
    }

    fn name(&self) -> &'static str {
        "multiply"
    }

    fn description(&self) -> &'static str {
        "Multiply two numbers"
    }

    fn syntax(&self) -> &'static str {
        "(multiply number1 number2)"
    }

    fn examples(&self) -> &'static str {
        "  (multiply 6 7)      ; Returns 42\n  (multiply 3 4)      ; Returns 12"
    }
}
