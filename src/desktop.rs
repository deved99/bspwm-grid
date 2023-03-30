use std::str::FromStr;
use crate::{Error, Result, COLUMNS, GRID_AREA, ROWS};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Target {
    Next(usize),
    Prev(usize),
    Absolute(usize),
}

impl FromStr for Target {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.starts_with(['+', '-']) {
            true => {
                let z: isize = s.parse()?;
                let n = z.abs_diff(0);
                let r = match z > 0 {
                    true => Self::Next(n),
                    false => Self::Prev(n),
                };
                Ok(r)
            },
            false => {
                let z: usize = s.parse()?;
                Ok(Self::Absolute(z))
            },
        }
    }
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

fn row_check(x: usize) -> Result<()> {
    match x < ROWS {
        true => Ok(()),
        false => Err(Error::RowTooHigh {
            given: x,
            limit: COLUMNS,
        }),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Desktop {
    x: usize,
    y: usize,
    z: usize,
}

impl Desktop {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    pub fn get_coords(&self) -> (usize, usize, usize) {
        (self.x, self.y, self.z)
    }

    pub fn from_usize(n: usize) -> Self {
        let x = n % COLUMNS;
        let y = (n / COLUMNS) % ROWS;
        let z = n / GRID_AREA;
        Self { x, y, z }
    }

    pub fn to_usize(self) -> usize {
        self.x + self.y * COLUMNS + self.z * GRID_AREA
    }

    pub fn with_column(self, x: Target) -> Result<Self> {
        let x = match x {
            Target::Next(n) => self.x + n,
            Target::Prev(n) => self.x - n,
            Target::Absolute(n) => n,
        };
        column_check(x)?;
        let res = Self {
            x,
            y: self.y,
            z: self.z,
        };
        Ok(res)
    }
    pub fn with_row(self, y: Target) -> Result<Self> {
        let y = match y {
            Target::Next(n) => self.y + n,
            Target::Prev(n) => self.y - n,
            Target::Absolute(n) => n,
        };
        row_check(y)?;
        let res = Self {
            x: self.x,
            y,
            z: self.z,
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_usize() {
        let result = Desktop::from_usize(0);
        let expected = Desktop { x: 0, y: 0, z: 0 };
        assert_eq!(expected, result);

        let result = Desktop::from_usize(ROWS);
        let expected = Desktop { x: 0, y: 1, z: 0 };
        assert_eq!(expected, result);

        let result = Desktop::from_usize(GRID_AREA);
        let expected = Desktop { x: 0, y: 0, z: 1 };
        assert_eq!(expected, result);

        let result = Desktop::from_usize(GRID_AREA - 1);
        let expected = Desktop {
            x: ROWS - 1,
            y: COLUMNS - 1,
            z: 0,
        };
        assert_eq!(expected, result);

        let result = Desktop::from_usize(2 * GRID_AREA - 1);
        let expected = Desktop {
            x: ROWS - 1,
            y: COLUMNS - 1,
            z: 1,
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn to_usize() {
        // to_usize and from_usize should be inverses.
        for i in 0..100 {
            let from = Desktop::from_usize(i);
            println!("{:?}", from);
            let result = Desktop::to_usize(from);
            assert_eq!(i, result)
        }
    }
}
