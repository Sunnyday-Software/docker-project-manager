//! # Lisp-style Command Interpreter Library
//!
//! A mini-library for building Lisp-style command interpreters for command line applications.
//! This library provides a complete framework for parsing S-expressions, managing commands,
//! and executing command pipelines in a functional programming style.
//!
//! ## Features
//! - S-expression parsing using lexpr
//! - Universal Value type system
//! - Dynamic command registration
//! - Context-aware command execution
//! - Built-in pipeline support
//! - Extensible command system
//!
//! ## Example Usage
//! ```rust
//! use lisp_interpreter::*;
//!
//! let mut registry = CommandRegistry::new();
//! register_builtin_commands(&mut registry);
//!
//! let mut context = Context::new(registry);
//! let result = evaluate_string("(pipe (sum 1 2 3) (print))", &mut context)?;
//! ```

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

use crate::context::Context;

/// Universal value type for the Lisp interpreter
/// Represents all possible values that can be passed between commands
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  /// Integer value
  Int(i64),
  /// String value
  Str(String),
  /// Boolean value
  Bool(bool),
  /// List of values
  List(Vec<Value>),
  /// Nil/null value
  Nil,
}

impl Value {
  /// Converts a lexpr::Value to our Value type
  pub fn from_lexpr(lexpr_value: &lexpr::Value) -> Result<Value, String> {
    match lexpr_value {
      lexpr::Value::Nil => Ok(Value::Nil),
      lexpr::Value::Bool(b) => Ok(Value::Bool(*b)),
      lexpr::Value::Number(n) => {
        if let Some(i) = n.as_i64() {
          Ok(Value::Int(i))
        } else if let Some(f) = n.as_f64() {
          Ok(Value::Int(f as i64))
        } else {
          Err(format!("Unsupported number format: {}", n))
        }
      }
      lexpr::Value::String(s) => Ok(Value::Str(s.to_string())),
      lexpr::Value::Symbol(s) => Ok(Value::Str(s.to_string())),
      lexpr::Value::Cons(cons) => {
        let mut result = Vec::new();
        let mut current = lexpr_value;

        loop {
          match current {
            lexpr::Value::Cons(cons) => {
              result.push(Value::from_lexpr(cons.car())?);
              current = cons.cdr();
            }
            lexpr::Value::Nil => break,
            _ => {
              result.push(Value::from_lexpr(current)?);
              break;
            }
          }
        }
        Ok(Value::List(result))
      }
      lexpr::Value::Null => Ok(Value::Nil),
      _ => Err(format!("Unsupported lexpr value type: {:?}", lexpr_value)),
    }
  }

  /// Converts our Value to lexpr::Value
  pub fn to_lexpr(&self) -> lexpr::Value {
    match self {
      Value::Nil => lexpr::Value::Nil,
      Value::Int(i) => lexpr::Value::Number((*i).into()),
      Value::Str(s) => lexpr::Value::String(s.clone().into()),
      Value::Bool(b) => lexpr::Value::Bool(*b),
      Value::List(list) => {
        let mut result = lexpr::Value::Nil;
        for item in list.iter().rev() {
          result = lexpr::Value::cons(item.to_lexpr(), result);
        }
        result
      }
    }
  }

  /// Checks if the value is truthy (non-nil and non-zero)
  pub fn is_truthy(&self) -> bool {
    match self {
      Value::Nil => false,
      Value::Int(0) => false,
      Value::Bool(b) => *b,
      _ => true,
    }
  }

  /// Converts value to string representation
  pub fn to_string(&self) -> String {
    match self {
      Value::Nil => "nil".to_string(),
      Value::Int(i) => i.to_string(),
      Value::Str(s) => s.clone(),
      Value::Bool(b) => {
        if *b {
          "true".to_string()
        } else {
          "false".to_string()
        }
      }
      Value::List(list) => {
        let items: Vec<String> = list.iter().map(|v| v.to_string()).collect();
        format!("({})", items.join(" "))
      }
    }
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

/// Tag for categorizing commands
/// Tags have an order for sorting and descriptive text
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
  pub name: &'static str,
  pub order: u32,
  pub text: &'static str,
}

impl Tag {
  pub fn new(name: &'static str, order: u32, text: &'static str) -> Self {
    Self { name, order, text }
  }
}

/// Predefined tags for commands
pub mod tags {
  use super::Tag;

