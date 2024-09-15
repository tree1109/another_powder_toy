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

    pub fn is_colliding_to(&self, target: &Self) -> bool {
        match self.cell_type {
            CellType::Air => true,
            CellType::Sand => match target.cell_type {
                CellType::Air => false,
                CellType::Sand => true,
                CellType::Wall => true,
            },
            CellType::Wall => true,
        }
    }
}
