pub mod cell;
pub mod cell_grid;
pub mod draw;

use cell::*;
use cell_grid::*;
use draw::*;
use nannou::prelude::*;

pub struct SandSimulationSystem {
    cell_grid: CellGrid,
    world_origin: Vec2,
    cell_size: f32,
    is_solid_border: bool,
}

impl SandSimulationSystem {
    pub fn new(origin: Vec2, size_x: usize, size_y: usize, cell_size: f32) -> Self {
        let cell_grid = get_test_sandworld(size_x, size_y);

        Self {
            cell_grid,
            world_origin: origin,
            cell_size,
            is_solid_border: false,
        }
    }

    pub fn update(&mut self, app: &App) {
        if app.mouse.buttons.left().is_down() {
            let mouse_position = app.mouse.position();
            let grid_position = self.window_to_grid_position(mouse_position);

            if app.keys.down.contains(&Key::LControl)
                && self
                    .cell_grid
                    .is_inside(grid_position.x as isize, grid_position.y as isize)
            {
                self.cell_grid.set_cell(
                    grid_position.x as isize,
                    grid_position.y as isize,
                    Cell::new(CellType::Air),
                );
            } else {
                self.cell_grid.set_cell(
                    grid_position.x as isize,
                    grid_position.y as isize,
                    Cell::new(CellType::Sand),
                );
            }
        } else if app.mouse.buttons.right().is_down() {
            let mouse_position = app.mouse.position();
            let grid_position = self.window_to_grid_position(mouse_position);

            if self
                .cell_grid
                .is_inside(grid_position.x as isize, grid_position.y as isize)
            {
                self.cell_grid.set_cell(
                    grid_position.x as isize,
                    grid_position.y as isize,
                    Cell::new(CellType::Wall),
                );
            }
        }

        self.update_cell_grid();
    }

    pub fn render(self: &Self, draw: &Draw) {
        draw_grid(&draw, &self.cell_grid, self.world_origin, self.cell_size);
        draw_cells(&draw, &self.cell_grid, self.world_origin, self.cell_size);
    }

    fn update_cell_grid(self: &mut Self) {
        let mut cell_move_changes: Vec<(isize, isize, isize, isize)> = vec![];

        for x in 0..self.cell_grid.get_size_x() as isize {
            for y in 0..self.cell_grid.get_size_y() as isize {
                let cell = self.cell_grid.get_cell(x as isize, y as isize);
                match cell.cell_type {
                    CellType::Sand => {
                        let is_move_bottom =
                            !self.is_cell_colliding_at(cell, x as isize, y as isize - 1);
                        if is_move_bottom {
                            cell_move_changes.push((x, y, x, y - 1));
                            continue;
                        }

                        let is_move_bottom_left =
                            !self.is_cell_colliding_at(cell, x as isize - 1, y as isize - 1);
                        let is_move_bottom_right =
                            !self.is_cell_colliding_at(cell, x as isize + 1, y as isize - 1);

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
        cell_move_changes.sort_by(|a, b| (a.2, a.3).cmp(&(b.2, b.3)));
        cell_move_changes.push((0, 0, 0, 0));

        let mut same_target_count = 0;
        for i in 0..cell_move_changes.len() - 1 {
            let (_, _, new_x, new_y) = cell_move_changes[i];
            let (_, _, next_new_x, next_new_y) = cell_move_changes[i + 1];

            same_target_count += 1;

            if new_x != next_new_x && new_y != next_new_y {
                let random_index = i - random_range(0, same_target_count);
                let (x, y, new_x, new_y) = cell_move_changes[random_index];

                self.move_cell_to(x, y, new_x, new_y);

                same_target_count = 0;
            }
        }
    }

    // Helper
    fn move_cell_to(self: &mut Self, x: isize, y: isize, new_x: isize, new_y: isize) {
        if self.cell_grid.is_inside(new_x, new_y) {
            let cell = *self.cell_grid.get_cell(x, y);
            let new_cell = *self.cell_grid.get_cell(new_x, new_y);
            self.cell_grid.set_cell(x, y, new_cell);
            self.cell_grid.set_cell(new_x, new_y, cell);
        } else {
            self.cell_grid.set_cell(x, y, Cell::new(CellType::Air));
        }
    }

    fn is_cell_colliding_at(self: &Self, cell: &Cell, x: isize, y: isize) -> bool {
        if !self.cell_grid.is_inside(x, y) {
            return self.is_solid_border;
        }

        let target_cell = self.cell_grid.get_cell(x, y);
        cell.is_colliding_to(target_cell)
    }

    fn window_to_grid_position(self: &Self, window_position: Vec2) -> Vec2 {
        (window_position - self.world_origin) / self.cell_size
    }
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
