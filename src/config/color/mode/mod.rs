mod default;
pub use default::{Config, ThemeManager};

// Platform-specific modules
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;
