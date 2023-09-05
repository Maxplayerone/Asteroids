const PI: f32 = 3.1415;

#[derive(Debug, Copy, Clone)]
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

pub fn vec_sub(vec1: &Vec2, vec2: &Vec2) -> Vec2{
    Vec2::new(vec1.x - vec2.x, vec1.y - vec2.y)
}

pub fn vec_mul_num(vec1: &Vec2, num: f32) -> Vec2{
    Vec2::new(vec1.x * num, vec1.y * num)
}
