mod config;
mod consts;
mod domain;
mod models;
mod schemas;
mod utils;

use crate::config::Config;
use crate::domain::Processor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = envy::from_env::<Config>()?;
    let pr = Processor::new(config);

    pr.run()?;

    Ok(())
}
