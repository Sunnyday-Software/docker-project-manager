// model.rs - Re-exports for backward compatibility

// Re-export everything from the new modular structure
pub use crate::core::*;

/// Environment variable names
pub const ENV_DOCKER_HOST_MAP: &str = "DOCKER_HOST_MAP";
pub const ENV_DOCKER_ENV_KEYS: &str = "DOCKER_ENV_KEYS";
pub const ENV_PROJECT_NAME: &str = "PROJECT_NAME";
pub const ENV_HOST_PROJECT_PATH: &str = "HOST_PROJECT_PATH";
pub const ENV_HOST_UID: &str = "HOST_UID";
pub const ENV_HOST_GID: &str = "HOST_GID";
pub const ENV_HOST_USER: &str = "HOST_USER";

pub const ERROR_CANNOT_DETERMINE_HOME: &str =
  "Unable to determine current user's home directory";

pub const MSG_UPDATING_VERSIONS: &str = "Updating versions in progress...";

pub const MSG_DOCKER_COMMAND_FAILED: &str = "Docker command failed";
