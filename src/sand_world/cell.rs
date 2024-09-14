#[derive(Clone, Copy, Default)]
pub enum CellType {
    #[default]
    Air,
    Sand,
    Wall,
}

#[derive(Clone, Copy, Default)]
pub struct Cell {
    pub cell_type: CellType,
}
