use std::fmt::{self, Formatter};

/// Print a padded key-value field with a custom separator for uniform CLI
/// output.
///
/// # Example
/// ```
/// print_field(f, "Name", "DISPLAY1", 11)?;
/// ```
pub fn pout_field<T: fmt::Display>(
  f: &mut Formatter<'_>,
  key: &str,
  value: T,
  pad: usize
) -> fmt::Result {
  writeln!(f, "    {key:<pad$}|=> {value}")
}

/// Macro for concise field printing, forwarding to `pout_field`.
#[macro_export]
macro_rules! printf {
  ($f:expr, $key:expr, $value:expr, $pad:expr) => {
    $crate::utils::print::pout_field($f, $key, $value, $pad)
  };
  ($f:expr, $key:expr, $value:expr) => {
    $crate::utils::print::pout_field($f, $key, $value, 24)
  };
}
