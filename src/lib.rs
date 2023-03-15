mod actions;
pub mod bspc;
mod cli;
pub mod desktop;
mod error;

pub use error::{Error, Result};

const ROWS: usize = 5;
const COLUMNS: usize = 5;
const GRID_AREA: usize = ROWS * COLUMNS;
const BSPC: &'static str = "bspc";

pub fn main() {
    let args: cli::Cli = argh::from_env();
    args.run().unwrap();
}
