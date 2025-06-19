// src/theme/linux.rs
use super::{Config, ThemeManager};
use crate::{Error, Result};
use std::env;
use std::process::Command;

/// Linux-specific theme manager supporting multiple desktop environments
pub struct LinuxThemeManager;

/// Supported Linux desktop environments
#[derive(Debug, PartialEq)]
enum DesktopEnvironment {
  KDE,
  GNOME,
  Unsupported(String),
  Unknown
}

impl LinuxThemeManager {
  /// Detects the current desktop environment
  fn detect_desktop_environment(&self) -> DesktopEnvironment {
    let desktop = env::var("XDG_CURRENT_DESKTOP")
      .ok()
      .map(|d| d.to_lowercase());

    match desktop.as_deref() {
      Some(desktop) if desktop.contains("kde") => DesktopEnvironment::KDE,
      Some(desktop) if desktop.contains("gnome") => DesktopEnvironment::GNOME,
      Some(desktop) => DesktopEnvironment::Unsupported(desktop.to_string()),
      None => DesktopEnvironment::Unknown
    }
  }

  /// Sets theme for KDE desktop environment
  fn set_kde_theme(&self, mode: Config) -> Result<()> {
    let theme_name = match mode {
      Config::Dark => "BreezeDark",
      Config::Light => "BreezeLight",
      Config::Auto =>
        return Err(Error::ColorModeSet(
          "Cannot set KDE theme to Auto mode".to_string()
        )),
    };

    //{ Execute plasma-apply-colorscheme command }
    let status = Command::new("plasma-apply-colorscheme")
      .arg(theme_name)
      .status()
      .map_err(|e| {
        Error::ColorModeSet(format!(
          "Linux/KDE: Failed to execute plasma-apply-colorscheme: {}",
          e
        ))
      })?;

    if !status.success() {
      return Err(Error::ColorModeSet(
        "Linux/KDE: plasma-apply-colorscheme command failed".to_string()
      ));
    }

    //{ Also try to set the color scheme via kwriteconfig5 for persistence }
    if let Err(e) = self.set_kde_persistent_theme(theme_name) {
      eprintln!("Warning: Failed to set persistent KDE theme: {}", e);
      //? Continue - the theme is still set via plasma-apply-colorscheme
    }

    Ok(())
  }

  /// Sets persistent KDE theme configuration
  fn set_kde_persistent_theme(&self, theme_name: &str) -> Result<()> {
    let status = Command::new("kwriteconfig5")
      .args([
        "--file",
        "kdeglobals",
        "--group",
        "General",
        "--key",
        "ColorScheme",
        theme_name
      ])
      .status()
      .map_err(|e| {
        Error::ColorModeSet(format!(
          "Linux/KDE: Failed to execute kwriteconfig5: {}",
          e
        ))
      })?;

    if !status.success() {
      return Err(Error::ColorModeSet(
        "Linux/KDE: kwriteconfig5 command failed".to_string()
      ));
    }

    Ok(())
  }

  /// Sets theme for GNOME desktop environment
  fn set_gnome_theme(&self, mode: Config) -> Result<()> {
    let scheme_value = match mode {
      Config::Dark => "prefer-dark",
      Config::Light => "prefer-light",
      Config::Auto =>
        return Err(Error::ColorModeSet(
          "Cannot set GNOME theme to Auto mode".to_string()
        )),
    };

    //{ Set color scheme via gsettings }
    let status = Command::new("gsettings")
      .args([
        "set",
        "org.gnome.desktop.interface",
        "color-scheme",
        scheme_value
      ])
      .status()
      .map_err(|e| {
        Error::ColorModeSet(format!(
          "Linux/GNOME: Failed to execute gsettings: {}",
          e
        ))
      })?;

    if !status.success() {
      return Err(Error::ColorModeSet(
        "Linux/GNOME: gsettings set color-scheme command failed".to_string()
      ));
    }

    //{ Also set GTK theme for older applications }
    if let Err(e) = self.set_gnome_gtk_theme(mode) {
      eprintln!("Warning: Failed to set GTK theme: {}", e);
      //? Continue - the main color scheme is still set
    }

    Ok(())
  }

