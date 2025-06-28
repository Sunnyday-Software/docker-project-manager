use std::env;
use std::io::{self, BufRead, BufReader};

mod clean_commands;
mod cli_docs;
mod commands;
mod config_commands;
mod context;
mod core;
mod docker;
mod emoji;
mod env_commands;
mod env_ops;
mod execution;
mod file_ops;
mod lisp_interpreter;
mod model;
mod run_commands;
mod utils;
mod version_commands;

use commands::{
  ConcatCommand, DebugCommand, MultiplyCommand, PipeCommand, PrintCommand,
  SumCommand, register_basedir_commands, register_help_commands,
  register_list_commands, register_all_rust_commands, register_app_commands,
};
use context::Context;
use lisp_interpreter::*;

/// Register all built-in commands in the registry
///
/// # Arguments
/// * `registry` - Mutable reference to the command registry
fn register_builtin_commands(registry: &mut CommandRegistry) {
  // Register struct-based commands
  registry.register(PrintCommand);
  registry.register(SumCommand);
  registry.register(PipeCommand);
  registry.register(MultiplyCommand);
  registry.register(ConcatCommand);
  registry.register(DebugCommand);

  // Register list utility commands
  register_list_commands(registry);

  // Register help commands
  register_help_commands(registry);

  // Register basedir commands
  register_basedir_commands(registry);

  // Register app commands
  register_app_commands(registry);

  // Register Rust standard library commands
  register_all_rust_commands(registry);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Step 1: Create command registry and register built-in commands
  let mut registry = CommandRegistry::new();
  register_builtin_commands(&mut registry);

  // Step 2: Create execution context
  let mut context = Context::new(registry);

  // Step 3: Get command line arguments
  let args: Vec<String> = env::args().skip(1).collect();

  if args.is_empty() {
    // No command line arguments provided, read from stdin
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines() {
      match line {
        Ok(input) => {
          let trimmed = input.trim();
          if trimmed.is_empty() {
            continue; // Skip empty lines
          }

          match evaluate_string(trimmed, &mut context) {
            Ok(_result) => {
              // Command executed successfully
            }
            Err(e) => {
              println!("Error: {}", e);
              // Continue processing other lines instead of exiting
            }
          }
        }
        Err(e) => {
          println!("Error reading from stdin: {}", e);
          return Err(e.into());
        }
      }
    }
  } else {
    // Command line arguments provided, evaluate them as Lisp expressions
    for (i, arg) in args.iter().enumerate() {
      match evaluate_string(arg, &mut context) {
        Ok(result) => {}
        Err(e) => {
          println!("Error: {}\n", e);
          return Err(e.into());
        }
      }
    }
  }

  Ok(())
}
