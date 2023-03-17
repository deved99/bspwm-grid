use itertools::iproduct;

use super::{bspc, desktop, Error, Result, COLUMNS, ROWS};

pub fn watch_desktop() -> Result<()> {
    // Get all desktops
    print_active_desktops()?;
    // Than watch for changes
    let args = ["subscribe", "desktop"];
    let lines = super::command_lines("bspc", &args)?;
    for _ in lines {
        print_active_desktops()?;
    }
    Ok(())
}

fn print_active_desktops() -> Result<()> {
    // Get current monitors
    let monitors_str = bspc::get_monitors()?;
    let monitors = monitors_str.iter().enumerate().map(|(n, _)| n);
    let desktop = iproduct!(0..COLUMNS, 0..ROWS, monitors)
        .map(|(x, y, z)| desktop::Desktop::new(x, y, z));
    // Then add active infos
    let ns = bspc::get_active_desktop()?;
    let active_desktops: Vec<desktop::DesktopActive> = desktop.map(|w| w.to_active(&ns)).collect();
    let active_desktops_json = serde_json::to_string(&active_desktops)?;
    println!("{}", active_desktops_json);
    Ok(())
}

fn column_check(x: usize) -> Result<()> {
    match x < COLUMNS {
        true => Ok(()),
        false => Err(Error::ColumnTooHigh {
            given: x,
            limit: COLUMNS,
        }),
    }
}

pub fn column_focus(x: usize) -> Result<()> {
    column_check(x)?;
    let n = bspc::get_focused_desktop()?;
    let ws = desktop::Desktop::from_usize(n).with_column(x);
    bspc::focus_desktop(ws.to_usize())
}

pub fn column_send(x: usize) -> Result<()> {
    column_check(x)?;
    if x >= COLUMNS {
        let err = Error::ColumnTooHigh {
            given: x,
            limit: COLUMNS,
        };
        return Err(err);
    }
    let n = bspc::get_focused_desktop()?;
    let ws = desktop::Desktop::from_usize(n).with_column(x);
    bspc::send_to_desktop(ws.to_usize())
}

pub fn set_desktops() -> Result<()> {
    let monitors = bspc::get_monitors()?;
    for (n, monitor) in monitors.iter().enumerate() {
        let workspaces: Vec<usize> = iproduct!(0..COLUMNS, 0..ROWS)
            .map(|(x, y)| desktop::Desktop::new(x, y, n))
            .map(|ws| ws.to_usize())
            .collect();
        bspc::set_desktops(&workspaces, monitor)?
    }
    Ok(())
}
