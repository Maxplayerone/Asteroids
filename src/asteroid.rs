use macroquad::prelude::*;
use macroquad::audio;
use ::rand::{thread_rng,Rng};

use crate::math;

pub struct Asteroid<'a>{
    pub pos: math::Vec2,
    pub health: u8,
    pub radius: f32,
    
    sides: u8,
    rotation: f32,
    dir: math::Vec2,
    speed: f32,
    color: Color,
    animation_frames: u8,
    damage_sound: &'a audio::Sound,
    destroy_sound: &'a audio::Sound,
}

const ASTEROID_COLOR: Color = Color::new(0.2, 0.2, 0.2, 1.0);
const ANIMATION_FRAME_COUNT: u8 = 5;

impl<'a> Asteroid<'a>{
    pub fn new(damage_sound: &'a audio::Sound, destroy_sound: &'a audio::Sound) -> Self{
        let mut rng = thread_rng();
        
        let x = rng.gen_range(0.0..crate::WINDOW_WIDTH as f32);
        let y = rng.gen_range(0.0..crate::WINDOW_HEIGHT as f32);
        let pos = math::Vec2::new(x, y);
        
        let radius = rng.gen_range(15.0..70.0);
        let sides = rng.gen_range(5..7);
        let rotation = rng.gen_range(0.0..360.0);
        
        let origin = math::Vec2::new(crate::WINDOW_WIDTH as f32 / 2.0, crate::WINDOW_HEIGHT as f32 / 2.0);
        let dir = math::normalize(&math::vec_sub(&origin, &pos));
        
        Self{
            pos,
            radius,
            sides,
            rotation,
            dir,
            speed: 1.0,
            health: 3,
            color: ASTEROID_COLOR,
            animation_frames: ANIMATION_FRAME_COUNT,
            damage_sound,
            destroy_sound,
        }
    }
    
    pub fn render(&self){
        draw_poly(self.pos.x, self.pos.y, self.sides, self.radius, self.rotation, self.color);
        draw_poly_lines(self.pos.x, self.pos.y, self.sides, self.radius, self.rotation, 3.0, WHITE);
    }
    
    pub fn update(&mut self){
        self.pos = math::vec_add(&self.pos, &math::vec_mul_num(&self.dir, self.speed));
    }
    
    pub fn reset_color(&mut self){
        if self.color != ASTEROID_COLOR{
            self.animation_frames -= 1;
            
            if self.animation_frames <= 0 {
                self.color = ASTEROID_COLOR;
                self.animation_frames = ANIMATION_FRAME_COUNT;
            }
        }
    }
    
    pub fn play_damage_animation(&mut self){
        self.color = Color::new(1.0, 0.46, 0.37, 1.0);
        audio::play_sound_once(self.damage_sound)        
    }
}
