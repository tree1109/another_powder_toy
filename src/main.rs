use another_powder_toy::sandbox::Sandbox;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    sandbox: Sandbox,
}

fn model(app: &App) -> Model {
    let window = app.window_rect().pad(20.0);
    let cell_size = 8.0;
    let gird_size_x = (window.w() / cell_size) as usize;
    let gird_size_y = (window.h() / cell_size) as usize;
    let sandbox = Sandbox::new(cell_size, gird_size_x, gird_size_y);

    Model { sandbox }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.sandbox.update(app);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect().pad(20.0);

    draw.background().color(srgb(0.1, 0.1, 0.1));

    model.sandbox.render(&draw, &window);

    draw.to_frame(app, &frame).unwrap();
}
