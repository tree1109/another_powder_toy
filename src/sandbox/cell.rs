use nannou::prelude::*;

#[derive(Default, Clone, Copy)]
pub enum CellType {
    #[default]
    Air,
    Sand,
    Water,
    Stone,
}

#[derive(Default, Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,
}

impl Cell {
    pub fn render(self: &Cell, draw: &Draw, cell_rect: &Rect) {
        draw.rect()
            .xy(cell_rect.xy())
            .wh(cell_rect.wh())
            .color(self.get_cell_color());
    }

    fn get_cell_color(self: &Cell) -> Rgba {
        match self.cell_type {
            CellType::Air => rgba(0.0, 0.0, 0.0, 0.0),
            CellType::Sand => rgba(1.0, 1.0, 0.2, 1.0),
            CellType::Water => rgba(0.2, 0.2, 1.0, 1.0),
            CellType::Stone => rgba(0.5, 0.5, 0.5, 1.0),
        }
    }
}
