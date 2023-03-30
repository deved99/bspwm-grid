use crate::{actions, desktop, Result};
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
    GetDesktop(GetDesktop),
    ColumnFocus(ColumnFocus),
    ColumnSend(ColumnSend),
    RowFocus(RowFocus),
    RowSend(RowSend),
}
impl Subcommands {
    fn run(&self) -> Result<()> {
        match self {
            Self::SetDesktops(_) => actions::set_desktops(),
            Self::GetDesktop(_) => actions::get_desktop(),
            Self::ColumnFocus(x) => x.run(),
            Self::ColumnSend(x) => x.run(),
            Self::RowFocus(x) => x.run(),
            Self::RowSend(x) => x.run(),
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Test
#[argh(subcommand, name = "get-desktop")]
struct GetDesktop {}

#[derive(FromArgs, PartialEq, Debug)]
/// Create desktops.
#[argh(subcommand, name = "init")]
struct SetDesktops {}

// Column related //

#[derive(FromArgs, PartialEq, Debug)]
/// Focus given column.
#[argh(subcommand, name = "column-focus")]
struct ColumnFocus {
    #[argh(positional)]
    /// column number.
    x: desktop::Target,
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
    /// column number.
    x: desktop::Target,
}
impl ColumnSend {
    fn run(&self) -> Result<()> {
        actions::column_send(self.x)
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Focus given row.
#[argh(subcommand, name = "row-focus")]
struct RowFocus {
    #[argh(positional)]
    /// row number.
    y: desktop::Target,
}
impl RowFocus {
    fn run(&self) -> Result<()> {
        actions::row_focus(self.y)
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Send to given row.
#[argh(subcommand, name = "row-send")]
struct RowSend {
    #[argh(positional)]
    /// row number.
    y: desktop::Target,
}
impl RowSend {
    fn run(&self) -> Result<()> {
        actions::row_send(self.y)
    }
}
