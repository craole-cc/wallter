//! Defines the configuration for user-specified color preferences,
//! including the system color mode (light/dark) and a list of
//! color tags for wallpaper filtering.

use super::Mode;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Holds user-defined color preferences, including system mode and color tags.
///
/// This configuration manages:
/// 1. The desired system color mode (Light/Dark), which can be applied system-wide.
/// 2. A list of color names or tags for filtering/tagging wallpapers.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
  /// The desired system color mode (Light/Dark).
  pub mode: Mode,
  /// A list of color names or tags specified by the user.
  pub colors: Vec<String>
}

impl Config {
  /// Creates a new `Config` with a specified mode and list of colors.
  pub fn new(mode: Mode, colors: Vec<String>) -> Self {
    Self { mode, colors }
  }

  /// Sets the system color mode for the `Config` instance.
  pub fn with_mode(mut self, mode: Mode) -> Self {
    self.mode = mode;
    self
  }

  /// Sets the list of color tags for the `Config` instance.
  pub fn with_colors(mut self, colors: Vec<String>) -> Self {
    self.colors = colors;
    self
  }
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    printf!(f, "Mode", self.mode)?;

    if self.colors.is_empty() {
      printf!(f, "Colors", "None specified")?;
    } else {
      printf!(f, "Colors", self.colors.join(", "))?;
    };

    Ok(())
  }
}
