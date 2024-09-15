use super::Cell;

pub struct CellGrid {
    size_x: i32,
    size_y: i32,
    cells: Vec<Cell>,
}

impl CellGrid {
    pub fn new(size_x: i32, size_y: i32) -> Self {
        let vector_size = (size_x * size_y) as usize;
        let cells = vec![Cell::default(); vector_size];
        Self {
            size_x,
            size_y,
            cells,
        }
    }

    // Setter
    pub fn set_cell(&mut self, x: i32, y: i32, cell: Cell) {
        let index = self.get_index(x, y);
        self.cells[index] = cell;
    }

    // Getter
    pub fn get_size_x(&self) -> i32 {
        self.size_x
    }

    pub fn get_size_y(&self) -> i32 {
        self.size_y
    }

    pub fn get_cell(&self, x: i32, y: i32) -> &Cell {
        let index = self.get_index(x, y);
        &self.cells[index]
    }

    // Helper
    fn get_index(&self, x: i32, y: i32) -> usize {
        if !self.is_inside(x, y) {
            panic!(
                "Index out of bounds: ({}, {}) which gird size is ({}, {})",
                x, y, self.size_x, self.size_y
            );
        }

        (y * self.size_x + x) as usize
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.size_x && y < self.size_y
    }
}