  pub const CORE: Tag = Tag {
    name: "core",
    order: 1000,
    text: "Core Commands",
  };
  pub const COMMANDS: Tag = Tag {
    name: "commands",
    order: 2,
    text: "Command Management",
  };
  pub const RUST: Tag = Tag {
    name: "rust",
    order: 9999,
    text: "Rust Standard Library",
  };
}

/// Command trait for implementing executable commands
/// All commands must implement this trait to be usable in the interpreter
pub trait Command: Send + Sync {
  /// Execute the command with given arguments and context
  ///
  /// # Arguments
  /// * `args` - Vector of arguments passed to the command
  /// * `ctx` - Mutable reference to the execution context
  ///
  /// # Returns
  /// * `Result<Value, String>` - The result value or an error message
  fn execute(
    &self,
    args: Vec<Value>,
    ctx: &mut Context,
  ) -> Result<Value, String>;

  /// Get the name of the command
  fn name(&self) -> &'static str;

  /// Get a description of the command for help/documentation
  fn description(&self) -> &'static str {
    "No description available"
  }

  /// Get the syntax of the command for help/documentation
  fn syntax(&self) -> &'static str {
    "Syntax not documented"
  }

  /// Get examples of the command for help/documentation
  fn examples(&self) -> &'static str {
    "Examples not available"
  }

  /// Get the tag for this command
  fn tag(&self) -> &Tag {
    &tags::CORE
  }
}

/// Type alias for boxed commands
pub type BoxedCommand = Box<dyn Command>;

/// Registry for managing commands
/// Provides thread-safe storage and retrieval of commands
#[derive(Clone)]
pub struct CommandRegistry {
  commands: Arc<Mutex<HashMap<String, Arc<dyn Command>>>>,
}

impl CommandRegistry {
  /// Create a new empty command registry
  pub fn new() -> Self {
    Self {
      commands: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  /// Register a command in the registry
  ///
  /// # Arguments
  /// * `command` - The command to register
  pub fn register<C: Command + 'static>(&mut self, command: C) {
    let mut commands = self.commands.lock().unwrap();
    commands.insert(command.name().to_string(), Arc::new(command));
  }

  /// Register a command using a closure
  ///
  /// # Arguments
  /// * `name` - Name of the command
  /// * `description` - Description of the command
  /// * `func` - Closure that implements the command logic
  pub fn register_closure<F>(
    &mut self,
    name: &'static str,
    description: &'static str,
    func: F,
  ) where
    F: Fn(Vec<Value>, &mut Context) -> Result<Value, String>
      + Send
      + Sync
      + 'static,
  {
    self.register_closure_with_tag(name, description, &tags::CORE, func);
  }

  /// Register a command using a closure with a specific tag
  ///
  /// # Arguments
  /// * `name` - Name of the command
  /// * `description` - Description of the command
  /// * `tag` - Tag for categorizing the command
  /// * `func` - Closure that implements the command logic
  pub fn register_closure_with_tag<F>(
    &mut self,
    name: &'static str,
    description: &'static str,
    tag: &'static Tag,
    func: F,
  ) where
    F: Fn(Vec<Value>, &mut Context) -> Result<Value, String>
      + Send
      + Sync
      + 'static,
  {
    struct ClosureCommand<F> {
      name: &'static str,
      description: &'static str,
      tag: &'static Tag,
      func: F,
    }

    impl<F> Command for ClosureCommand<F>
    where
      F: Fn(Vec<Value>, &mut Context) -> Result<Value, String> + Send + Sync,
    {
      fn execute(
        &self,
        args: Vec<Value>,
        ctx: &mut Context,
      ) -> Result<Value, String> {
        (self.func)(args, ctx)
      }

      fn name(&self) -> &'static str {
        self.name
      }

      fn description(&self) -> &'static str {
        self.description
      }

      fn tag(&self) -> &Tag {
        self.tag
      }
    }

