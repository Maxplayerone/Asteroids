use macroquad::prelude::*;
use macroquad::audio::load_sound;

mod math;
mod player;
mod bullet;

pub const WINDOW_WIDTH: i32 = 960;
pub const WINDOW_HEIGHT: i32 = 720;

fn window_conf() -> Conf {
    Conf {
        window_title: "asteroids".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let shooting_sound = load_sound("res/audio/shooting.wav").await.unwrap();

    let mut player = player::Player {
        pos: math::Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
        radius: 20.0,
        color: Color::new(0.0, 1.0, 0.0, 1.0),
        angle: 0.0,
        speed: 10.0,
        shoot_sound: shooting_sound,

        bullet_speed: 15.0,
        bullet_color: Color::new(0.0, 0.8, 0.0, 1.0),
        bullet_radius: 8.0,
    };

    let mut bullets = Vec::new();

    loop {
        clear_background(GRAY);

        let mut bullet_oob_index = Vec::new(); //bullet out of bounds index

        let maybe_bullet = player.update();
        if bullets.len() < 10{
            match maybe_bullet{
                Some(bullet) => bullets.push(bullet),
                None => (),
            }
        }

        player.render();
        player.render_crosshair();

        for (i, bullet) in bullets.iter_mut().enumerate(){
            bullet.update();
            bullet.render();
            if bullet.is_outside_window(){
                bullet_oob_index.push(i);
            }
        }
        for i in bullet_oob_index.iter(){
            let last_index = bullets.len() - 1;
            let last_item = bullets[last_index];
            bullets[last_index] = bullets[*i];
            bullets[*i] = last_item;
            bullets.pop();
        }
        bullet_oob_index.clear();
        
        if is_key_down(KeyCode::Escape) {
            break;
        }
        next_frame().await
    }
}
