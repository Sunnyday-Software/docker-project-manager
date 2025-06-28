pub mod env;
pub mod path;
pub mod fs;
pub mod process;

pub use env::register_env_commands;
pub use path::register_path_commands;
pub use fs::register_fs_commands;
pub use process::register_process_commands;

use crate::CommandRegistry;

/// Register all Rust standard library commands
pub fn register_all_rust_commands(registry: &mut CommandRegistry) {
    register_env_commands(registry);
    register_path_commands(registry);
    register_fs_commands(registry);
    register_process_commands(registry);
}