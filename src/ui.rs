use macroquad::prelude::*;

pub struct UiObject {
    pub score: u32,
    pub health: u8,
    pub heart: Texture2D,
}

impl UiObject {
    pub fn update(&mut self, score: u32, health: u8) {
        self.score += score;
        self.health = health;
    }
    pub fn render(&self) {
        draw_text(
            format!("score: {}", self.score).as_str(),
            crate::WINDOW_WIDTH as f32 - 240.0,
            20.0,
            30.0,
            WHITE,
        );
        for i in 0..self.health {
            draw_texture_ex(
                &self.heart,
                crate::WINDOW_WIDTH as f32 - 50.0 - (i as f32 * 50.0),
                30.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(40.0, 40.0)),
                    ..Default::default()
                },
            );
        }
    }
}
