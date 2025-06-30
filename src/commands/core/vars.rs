use crate::commands::core::read_env::interpolate_variables;
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
      debug_log(ctx, "set-var", &format!("received {} arguments", args.len()));

      // Validate argument count
      if args.len() != 2 {
        let error_msg = "set-var expects exactly two arguments (key, value)".to_string();
        debug_log(ctx, "set-var", &format!("argument validation failed: {}", error_msg));
        return Err(error_msg);
      }
      debug_log(ctx, "set-var", "argument count validation passed");

      // Validate and extract key
      debug_log(ctx, "set-var", "validating key argument");
      let key = match &args[0] {
        Value::Str(s) => {
          debug_log(ctx, "set-var", &format!("key validation passed: '{}'", s));
          s.clone()
        },
        other => {
          let error_msg = "set-var key must be a string".to_string();
          debug_log(ctx, "set-var", &format!("key validation failed: expected string, got {:?}", other));
          return Err(error_msg);
        },
      };

      // Validate and extract value
      debug_log(ctx, "set-var", "validating value argument");
      let value = match &args[1] {
        Value::Str(s) => {
          debug_log(ctx, "set-var", &format!("value validation passed: '{}'", s));
          s.clone()
        },
        other => {
          let error_msg = "set-var value must be a string".to_string();
          debug_log(ctx, "set-var", &format!("value validation failed: expected string, got {:?}", other));
          return Err(error_msg);
        },
      };

      debug_log(ctx, "set-var", &format!("setting variable: {} = {}", key, value));

      // Interpolate variables in the value
      debug_log(ctx, "set-var", "starting variable interpolation");
      let interpolated_value = match interpolate_variables(&value, ctx) {
        Ok(val) => {
          debug_log(ctx, "set-var", "variable interpolation successful");
          val
        },
        Err(e) => {
          let error_msg = format!("Error interpolating variable '{}': {}", key, e);
          debug_log(ctx, "set-var", &format!("variable interpolation failed: {}", e));
          return Err(error_msg);
        },
      };

      debug_log(ctx, "set-var", &format!("interpolated value: {} = {}", key, interpolated_value));

      // Store the variable in the context
      debug_log(ctx, "set-var", "storing variable in context");
      ctx.set_variable(key.clone(), Value::Str(interpolated_value.clone()));
      debug_log(ctx, "set-var", "variable successfully stored in context");

      let result_msg = format!("Variable '{}' set to '{}'", key, interpolated_value);
      debug_log(ctx, "set-var", &format!("completed: {}", result_msg));

      Ok(Value::Str(result_msg))
    },
  );
}

/// Register both variable commands
pub fn register_var_commands(registry: &mut CommandRegistry) {
  register_get_var_command(registry);
  register_set_var_command(registry);
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::context::Context;
  use crate::lisp_interpreter::CommandRegistry;

  // Tests for get-var command
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

  // Tests for set-var command
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
      Some(Value::Str("test_value".to_string()))
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

  // Test for combined registration
  #[test]
  fn test_register_var_commands() {
    let mut registry = CommandRegistry::new();
    register_var_commands(&mut registry);
    let mut ctx = Context::new(registry);

    // Test that both commands are registered
    assert!(ctx.registry.get("get-var").is_some());
    assert!(ctx.registry.get("set-var").is_some());

    // Test setting and getting a variable
    let set_args = vec![
      Value::Str("test_key".to_string()),
      Value::Str("test_value".to_string()),
    ];
    ctx
      .registry
      .get("set-var")
      .unwrap()
      .execute(set_args, &mut ctx)
      .unwrap();

    let get_args = vec![Value::Str("test_key".to_string())];
    let result = ctx
      .registry
      .get("get-var")
      .unwrap()
      .execute(get_args, &mut ctx)
      .unwrap();

    assert_eq!(result, Value::Str("test_value".to_string()));
  }
}
