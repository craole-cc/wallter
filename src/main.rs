use walter::{config, Result};

fn main() -> Result<()> {
  println!("Welcome to Walter!");
    let config = config::init()?;
    println!("{config}");
    Ok(())
}
