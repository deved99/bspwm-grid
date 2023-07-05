mod actions;
pub mod bspc;
mod cli;
pub mod desktop;
mod error;
pub mod monitor_status;

pub use error::{Error, Result};

const BSPC: &str = "bspc";
const ROWS: usize = 5;
const COLUMNS: usize = 5;
const COLUMNS_ARRAY: [usize; COLUMNS] = get_columns_array();
const GRID_AREA: usize = ROWS * COLUMNS;

pub fn main() {
    let args: cli::Cli = argh::from_env();
    args.run().unwrap();
}

const fn get_columns_array() -> [usize; COLUMNS] {
    let mut res = [0; COLUMNS];
    let mut i = 0;
    while i < COLUMNS {
        res[i] = i;
        i += 1;
    }
    res
}
