use crate::{CommandRegistry, Context, Value};

/// Register help commands
pub fn register_help_commands(registry: &mut CommandRegistry) {
  // Help commands
  registry.register_closure_with_help(
    "help",
    "Show short help for all commands",
    "(help)",
    "  (help)              ; Shows short help",
    |args, ctx| {
      if !args.is_empty() {
        return Err("help expects no arguments".to_string());
      }

      let tag_groups = ctx.registry.get_commands_grouped_by_tags();

      let mut help_text = String::from("Available commands:\n\n");
      for (tag, commands) in tag_groups {
        help_text.push_str(&format!("=== {} ===\n", tag.text));
        for (name, description) in commands {
          help_text.push_str(&format!("  {:<12} - {}\n", name, description));
        }
        help_text.push_str("\n");
      }

      println!("{}", help_text);
      Ok(Value::Str(help_text))
    },
  );

  registry.register_closure_with_help(
    "help-long",
    "Show detailed help with syntax and examples",
    "(help-long)",
    "  (help-long)         ; Shows this detailed help",
    |args, ctx| {
      if !args.is_empty() {
        return Err("help-long expects no arguments".to_string());
      }

      let tag_groups = ctx.registry.get_commands_grouped_by_tags_with_help();

      let mut help_text =
        String::from("=== DETAILED COMMAND REFERENCE ===\n\n");

      for (tag, commands) in tag_groups {
        help_text.push_str(&format!("=== {} ===\n\n", tag.text));
        for (name, description, syntax, examples) in commands {
          help_text.push_str(&format!("Command: {}\n", name));
          help_text.push_str(&format!("Description: {}\n", description));
          help_text.push_str(&format!("Syntax: {}\n", syntax));
          help_text.push_str("Examples:\n");
          help_text.push_str(&format!("{}\n", examples));
          help_text.push_str("\n");
        }
        help_text.push_str("\n");
      }

      help_text.push_str("=== GENERAL USAGE ===\n");
      help_text
        .push_str("All commands use Lisp-style syntax with parentheses:\n");
      help_text.push_str("  (command-name arg1 arg2 ...)\n\n");
      help_text.push_str("Commands can be nested:\n");
      help_text
        .push_str("  (print (sum 1 2 3))  ; Prints the result of sum\n\n");
      help_text.push_str("Multiple expressions can be evaluated:\n");
      help_text.push_str("  ./dpm '(sum 1 2 3)' '(print \"Hello\")'\n");

      println!("{}", help_text);
      Ok(Value::Str(help_text))
    },
  );
}
