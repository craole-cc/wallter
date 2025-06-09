use wallter::{config, Result};

fn main() -> Result<()> {
  println!("Welcome to wallter!");
    let config = config::init()?;
    println!("{config}");
    Ok(())
}
