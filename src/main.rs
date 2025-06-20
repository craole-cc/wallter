use wallter::{Result, config};

fn main() -> Result<()> {
  println!("Welcome to {}!", env!("CARGO_PKG_NAME"));

  let config = config::init()?;
  println!("{config}");
  // config::ColorMode::toggle()?;
  Ok(())
}