    self.register(ClosureCommand {
      name,
      description,
      tag,
      func,
    });
  }

  /// Register a command using a closure with help information
  ///
  /// # Arguments
  /// * `name` - Name of the command
  /// * `description` - Description of the command
  /// * `syntax` - Syntax of the command
  /// * `examples` - Examples of the command
  /// * `func` - Closure that implements the command logic
  pub fn register_closure_with_help<F>(
    &mut self,
    name: &'static str,
    description: &'static str,
    syntax: &'static str,
    examples: &'static str,
    func: F,
  ) where
    F: Fn(Vec<Value>, &mut Context) -> Result<Value, String>
      + Send
      + Sync
      + 'static,
  {
    self.register_closure_with_help_and_tag(
      name,
      description,
      syntax,
      examples,
      &tags::CORE,
      func,
    );
  }

  /// Register a command using a closure with help information and a specific tag
  ///
  /// # Arguments
  /// * `name` - Name of the command
  /// * `description` - Description of the command
  /// * `syntax` - Syntax of the command
  /// * `examples` - Examples of the command
  /// * `tag` - Tag for categorizing the command
  /// * `func` - Closure that implements the command logic
  pub fn register_closure_with_help_and_tag<F>(
    &mut self,
    name: &'static str,
    description: &'static str,
    syntax: &'static str,
    examples: &'static str,
    tag: &'static Tag,
    func: F,
  ) where
    F: Fn(Vec<Value>, &mut Context) -> Result<Value, String>
      + Send
      + Sync
      + 'static,
  {
    struct ClosureCommandWithHelp<F> {
      name: &'static str,
      description: &'static str,
      syntax: &'static str,
      examples: &'static str,
      tag: &'static Tag,
      func: F,
    }

    impl<F> Command for ClosureCommandWithHelp<F>
    where
      F: Fn(Vec<Value>, &mut Context) -> Result<Value, String> + Send + Sync,
    {
      fn execute(
        &self,
        args: Vec<Value>,
        ctx: &mut Context,
      ) -> Result<Value, String> {
        (self.func)(args, ctx)
      }

      fn name(&self) -> &'static str {
        self.name
      }

      fn description(&self) -> &'static str {
        self.description
      }

      fn syntax(&self) -> &'static str {
        self.syntax
      }

      fn examples(&self) -> &'static str {
        self.examples
      }

      fn tag(&self) -> &Tag {
        self.tag
      }
    }

    self.register(ClosureCommandWithHelp {
      name,
      description,
      syntax,
      examples,
      tag,
      func,
    });
  }

  /// Get a command by name
  ///
  /// # Arguments
  /// * `name` - Name of the command to retrieve
  ///
  /// # Returns
  /// * `Option<Arc<dyn Command>>` - The command if found
  pub fn get(&self, name: &str) -> Option<Arc<dyn Command>> {
    let commands = self.commands.lock().unwrap();
    commands.get(name).cloned()
  }

  /// List all registered command names
  pub fn list_commands(&self) -> Vec<String> {
    let commands = self.commands.lock().unwrap();
    commands.keys().cloned().collect()
  }

  /// Get all commands with their descriptions
  pub fn get_commands_with_descriptions(&self) -> Vec<(String, String)> {
    let commands = self.commands.lock().unwrap();
    commands
      .iter()
      .map(|(name, command)| (name.clone(), command.description().to_string()))
      .collect()
  }

  /// Get all commands with their help information (description, syntax, examples)
  pub fn get_commands_with_help(
    &self,
  ) -> Vec<(String, String, String, String)> {
    let commands = self.commands.lock().unwrap();
    commands
      .iter()
      .map(|(name, command)| {
        (
          name.clone(),
          command.description().to_string(),
          command.syntax().to_string(),
          command.examples().to_string(),
        )
      })
      .collect()
  }

  /// Get commands grouped by tags with descriptions
  pub fn get_commands_grouped_by_tags(
    &self,
  ) -> Vec<(Tag, Vec<(String, String)>)> {
    let commands = self.commands.lock().unwrap();
    let mut tag_groups: HashMap<String, (Tag, Vec<(String, String)>)> =
      HashMap::new();

    for (name, command) in commands.iter() {
      let tag = command.tag().clone();
      let entry = tag_groups
        .entry(tag.name.to_string())
        .or_insert((tag, Vec::new()));
      entry
        .1
        .push((name.clone(), command.description().to_string()));
    }

    let mut result: Vec<(Tag, Vec<(String, String)>)> =
      tag_groups.into_values().collect();

    // Sort by tag order
    result.sort_by(|a, b| a.0.order.cmp(&b.0.order));

    // Sort commands within each tag
    for (_, commands) in result.iter_mut() {
      commands.sort_by(|a, b| a.0.cmp(&b.0));
    }

    result
  }

  /// Get commands grouped by tags with full help information
  pub fn get_commands_grouped_by_tags_with_help(
    &self,
  ) -> Vec<(Tag, Vec<(String, String, String, String)>)> {
    let commands = self.commands.lock().unwrap();
    let mut tag_groups: HashMap<
      String,
      (Tag, Vec<(String, String, String, String)>),
    > = HashMap::new();

    for (name, command) in commands.iter() {
      let tag = command.tag().clone();
      let entry = tag_groups
        .entry(tag.name.to_string())
        .or_insert((tag, Vec::new()));
      entry.1.push((
        name.clone(),
        command.description().to_string(),
        command.syntax().to_string(),
        command.examples().to_string(),
      ));
    }

    let mut result: Vec<(Tag, Vec<(String, String, String, String)>)> =
      tag_groups.into_values().collect();

    // Sort by tag order
    result.sort_by(|a, b| a.0.order.cmp(&b.0.order));

    // Sort commands within each tag
    for (_, commands) in result.iter_mut() {
      commands.sort_by(|a, b| a.0.cmp(&b.0));
    }

    result
  }
}

