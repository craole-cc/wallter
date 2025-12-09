//! Provides generic utilities for interacting with the Windows Registry.

#![cfg(target_os = "windows")]

use crate::{Error, Result};
use std::io;
use winreg::{
  enums::{KEY_READ, KEY_SET_VALUE, REG_BINARY},
  RegKey, RegValue, HKEY
};

/// Reads a raw binary value from the specified registry key and value name.
pub fn read_bytes(hive: HKEY, path: &str, name: &str) -> Result<Vec<u8>> {
  let root = RegKey::predef(hive);
  let key = root.open_subkey_with_flags(path, KEY_READ).map_err(|e| {
    Error::IO(io::Error::new(
      io::ErrorKind::NotFound,
      format!("Failed to open registry key '{path}': {e}")
    ))
  })?;

  let value = key.get_raw_value(name).map_err(|e| {
    Error::IO(io::Error::new(
      io::ErrorKind::NotFound,
      format!("Failed to read registry value '{name}' from key '{path}': {e}")
    ))
  })?;

  Ok(value.bytes)
}

/// Writes a raw binary value to the specified registry key and value name.
pub fn write_bytes(
  hive: HKEY,
  path: &str,
  name: &str,
  data: &[u8]
) -> Result<()> {
  let root = RegKey::predef(hive);
  let key = root
    .open_subkey_with_flags(path, KEY_SET_VALUE)
    .map_err(|e| {
      Error::IO(io::Error::new(
        io::ErrorKind::PermissionDenied,
        format!("Failed to open registry key '{path}' for writing: {e}")
      ))
    })?;

  let reg_value = RegValue {
    bytes: data.to_vec(),
    vtype: REG_BINARY
  };
  key.set_raw_value(name, &reg_value).map_err(|e| {
    Error::IO(io::Error::new(
      io::ErrorKind::PermissionDenied,
      format!("Failed to write registry value '{name}' to key '{path}': {e}")
    ))
  })
}

/// Checks if a registry key exists.
pub fn key_exists(hive: HKEY, path: &str) -> bool {
  let root = RegKey::predef(hive);
  root.open_subkey(path).is_ok()
}

/// Checks if a registry value exists within a given key.
pub fn value_exists(hive: HKEY, path: &str, name: &str) -> bool {
  let root = RegKey::predef(hive);
  root
    .open_subkey(path)
    .and_then(|key| key.get_raw_value(name))
    .is_ok()
}
