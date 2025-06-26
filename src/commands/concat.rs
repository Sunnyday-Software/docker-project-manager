use crate::{Command, Value, Context};

/// Concat command - concatenates strings
pub struct ConcatCommand;

impl Command for ConcatCommand {
    fn execute(&self, args: Vec<Value>, _ctx: &mut Context) -> Result<Value, String> {
        let result = args
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("");
        Ok(Value::Str(result))
    }

    fn name(&self) -> &'static str {
        "concat"
    }

    fn description(&self) -> &'static str {
        "Concatenate strings"
    }

    fn syntax(&self) -> &'static str {
        "(concat string1 string2 ...)"
    }

    fn examples(&self) -> &'static str {
        "  (concat \"Hello\" \" \" \"World\") ; Returns \"Hello World\"\n  (concat \"A\" \"B\" \"C\")         ; Returns \"ABC\""
    }
}
