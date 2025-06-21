use clap::{Arg, ArgMatches, Command};
use std::collections::HashMap;

mod cli_docs;
mod docker;
mod env_ops;
mod file_ops;
mod model;
mod utils;

use cli_docs::*;
use docker::execute_docker_command;
use env_ops::{combine_env_files, create_dir_env_map_and_calculate_md5};
use file_ops::write_env_to_file;
use model::*;
use utils::{setup_project_paths, update_versions};

/// Docker Project Manager - Manages Docker project pipelines
/// with MD5 calculation, version updates and automated command execution.
///
/// This tool allows you to:
/// - Read configurations from specific .env files
/// - Calculate MD5 hashes of directories for versioning
/// - Write .env files with dynamically calculated variables
/// - Update Docker component versions
/// - Execute Docker commands with appropriate configurations

/// Builds the CLI programmatically using clap's Command builder
fn build_cli() -> Command {
  Command::new("dpm")
    .author(CLI_AUTHOR)
    .version(env!("CARGO_PKG_VERSION"))
    .about(CLI_ABOUT)
    .arg(
      Arg::new("verbose")
        .short('v')
        .long("verbose")
        .action(clap::ArgAction::SetTrue)
        .help(CLI_VERBOSE_HELP),
    )
    .arg(
      Arg::new("cfg")
        .short('c')
        .long("cfg")
        .value_name("KEY=VALUE")
        .action(clap::ArgAction::Append)
        .help(CLI_CFG_HELP)
        .long_help(CLI_CFG_LONG_HELP),
    )
    .arg(
      Arg::new("input_env")
        .short('i')
        .long("input-env")
        .value_name("FILE")
        .default_value(DEFAULT_INPUT_ENV)
        .help(CLI_INPUT_ENV_HELP)
        .long_help(CLI_INPUT_ENV_LONG_HELP),
    )
    .arg(
      Arg::new("output_env")
        .short('o')
        .long("output-env")
        .value_name("FILE")
        .help(CLI_OUTPUT_ENV_HELP)
        .long_help(CLI_OUTPUT_ENV_LONG_HELP),
    )
    .arg(
      Arg::new("write_env")
        .short('w')
        .long("write-env")
        .action(clap::ArgAction::Count)
        .help(CLI_WRITE_ENV_HELP)
        .long_help(CLI_WRITE_ENV_LONG_HELP),
    )
    .arg(
      Arg::new("update_versions")
        .short('u')
        .long("update-versions")
        .action(clap::ArgAction::Count)
        .help(CLI_UPDATE_VERSIONS_HELP)
        .long_help(CLI_UPDATE_VERSIONS_LONG_HELP),
    )
    .arg(
      Arg::new("run")
        .short('r')
        .long("run")
        .action(clap::ArgAction::Count)
        .help(CLI_RUN_HELP)
        .long_help(CLI_RUN_LONG_HELP),
    )
    .arg(
      Arg::new("args")
        .value_name("ARGS")
        .num_args(0..)
        .trailing_var_arg(true)
        .help(CLI_ARGS_HELP)
        .long_help(CLI_ARGS_LONG_HELP),
    )
}

