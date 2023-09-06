use macroquad::prelude::*;
use macroquad::audio::{play_sound_once, Sound};

use crate::math;
use crate::bullet;

const ANGLE_INCREMENT: f32 = 10.0;

pub struct Player {
    pub pos: math::Vec2,
    pub radius: f32,
    pub color: Color,
    pub angle: f32,
    pub speed: f32,
    pub shoot_sound: Sound,
    
    //the values are in player cuz I want to have every
    //single customazible variable in main
    pub bullet_color: Color,
    pub bullet_speed: f32,
    pub bullet_radius: f32,
}

impl Player {
    pub fn render(&self) {
        draw_poly(self.pos.x, self.pos.y, 3, self.radius, self.angle + 210.0, self.color);
        draw_poly_lines(self.pos.x, self.pos.y, 3, self.radius, self.angle + 210.0, 1.5, WHITE);
    }

    pub fn render_crosshair(&self){
        let rotated_vec = math::rotate_vec(&math::Vec2::new(0.0, 1.0), self.angle);
        let new_pos = math::vec_add(&self.pos, &math::vec_mul_num(&rotated_vec, self.radius * 2.0));
        draw_circle(new_pos.x, new_pos.y, 5.0, WHITE);
    }

    pub fn update(&mut self) -> Option<bullet::Bullet>{
        //(NOTE): movement
        if is_key_down(KeyCode::J){
            self.angle -= ANGLE_INCREMENT;
            if self.angle < 0.0{
                self.angle += 360.0;
            }
        }
        if is_key_down(KeyCode::L){
            self.angle += ANGLE_INCREMENT;
            if self.angle > 360.0{
                self.angle -= 360.0;
            }
        }
        /*
        if is_key_down(KeyCode::W){
            let rotated_vec = math::rotate_vec(&math::Vec2::new(0.0, 1.0), self.angle);
            let new_pos = math::vec_add(&self.pos, &math::vec_mul_num(&rotated_vec, self.speed));
            self.pos = new_pos;
        }
        if is_key_down(KeyCode::S){
            let rotated_vec = math::rotate_vec(&math::Vec2::new(0.0, 1.0), self.angle);
            let new_pos = math::vec_sub(&self.pos, &math::vec_mul_num(&rotated_vec, self.speed));
            self.pos = new_pos;
        }
        */    
        if is_key_down(KeyCode::W){
            self.pos = math::Vec2::new(self.pos.x, self.pos.y - self.speed);
        }
        if is_key_down(KeyCode::S){
            self.pos = math::Vec2::new(self.pos.x, self.pos.y + self.speed);
        }
        if is_key_down(KeyCode::A){
            self.pos = math::Vec2::new(self.pos.x - self.speed, self.pos.y);
        }
        if is_key_down(KeyCode::D){
            self.pos = math::Vec2::new(self.pos.x + self.speed, self.pos.y);
        }
        
        //(NOTE): shooting
        if is_key_pressed(KeyCode::K){
            play_sound_once(&self.shoot_sound);
            
            let rotated_vec = math::rotate_vec(&math::Vec2::new(0.0, 1.0), self.angle);
            let new_pos = math::vec_add(&self.pos, &math::vec_mul_num(&rotated_vec, self.radius * 1.5));
            
            return Some(bullet::Bullet{
                pos: new_pos,
                dir: rotated_vec,
                radius: self.bullet_radius,  
                color: self.bullet_color,
                speed: self.bullet_speed,
            });
        }
        None
    }
}
