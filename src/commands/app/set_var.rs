use crate::utils::debug_log;
use crate::{CommandRegistry, Value, tags};

/// Register set-var command
pub fn register_set_var_command(registry: &mut CommandRegistry) {
  registry.register_closure_with_help_and_tag(
    "set-var",
    "Set a variable in the context with the given key and value",
    "(set-var key value)",
    "  (set-var \"name\" \"John\")        ; Set variable 'name' to 'John'\n  (set-var \"count\" \"42\")         ; Set variable 'count' to '42'\n  (set-var \"path\" \"/home/user\")   ; Set variable 'path' to '/home/user'",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "set-var", "executing set-var command");

      if args.len() != 2 {
        return Err("set-var expects exactly two arguments (key, value)".to_string());
      }

      let key = match &args[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("set-var key must be a string".to_string()),
      };

      let value = match &args[1] {
        Value::Str(s) => s.clone(),
        _ => return Err("set-var value must be a string".to_string()),
      };

      debug_log(ctx, "set-var", &format!("setting variable: {} = {}", key, value));

      // Store the variable in the context
      ctx.set_variable(key.clone(), Value::Str(value.clone()));

      let result_msg = format!("Variable '{}' set to '{}'", key, value);
      debug_log(ctx, "set-var", &format!("completed: {}", result_msg));

      Ok(Value::Str(result_msg))
    },
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lisp_interpreter::CommandRegistry;
  use crate::context::Context;

  #[test]
  fn test_set_var_command() {
    let mut registry = CommandRegistry::new();
    register_set_var_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test setting a variable
    let args = vec![
      Value::Str("test_key".to_string()),
      Value::Str("test_value".to_string()),
    ];
    let result = ctx
      .registry
      .get("set-var")
      .unwrap()
      .execute(args, &mut ctx)
      .unwrap();

    // Check that the command returns success message
    assert_eq!(
      result,
      Value::Str("Variable 'test_key' set to 'test_value'".to_string())
    );

    // Check that the variable was actually set in the context
    assert_eq!(
      ctx.get_variable("test_key"),
      Some(&Value::Str("test_value".to_string()))
    );
  }

  #[test]
  fn test_set_var_wrong_arg_count() {
    let mut registry = CommandRegistry::new();
    register_set_var_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test with wrong number of arguments
    let args = vec![Value::Str("only_one_arg".to_string())];
    let result = ctx.registry.get("set-var").unwrap().execute(args, &mut ctx);

    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err(),
      "set-var expects exactly two arguments (key, value)"
    );
  }

  #[test]
  fn test_set_var_non_string_args() {
    let mut registry = CommandRegistry::new();
    register_set_var_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test with non-string key
    let args = vec![Value::Int(123), Value::Str("value".to_string())];
    let result = ctx.registry.get("set-var").unwrap().execute(args, &mut ctx);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "set-var key must be a string");

    // Test with non-string value
    let args = vec![Value::Str("key".to_string()), Value::Int(456)];
    let result = ctx.registry.get("set-var").unwrap().execute(args, &mut ctx);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "set-var value must be a string");
  }
}
