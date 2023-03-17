use super::{actions, Result};
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Script to simulate a 2D grid on each monitor.
pub struct Cli {
    #[argh(subcommand)]
    nested: Subcommands,
}
impl Cli {
    pub fn run(&self) -> Result<()> {
        self.nested.run()
    }
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    SetDesktops(SetDesktops),
    ColumnFocus(ColumnFocus),
    ColumnSend(ColumnSend),
    WatchDesktop(WatchDesktop),
}
impl Subcommands {
    fn run(&self) -> Result<()> {
        match self {
            Self::ColumnFocus(x) => x.run(),
            Self::ColumnSend(x) => x.run(),
            Self::SetDesktops(_) => actions::set_desktops(),
            Self::WatchDesktop(_) => actions::watch_desktop(),
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Test
#[argh(subcommand, name = "watch-desktop")]
struct WatchDesktop {}

#[derive(FromArgs, PartialEq, Debug)]
/// Create desktops.
#[argh(subcommand, name = "init")]
struct SetDesktops {}

#[derive(FromArgs, PartialEq, Debug)]
/// Focus given column.
#[argh(subcommand, name = "column-focus")]
struct ColumnFocus {
    #[argh(positional)]
    x: usize,
}
impl ColumnFocus {
    fn run(&self) -> Result<()> {
        actions::column_focus(self.x)
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Send to given column.
#[argh(subcommand, name = "column-send")]
struct ColumnSend {
    #[argh(positional)]
    x: usize,
}
impl ColumnSend {
    fn run(&self) -> Result<()> {
        actions::column_send(self.x)
    }
}
