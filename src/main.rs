pub mod ui;
pub mod app;
pub mod backend;
pub mod helper_functions;

use anyhow::{Result, Context};

fn main() -> Result<()> {
    app::run().context("[main()] Failed to run app")?;
    Ok(())
}