/// Enhanced parsing function that handles multi-line expressions
///
/// # Arguments
/// * `input` - String containing S-expressions
///
/// # Returns
/// * `Result<Vec<lexpr::Value>, String>` - Vector of parsed AST nodes or error
pub fn parse_string(input: &str) -> Result<Vec<lexpr::Value>, String> {
  let mut results = Vec::new();
  let trimmed = input.trim();

  if trimmed.is_empty() {
    return Ok(results);
  }

  // Try simple parsing first
  match lexpr::from_str(trimmed) {
    Ok(value) => {
      results.push(value);
      return Ok(results);
    }
    Err(_) => {} // Continue with advanced parsing
  }

  // Advanced parsing for multi-line expressions
  let mut chars = trimmed.chars().peekable();
  let mut current_expr = String::new();
  let mut paren_depth = 0;
  let mut in_string = false;
  let mut escape_next = false;

  while let Some(ch) = chars.next() {
    if escape_next {
      current_expr.push(ch);
      escape_next = false;
      continue;
    }

    match ch {
      '\\' if in_string => {
        current_expr.push(ch);
        escape_next = true;
      }
      '"' => {
        current_expr.push(ch);
        in_string = !in_string;
      }
      '(' if !in_string => {
        current_expr.push(ch);
        paren_depth += 1;
      }
      ')' if !in_string => {
        current_expr.push(ch);
        paren_depth -= 1;

        if paren_depth == 0 {
          let expr = current_expr.trim();
          if !expr.is_empty() {
            match lexpr::from_str(expr) {
              Ok(value) => results.push(value),
              Err(e) => {
                return Err(format!(
                  "Parse error in expression '{}': {}",
                  expr, e
                ));
              }
            }
          }
          current_expr.clear();
        }
      }
      _ => {
        current_expr.push(ch);
      }
    }
  }

  // Handle remaining expression
  let remaining = current_expr.trim();
  if !remaining.is_empty() {
    if paren_depth != 0 {
      return Err(format!("Unbalanced parentheses: {}", remaining));
    }

    match lexpr::from_str(remaining) {
      Ok(value) => results.push(value),
      Err(e) => return Err(format!("Parse error: {}", e)),
    }
  }

  if results.is_empty() {
    return Err("No valid expressions found".to_string());
  }

  Ok(results)
}

