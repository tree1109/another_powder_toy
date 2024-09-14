use super::Cell;

pub struct CellGrid {
    size_x: usize,
    size_y: usize,
    cells: Vec<Cell>,
}

impl CellGrid {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        let cells = vec![Cell::default(); size_x * size_y];
        Self {
            size_x,
            size_y,
            cells,
        }
    }

    // Setter
    pub fn set_cell(&mut self, x: isize, y: isize, cell: Cell) {
        let index = self.get_index(x, y);
        self.cells[index] = cell;
    }

    // Getter
    pub fn get_size_x(&self) -> usize {
        self.size_x
    }

    pub fn get_size_y(&self) -> usize {
        self.size_y
    }

    pub fn get_cell(&self, x: isize, y: isize) -> &Cell {
        let index = self.get_index(x, y);
        &self.cells[index]
    }

    // Helper
    fn get_index(&self, x: isize, y: isize) -> usize {
        if x < 0 || y < 0 || x >= self.size_x as isize || y >= self.size_y as isize {
            panic!("Index out of bounds: ({}, {})", x, y);
        }

        (y * self.size_x as isize + x) as usize
    }

    pub fn is_inside(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.size_x as isize && y < self.size_y as isize
    }
}