/// Parses command line arguments and creates an execution plan
///
/// # Returns
/// * `Result<ExecutionPlan, Box<dyn std::error::Error>>` - ExecutionPlan if parsing is successful, Err otherwise
fn parse_arguments() -> Result<ExecutionPlan, Box<dyn std::error::Error>> {
  // Command line parameter parsing
  let matches = build_cli().get_matches();

  // Extract verbose flag early so it can be used throughout
  let verbose = matches.get_flag("verbose");

  // Extract CLI arguments
  let input_env = matches.get_one::<String>("input_env").unwrap().clone();
  let output_env = matches.get_one::<String>("output_env").cloned();
  let write_env_count = matches.get_count("write_env");
  let update_versions_count = matches.get_count("update_versions");
  let run_count = matches.get_count("run");
  let args: Vec<String> = matches
    .get_many::<String>("args")
    .unwrap_or_default()
    .cloned()
    .collect();

  // Create execution plan
  let mut plan = ExecutionPlan::new(input_env, output_env, args, verbose);

  // Parse command line arguments to determine operation order
  let raw_args: Vec<String> = std::env::args().collect();
  let mut i = 1; // Skip program name
  while i < raw_args.len() {
    let arg = &raw_args[i];
    match arg.as_str() {
      "-w" | "--write-env" => plan.add_operation(Operation::WriteEnv),
      "-u" | "--update-versions" => plan.add_operation(Operation::UpdateVersions),
      "-r" | "--run" => plan.add_operation(Operation::Run),
      "-c" | "--cfg" => {
        // Handle cfg operations in order
        if i + 1 < raw_args.len() {
          let cfg_value = &raw_args[i + 1];
          if let Some((key, value)) = cfg_value.split_once('=') {
            plan.add_operation(Operation::Config {
              key: key.to_string(),
              value: value.to_string(),
            });
          } else {
            let error_msg = format!(
              "Invalid configuration format: '{}'. Use key=value format",
              cfg_value
            );
            return Err(error_msg.into());
          }
          i += 1; // Skip the cfg value
        }
      },
      "--" => break, // Stop parsing at -- separator
      _ => {
        // Handle long form arguments with =
        if arg.starts_with("--cfg=") {
          let cfg_value = &arg[6..]; // Skip "--cfg="
          if let Some((key, value)) = cfg_value.split_once('=') {
            plan.add_operation(Operation::Config {
              key: key.to_string(),
              value: value.to_string(),
            });
          } else {
            let error_msg = format!(
              "Invalid configuration format: '{}'. Use key=value format",
              cfg_value
            );
            return Err(error_msg.into());
          }
        } else if arg.starts_with("-i")
          || arg.starts_with("--input-env")
          || arg.starts_with("-o")
          || arg.starts_with("--output-env")
        {
          if arg.contains('=') {
            // Long form with = (e.g., --input-env=file.env)
            // No need to skip next arg
          } else {
            // Short form or long form without = (e.g., -i file.env)
            i += 1; // Skip the value
          }
        }
      }
    }
    i += 1;
  }

  if verbose && plan.has_operations() {
    let operation_names = plan.operation_names();
    println!(
      "{}",
      MSG_OPERATIONS_DETECTED.replace("{:?}", &format!("{:?}", operation_names))
    );
  }

  // Check if any action flags have been set
  if write_env_count == 0 && update_versions_count == 0 && run_count == 0 {
    // Show complete help when no action is specified
    let mut cmd = build_cli();
    cmd.print_long_help()?;
    println!(); // Empty line for separation
    return Err("No operations specified".into());
  }

  // Validation: if write_env is active, output_env must be specified
  if write_env_count > 0 && plan.output_env.is_none() {
    eprintln!("{}", ERROR_WRITE_ENV_REQUIRES_OUTPUT);
    return Err(ERROR_MISSING_PARAMETERS.into());
  }

  if verbose {
    println!("{}", MSG_INPUT_ENV_SELECTED.replace("{}", &plan.input_env));
    if let Some(ref output_env) = plan.output_env {
      println!("{}", MSG_OUTPUT_ENV_SELECTED.replace("{}", output_env));
    }
    if !plan.args.is_empty() {
      println!(
        "{}",
        MSG_ADDITIONAL_ARGS.replace("{:?}", &format!("{:?}", plan.args))
      );
    }
  }

  Ok(plan)
}


/// Main function of the program that manages the entire execution pipeline.
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if all operations are completed successfully, Err otherwise
///
/// # Note
/// - Handles command line parameter parsing
/// - Configures project paths
/// - Calculates MD5 hashes of Docker directories
/// - Combines environment variables from different .env files
/// - Executes requested operations (.env writing, version updates, Docker command execution)
/// - Handles parameter validation and errors
fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Phase 1: Parse command line arguments and create execution plan
  let execution_plan = match parse_arguments() {
    Ok(plan) => plan,
    Err(e) => {
      // If the error is "No operations specified", it means help was shown
      if e.to_string() == "No operations specified" {
        return Ok(());
      }
      return Err(e);
    }
  };

  // Phase 2: Execute all commands in the planned order using Command pattern
  execution_plan.execute()?;

  Ok(())
}
