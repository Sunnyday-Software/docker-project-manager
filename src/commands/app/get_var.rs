use crate::utils::debug_log;
use crate::{CommandRegistry, Value, tags};

/// Register get-var command
pub fn register_get_var_command(registry: &mut CommandRegistry) {
  registry.register_closure_with_help_and_tag(
    "get-var",
    "Get a variable from the context with the given key",
    "(get-var key)",
    "  (get-var \"name\")        ; Get variable 'name'\n  (get-var \"count\")       ; Get variable 'count'\n  (get-var \"path\")        ; Get variable 'path'",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "get-var", "executing get-var command");

      if args.len() != 1 {
        return Err("get-var expects exactly one argument (key)".to_string());
      }

      let key = match &args[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("get-var key must be a string".to_string()),
      };

      debug_log(ctx, "get-var", &format!("getting variable: {}", key));

      // Get the variable from the context
      match ctx.get_variable(&key) {
        Some(value) => {
          debug_log(ctx, "get-var", &format!("found variable: {} = {}", key, value));
          Ok(value.clone())
        },
        None => {
          let error_msg = format!("Variable '{}' not found", key);
          debug_log(ctx, "get-var", &error_msg);
          Err(error_msg)
        }
      }
    },
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lisp_interpreter::CommandRegistry;
  use crate::context::Context;

  #[test]
  fn test_get_var_command() {
    let mut registry = CommandRegistry::new();
    register_get_var_command(&mut registry);
    let mut ctx = Context::new(registry);

    // First set a variable
    ctx.set_variable(
      "test_key".to_string(),
      Value::Str("test_value".to_string()),
    );

    // Test getting the variable
    let args = vec![Value::Str("test_key".to_string())];
    let result = ctx
      .registry
      .get("get-var")
      .unwrap()
      .execute(args, &mut ctx)
      .unwrap();

    // Check that the command returns the variable value
    assert_eq!(result, Value::Str("test_value".to_string()));
  }

  #[test]
  fn test_get_var_not_found() {
    let mut registry = CommandRegistry::new();
    register_get_var_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test getting a non-existent variable
    let args = vec![Value::Str("nonexistent_key".to_string())];
    let result = ctx.registry.get("get-var").unwrap().execute(args, &mut ctx);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Variable 'nonexistent_key' not found");
  }

  #[test]
  fn test_get_var_wrong_arg_count() {
    let mut registry = CommandRegistry::new();
    register_get_var_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test with wrong number of arguments
    let args = vec![
      Value::Str("key1".to_string()),
      Value::Str("key2".to_string()),
    ];
    let result = ctx.registry.get("get-var").unwrap().execute(args, &mut ctx);

    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err(),
      "get-var expects exactly one argument (key)"
    );
  }

  #[test]
  fn test_get_var_non_string_key() {
    let mut registry = CommandRegistry::new();
    register_get_var_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test with non-string key
    let args = vec![Value::Int(123)];
    let result = ctx.registry.get("get-var").unwrap().execute(args, &mut ctx);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "get-var key must be a string");
  }

  #[test]
  fn test_get_var_different_value_types() {
    let mut registry = CommandRegistry::new();
    register_get_var_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test with integer value
    ctx.set_variable("int_key".to_string(), Value::Int(42));
    let args = vec![Value::Str("int_key".to_string())];
    let result = ctx
      .registry
      .get("get-var")
      .unwrap()
      .execute(args, &mut ctx)
      .unwrap();
    assert_eq!(result, Value::Int(42));

    // Test with list value
    ctx.set_variable(
      "list_key".to_string(),
      Value::List(vec![
        Value::Str("a".to_string()),
        Value::Str("b".to_string()),
      ]),
    );
    let args = vec![Value::Str("list_key".to_string())];
    let result = ctx
      .registry
      .get("get-var")
      .unwrap()
      .execute(args, &mut ctx)
      .unwrap();
    assert_eq!(
      result,
      Value::List(vec![
        Value::Str("a".to_string()),
        Value::Str("b".to_string())
      ])
    );
  }
}
