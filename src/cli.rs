use argh::FromArgs;
use super::{Result, actions};

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
    GetDesktop(GetDesktop),
    FocusDesktop(FocusDesktop),
    SendToDesktop(SendToDesktop),
    SetDesktops(SetDesktops),
}
impl Subcommands {
    fn run(&self) -> Result<()> {
        match self {
            Self::GetDesktop(_) => actions::get_desktop(),
            Self::FocusDesktop(x) => x.run(),
            Self::SendToDesktop(x) => x.run(),
            Self::SetDesktops(_) => actions::set_monitors(),
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Get currently focused desktop.
#[argh(subcommand, name = "get-desktop")]
struct GetDesktop { }

#[derive(FromArgs, PartialEq, Debug)]
/// Get currently focused desktop.
#[argh(subcommand, name = "focus-desktop")]
struct FocusDesktop {
    #[argh(positional)]
    x: usize
}
impl FocusDesktop {
    fn run(&self) -> Result<()> {
        actions::focus_desktop(self.x)
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Get currently focused desktop.
#[argh(subcommand, name = "send-to-desktop")]
struct SendToDesktop {
    #[argh(positional)]
    x: usize
}
impl SendToDesktop {
    fn run(&self) -> Result<()> {
        actions::send_to_desktop(self.x)
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Get currently focused desktop.
#[argh(subcommand, name = "set-desktops")]
struct SetDesktops { }
