//! Manages system color mode (light/dark) settings specifically for Windows.
//!
//! This module interacts with the Windows Registry to set the application and
//! system themes and can optionally broadcast a message to other applications
//! to notify them of the theme change.

#![cfg_attr(feature = "windows-broadcast", allow(unsafe_code))]
use super::Config;
use crate::{Error, Result};
use winreg::{RegKey, enums::*};

/// A manager for Windows system color mode settings.
///
/// This struct is a zero-sized type used as a marker to group Windows-specific
/// theme management logic.
pub struct Manager;

impl Manager {
  /// The Windows Registry path where theme settings are stored.
  const REGISTRY_PATH: &'static str =
    r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";
  /// The registry key name for the application theme setting (light/dark).
  const APPS_THEME_KEY: &'static str = "AppsUseLightTheme";
  /// The registry key name for the system theme setting (e.g., taskbar, Start
  /// menu).
  const SYSTEM_THEME_KEY: &'static str = "SystemUsesLightTheme";

  /// The registry DWORD value representing light mode for theme settings.
  const LIGHT_MODE_REG_VALUE: u32 = 1;
  /// The registry DWORD value representing dark mode for theme settings.
  const DARK_MODE_REG_VALUE: u32 = 0;
}

impl super::Manager for Manager {
  /// Sets the Windows system color mode (light or dark).
  ///
  /// This function updates the relevant Windows Registry keys to apply the
  /// specified color mode. After successfully setting the registry values,
  /// it attempts to notify other running applications of the change via a
  /// system broadcast message if the `windows-broadcast` feature is enabled.
  ///
  /// # Arguments
  ///
  /// * `config` - The desired color mode (`Light` or `Dark`) to apply.
  ///
  /// # Errors
  ///
  /// Returns `Error::ColorMode` if:
  /// * It fails to open the necessary registry key.
  /// * It fails to set the `AppsUseLightTheme` or `SystemUsesLightTheme`
  ///   registry values.
  fn set(&self, config: Config) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
      .open_subkey_with_flags(Self::REGISTRY_PATH, KEY_ALL_ACCESS)
      .map_err(|e| {
        Error::ColorMode(format!(
          "Windows: Failed to open registry key '{}': {e}",
          Self::REGISTRY_PATH
        ))
      })?;
    let value = match config {
      Config::Light => Self::LIGHT_MODE_REG_VALUE,
      Config::Dark => Self::DARK_MODE_REG_VALUE
    };

    // Set the application theme (controls the appearance of most app windows).
    key.set_value(Self::APPS_THEME_KEY, &value).map_err(|e| {
      Error::ColorMode(format!(
        "Windows: Failed to set application theme registry value ('{}'): {e}",
        Self::APPS_THEME_KEY
      ))
    })?;

    // Set the system theme (controls elements like the taskbar and Start menu).
    key.set_value(Self::SYSTEM_THEME_KEY, &value).map_err(|e| {
      Error::ColorMode(format!(
        "Windows: Failed to set system theme registry value ('{}'): {e}",
        Self::SYSTEM_THEME_KEY
      ))
    })?;

    // Attempt to notify other applications of the theme change.
    // If this fails, a warning is printed, but the overall operation is
    // still considered successful as the theme has been set in the registry.
    if let Err(e) = self.notify() {
      eprintln!("Warning: Failed to broadcast theme change: {e}");
    }

    Ok(())
  }

  /// Notifies other running applications of a theme change.
  ///
  /// This function is only active if the `windows-broadcast` feature is
  /// enabled. When active, it sends a `WM_SETTINGCHANGE` message to all
  /// top-level windows, indicating that the "ImmersiveColorSet" (system
  /// theme) has changed. This allows theme-aware applications to update their
  /// appearance immediately.
  ///
  /// If the `windows-broadcast` feature is not enabled, this function is a
  /// no-op and returns `Ok(())`.
  ///
  /// # Safety
  ///
  /// This function uses an `unsafe` block to call `SendMessageTimeoutW`, which
  /// is a Windows API function. This is necessary for FFI (Foreign Function
  /// Interface) calls. The `allow(unsafe_code)` attribute is conditionally
  /// applied at the module level when the `windows-broadcast` feature is
  /// enabled.
  ///
  /// # Returns
  ///
  /// Always returns `Ok(())`. The success or failure of the broadcast message
  /// itself is not indicated by the return value of this function in the
  /// current implementation (it doesn't check the result of
  /// `SendMessageTimeoutW`).
  fn notify(&self) -> Result<()> {
    #[cfg(feature = "windows-broadcast")]
    {
      println!(
        "DEBUG: windows-broadcast feature IS ACTIVE and notify is called!"
      ); // This is a temporary debug message and should be removed in production.

      use std::ptr;
      use winapi::um::winuser::{
        HWND_BROADCAST, SMTO_ABORTIFHUNG, SendMessageTimeoutW, WM_SETTINGCHANGE
      };

      use std::ffi::CString;
      // The "ImmersiveColorSet" string tells applications that system
      // colors/theme have changed. CString is used to ensure a
      // null-terminated string as required by the Windows API.
      // .unwrap() is used here as "ImmersiveColorSet" is a valid C-string and
      // won't contain null bytes.
      let message = CString::new("ImmersiveColorSet").unwrap();
      unsafe {
        SendMessageTimeoutW(
          HWND_BROADCAST,
          WM_SETTINGCHANGE,
          0,
          message.as_ptr() as isize, //? LPARAM must be a pointer to a null-terminated string
          SMTO_ABORTIFHUNG,
          1000, // Timeout for the message in milliseconds.
          ptr::null_mut() // Not interested in the result of the send operation.
        );
      }
    }
    Ok(())
  }
}
