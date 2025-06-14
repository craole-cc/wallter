use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Represents the pixel dimensions of a monitor.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resolution {
  /// The width in pixels.
  pub width: u32,
  /// The height in pixels.
  pub height: u32
}

impl Display for Resolution {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}x{}", self.width, self.height)
  }
}
