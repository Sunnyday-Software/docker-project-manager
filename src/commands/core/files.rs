use crate::utils::debug_log;
use crate::{CommandRegistry, Value, tags};
use regex::Regex;
use std::fs;
use std::path::Path;

/// Register filesystem-related core commands
pub fn register_file_commands(registry: &mut CommandRegistry) {
  // fs-list command
  registry.register_closure_with_help_and_tag(
    "fs-list",
    "List files in the current directory matching a wildcard pattern",
    "(fs-list pattern)",
    "  (fs-list \"*.rs\"); List Rust source files in current dir\n  (fs-list \"config.*\")    ; List files starting with 'config.'",
    &tags::COMMANDS,
    |args, ctx| {
      debug_log(ctx, "fs-list", "executing fs-list command");

      if args.len() != 1 {
        return Err("fs-list expects exactly one argument (pattern string)".to_string());
      }

      let pattern = match &args[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("fs-list pattern must be a string".to_string()),
      };

      debug_log(ctx, "fs-list", &format!("received pattern: {}", pattern));

      // Convert wildcard pattern (* and ?) to a regex
      let regex_str = wildcard_to_regex(&pattern);
      let re = match Regex::new(&regex_str) {
        Ok(r) => r,
        Err(e) => return Err(format!("Invalid pattern after conversion to regex: {}", e)),
      };

      debug_log(ctx, "fs-list", &format!("converted to regex: {}", regex_str));

      // Read current directory entries
      let mut results: Vec<Value> = Vec::new();
      let mut count = 0;
      let read_dir = match fs::read_dir(".") {
        Ok(rd) => rd,
        Err(e) => return Err(format!("Failed to read current directory: {}", e)),
      };

      for entry_res in read_dir {
        match entry_res {
          Ok(entry) => {
            let path = entry.path();
            let file_name = match path.file_name().and_then(|s| s.to_str()) {
              Some(n) => n,
              None => continue, // skip non-unicode names
            };

            // Only include files (not directories)
            let is_file = match fs::metadata(&path) {
              Ok(m) => m.is_file(),
              Err(_) => false,
            };

            if is_file && re.is_match(file_name) {
              results.push(Value::Str(file_name.to_string()));
              count += 1;
            }
          }
          Err(e) => {
            debug_log(ctx, "fs-list", &format!("failed to read a directory entry: {}", e));
          }
        }
      }

      debug_log(ctx, "fs-list", &format!("matched {} files", count));
      Ok(Value::List(results))
    },
  );
}

/// Convert a shell-like wildcard pattern to a regular expression string.
/// Supported wildcards:
///  - '*' matches any sequence of characters (including empty)
///  - '?' matches any single character
/// Other characters are escaped to match literally.
fn wildcard_to_regex(pattern: &str) -> String {
  let mut regex = String::from("^");
  for ch in pattern.chars() {
    match ch {
      '*' => regex.push_str(".*"),
      '?' => regex.push('.'),
      // Escape regex metacharacters
      '.' | '+' | '(' | ')' | '|' | '{' | '}' | '[' | ']' | '^' | '$' | '\\' => {
        regex.push('\\');
        regex.push(ch);
      }
      _ => regex.push(ch),
    }
  }
  regex.push('$');
  regex
}
