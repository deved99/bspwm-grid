use super::{COLUMNS, GRID_AREA, ROWS};

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

    pub fn from_usize(n: usize) -> Self {
        let x = n % COLUMNS;
        let y = (n / COLUMNS) % ROWS;
        let z = n / GRID_AREA;
        Self { x, y, z }
    }

    pub fn to_usize(self) -> usize {
        self.x + self.y * COLUMNS + self.z * GRID_AREA
    }

    pub fn with_column(self, x: usize) -> Self {
        Self { x, y: self.y, z: self.z}
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
