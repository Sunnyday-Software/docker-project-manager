pub struct EmojiCatalog;

impl EmojiCatalog {
  // Debug e sviluppo
  pub const DEBUG: &'static str = "{EMOJI_THINKING}";
  pub const INFO: &'static str = "{EMOJI_INFO}";
  pub const WARNING: &'static str = "{EMOJI_WARNING}";
  pub const ERROR: &'static str = "{EMOJI_CROSS}";
  pub const SUCCESS: &'static str = "{EMOJI_CHECK}";

  // Metodi per combinazioni comuni
  pub fn debug_enabled() -> String {
    format!("{} Debug printing enabled", Self::DEBUG)
  }

  pub fn debug_disabled() -> String {
    format!("{} Debug printing disabled", Self::DEBUG)
  }

  pub fn status(enabled: bool) -> &'static str {
    if enabled { "🟢" } else { "🔴" }
  }
}