/// Normalize whitespace and parse multi-line expressions
///
/// # Arguments
/// * `input` - String containing S-expressions with potential comments and multi-line formatting
///
/// # Returns
/// * `Result<Vec<lexpr::Value>, String>` - Vector of parsed AST nodes or error
pub fn parse_string_normalized(
  input: &str,
) -> Result<Vec<lexpr::Value>, String> {
  let normalized = input
    .lines()
    .map(|line| {
      // Remove inline comments
      let without_comment = if let Some(pos) = line.find(';') {
        &line[..pos]
      } else {
        line
      };
      without_comment.trim()
    })
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

  parse_string(&normalized)
}

/// Format multi-line S-expression to single line
///
/// # Arguments
/// * `input` - String containing potentially multi-line S-expressions
///
/// # Returns
/// * `String` - Formatted single-line S-expression
pub fn format_sexpr(input: &str) -> String {
  input
    .lines()
    .map(|line| {
      let trimmed = if let Some(comment_pos) = line.find(';') {
        line[..comment_pos].trim()
      } else {
        line.trim()
      };
      trimmed
    })
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>()
    .join(" ")
}

/// Evaluate a single AST node
///
/// # Arguments
/// * `ast` - The AST node to evaluate
/// * `ctx` - Mutable reference to the execution context
///
/// # Returns
/// * `Result<Value, String>` - The result value or error
pub fn evaluate(
  ast: &lexpr::Value,
  ctx: &mut Context,
) -> Result<Value, String> {
  match ast {
    lexpr::Value::Cons(cons) => {
      // This is a function call
      let car = cons.car();
      let command_name = match car {
        lexpr::Value::Symbol(s) => s.to_string(),
        _ => {
          return Err(
            "First element of list must be a command name".to_string(),
          );
        }
      };

      // Get the command from registry
      let command = ctx
        .registry
        .get(&command_name)
        .ok_or_else(|| format!("Unknown command: {}", command_name))?;

      // Evaluate arguments
      let mut args = Vec::new();
      let mut current = cons.cdr();

      loop {
        match current {
          lexpr::Value::Cons(cons) => {
            let arg_value = evaluate(cons.car(), ctx)?;
            args.push(arg_value);
            current = cons.cdr();
          }
          lexpr::Value::Nil | lexpr::Value::Null => {
            break;
          }
          _ => {
            let arg_value = evaluate(current, ctx)?;
            args.push(arg_value);
            break;
          }
        }
      }

      // Execute the command
      command.execute(args, ctx)
    }
    _ => {
      // This is a literal value
      Value::from_lexpr(ast)
    }
  }
}

/// Evaluate a string containing S-expressions
///
/// # Arguments
/// * `input` - String containing S-expressions
/// * `ctx` - Mutable reference to the execution context
///
/// # Returns
/// * `Result<Value, String>` - The result of the last expression or error
pub fn evaluate_string(
  input: &str,
  ctx: &mut Context,
) -> Result<Value, String> {
  let ast_nodes =
    parse_string_normalized(input).or_else(|_| parse_string(input))?;
  let mut last_result = Value::Nil;

  for ast in ast_nodes {
    last_result = evaluate(&ast, ctx)?;
  }

  Ok(last_result)
}

