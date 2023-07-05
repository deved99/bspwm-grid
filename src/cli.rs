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
    Focus(Focus),
    Send(SendWindow),
}
impl Subcommands {
    fn run(&self) -> Result<()> {
        match self {
            Self::SetDesktops(_) => actions::set_desktops(),
            Self::GetDesktop(_) => actions::get_desktop(),
            Self::Focus(x) => x.run(),
            Self::Send(x) => x.run(),
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Get current state of desktops.
#[argh(subcommand, name = "get-desktop")]
struct GetDesktop {}

#[derive(FromArgs, PartialEq, Debug)]
/// Create desktops.
#[argh(subcommand, name = "init")]
struct SetDesktops {}

#[derive(FromArgs, PartialEq, Debug)]
/// Change focused desktop.
#[argh(subcommand, name = "focus")]
struct Focus {
    /// column to switch to
    #[argh(option, short = 'x')]
    column: Option<desktop::Target>,
    /// row to switch to
    #[argh(option, short = 'y')]
    row: Option<desktop::Target>,
    /// desktop to switch to
    #[argh(option, short = 'z')]
    desktop: Option<String>,
}
impl Focus {
    fn run(&self) -> Result<()> {
        actions::focus(self.column, self.row, self.desktop.as_deref())
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Send current window to desktop.
#[argh(subcommand, name = "send")]
struct SendWindow {
    /// column to switch to
    #[argh(option, short = 'x')]
    column: Option<desktop::Target>,
    /// row to switch to
    #[argh(option, short = 'y')]
    row: Option<desktop::Target>,
    /// desktop to switch to
    #[argh(option, short = 'z')]
    desktop: Option<String>,
}
impl SendWindow {
    fn run(&self) -> Result<()> {
        actions::send(self.column, self.row, self.desktop.as_deref())
    }
}
