// src/theme/windows.rs
use super::{Config, ThemeManager};
use crate::{Error, Result};
use winreg::{RegKey, enums::*};

/// Windows-specific theme manager using registry manipulation
pub struct WindowsThemeManager;

impl WindowsThemeManager {
  const REGISTRY_PATH: &'static str =
    r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";
  const APPS_THEME_KEY: &'static str = "AppsUseLightTheme";
  const SYSTEM_THEME_KEY: &'static str = "SystemUsesLightTheme";

  /// Sets both app and system themes for consistency
  fn set_registry_values(&self, is_light: bool) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
      .open_subkey_with_flags(Self::REGISTRY_PATH, KEY_ALL_ACCESS)
      .map_err(|e| {
        Error::ColorModeSet(format!(
          "Windows: Failed to open registry key '{}': {}",
          Self::REGISTRY_PATH,
          e
        ))
      })?;

    let value = if is_light { 1u32 } else { 0u32 };

    //{ Set app theme }
    key.set_value(Self::APPS_THEME_KEY, &value).map_err(|e| {
      Error::ColorModeSet(format!(
        "Windows: Failed to set registry value '{}': {}",
        Self::APPS_THEME_KEY,
        e
      ))
    })?;

    //{ Set system theme for taskbar, etc. }
    key.set_value(Self::SYSTEM_THEME_KEY, &value).map_err(|e| {
      Error::ColorModeSet(format!(
        "Windows: Failed to set registry value '{}': {}",
        Self::SYSTEM_THEME_KEY,
        e
      ))
    })?;

    Ok(())
  }

  /// Triggers a broadcast message to notify applications of theme change
  #[cfg(feature = "windows-broadcast")]
  fn broadcast_theme_change(&self) -> Result<()> {
    use std::ptr;
    use winapi::shared::ntdef::NULL;
    use winapi::um::winuser::{
      HWND_BROADCAST, SMTO_ABORTIFHUNG, SendMessageTimeoutW, WM_SETTINGCHANGE
    };

    unsafe {
      SendMessageTimeoutW(
        HWND_BROADCAST,
        WM_SETTINGCHANGE,
        0,
        b"ImmersiveColorSet\0".as_ptr() as isize,
        SMTO_ABORTIFHUNG,
        1000, // 1 second timeout
        ptr::null_mut()
      );
    }

    Ok(())
  }
}

impl ThemeManager for WindowsThemeManager {
  fn set_theme(&self, mode: Config) -> Result<()> {
    let is_light = match mode {
      Config::Light => true,
      Config::Dark => false,
      Config::Auto =>
        return Err(Error::ColorModeSet(
          "Cannot set Windows theme to Auto mode".to_string()
        )),
    };

    //{ Set registry values for theme }
    self.set_registry_values(is_light)?;

    //{ Optionally broadcast change notification }
    #[cfg(feature = "windows-broadcast")]
    {
      if let Err(e) = self.broadcast_theme_change() {
        eprintln!("Warning: Failed to broadcast theme change: {}", e);
        //? Continue execution - theme is set even if broadcast fails
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_windows_constants() {
    //{ Verify Windows-specific constants are defined }
    assert_eq!(
      WindowsThemeManager::REGISTRY_PATH,
      r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"
    );
    assert_eq!(WindowsThemeManager::APPS_THEME_KEY, "AppsUseLightTheme");
    assert_eq!(
      WindowsThemeManager::SYSTEM_THEME_KEY,
      "SystemUsesLightTheme"
    );
  }

  #[test]
  fn test_windows_theme_manager_creation() {
    //{ Verify WindowsThemeManager can be created }
    let manager = WindowsThemeManager;
    //? Manager is a unit struct, so this mainly tests compilation
  }

  #[test]
  fn test_windows_auto_error() {
    let manager = WindowsThemeManager;

    //{ Windows theme setting should reject Auto mode }
    let result = manager.set_theme(Config::Auto);
    assert!(result.is_err());

    if let Err(Error::ColorModeSet(msg)) = result {
      assert!(msg.contains("Auto mode"));
    }
  }

  //? Integration tests would require actual registry access
  //? and should be run in a controlled Windows environment
}
