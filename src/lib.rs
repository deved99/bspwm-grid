mod actions;
pub mod bspc;
mod cli;
pub mod desktop;
mod error;
pub mod monitor_status;

pub use error::{Error, Result};

const BSPC: &'static str = "bspc";
const ROWS: usize = 1;
const COLUMNS: usize = 5;
const COLUMNS_ARRAY: [usize; COLUMNS] = get_columns_array();
const GRID_AREA: usize = ROWS * COLUMNS;

pub fn main() {
    let args: cli::Cli = argh::from_env();
    args.run().unwrap();
}

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
pub fn command_lines(cmd: &str, args: &[&str]) -> Result<impl Iterator<Item = Result<String>>> {
    let cmd = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?;
    let stdout = BufReader::new(cmd.stdout.unwrap());
    let iter = stdout.lines().map(|x| x.map_err(Error::from));
    Ok(iter)
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
