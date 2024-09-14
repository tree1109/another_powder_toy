use super::Cell;

pub struct CellGrid {
    size_x: usize,
    size_y: usize,
    cells: Vec<Cell>,
}

impl CellGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::default(); width * height];
        Self {
            size_x: width,
            size_y: height,
            cells,
        }
    }

    // Getter
    pub fn get_size_x(&self) -> usize {
        self.size_x
    }

    pub fn get_size_y(&self) -> usize {
        self.size_y
    }

    pub fn get_cell(&self, x: isize, y: isize) -> &Cell {
        &self.cells[self.get_index(x, y)]
    }

    // Helper
    fn get_index(&self, x: isize, y: isize) -> usize {
        if x < 0 || y < 0 || x >= self.size_x as isize || y >= self.size_y as isize {
            panic!("Index out of bounds: ({}, {})", x, y);
        }

        (y * self.size_x as isize + x) as usize
    }
}
