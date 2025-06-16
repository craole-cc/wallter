use super::Mode;
use crate::{Error, Result};
use dark_light::{Mode as DarkLightMode, detect};
use serde::{Deserialize, Serialize};
use std::{
  // cell::RefCell,
  fmt::{self, Display, Formatter}
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
  pub mode: Mode
}

impl Config {
  /// Returns the effective color mode.
  /// If the configured mode is `Auto`, this method attempts to detect the
  /// current system theme.
  /// Falls back to `Mode::Dark` if detection for `Auto` is unspecified or
  /// fails.
  pub fn get_effective_mode() -> Result<Self> {
    //{ Detect the color mode and update the config}
    let mode = Mode::detect_system_mode()?;
    Ok(Self { mode })
  }
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let color_mode_str = match &self.mode {
      Mode::Light => "Light",
      Mode::Dark => "Dark",
      Mode::Auto => {
        // If mode is Auto, try to detect current system theme for display
        match detect() {
          Ok(DarkLightMode::Light) => "Auto (Currently Light)",
          Ok(DarkLightMode::Dark) => "Auto (Currently Dark)",
          Ok(DarkLightMode::Unspecified) => "Auto (System Unspecified)",
          Err(_) => "Auto (Detection Failed)"
        }
      }
    };

    printf!(f, "Mode", color_mode_str)?;
    Ok(())
  }
}
