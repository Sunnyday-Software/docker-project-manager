use crate::{CommandRegistry, Context, Value, tags};
use crate::utils::debug_log;
use std::process::Command;

/// Register process commands
pub fn register_process_commands(registry: &mut CommandRegistry) {
    // rust-process-command command
    registry.register_closure_with_help_and_tag(
        "rust-process-command",
        "Execute a system command and return the exit status",
        "(rust-process-command program arg1 arg2 ...)",
        "  (rust-process-command \"ls\" \"-la\")  ; List files with details\n  (rust-process-command \"echo\" \"Hello World\")  ; Echo a message",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-process", "executing rust-process-command command");

            if args.is_empty() {
                return Err("rust-process-command expects at least one argument (program name)".to_string());
            }

            let mut command_args = Vec::new();
            for arg in &args {
                match arg {
                    Value::Str(s) => command_args.push(s.clone()),
                    _ => return Err("rust-process-command all arguments must be strings".to_string()),
                }
            }

            let program = &command_args[0];
            let args = &command_args[1..];

            debug_log(ctx, "rust-process", &format!("executing system command: {} with {} arguments", program, args.len()));
            let mut cmd = Command::new(program);
            cmd.args(args);

            match cmd.status() {
                Ok(status) => {
                    let success = status.success();
                    let code = status.code().unwrap_or(-1);
                    debug_log(ctx, "rust-process", &format!("command completed with success: {}, exit code: {}", success, code));
                    Ok(Value::List(vec![
                        Value::Bool(success),
                        Value::Int(code as i64),
                    ]))
                }
                Err(e) => Err(format!("Failed to execute command '{}': {}", program, e)),
            }
        },
    );

    // rust-process-output command
    registry.register_closure_with_help_and_tag(
        "rust-process-output",
        "Execute a system command and return the output (stdout, stderr, status)",
        "(rust-process-output program arg1 arg2 ...)",
        "  (rust-process-output \"echo\" \"Hello\")  ; Get echo output\n  (rust-process-output \"ls\" \"-la\" \"/tmp\")  ; Get directory listing",
        &tags::RUST,
        |args, ctx| {
            debug_log(ctx, "rust-process", "executing rust-process-output command");

            if args.is_empty() {
                return Err("rust-process-output expects at least one argument (program name)".to_string());
            }

            let mut command_args = Vec::new();
            for arg in &args {
                match arg {
                    Value::Str(s) => command_args.push(s.clone()),
                    _ => return Err("rust-process-output all arguments must be strings".to_string()),
                }
            }

            let program = &command_args[0];
            let args = &command_args[1..];

            debug_log(ctx, "rust-process", &format!("executing system command with output capture: {} with {} arguments", program, args.len()));
            let mut cmd = Command::new(program);
            cmd.args(args);

            match cmd.output() {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    let success = output.status.success();
                    let code = output.status.code().unwrap_or(-1);

                    debug_log(ctx, "rust-process", &format!("command completed with success: {}, exit code: {}, stdout: {} bytes, stderr: {} bytes", 
                        success, code, stdout.len(), stderr.len()));

                    Ok(Value::List(vec![
                        Value::Str(stdout),
                        Value::Str(stderr),
                        Value::Bool(success),
                        Value::Int(code as i64),
                    ]))
                }
                Err(e) => Err(format!("Failed to execute command '{}': {}", program, e)),
            }
        },
    );
}
