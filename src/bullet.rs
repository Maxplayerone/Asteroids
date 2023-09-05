use macroquad::prelude::*;

use crate::math;

#[derive(Copy, Clone)]
pub struct Bullet{
    pub pos: math::Vec2,
    pub radius: f32,
    pub color: Color,
    pub speed: f32,
    pub dir: math::Vec2,
}

impl Bullet{
    pub fn render(&self){
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
    }
    
    pub fn update(&mut self){
        self.pos = math::vec_add(&self.pos, &math::vec_mul_num(&self.dir, self.speed));
    }
    
    pub fn is_outside_window(&self) -> bool{
        let width = crate::WINDOW_WIDTH as f32;
        let height = crate::WINDOW_HEIGHT as f32;
        if self.pos.x < 0.0 || self.pos.x > width || self.pos.y < 0.0 || self.pos.y > height{
            return true;
        }
        false
    }
}
