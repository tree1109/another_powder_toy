use nannou::prelude::*;

pub struct Sandbox {
    cell_display_ratio: f32,
    grid_size_x: usize,
    grid_size_y: usize,
}

impl Sandbox {
    pub fn new(cell_display_ratio: f32, grid_size_x: usize, grid_size_y: usize) -> Sandbox {
        Sandbox {
            cell_display_ratio,
            grid_size_x,
            grid_size_y,
        }
    }

    pub fn update(self: &mut Sandbox, _app: &App) {}
    pub fn display(self: &Sandbox, draw: &Draw, window: &Rect) {
        self.draw_bounding_box(draw, window);
    }

    fn draw_bounding_box(self: &Sandbox, draw: &Draw, window: &Rect) {
        let sandbox_rect = Rect::from_w_h(
            self.grid_size_x as f32 * self.cell_display_ratio as f32,
            self.grid_size_y as f32 * self.cell_display_ratio as f32,
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
