use macroquad::prelude::*;

const PI: f32 = 3.1415;

#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2{
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub fn deg_to_rad(angle_deg: f32) -> f32{
    angle_deg * PI / 180.0
}

pub fn rotate_vec(vec: &Vec2, angle_deg: f32) -> Vec2{
    let angle_rad = deg_to_rad(angle_deg);

    let cos = angle_rad.cos();
    let sin = angle_rad.sin();

    let x = vec.x * cos - vec.y * sin;
    let y = vec.x * sin + vec.y * cos;

    Vec2::new(x, y)
}

pub fn vec_add(vec1: &Vec2, vec2: &Vec2) -> Vec2{
    Vec2::new(vec1.x + vec2.x, vec1.y + vec2.y)
}

pub fn vec_mul_num(vec1: &Vec2, num: f32) -> Vec2{
    Vec2::new(vec1.x * num, vec1.y * num)
}

pub fn vec_add_num(vec: &Vec2, num: f32) -> Vec2{
    Vec2::new(vec.x + num, vec.y + num)
}

const ANGLE_INCREMENT: f32 = 10.0;

struct Player {
    pos: Vec2,
    radius: f32,
    color: Color,
    angle: f32,
    speed: f32,
}

impl Player {
    pub fn render(&self) {
        draw_poly(self.pos.x, self.pos.y, 3, self.radius, self.angle + 210.0, self.color);
    }

    pub fn render_crosshair(&self){
        let rotated_vec = rotate_vec(&Vec2::new(0.0, 1.0), self.angle);
        //let top_point = Vec2::new(self.pos.x + self.radius)
        let new_pos = vec_add(&self.pos, &vec_mul_num(&rotated_vec, self.radius * 2.0));
        draw_circle(new_pos.x, new_pos.y, 5.0, BLUE);
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::A){
            self.angle -= ANGLE_INCREMENT;
            if self.angle < 0.0{
                self.angle += 360.0;
            }
        }
        if is_key_down(KeyCode::D){
            self.angle += ANGLE_INCREMENT;
            if self.angle > 360.0{
                self.angle -= 360.0;
            }
        }
        if is_key_down(KeyCode::W){
            let rotated_vec = rotate_vec(&Vec2::new(0.0, 1.0), self.angle);
            let new_pos = vec_add(&self.pos, &vec_mul_num(&rotated_vec, self.speed));
            self.pos = new_pos;
        }
    }
}

#[macroquad::main("Asteroids")]
async fn main() {
    let mut player = Player {
        pos: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
        radius: 20.0,
        color: Color::new(0.0, 1.0, 0.0, 1.0),
        angle: 0.0,
        speed: 10.0,
    };

    loop {
        clear_background(GRAY);

        player.update();
        player.render();
        player.render_crosshair();
        println!("self angle {}", player.angle);

        if is_key_down(KeyCode::Escape) {
            break;
        }
        next_frame().await
    }
}
