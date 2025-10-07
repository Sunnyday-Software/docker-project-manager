use std::env;
use std::io::{self, BufRead, BufReader};

mod commands;
mod config_commands;
mod context;
mod core;
mod docker;
mod emoji;
mod env_commands;
mod env_ops;
mod file_ops;
mod lisp_interpreter;
mod model;
mod utils;

use commands::{
  ConcatCommand, DebugCommand, MultiplyCommand, PipeCommand, PrintCommand,
  SumCommand, register_all_rust_commands, register_app_commands,
  register_basedir_commands, register_help_commands, register_list_commands,
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

fn print_usage() {
  println!(
    "Usage:\n  --pipe                 Read commands from standard input (pipe)\n  --command <string>     Execute the provided command string\n  --file <path>          Read command(s) from the specified file\n\nExamples:\n  echo \"(print \"Hello\")\" | dpm --pipe\n  dpm --command \"(print \"Hello\")\"\n  dpm --file script.lisp"
  );
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
    // No arguments: show usage and exit
    print_usage();
    return Ok(());
  }

  match args[0].as_str() {
    "--pipe" => {
      // Read from stdin
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
              Ok(_) => {}
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
    }
    "--command" => {
      if args.len() < 2 {
        println!("Error: --command requires a command string\n");
        print_usage();
        return Err("missing --command argument".into());
      }
      // Join remaining args to support spaces without quoting across some shells
      let cmd = args[1..].join(" ");
      match evaluate_string(&cmd, &mut context) {
        Ok(_) => {}
        Err(e) => {
          println!("Error: {}\n", e);
          return Err(e.into());
        }
      }
    }
    "--file" => {
      if args.len() < 2 {
        println!("Error: --file requires a path to a file\n");
        print_usage();
        return Err("missing --file argument".into());
      }
      let path = &args[1];
      let content = std::fs::read_to_string(path)?;
      match evaluate_string(&content, &mut context) {
        Ok(_) => {}
        Err(e) => {
          println!("Error: {}\n", e);
          return Err(e.into());
        }
      }
    }
    _ => {
      // Unknown option: show usage
      print_usage();
      return Ok(());
    }
  }

  Ok(())
}
