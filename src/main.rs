use std::collections::HashMap;
use std::env;

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

/// Parses command line arguments and creates an execution plan using pipeline approach
///
/// # Returns
/// * `Result<ExecutionPlan, Box<dyn std::error::Error>>` - ExecutionPlan if parsing is successful, Err otherwise
fn parse_arguments() -> Result<ExecutionPlan, Box<dyn std::error::Error>> {
  // Get command line arguments, skipping the program name
  let args: Vec<String> = env::args().skip(1).collect();

  if args.is_empty() {
    return Err("No steps specified".into());
  }

  println!("Pipeline arguments: {:?}", args);

  // Parse pipeline into steps
  let steps = parse_pipeline(args)?;

  let verbose = false; // For now, we'll set verbose to false by default

  if verbose {
    println!(
      "Pipeline steps detected: {:?}",
      steps.iter().map(|s| s.display()).collect::<Vec<_>>()
    );
  }

  // Create execution plan with default values
  let mut plan = ExecutionPlan::new(
    DEFAULT_INPUT_ENV.to_string(),
    None,
    Vec::new(),
    verbose,
  );

  // Process each step and convert to operations
  for step in steps {
    match &step {
      Step::Clean { .. } => {
        // Clean step - for now we'll skip this as it's not in the original operations
        if verbose {
          println!("Clean step: {}", step.display());
        }
      }
      Step::Config { .. } => {
        plan.add_command(step.to_command());
      }
      Step::WriteEnv { output } => {
        plan.output_env = Some(output.clone());
        plan.add_command(step.to_command());
      }
      Step::UpdateVersions => {
        plan.add_command(step.to_command());
      }
      Step::Run { args } => {
        plan.args = args.clone();
        plan.add_command(step.to_command());
      }
    }
  }

  if verbose {
    println!("Input env file: {}", plan.input_env);
    if let Some(ref output_env) = plan.output_env {
      println!("Output env file: {}", output_env);
    }
    if !plan.args.is_empty() {
      println!("Additional args: {:?}", plan.args);
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
      // If the error is "No pipeline steps specified", it means help was shown
      if e.to_string() == "No pipeline steps specified" {
        return Ok(());
      }
      return Err(e);
    }
  };

  // Phase 2: Execute all commands in the planned order using Command pattern
  execution_plan.execute()?;

  Ok(())
}
