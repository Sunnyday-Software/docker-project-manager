//! Context module for the Lisp interpreter
//!
//! This module contains the execution context that holds the command registry
//! and shared state for command execution.

use crate::lisp_interpreter::{CommandRegistry, Value};
use std::collections::HashMap;
use std::path::PathBuf;

/// Execution context for commands
/// Contains the command registry and any shared state
pub struct Context {
  /// Command registry for looking up commands
  pub registry: CommandRegistry,
  /// Variables storage for the session
  pub variables: HashMap<String, Value>,
  /// Debug printing flag - fixed context variable
  pub debug_print: bool,
  pub basedir: PathBuf,
}

impl Context {
  /// Create a new context with the given registry
  pub fn new(registry: CommandRegistry) -> Self {
    Self {
      registry,
      variables: HashMap::new(),
      debug_print: false,
      basedir: PathBuf::from("."),
    }
  }

  /// Set a variable in the context
  pub fn set_variable(&mut self, name: String, value: Value) {
    self.variables.insert(name, value);
  }

  /// Get a variable from the context
  pub fn get_variable(&self, name: &str) -> Option<&Value> {
    self.variables.get(name)
  }

  /// Set the debug print flag
  pub fn set_debug_print(&mut self, enabled: bool) {
    self.debug_print = enabled;
  }

  /// Get the debug print flag
  pub fn get_debug_print(&self) -> bool {
    self.debug_print
  }

  /// Set the base directory
  pub fn set_basedir(&mut self, path: PathBuf) {
    self.basedir = path;
  }

  /// Get the base directory
  pub fn get_basedir(&self) -> &PathBuf {
    &self.basedir
  }

  /// Print the current context state
  /// Returns a formatted string with all context information
  pub fn print_debug_info(&self) -> String {
    let mut output = String::new();

    // Print header
    output.push_str("\n=== DEBUG: Current Program State ===\n");

    // Print fixed context variables
    output.push_str("\n--- Fixed Context Variables ---\n");
    output.push_str(&format!("  debugPrint = {}\n", self.get_debug_print()));
    output.push_str(&format!(
      "  basedir = {}\n",
      self.get_basedir().to_string_lossy()
    ));

    // Print current variables
    output.push_str("\n--- Session Variables ---\n");
    if self.variables.is_empty() {
      output.push_str("  (no variables set)\n");
    } else {
      for (name, value) in &self.variables {
        output.push_str(&format!("  {} = {}\n", name, value.to_string()));
      }
    }

    output.push_str("\n=== End Debug Info ===\n");

    output
  }
}
