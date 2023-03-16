use itertools::iproduct;

use super::{bspc, desktop, Result, COLUMNS, ROWS};

pub fn get_desktop() -> Result<()> {
    let n = bspc::get_focused_desktop()?;
    let ws = desktop::Desktop::from_usize(n);
    println!("{}: {:?}", n, ws);
    Ok(())
}

pub fn focus_desktop(x: usize) -> Result<()> {
    let n = bspc::get_focused_desktop()?;
    let ws = desktop::Desktop::from_usize(n)
        .with_column(x);
    bspc::focus_desktop(ws.to_usize())
}

pub fn send_to_desktop(x: usize) -> Result<()> {
    let n = bspc::get_focused_desktop()?;
    let ws = desktop::Desktop::from_usize(n)
        .with_column(x);
    bspc::send_to_desktop(ws.to_usize())
}


pub fn set_monitors() -> Result<()> {
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