/// Utility macro for easy command registration
///
/// # Example
/// ```rust
/// register_command!(registry, "my_cmd", "My command description", |args, ctx| {
///     // Command implementation
///     Ok(Value::Str("Hello".to_string()))
/// });
/// ```
#[macro_export]
macro_rules! register_command {
  ($registry:expr, $name:expr, $desc:expr, $func:expr) => {
    $registry.register_closure($name, $desc, $func);
  };
}

/// Utility function to convert a vector of strings to Values
pub fn strings_to_values(strings: Vec<String>) -> Vec<Value> {
  strings.into_iter().map(|s| Value::Str(s)).collect()
}

/// Utility function to convert a vector of integers to Values
pub fn ints_to_values(ints: Vec<i64>) -> Vec<Value> {
  ints.into_iter().map(|i| Value::Int(i)).collect()
}

/// Utility function to convert a vector of booleans to Values
pub fn bools_to_values(bools: Vec<bool>) -> Vec<Value> {
  bools.into_iter().map(|b| Value::Bool(b)).collect()
}

/// Helper function to extract integer from Value
pub fn value_to_int(value: &Value) -> Result<i64, String> {
  match value {
    Value::Int(i) => Ok(*i),
    _ => Err(format!("Expected integer, got: {}", value)),
  }
}

/// Helper function to extract string from Value
pub fn value_to_string(value: &Value) -> Result<String, String> {
  match value {
    Value::Str(s) => Ok(s.clone()),
    _ => Err(format!("Expected string, got: {}", value)),
  }
}

/// Helper function to extract list from Value
pub fn value_to_list(value: &Value) -> Result<Vec<Value>, String> {
  match value {
    Value::List(list) => Ok(list.clone()),
    _ => Err(format!("Expected list, got: {}", value)),
  }
}

