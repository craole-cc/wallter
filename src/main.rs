use wallter::{Result, config};

fn main() -> Result<()> {
  println!("Welcome to wallter!");

  let config = config::init()?;
  println!("{config}");
  config::ColorMode::toggle_theme()?;
  Ok(())
}
