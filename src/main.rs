use walter::{config, Result};

fn main() -> Result<()> {
    let config = config::init()?;
    println!("{config}");
    Ok(())
}
