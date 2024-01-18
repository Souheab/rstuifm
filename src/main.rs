pub mod ui;
pub mod app;
pub mod backend;

use anyhow::{Result, Context};

fn main() -> Result<()> {
    app::run().context("[main()] Failed to run app")?;
    Ok(())
}
