use crate::{Error, Result};
use dark_light::{Mode, detect};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

pub trait Manager {
  fn set(&self, config: Config) -> Result<()>;
  fn notify(&self) -> Result<()>;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum Config {
  Light,
  Dark
}

impl Default for Config {
  /// Returns the default color configuration based on the system's current
  /// theme. Attempts to detect the current mode and returns the corresponding
  /// `Config`. Falls back to `Dark` mode if detection fails or if the
  /// system's theme is unspecified.
  fn default() -> Self {
    let default_mode = Self::Dark;
    match detect() {
      Ok(Mode::Dark) => Self::Dark,
      Ok(Mode::Light) => Self::Light,
      Ok(Mode::Unspecified) => {
        eprintln!(
          "System color mode is unspecified.\nUsing default mode: {default_mode}"
        );
        default_mode
      }
      Err(e) => {
        eprintln!(
          "Failed to detect the system's color mode: {e}.\nUsing default mode: {default_mode}"
        );
        default_mode
      }
    }
  }
}

impl Config {
  /// Creates a new `Config` instance using the default color configuration.
  /// The default mode is determined based on the system's current theme.
  /// Returns a `Result` containing the newly created `Config` instance.
  pub fn new() -> Result<Self> {
    Ok(Self::default())
  }

  /// Toggles the current color mode between `Light` and `Dark`.
  /// This function detects the current mode using the default detection logic,
  /// switches to the opposite mode, and applies the change.
  /// Returns the new mode upon successful application.
  pub fn toggle() -> Result<Self> {
    let current_mode = Self::default();
    let desired_mode = match current_mode {
      Self::Light => Self::Dark,
      Self::Dark => Self::Light
    };
    desired_mode.apply().map(|_| desired_mode)
  }

  pub fn apply(&self) -> Result<()> {
    let current = Self::default();
    let desired = *self;

    //{ Early return if mode is already set }
    if current == desired {
      println!("System mode is already {desired:?}");
      return Ok(());
    };

    //{ Set the system mode using the necessary platform-specific manager }
    println!("Setting system mode to {desired:?}");
    let manager: Box<dyn self::Manager> = {
      #[cfg(target_os = "windows")]
      {
        Box::new(super::windows::Manager)
      }
      #[cfg(target_os = "linux")]
      {
        Box::new(super::linux::Manager)
      }
      #[cfg(not(any(target_os = "windows", target_os = "linux")))]
      {
        // Define and implement UnsupportedManager directly here
        struct UnsupportedManager;
        impl self::Manager for UnsupportedManager {
          fn set(&self, _config: Config) -> Result<()> {
            eprintln!(
              "System theme setting is not supported on this platform."
            );
            Ok(())
          }

          fn notify(&self) -> Result<()> {
            // No-op for unsupported platforms
            Ok(())
          }
        }
        Box::new(UnsupportedManager)
      }
    };
    manager.set(desired)
  }
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Light => write!(f, "Light"),
      Self::Dark => write!(f, "Dark")
    }
  }
}
