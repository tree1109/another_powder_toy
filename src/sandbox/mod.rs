use cell::{Cell, CellType};
use nannou::{prelude::*, rand};

mod cell;

pub struct Sandbox {
    cell_size: f32,
    grid_size_x: usize,
    grid_size_y: usize,
    cells: Vec<Cell>,
}

impl Sandbox {
    pub fn new(cell_size: f32, grid_size_x: usize, grid_size_y: usize) -> Sandbox {
        let mut cells = vec![Cell::default(); grid_size_x * grid_size_y];

        for index in 0..(grid_size_x * grid_size_y) {
            cells[index].cell_type = match rand::random::<usize>() {
                x if x % 4 == 0 => CellType::Air,
                x if x % 4 == 1 => CellType::Sand,
                x if x % 4 == 2 => CellType::Water,
                x if x % 4 == 3 => CellType::Stone,
                _ => CellType::Air,
            };
        }

        Sandbox {
            cell_size,
            grid_size_x,
            grid_size_y,
            cells,
        }
    }

    pub fn update(self: &mut Sandbox, _app: &App) {}
    pub fn render(self: &Sandbox, draw: &Draw, window: &Rect) {
        self.draw_bounding_box(draw, window);

        let cell_half_size = self.cell_size / 2.0;

        for x in 0..self.grid_size_x {
            for y in 0..self.grid_size_y {
                let index = y * self.grid_size_x + x;
                let cell: &Cell = &self.cells[index];

                let cell_rect = Rect::from_x_y_w_h(
                    window.left() + (self.cell_size as f32 * x as f32) + cell_half_size,
                    window.bottom() + (self.cell_size as f32 * y as f32) + cell_half_size,
                    self.cell_size as f32,
                    self.cell_size as f32,
                );

                cell.render(draw, &cell_rect);
            }
        }
    }

    fn draw_bounding_box(self: &Sandbox, draw: &Draw, window: &Rect) {
        let sandbox_rect = Rect::from_w_h(
            self.grid_size_x as f32 * self.cell_size as f32,
            self.grid_size_y as f32 * self.cell_size as f32,
        )
        .bottom_left_of(*window);

        draw.rect()
            .xy(sandbox_rect.xy())
            .wh(sandbox_rect.wh())
            .stroke_color(WHITE)
            .stroke_weight(1.0)
            .no_fill();
    }
}
