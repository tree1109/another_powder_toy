use another_powder_toy::sand_world::*;
use log::info;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    sand_world: CellGrid,
    cell_size: f32,
}

fn model(app: &App) -> Model {
    let window = app.window_rect();
    let cell_size = 32.0;
    let width = (window.w() / cell_size) as usize;
    let height = (window.h() / cell_size) as usize;

    info!("World Width: {}, Height: {}", width, height);

    Model {
        sand_world: CellGrid::new(width, height),
        cell_size,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();

    draw.background().color(srgb(0.1, 0.1, 0.1));

    let sand_world = &model.sand_world;

    // Draw the grid.
    let grid_origin = vec2(window.left(), window.bottom());
    draw_grid(
        &draw,
        grid_origin,
        sand_world.get_size_x(),
        sand_world.get_size_y(),
        model.cell_size,
    );

    // Draw the cells.
    for x in 0..sand_world.get_size_x() {
        for y in 0..sand_world.get_size_y() {
            let pos = Vec2::new(x as f32, y as f32) * model.cell_size
                + grid_origin
                + (model.cell_size / 2.0);
            let size = vec2(model.cell_size, model.cell_size);
            let rect = Rect::from_xy_wh(pos, size);

            let cell = sand_world.get_cell(x as isize, y as isize);

            draw_cell(&draw, &rect, &cell.cell_type);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
