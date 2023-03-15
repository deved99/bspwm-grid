use std::process::Command;

use super::{Result, BSPC};

pub fn get_focused_desktop() -> Result<usize> {
    let cmd = Command::new(BSPC)
        .args(["query", "--desktops", "-d", ".focused", "--names"])
        .output()?;
    let res = String::from_utf8(cmd.stdout)?;
    let n: usize = res.trim().parse()?;
    Ok(n)
}

pub fn focus_desktop(n: usize) -> Result<()> {
    let s = n.to_string();
    Command::new(BSPC).args(["desktop", "--focus", &s]).status()?;
    Ok(())
}

pub fn get_monitors() -> Result<Vec<String>> {
    let cmd = Command::new(BSPC)
        .args(["query", "-M", "--names"])
        .output()?;
    let res = String::from_utf8(cmd.stdout)?;
    let lines = res.lines().map(|s| s.to_string()).collect();
    Ok(lines)
}

pub fn set_desktops(ns: &[usize], monitor: &str) -> Result<()> {
    let desktops: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
    let desktops_ref: Vec<&str> = desktops.iter().map(|s| s.as_str()).collect();
    // Create monitors
    let mut args = vec!["monitor", monitor, "-d"];
    args.extend(desktops_ref);
    Command::new(BSPC).args(args).status()?;
    Ok(())
}