use crate::file_ops::{compute_dir_md5, read_env_file, write_env_file};
use crate::context::VersionInfo;
use crate::utils::debug_log;
use crate::{CommandRegistry, Value, tags};
use std::fs;
use std::collections::HashMap;

/// Register version-check command
pub fn register_version_check_command(registry: &mut CommandRegistry) {
  registry.register_closure_with_help_and_tag(
    "version-check",
    "Process subdirectories and create version check data structure",
    "(version-check path)",
    "  (version-check \"docker\")        ; Process subdirectories in docker folder\n  (version-check \"configs\")       ; Process subdirectories in configs folder",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "version-check", "executing version-check command");

      if args.len() != 1 {
        return Err("version-check expects exactly one argument (path)".to_string());
      }

      let path_arg = match &args[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("version-check path must be a string".to_string()),
      };

      debug_log(ctx, "version-check", &format!("processing path argument: {}", path_arg));

      // Resolve path relative to basedir
      let basedir = ctx.get_basedir();
      let version_check_base_dir = basedir.join(&path_arg);

      debug_log(ctx, "version-check", &format!("resolved path: {}", version_check_base_dir.display()));

      // Check if directory exists
      if !version_check_base_dir.exists() {
        return Err(format!("Directory does not exist: {}", version_check_base_dir.display()));
      }

      if !version_check_base_dir.is_dir() {
        return Err(format!("Path is not a directory: {}", version_check_base_dir.display()));
      }

      // Read subdirectories
      let entries = match fs::read_dir(&version_check_base_dir) {
        Ok(entries) => entries,
        Err(e) => return Err(format!("Failed to read directory {}: {}", version_check_base_dir.display(), e)),
      };

      debug_log(ctx, "version-check", "processing subdirectories");

      let mut processed_count = 0;

      for entry in entries {
        let entry = match entry {
          Ok(entry) => entry,
          Err(e) => {
            debug_log(ctx, "version-check", &format!("skipping entry due to error: {}", e));
            continue;
          }
        };

        let entry_path = entry.path();

        // Only process directories
        if !entry_path.is_dir() {
          debug_log(ctx, "version-check", &format!("skipping non-directory: {}", entry_path.display()));
          continue;
        }

        let real_name = match entry.file_name().to_str() {
          Some(name) => name.to_string(),
          None => {
            debug_log(ctx, "version-check", &format!("skipping directory with invalid name: {}", entry_path.display()));
            continue;
          }
        };

        debug_log(ctx, "version-check", &format!("processing directory: {}", real_name));

        // Create v_name: uppercase with non-alphanumeric chars replaced by underscore
        let v_name = real_name
          .to_uppercase()
          .chars()
          .map(|c| if c.is_alphanumeric() { c } else { '_' })
          .collect::<String>();

        // Calculate checksum
        let checksum = match compute_dir_md5(&entry_path.to_string_lossy()) {
          Ok(checksum) => checksum,
          Err(e) => {
            debug_log(ctx, "version-check", &format!("failed to compute checksum for {}: {}", real_name, e));
            continue;
          }
        };

        debug_log(ctx, "version-check", &format!("computed data for {}: v_name={}, checksum={}", real_name, v_name, checksum));

        // Create VersionInfo object
        let version_info = VersionInfo {
          v_name: v_name.clone(),
          real_name: real_name.clone(),
          checksum: checksum.clone(),
        };

        // Store in versions HashMap using v_name as key
        ctx.set_version(v_name, version_info);
        processed_count += 1;
      }

      // Version tracking functionality
      debug_log(ctx, "version-check", "starting version tracking");

      let versions_file_path = version_check_base_dir.join("versions.properties");
      debug_log(ctx, "version-check", &format!("versions file path: {}", versions_file_path.display()));

      // Read existing versions from versions.properties file if it exists
      let existing_versions = if versions_file_path.exists() {
        debug_log(ctx, "version-check", "reading existing versions.properties file");
        match read_env_file(&versions_file_path.to_string_lossy()) {
          Ok(versions) => versions,
          Err(e) => {
            debug_log(ctx, "version-check", &format!("failed to read versions.properties: {}", e));
            HashMap::new()
          }
        }
      } else {
        debug_log(ctx, "version-check", "versions.properties file does not exist, starting fresh");
        HashMap::new()
      };

      // Prepare updated versions data
      let mut updated_versions = HashMap::new();
      let mut version_changes = 0;

      // Process each versioned element
      for (v_name, version_info) in ctx.get_all_versions() {
        let current_checksum = &version_info.checksum;

        // Parse existing version and checksum entries (separate keys)
        let version_key = format!("{}_VERSION", v_name);
        let checksum_key = format!("{}_CHECKSUM", v_name);

        let version_number = if let Some(version_str) = existing_versions.get(&version_key) {
          version_str.parse::<u32>().unwrap_or(1)
        } else {
          // Check for old format (version.checksum) for backward compatibility
          if let Some(existing_entry) = existing_versions.get(v_name) {
            if let Some(dot_pos) = existing_entry.find('.') {
              let version_str = &existing_entry[..dot_pos];
              version_str.parse::<u32>().unwrap_or(1)
            } else {
              1
            }
          } else {
            1
          }
        };

        let stored_checksum = if let Some(checksum_str) = existing_versions.get(&checksum_key) {
          checksum_str.clone()
        } else {
          // Check for old format (version.checksum) for backward compatibility
          if let Some(existing_entry) = existing_versions.get(v_name) {
            if let Some(dot_pos) = existing_entry.find('.') {
              let checksum_str = &existing_entry[dot_pos + 1..];
              checksum_str.to_string()
            } else {
              String::new()
            }
          } else {
            String::new()
          }
        };

        // Check if checksum has changed
        let new_version_number = if stored_checksum != *current_checksum {
          debug_log(ctx, "version-check", &format!("checksum changed for {}: {} -> {}", v_name, stored_checksum, current_checksum));
          version_changes += 1;
          if stored_checksum.is_empty() {
            // New element
            1
          } else {
            // Increment version
            version_number + 1
          }
        } else {
          debug_log(ctx, "version-check", &format!("checksum unchanged for {}: {}", v_name, current_checksum));
          version_number
        };

        // Store updated version and checksum entries (separate keys)
        let version_key = format!("{}_VERSION", v_name);
        let checksum_key = format!("{}_CHECKSUM", v_name);
        updated_versions.insert(version_key, new_version_number.to_string());
        updated_versions.insert(checksum_key, current_checksum.clone());

        debug_log(ctx, "version-check", &format!("version entry for {}: version={}, checksum={}", v_name, new_version_number, current_checksum));
      }

      // Write updated versions.properties file
      debug_log(ctx, "version-check", &format!("writing versions.properties with {} entries ({} elements with version and checksum)", updated_versions.len(), updated_versions.len() / 2));
      match write_env_file(&versions_file_path.to_string_lossy(), &updated_versions) {
        Ok(_) => {
          debug_log(ctx, "version-check", "successfully wrote versions.properties file");
        }
        Err(e) => {
          debug_log(ctx, "version-check", &format!("failed to write versions.properties: {}", e));
          return Err(format!("Failed to write versions.properties file: {}", e));
        }
      }

      let result_msg = format!(
        "Processed {} directories from {} and stored version check data. Version tracking: {} changes detected, versions.properties updated.",
        processed_count,
        version_check_base_dir.display(),
        version_changes
      );

      debug_log(ctx, "version-check", &format!("completed: {}", result_msg));
      Ok(Value::Str(result_msg))
    },
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lisp_interpreter::CommandRegistry;
  use crate::context::Context;
  use std::fs;
  use std::path::PathBuf;

  #[test]
  fn test_version_check_command() {
    // Create a temporary directory structure for testing
    let temp_dir = std::env::temp_dir().join("version_check_test");
    let _ = fs::remove_dir_all(&temp_dir); // Clean up if exists
    fs::create_dir_all(&temp_dir).unwrap();

    // Create test subdirectories
    let subdir1 = temp_dir.join("test-dir-1");
    let subdir2 = temp_dir.join("test_dir_2");
    fs::create_dir_all(&subdir1).unwrap();
    fs::create_dir_all(&subdir2).unwrap();

    // Create some test files in subdirectories
    fs::write(subdir1.join("test.txt"), "test content 1").unwrap();
    fs::write(subdir2.join("test.txt"), "test content 2").unwrap();

    let mut registry = CommandRegistry::new();
    register_version_check_command(&mut registry);
    let mut ctx = Context::new(registry);
    ctx.set_basedir(temp_dir.parent().unwrap().to_path_buf());

    // Test the command
    let args = vec![Value::Str("version_check_test".to_string())];
    let result = ctx
      .registry
      .get("version-check")
      .unwrap()
      .execute(args, &mut ctx)
      .unwrap();

    // Check that the command returns success message
    assert!(result.to_string().contains("Processed 2 directories"));

    // Check that the version check data was stored in versions HashMap
    let versions = ctx.get_all_versions();
    assert_eq!(versions.len(), 2);

    // Check that we have the expected version entries
    let has_test_dir_1 = versions.contains_key("TEST_DIR_1");
    let has_test_dir_2 = versions.contains_key("TEST_DIR_2");
    assert!(has_test_dir_1 || has_test_dir_2, "Should have at least one of the expected directories");

    // Check structure of a version entry
    let version_entry = versions.values().next().unwrap();
    assert!(!version_entry.v_name.is_empty());
    assert!(!version_entry.real_name.is_empty());
    assert!(!version_entry.checksum.is_empty());
    assert_eq!(version_entry.checksum.len(), 8); // MD5 short should be 8 characters

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
  }

  #[test]
  fn test_version_check_wrong_arg_count() {
    let mut registry = CommandRegistry::new();
    register_version_check_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test with wrong number of arguments
    let args = vec![];
    let result = ctx.registry.get("version-check").unwrap().execute(args, &mut ctx);

    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err(),
      "version-check expects exactly one argument (path)"
    );
  }

  #[test]
  fn test_version_check_non_string_arg() {
    let mut registry = CommandRegistry::new();
    register_version_check_command(&mut registry);
    let mut ctx = Context::new(registry);

    // Test with non-string argument
    let args = vec![Value::Int(123)];
    let result = ctx.registry.get("version-check").unwrap().execute(args, &mut ctx);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "version-check path must be a string");
  }

  #[test]
  fn test_version_tracking_functionality() {
    // Create a temporary directory structure for testing
    let temp_dir = std::env::temp_dir().join("version_tracking_test");
    let _ = fs::remove_dir_all(&temp_dir); // Clean up if exists
    fs::create_dir_all(&temp_dir).unwrap();

    // Create test subdirectory
    let subdir1 = temp_dir.join("test-dir");
    fs::create_dir_all(&subdir1).unwrap();

    // Create initial test file
    fs::write(subdir1.join("test.txt"), "initial content").unwrap();

    let mut registry = CommandRegistry::new();
    register_version_check_command(&mut registry);
    let mut ctx = Context::new(registry);
    ctx.set_basedir(temp_dir.parent().unwrap().to_path_buf());

    // First run - should create versions.properties with version 1
    let args = vec![Value::Str("version_tracking_test".to_string())];
    let result1 = ctx
      .registry
      .get("version-check")
      .unwrap()
      .execute(args.clone(), &mut ctx)
      .unwrap();

    // Check that versions.properties file was created
    let versions_file = temp_dir.join("versions.properties");
    assert!(versions_file.exists(), "versions.properties file should be created");

    // Read and verify the versions.properties content
    let versions_content = fs::read_to_string(&versions_file).unwrap();
    assert!(versions_content.contains("TEST_DIR_VERSION=1"), "Should contain TEST_DIR_VERSION with version 1");
    assert!(versions_content.contains("TEST_DIR_CHECKSUM="), "Should contain TEST_DIR_CHECKSUM");

    // Extract the initial checksum
    let initial_checksum = versions_content
      .lines()
      .find(|line| line.starts_with("TEST_DIR_CHECKSUM="))
      .unwrap()
      .split('=')
      .nth(1)
      .unwrap();

    // Modify the file to change the checksum
    fs::write(subdir1.join("test.txt"), "modified content").unwrap();

    // Second run - should increment version to 2
    let result2 = ctx
      .registry
      .get("version-check")
      .unwrap()
      .execute(args, &mut ctx)
      .unwrap();

    // Check that the result message indicates changes were detected
    assert!(result2.to_string().contains("1 changes detected"), "Should detect 1 change");

    // Read and verify the updated versions.properties content
    let updated_versions_content = fs::read_to_string(&versions_file).unwrap();
    assert!(updated_versions_content.contains("TEST_DIR_VERSION=2"), "Should contain TEST_DIR_VERSION with version 2");

    // Extract the new checksum and verify it's different
    let new_checksum = updated_versions_content
      .lines()
      .find(|line| line.starts_with("TEST_DIR_CHECKSUM="))
      .unwrap()
      .split('=')
      .nth(1)
      .unwrap();

    assert_ne!(initial_checksum, new_checksum, "Checksum should be different after file modification");

    // Third run with no changes - should keep version 2
    let result3 = ctx
      .registry
      .get("version-check")
      .unwrap()
      .execute(vec![Value::Str("version_tracking_test".to_string())], &mut ctx)
      .unwrap();

    // Check that no changes were detected
    assert!(result3.to_string().contains("0 changes detected"), "Should detect 0 changes");

    // Verify version is still 2
    let final_versions_content = fs::read_to_string(&versions_file).unwrap();
    assert!(final_versions_content.contains("TEST_DIR_VERSION=2"), "Should still contain TEST_DIR_VERSION with version 2");

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
  }
}
