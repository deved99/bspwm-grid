use serde::Serialize;

use crate::{COLUMNS, COLUMNS_ARRAY};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MonitorStatus {
    x: usize,
    y: usize,
    columns: [usize; COLUMNS],
    occupied_xs: Vec<usize>,
}
impl MonitorStatus {
    pub fn new(x: usize, y: usize, occupied_xs: Vec<usize>) -> Self {
        Self {
            x,
            y,
            columns: COLUMNS_ARRAY,
            occupied_xs,
        }
    }
}
