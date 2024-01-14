pub mod ui;
pub mod app;
pub mod app_backend;

use anyhow::{Result, Context};

fn main() -> Result<()> {
    app::run().context("[main()] Failed to run app")?;
    Ok(())
}
