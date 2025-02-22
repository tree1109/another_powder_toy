use nannou::prelude::*;

pub struct DebugSystem {
    fps: f32,
    window: Rect,
}

impl DebugSystem {
    pub fn new() -> Self {
        Self {
            fps: 0.0,
            window: Rect::from_w_h(0.0, 0.0),
        }
    }

    pub fn update(&mut self, app: &App, _update: &Update) {
        self.fps = app.fps();
        self.window = app.window_rect();
    }

    pub fn render(&self, draw: &Draw, _frame: &Frame) {
        // align top left
        {
            let fps_rect = Rect::from_w_h(100.0, 50.0).top_left_of(self.window);
            draw.text(&format!("FPS: {:.1}", self.fps))
                .xy(fps_rect.xy())
                .wh(fps_rect.wh())
                .left_justify()
                .align_text_top()
                .font_size(20)
                .color(srgb(1.0, 1.0, 1.0));
        }

        // key bindings
        {
            let text = "Key Bindings:
- Left Click: Place Sand
- Ctrl + Left Click: Remove Sand";
            let hint_rect = Rect::from_w_h(300.0, 200.0).mid_left_of(self.window);
            draw.text(&text)
                .xy(hint_rect.xy())
                .wh(hint_rect.wh())
                .left_justify()
                .align_text_middle_y()
                .font_size(18)
                .color(srgb(1.0, 1.0, 1.0));
        }
    }
}
