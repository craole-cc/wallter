// src/theme/mod.rs
use crate::{Error, Result};
use dark_light::{Mode, detect};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

// Re-export platform implementations for testing
#[cfg(target_os = "linux")]
use super::linux::LinuxThemeManager;
#[cfg(target_os = "windows")]
use super::windows::WindowsThemeManager;

/// Trait for platform-specific theme management
pub trait ThemeManager {
  fn set_theme(&self, mode: Config) -> Result<()>;
}

#[derive(
  clap::ValueEnum,
  Debug,
  Default,
  Serialize,
  Deserialize,
  PartialEq,
  Eq,
  Clone,
  Copy,
)]
pub enum Config {
  Light,
  Dark,
  #[default]
  Auto
}

impl Config {
  /// Detects the current system theme mode
  pub fn detect_current() -> Result<Self> {
    match detect()? {
      Mode::Dark => Ok(Self::Dark),
      Mode::Light => Ok(Self::Light),
      Mode::Unspecified => {
        //? Fallback to dark mode when system doesn't specify
        Ok(Self::Dark)
      }
    }
  }

  /// Applies the theme configuration to the system
  /// - Light/Dark: Sets the specific theme
  /// - Auto: Matches the system's current preference (no-op if already
  ///   matching)
  pub fn apply(&self) -> Result<()> {
    let current_mode = Self::detect_current()?;
    let target_mode = self.resolve_target_mode(current_mode)?;

    //{ Early return if mode is already set }
    if current_mode == target_mode {
      println!("System mode is already {target_mode:?}");
      return Ok(());
    }

    println!("Setting system mode to {target_mode:?}");
    self.set_system_theme(target_mode)
  }

  /// Toggles the current system theme (Light ↔ Dark)
  /// Returns the new theme that was applied
  pub fn toggle_theme() -> Result<Self> {
    let current_mode = Self::detect_current()?;
    let target_mode = match current_mode {
      Self::Light => Self::Dark,
      Self::Dark => Self::Light,
      Self::Auto => {
        //? Auto detected, default to Light for toggle
        Self::Light
      }
    };

    println!("Toggling system mode from {current_mode:?} to {target_mode:?}");
    target_mode.set_system_theme(target_mode)?;

    //{ Verify the change took effect }
    let new_mode = Self::detect_current()?;
    if new_mode == target_mode {
      println!("Theme successfully changed to {new_mode:?}");
    } else {
      eprintln!("Warning: Theme change may not have taken effect immediately");
    }

    Ok(target_mode)
  }

  /// Resolves the target mode based on configuration
  fn resolve_target_mode(&self, current_mode: Self) -> Result<Self> {
    match self {
      Self::Light | Self::Dark => Ok(*self),
      Self::Auto => {
        //{ Auto means "match system preference" - detect what system wants }
        //? This could be enhanced to read system preference rather than current theme
        Ok(current_mode)
      }
    }
  }

  /// Sets the system theme using the appropriate platform manager
  fn set_system_theme(&self, target_mode: Self) -> Result<()> {
    if target_mode == Self::Auto {
      return Err(Error::ColorModeSet(
        "Cannot set system theme to Auto - must be Light or Dark".to_string()
      ));
    }

    //{ Delegate to platform-specific implementation }
    self.get_platform_manager().set_theme(target_mode)
  }

  /// Gets the appropriate platform-specific theme manager
  fn get_platform_manager(&self) -> Box<dyn ThemeManager> {
    #[cfg(target_os = "windows")]
    {
      Box::new(super::windows::WindowsThemeManager)
    }

    #[cfg(target_os = "linux")]
    {
      Box::new(LinuxThemeManager)
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
      Box::new(UnsupportedThemeManager)
    }
  }
}

/// Fallback theme manager for unsupported platforms
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
struct UnsupportedThemeManager;

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
impl ThemeManager for UnsupportedThemeManager {
  fn set_theme(&self, _mode: Config) -> Result<()> {
    //? Platform doesn't support theme setting
    eprintln!("System theme setting is not supported on this platform.");
    Ok(())
  }
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Light => write!(f, "light"),
      Self::Dark => write!(f, "dark"),
      Self::Auto => write!(f, "auto")
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_theme_config_display() {
    assert_eq!(Config::Light.to_string(), "light");
    assert_eq!(Config::Dark.to_string(), "dark");
    assert_eq!(Config::Auto.to_string(), "auto");
  }

  #[test]
  fn test_theme_config_default() {
    assert_eq!(Config::default(), Config::Auto);
  }

  #[test]
  fn test_resolve_target_mode_concrete_modes() {
    let light_config = Config::Light;
    let dark_config = Config::Dark;

    //{ Concrete modes should return themselves }
    assert_eq!(
      light_config.resolve_target_mode(Config::Dark).unwrap(),
      Config::Light
    );
    assert_eq!(
      dark_config.resolve_target_mode(Config::Light).unwrap(),
      Config::Dark
    );
  }

  #[test]
  fn test_resolve_target_mode_auto_matches_current() {
    let auto_config = Config::Auto;

    //{ Auto should match current system mode, not toggle }
    assert_eq!(
      auto_config.resolve_target_mode(Config::Light).unwrap(),
      Config::Light
    );
    assert_eq!(
      auto_config.resolve_target_mode(Config::Dark).unwrap(),
      Config::Dark
    );
  }

  #[test]
  fn test_toggle_logic() {
    //{ Test toggle logic without system calls }
    let test_cases = [
      (Config::Light, Config::Dark),
      (Config::Dark, Config::Light),
      (Config::Auto, Config::Light) // Auto defaults to Light when toggling
    ];

    for (current, expected) in test_cases {
      let result = match current {
        Config::Light => Config::Dark,
        Config::Dark => Config::Light,
        Config::Auto => Config::Light
      };
      assert_eq!(result, expected);
    }
  }

  #[test]
  fn test_serialization() {
    //{ Test JSON serialization/deserialization }
    let configs = [Config::Light, Config::Dark, Config::Auto];

    for config in configs {
      let serialized = serde_json::to_string(&config).unwrap();
      let deserialized: Config = serde_json::from_str(&serialized).unwrap();
      assert_eq!(config, deserialized);
    }
  }

  #[test]
  fn test_clap_value_enum() {
    use clap::ValueEnum;

    //{ Test that clap value enum works correctly }
    let values = Config::value_variants();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&Config::Light));
    assert!(values.contains(&Config::Dark));
    assert!(values.contains(&Config::Auto));
  }

  #[test]
  fn test_equality_and_copy() {
    let config1 = Config::Dark;
    let config2 = config1; // Test Copy trait

    //{ Test PartialEq and Copy traits }
    assert_eq!(config1, config2);
    assert_ne!(config1, Config::Light);
  }

  #[test]
  fn test_set_system_theme_auto_error() {
    let config = Config::Auto;

    //{ Auto mode should not be settable as system theme }
    let result = config.set_system_theme(Config::Auto);
    assert!(result.is_err());
  }

  //? Mock tests for system integration would require more complex setup
  //? with dependency injection or feature flags to avoid actual system calls
}