/// Helper function to extract boolean from Value
pub fn value_to_bool(value: &Value) -> Result<bool, String> {
  match value {
    Value::Bool(b) => Ok(*b),
    _ => Err(format!("Expected boolean, got: {}", value)),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::commands::{DebugCommand, PipeCommand, PrintCommand, SumCommand};
  use crate::commands::{register_help_commands, register_list_commands};

  /// Test helper function to register builtin commands for testing
  fn register_test_commands(registry: &mut CommandRegistry) {
    // Register struct-based commands
    registry.register(PrintCommand);
    registry.register(SumCommand);
    registry.register(PipeCommand);
    registry.register(DebugCommand);

    // Register list utility commands
    register_list_commands(registry);

    // Register help commands
    register_help_commands(registry);
  }

  #[test]
  fn test_value_conversions() {
    let int_val = Value::Int(42);
    let lexpr_val = int_val.to_lexpr();
    let back_val = Value::from_lexpr(&lexpr_val).unwrap();
    assert_eq!(int_val, back_val);

    // Test boolean conversions
    let bool_true = Value::Bool(true);
    let lexpr_true = bool_true.to_lexpr();
    let back_true = Value::from_lexpr(&lexpr_true).unwrap();
    assert_eq!(bool_true, back_true);

    let bool_false = Value::Bool(false);
    let lexpr_false = bool_false.to_lexpr();
    let back_false = Value::from_lexpr(&lexpr_false).unwrap();
    assert_eq!(bool_false, back_false);
  }

  #[test]
  fn test_basic_evaluation() {
    let mut registry = CommandRegistry::new();
    register_test_commands(&mut registry);
    let mut ctx = Context::new(registry);

    let result = evaluate_string("(sum 1 2 3)", &mut ctx).unwrap();
    assert_eq!(result, Value::Int(6));
  }

  #[test]
  fn test_boolean_functionality() {
    // Test boolean utility functions
    let bool_values = bools_to_values(vec![true, false, true]);
    assert_eq!(bool_values.len(), 3);
    assert_eq!(bool_values[0], Value::Bool(true));
    assert_eq!(bool_values[1], Value::Bool(false));
    assert_eq!(bool_values[2], Value::Bool(true));

    // Test value_to_bool function
    let true_val = Value::Bool(true);
    let false_val = Value::Bool(false);
    assert_eq!(value_to_bool(&true_val).unwrap(), true);
    assert_eq!(value_to_bool(&false_val).unwrap(), false);

    // Test is_truthy with boolean values
    assert_eq!(true_val.is_truthy(), true);
    assert_eq!(false_val.is_truthy(), false);

    // Test to_string with boolean values
    assert_eq!(true_val.to_string(), "true");
    assert_eq!(false_val.to_string(), "false");
  }

  #[test]
  fn test_print_command() {
    let mut registry = CommandRegistry::new();
    register_test_commands(&mut registry);
    let mut ctx = Context::new(registry);

    let result =
      evaluate_string("(print \"Hello\" \"World\")", &mut ctx).unwrap();
    assert_eq!(result, Value::Str("Hello World".to_string()));
  }

  #[test]
  fn test_debug_command() {
    let mut registry = CommandRegistry::new();
    register_test_commands(&mut registry);
    let mut ctx = Context::new(registry);

    // Test debug command with "true" parameter
    let result = evaluate_string("(debug \"true\")", &mut ctx).unwrap();
    assert_eq!(result, Value::Str("Debug printing enabled".to_string()));

    // Verify debugPrint is set to true
    assert_eq!(ctx.get_debug_print(), true);

    // Test debug command with "false" parameter
    let result = evaluate_string("(debug \"false\")", &mut ctx).unwrap();
    assert_eq!(result, Value::Str("Debug printing disabled".to_string()));

    // Verify debugPrint is set to false
    assert_eq!(ctx.get_debug_print(), false);

    // Test debug command with case insensitive "TRUE"
    let result = evaluate_string("(debug \"TRUE\")", &mut ctx).unwrap();
    assert_eq!(result, Value::Str("Debug printing enabled".to_string()));

    // Test debug command with case insensitive "False"
    let result = evaluate_string("(debug \"False\")", &mut ctx).unwrap();
    assert_eq!(result, Value::Str("Debug printing disabled".to_string()));

    // Test debug command with no parameters (original behavior)
    // Set a test variable first
    ctx.set_variable("testVar".to_string(), Value::Int(42));
    let result = evaluate_string("(debug)", &mut ctx).unwrap();
    // The result should contain debug information as a string
    assert!(result.to_string().contains("testVar = 42"));
    assert!(result.to_string().contains("debugPrint = false"));

    // Test error cases
    let error_result = evaluate_string("(debug \"invalid\")", &mut ctx);
    assert!(error_result.is_err());
    assert!(
      error_result
        .unwrap_err()
        .contains("must be 'true' or 'false'")
    );

    let error_result = evaluate_string("(debug \"true\" \"extra\")", &mut ctx);
    assert!(error_result.is_err());
    assert!(error_result.unwrap_err().contains("exactly one argument"));
  }

  #[test]
  fn test_multiline_parsing_issue() {
    // Test case from the issue description - this should fail with current implementation
    let multiline_input = r#"(docker-compose-args 
        "compose" 
        "-f" 
        "docker-compose.core.yml" 
        "-f" 
        "docker-compose.yml" 
        "run" 
        "--rm" 
        "--no-deps" 
        "-T")"#;

    // Test current parse_string function - this should fail
    let result = parse_string(multiline_input);
    println!("Multi-line parsing result: {:?}", result);

    // For now, we expect this to fail, but after implementing the fix it should succeed
    // assert!(result.is_err(), "Current implementation should fail on multi-line input");

    // Test that lexpr::from_str works with normalized input
    let normalized_input = multiline_input
      .lines()
      .map(|line| line.trim())
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>()
      .join(" ");

    let lexpr_result = lexpr::from_str(&normalized_input);
    assert!(
      lexpr_result.is_ok(),
      "lexpr should work with normalized input"
    );
  }
}
