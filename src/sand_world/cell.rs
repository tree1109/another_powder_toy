#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum CellType {
    #[default]
    Air,
    Sand,
    Wall,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Cell {
    pub cell_type: CellType,
}

impl Cell {
    pub fn new(cell_type: CellType) -> Self {
        Self { cell_type }
    }
}
