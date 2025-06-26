use std::env;

mod clean_commands;
mod cli_docs;
mod commands;
mod config_commands;
mod core;
mod docker;
mod env_commands;
mod env_ops;
mod execution;
mod file_ops;
mod lisp_interpreter;
mod model;
mod run_commands;
mod utils;
mod version_commands;

use lisp_interpreter::*;
use commands::{PrintCommand, SumCommand, PipeCommand, MultiplyCommand, ConcatCommand, register_list_commands, register_help_commands, register_basedir_commands};

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

  // Register list utility commands
  register_list_commands(registry);

  // Register help commands
  register_help_commands(registry);

  // Register basedir commands
  register_basedir_commands(registry);
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
    // Execute help command when no arguments are provided
    match evaluate_string("(help)", &mut context) {
      Ok(_) => {}, // Help command already prints its output
      Err(e) => {
        println!("Error executing help command: {}", e);
        return Err(e.into());
      }
    }
  } else {
    // Command line arguments provided, evaluate them as Lisp expressions

    for (i, arg) in args.iter().enumerate() {
      match evaluate_string(arg, &mut context) {
        Ok(result) => print!("{}", result),
        Err(e) => {
          println!("Error: {}\n", e);
          return Err(e.into());
        }
      }
    }
  }

  Ok(())
}
