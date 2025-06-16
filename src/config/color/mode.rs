use crate::{Error, Result};
use dark_light::{Mode, detect};
use serde::{Deserialize, Serialize};

#[derive(
  Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone, Copy,
)]
pub enum Config {
  Light,
  Dark,
  #[default]
  Auto
}

impl Config {
  pub fn detect_system_mode() -> Result<Self> {
    Ok(match detect()? {
      Mode::Dark => Self::Dark,
      Mode::Light => Self::Light,
      Mode::Unspecified => Self::Light // fallback to light mode
    })
  }

  pub fn get_effective_mode(&self) -> Result<Self> {
    Ok(match self {
      Config::Auto => Self::detect_system_mode()?,
      Config::Light => Config::Light,
      Config::Dark => Config::Dark
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_effective_mode() {
    let auto = Config::Auto;
    let light = Config::Light;
    let dark = Config::Dark;

    // Auto should return either Light or Dark based on system
    assert!(matches!(
      auto.get_effective_mode().unwrap(),
      Config::Light | Config::Dark
    ));

    // Manual settings should return themselves
    assert_eq!(light.get_effective_mode().unwrap(), Config::Light);
    assert_eq!(dark.get_effective_mode().unwrap(), Config::Dark);
  }
}
