use another_powder_toy::sand_world::{self, *};
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
    let size_x = (window.w() / cell_size) as usize;
    let size_y = (window.h() / cell_size) as usize;

    info!("World Width: {}, Height: {}", size_x, size_y);

    let sand_world = get_test_sandworld(size_x, size_y);

    Model {
        sand_world,
        cell_size,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut cell_move_changes: Vec<(isize, isize, isize, isize)> = vec![];
    let sand_world = &mut model.sand_world;

    // Update the sand world.
    for x in 0..sand_world.get_size_x() as isize {
        for y in 0..sand_world.get_size_y() as isize {
            let cell = sand_world.get_cell(x as isize, y as isize);
            match cell.cell_type {
                CellType::Sand => {
                    let is_move_bottom = !is_colliding(sand_world, x as isize, y as isize - 1);
                    if is_move_bottom {
                        cell_move_changes.push((x, y, x, y - 1));
                    }

                    let is_move_bottom_left =
                        !is_colliding(sand_world, x as isize - 1, y as isize - 1);
                    let is_move_bottom_right =
                        !is_colliding(sand_world, x as isize + 1, y as isize - 1);

                    if is_move_bottom_left && is_move_bottom_right {
                        let is_move_left = random_f32() < 0.5;
                        let new_x = if is_move_left { x - 1 } else { x + 1 };
                        cell_move_changes.push((x, y, new_x, y - 1));
                    } else if is_move_bottom_left {
                        cell_move_changes.push((x, y, x - 1, y - 1));
                    } else if is_move_bottom_right {
                        cell_move_changes.push((x, y, x + 1, y - 1));
                    }
                }
                _ => {}
            }
        }
    }

    // Apply the changes.
    for (x, y, new_x, new_y) in cell_move_changes {
        let cell = *sand_world.get_cell(x, y);
        let new_cell = *sand_world.get_cell(new_x, new_y);
        sand_world.set_cell(x, y, new_cell);
        sand_world.set_cell(new_x, new_y, cell);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();

    draw.background().color(srgb(0.1, 0.1, 0.1));

    {
        let sand_world = &model.sand_world;
        let grid_origin = vec2(window.left(), window.bottom());
        draw_grid(&draw, &sand_world, grid_origin, model.cell_size);
        draw_cells(&draw, &sand_world, grid_origin, model.cell_size);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn get_test_sandworld(size_x: usize, size_y: usize) -> CellGrid {
    // Initialize the sand world with test data.
    let mut sand_world = CellGrid::new(size_x, size_y);
    // Place ground.
    for x in 0..size_x {
        sand_world.set_cell(x as isize, 0, Cell::new(CellType::Wall));
    }
    // Place some sand at middle.
    let center = vec2(size_x as f32 / 2.0, size_y as f32 / 2.0);
    let radius = 4.0;
    for x in 0..size_x {
        for y in 0..size_y {
            let distance = center.distance(Vec2::new(x as f32, y as f32));
            if distance < radius {
                sand_world.set_cell(x as isize, y as isize, Cell::new(CellType::Sand));
            }
        }
    }

    sand_world
}

fn is_colliding(sand_world: &CellGrid, x: isize, y: isize) -> bool {
    if !sand_world.is_inside(x, y) {
        return true;
    }

    let cell = sand_world.get_cell(x, y);
    cell.cell_type != CellType::Air
}