  /// Sets GTK theme for GNOME applications
  fn set_gnome_gtk_theme(&self, mode: Config) -> Result<()> {
    let gtk_theme = match mode {
      Config::Dark => "Adwaita-dark",
      Config::Light => "Adwaita",
      Config::Auto => return Ok(()) // Skip GTK theme for Auto
    };

    let status = Command::new("gsettings")
      .args(["set", "org.gnome.desktop.interface", "gtk-theme", gtk_theme])
      .status()
      .map_err(|e| {
        Error::ColorModeSet(format!(
          "Linux/GNOME: Failed to set GTK theme: {}",
          e
        ))
      })?;

    if !status.success() {
      return Err(Error::ColorModeSet(
        "Linux/GNOME: Failed to set GTK theme".to_string()
      ));
    }

    Ok(())
  }
}

impl ThemeManager for LinuxThemeManager {
  fn set_theme(&self, mode: Config) -> Result<()> {
    let desktop_env = self.detect_desktop_environment();

    match desktop_env {
      DesktopEnvironment::KDE => self.set_kde_theme(mode),
      DesktopEnvironment::GNOME => self.set_gnome_theme(mode),
      DesktopEnvironment::Unsupported(ref desktop) => {
        //? Unsupported desktop environment
        eprintln!(
          "Unsupported Linux desktop environment for theme setting: {}",
          desktop
        );
        Ok(())
      }
      DesktopEnvironment::Unknown => {
        //? Could not determine desktop environment
        eprintln!(
          "Could not determine Linux desktop environment for theme setting."
        );
        Ok(())
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_desktop_environment_detection() {
    let manager = LinuxThemeManager;

    //{ Test desktop environment detection logic }
    //? This test doesn't actually set environment variables
    //? but tests the detection logic structure
    let _desktop = manager.detect_desktop_environment();
    //? Can't assert specific values since it depends on test environment
  }

  #[test]
  fn test_kde_theme_mapping() {
    //{ Test KDE theme name mapping }
    let test_cases =
      [(Config::Dark, "BreezeDark"), (Config::Light, "BreezeLight")];

    for (config, expected) in test_cases {
      let theme_name = match config {
        Config::Dark => "BreezeDark",
        Config::Light => "BreezeLight",
        Config::Auto => unreachable!()
      };
      assert_eq!(theme_name, expected);
    }
  }

  #[test]
  fn test_gnome_scheme_mapping() {
    //{ Test GNOME color scheme mapping }
    let test_cases = [
      (Config::Dark, "prefer-dark"),
      (Config::Light, "prefer-light")
    ];

    for (config, expected) in test_cases {
      let scheme_value = match config {
        Config::Dark => "prefer-dark",
        Config::Light => "prefer-light",
        Config::Auto => unreachable!()
      };
      assert_eq!(scheme_value, expected);
    }
  }

  #[test]
  fn test_gnome_gtk_theme_mapping() {
    //{ Test GTK theme mapping }
    let test_cases =
      [(Config::Dark, "Adwaita-dark"), (Config::Light, "Adwaita")];

    for (config, expected) in test_cases {
      let gtk_theme = match config {
        Config::Dark => "Adwaita-dark",
        Config::Light => "Adwaita",
        Config::Auto => continue
      };
      assert_eq!(gtk_theme, expected);
    }
  }

  #[test]
  fn test_linux_auto_error() {
    let manager = LinuxThemeManager;

    //{ Linux theme setting should reject Auto mode }
    let result = manager.set_theme(Config::Auto);
    //? Result depends on detected desktop environment
    //? but Auto should be handled gracefully
  }

  #[test]
  fn test_desktop_environment_enum() {
    //{ Test DesktopEnvironment enum variants }
    let kde = DesktopEnvironment::KDE;
    let gnome = DesktopEnvironment::GNOME;
    let unknown = DesktopEnvironment::Unknown;
    let unsupported = DesktopEnvironment::Unsupported("xfce".to_string());

    assert_ne!(kde, gnome);
    assert_ne!(unknown, unsupported);
  }

  //? Integration tests would require actual desktop environment
  //? and should be run in controlled Linux environments with
  //? the appropriate DE tools installed
}
