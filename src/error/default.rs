#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("API error: {0}")]
  ApiError(String),

  #[error("Configuration error: {0}")]
  ConfigError(String),

  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Network error: {0}")]
  NetworkError(#[from] reqwest::Error),

  #[error("Image processing error: {0}")]
  ImageError(String),

  #[error("Monitor detection error: {0}")]
  MonitorError(String),

  #[error("Invalid settings: {0}")]
  SettingsError(String),

  #[error("Color Mode Detection Error: {0}")]
  ColorMode(#[from] dark_light::Error)
}
