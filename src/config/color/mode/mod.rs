mod default;
pub use default::{Config, Manager};

// Platform-specific modules
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;
