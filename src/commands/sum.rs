use crate::{Command, Value, Context};

/// Sum command - sums a list of integers
pub struct SumCommand;

impl Command for SumCommand {
    fn execute(&self, args: Vec<Value>, _ctx: &mut Context) -> Result<Value, String> {
        let mut total = 0i64;

        for arg in args {
            match arg {
                Value::Int(i) => total += i,
                Value::List(list) => {
                    for item in list {
                        if let Value::Int(i) = item {
                            total += i;
                        } else {
                            return Err(format!("Cannot sum non-integer value: {}", item));
                        }
                    }
                }
                _ => return Err(format!("Cannot sum non-integer value: {}", arg)),
            }
        }

        Ok(Value::Int(total))
    }

    fn name(&self) -> &'static str {
        "sum"
    }

    fn description(&self) -> &'static str {
        "Sum a list of integers"
    }

    fn syntax(&self) -> &'static str {
        "(sum number1 number2 ...)"
    }

    fn examples(&self) -> &'static str {
        "  (sum 1 2 3)        ; Returns 6\n  (sum 10 20)        ; Returns 30"
    }
}
