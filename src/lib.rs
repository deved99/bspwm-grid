mod actions;
pub mod bspc;
mod cli;
pub mod desktop;
mod error;

pub use error::{Error, Result};

const ROWS: usize = 1;
const COLUMNS: usize = 5;
const GRID_AREA: usize = ROWS * COLUMNS;
const BSPC: &'static str = "bspc";

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
