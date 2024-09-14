use nannou::prelude::*;

use super::{CellGrid, CellType};

pub fn draw_grid(draw: &Draw, sand_world: &CellGrid, origin: Vec2, cell_size: f32) {
    for x in 0..sand_world.get_size_x() {
        for y in 0..sand_world.get_size_y() {
            let x = x as f32 * cell_size + cell_size / 2.0;
            let y = y as f32 * cell_size + cell_size / 2.0;
            let pos = origin + vec2(x, y);
            let rect = Rect::from_x_y_w_h(pos.x, pos.y, cell_size, cell_size);

            draw.rect()
                .xy(rect.xy())
                .wh(rect.wh())
                .no_fill()
                .stroke_weight(1.0)
                .stroke_color(srgb(0.5, 0.5, 0.5));
        }
    }
}

pub fn draw_cells(draw: &Draw, sand_world: &CellGrid, origin: Vec2, cell_size: f32) {
    for x in 0..sand_world.get_size_x() {
        for y in 0..sand_world.get_size_y() {
            let pos = vec2(x as f32, y as f32) * cell_size + origin + (cell_size / 2.0);
            let size = vec2(cell_size, cell_size);
            let rect = Rect::from_xy_wh(pos, size);

            let cell = sand_world.get_cell(x as isize, y as isize);

            draw_cell(draw, &rect, &cell.cell_type);
        }
    }
}

pub fn draw_cell(draw: &Draw, cell_rect: &Rect, cell_type: &CellType) {
    match cell_type {
        CellType::Air => {
            draw_empty_cell(draw, cell_rect, SKYBLUE);
        }
        CellType::Sand => {
            draw_solid_cell(draw, cell_rect, GOLD);
        }
        CellType::Wall => {
            draw_solid_cell(draw, cell_rect, GRAY);
        }
    }
}

pub fn draw_empty_cell(draw: &Draw, rect: &Rect, color: Srgb<u8>) {
    draw.rect()
        .xy(rect.xy())
        .wh(rect.wh())
        .no_fill()
        .stroke_weight(1.0)
        .stroke_color(SKYBLUE);
    draw.ellipse()
        .xy(rect.xy())
        .wh(rect.wh() * 0.5)
        .no_fill()
        .stroke_weight(1.0)
        .stroke_color(color);
}

pub fn draw_solid_cell(draw: &Draw, rect: &Rect, color: Srgb<u8>) {
    draw.rect().xy(rect.xy()).wh(rect.wh()).color(color);
}
