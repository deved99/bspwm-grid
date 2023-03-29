use std::collections::HashMap;

use itertools::iproduct;

use crate::monitor_status::MonitorStatus;
use crate::{bspc, desktop, Error, Result, COLUMNS, ROWS};

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

pub fn get_desktop() -> Result<()> {
    // Get current monitors
    let ns: HashMap<usize, (usize, usize)> = bspc::get_active_desktop()?
        .iter()
        .map(|x| x.get_coords())
        .map(|(x, y, z)| (z, (x, y)))
        .collect();
    let occupied_ns = bspc::get_occupied_desktop()?;
    let occupied_on = |monitor, row| {
        occupied_ns
            .iter()
            .map(|d| d.get_coords())
            .filter(|(_, y, z)| *z == monitor && *y == row)
            .map(|(x, _, _)| x)
            .collect()
    };
    let monitors: HashMap<String, MonitorStatus> = bspc::get_monitors()?
        .into_iter()
        .enumerate()
        .map(|(z, s)| (s, ns[&z], z))
        .map(|(s, (x, y), z)| (s, (x, y), occupied_on(z, y)))
        .map(|(s, (x, y), o)| (s, MonitorStatus::new(x, y, o)))
        .collect();
    let json = serde_json::to_string(&monitors)?;
    println!("{}", json);
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

fn row_check(x: usize) -> Result<()> {
    match x < ROWS {
        true => Ok(()),
        false => Err(Error::RowTooHigh {
            given: x,
            limit: COLUMNS,
        }),
    }
}

pub fn row_focus(x: usize) -> Result<()> {
    row_check(x)?;
    let n = bspc::get_focused_desktop()?;
    let ws = desktop::Desktop::from_usize(n).with_row(x);
    bspc::focus_desktop(ws.to_usize())
}

pub fn row_send(x: usize) -> Result<()> {
    row_check(x)?;
    if x >= COLUMNS {
        let err = Error::ColumnTooHigh {
            given: x,
            limit: COLUMNS,
        };
        return Err(err);
    }
    let n = bspc::get_focused_desktop()?;
    let ws = desktop::Desktop::from_usize(n).with_row(x);
    bspc::send_to_desktop(ws.to_usize())
}
