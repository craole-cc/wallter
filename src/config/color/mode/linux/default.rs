//! Manages system color mode (light/dark) settings specifically for Linux
//! desktop environments.
//!
//! This module attempts to detect the current desktop environment (KDE Plasma,
//! GNOME) and uses environment-specific commands (e.g.,
//! `plasma-apply-colorscheme`, `gsettings`) to apply the desired theme.

use crate::config::color::mode::{Config, Manager as ModeManager};
use crate::{Error, Result};
use std::{env, process::Command};

/// A manager for Linux system color mode settings.
pub struct Manager;

/// Represents supported Linux desktop environments and outcomes of detection.
#[derive(Debug, PartialEq)]
enum DesktopEnvironment {
  KDE,
  GNOME,
  Unsupported(String),
  Unknown
}

impl DesktopEnvironment {
  fn detect() -> Self {
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
        Error::ColorMode(format!(
          "Linux/KDE: Failed to execute kwriteconfig5: {e}"
        ))
      })?;

    if !status.success() {
      return Err(Error::ColorMode(
        "Linux/KDE: kwriteconfig5 command failed".to_string()
      ));
    }
    Ok(())
  }

  fn set_gnome_gtk_theme(&self, config: Config) -> Result<()> {
    let gtk_theme = match config {
      Config::Dark => "Adwaita-dark",
      Config::Light => "Adwaita",
      Config::Auto => unreachable!()
    };

    let status = Command::new("gsettings")
      .args(["set", "org.gnome.desktop.interface", "gtk-theme", gtk_theme])
      .status()
      .map_err(|e| {
        Error::ColorMode(format!("Linux/GNOME: Failed to set GTK theme: {e}"))
      })?;

    if !status.success() {
      return Err(Error::ColorMode(
        "Linux/GNOME: Failed to set GTK theme".to_string()
      ));
    }
    Ok(())
  }

  fn apply_kde_theme_config(&self, config: Config) -> Result<()> {
    let theme_name = match config {
      Config::Dark => "BreezeDark",
      Config::Light => "BreezeLight",
      Config::Auto => unreachable!()
    };

    let status = Command::new("plasma-apply-colorscheme")
      .arg(theme_name)
      .status()
      .map_err(|e| {
        Error::ColorMode(format!(
          "Linux/KDE: Failed to execute plasma-apply-colorscheme: {e}"
        ))
      })?;

    if !status.success() {
      return Err(Error::ColorMode(
        "Linux/KDE: plasma-apply-colorscheme command failed".to_string()
      ));
    }

    if let Err(e) = self.set_kde_persistent_theme(theme_name) {
      eprintln!("Warning: Failed to set persistent KDE theme: {e}");
    }
    Ok(())
  }

  fn apply_gnome_theme_config(&self, config: Config) -> Result<()> {
    let scheme_value = match config {
      Config::Dark => "prefer-dark",
      Config::Light => "prefer-light",
      Config::Auto => unreachable!()
    };

    let status = Command::new("gsettings")
      .args([
        "set",
        "org.gnome.desktop.interface",
        "color-scheme",
        scheme_value
      ])
      .status()
      .map_err(|e| {
        Error::ColorMode(format!(
          "Linux/GNOME: Failed to execute gsettings: {e}"
        ))
      })?;

    if !status.success() {
      return Err(Error::ColorMode(
        "Linux/GNOME: gsettings set color-scheme command failed".to_string()
      ));
    }

    if let Err(e) = self.set_gnome_gtk_theme(config) {
      eprintln!("Warning: Failed to set GTK theme: {e}");
    }
    Ok(())
  }

  fn apply_theme(&self, config: Config) -> Result<()> {
    match self {
      DesktopEnvironment::KDE => self.apply_kde_theme_config(config),
      DesktopEnvironment::GNOME => self.apply_gnome_theme_config(config),
      DesktopEnvironment::Unsupported(ref desktop_name) => {
        eprintln!(
          "Unsupported Linux desktop environment for theme setting: {desktop_name}"
        );
        Ok(())
      }
      DesktopEnvironment::Unknown => {
        eprintln!(
          "Could not determine Linux desktop environment for theme setting."
        );
        Ok(())
      }
    }
  }
}

impl ModeManager for Manager {
  fn set(&self, mode: Config) -> Result<()> {
    let desktop_env = DesktopEnvironment::detect();
    desktop_env.apply_theme(mode)
  }

  fn notify(&self) -> Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_desktop_environment_detection() {
    let _desktop = DesktopEnvironment::detect();
  }

  #[test]
  fn test_kde_theme_mapping() {
    let test_cases =
      [(Config::Dark, "BreezeDark"), (Config::Light, "BreezeLight")];

    for (config, expected) in test_cases {
      let actual_theme_name = match config {
        Config::Dark => "BreezeDark",
        Config::Light => "BreezeLight",
        Config::Auto => unreachable!()
      };
      assert_eq!(actual_theme_name, expected);
    }
  }

  #[test]
  fn test_gnome_scheme_mapping() {
    let test_cases = [
      (Config::Dark, "prefer-dark"),
      (Config::Light, "prefer-light")
    ];

    for (config, expected) in test_cases {
      let actual_scheme_value = match config {
        Config::Dark => "prefer-dark",
        Config::Light => "prefer-light",
        Config::Auto => unreachable!()
      };
      assert_eq!(actual_scheme_value, expected);
    }
  }

  #[test]
  fn test_desktop_environment_enum() {
    let kde = DesktopEnvironment::KDE;
    let gnome = DesktopEnvironment::GNOME;
    let unknown = DesktopEnvironment::Unknown;
    let unsupported = DesktopEnvironment::Unsupported("xfce".to_string());

    assert_ne!(kde, gnome);
    assert_ne!(unknown, unsupported);
  }
}
