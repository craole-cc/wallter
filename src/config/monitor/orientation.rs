use crate::config::monitor::Resolution;
use serde::{Deserialize, Serialize};
use std::{
  cmp::Ordering,
  fmt::{self, Display, Formatter}
};

/// Represents the orientation of a monitor based on its resolution.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Orientation {
  /// Width > Height (e.g., 1920x1080)
  Landscape,
  /// Height > Width (e.g., 1080x1920)
  Portrait,
  /// Width == Height (e.g., 1024x1024)
  Square
}

impl Orientation {
  pub fn from_resolution(res: &Resolution) -> Self {
    match res.width.cmp(&res.height) {
      Ordering::Greater => Self::Landscape,
      Ordering::Less => Self::Portrait,
      Ordering::Equal => Self::Square
    }
  }
}

impl Display for Orientation {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Orientation::Landscape => write!(f, "Landscape"),
      Orientation::Portrait => write!(f, "Portrait"),
      Orientation::Square => write!(f, "Square")
    }
  }
}
