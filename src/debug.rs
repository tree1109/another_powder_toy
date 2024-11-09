use nannou::prelude::*;

pub struct DebugSystem {
    fps: f32,
    fps_position: Vec2,
}

impl DebugSystem {
    pub fn new() -> Self {
        Self {
            fps: 0.0,
            fps_position: vec2(0.0, 0.0),
        }
    }

    pub fn update(&mut self, app: &App) {
        self.fps = app.fps();

        let window = app.window_rect();
        self.fps_position = window.top_left();
    }

    pub fn render(&self, draw: &Draw) {
        // align top left
        draw.text(&format!("FPS: {:.1}", self.fps))
            .xy(self.fps_position)
            .right_justify()
            .align_text_bottom()
            .font_size(24)
            .color(srgb(1.0, 1.0, 1.0));
    }
}
