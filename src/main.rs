use another_powder_toy::debug::*;
use another_powder_toy::sand_world::*;
use log::info;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    sand_simulation_system: SandSimulationSystem,
    debug_system: DebugSystem,
}

fn model(app: &App) -> Model {
    let window = app.window_rect();

    let cell_size = 16.0;
    let origin = vec2(window.left(), window.bottom());
    let size_x = (window.w() / cell_size) as i32;
    let size_y = (window.h() / cell_size) as i32;

    info!("World Width: {}, Height: {}", size_x, size_y);

    Model {
        sand_simulation_system: SandSimulationSystem::new(origin, size_x, size_y, cell_size),
        debug_system: DebugSystem::new(),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.sand_simulation_system.update(app, &update);
    model.debug_system.update(app, &update);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(srgb(0.1, 0.1, 0.1));
    // Render
    {
        model.sand_simulation_system.render(&draw, &frame);
        model.debug_system.render(&draw, &frame);
    }
    draw.to_frame(app, &frame).unwrap();
}
