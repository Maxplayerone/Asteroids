use macroquad::audio::load_sound;
use macroquad::prelude::*;

mod asteroid;
mod bullet;
mod math;
mod player;
mod ui;
mod utils;

pub const WINDOW_WIDTH: i32 = 960;
pub const WINDOW_HEIGHT: i32 = 720;
pub const MAX_ASTEROIDS_COUNT: usize = 25;
pub const SCORE_FOR_ASTEROID: usize = 100;

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
    let asteroid_damage_sound = load_sound("res/audio/asteroid_damage.wav").await.unwrap();
    let asteroid_destroy_sound = load_sound("res/audio/asteroid_destroy.wav").await.unwrap();
    let damaged_sound = load_sound("res/audio/player_damaged.wav").await.unwrap();

    let heart: Texture2D = load_texture("res/images/heart.png").await.unwrap();

    let mut player = player::Player {
        pos: math::Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
        radius: 20.0,
        color: Color::new(0.0, 1.0, 0.0, 1.0),
        angle: 0.0,
        speed: 10.0,
        shoot_sound: shooting_sound,
        invincibility_frames: 60,
        cur_inv_frames: 0,
        damaged_sound,
        health: 3,

        bullet_speed: 15.0,
        bullet_color: Color::new(0.0, 0.8, 0.0, 1.0),
        bullet_radius: 8.0,
    };

    let mut bullets = Vec::new();

    let mut asteroids = Vec::new();

    let mut ui_obj = ui::UiObject {
        score: 0,
        health: player.health,
        heart,
    };

    loop {
        clear_background(GRAY);

        let mut score = 0;

        if is_key_pressed(KeyCode::I) && asteroids.len() < MAX_ASTEROIDS_COUNT {
            asteroids.push(asteroid::Asteroid::new(
                &asteroid_damage_sound,
                &asteroid_destroy_sound,
            ));
        }

        for asteroid in asteroids.iter_mut() {
            asteroid.reset_color();
            asteroid.update();
            asteroid.render();
        }

        let mut bullet_oob_index = Vec::new(); //bullet out of bounds index
        let mut destroyed_asteroids = Vec::new();

        if let Some(bullet) = player.update() {
            if bullets.len() < 10 {
                bullets.push(bullet);
            }
        }

        //(NOTE): check player collission with asteroids
        for asteroid in asteroids.iter() {
            if utils::check_collission_asteroid_player(&player, &asteroid) {
                player.damage_player();
            }
        }
        if player.is_dead() {
            println!("game over");
            break;
        }

        player.render();
        player.render_crosshair();

        for (i, bullet) in bullets.iter_mut().enumerate() {
            bullet.update();
            bullet.render();
            if bullet.is_outside_window() {
                bullet_oob_index.push(i);
            }
        }

        //(NOTE): checking bullet collission with asteroids
        for (j, bullet) in bullets.iter().enumerate() {
            for (i, asteroid) in asteroids.iter_mut().enumerate() {
                if utils::check_collission_with_asteroid(&bullet, &asteroid) {
                    asteroid.health -= 1;
                    asteroid.play_damage_animation();
                    bullet_oob_index.push(j);
                    if asteroid.health <= 0 {
                        destroyed_asteroids.push(i);
                    }
                }
            }
        }
        //(NOTE): deleting the bullets outside the screen
        //and that hit the asteroid
        for i in bullet_oob_index.iter() {
            if *i == bullets.len() {
                println!("bugggg");
                break;
            }

            let last_index = bullets.len() - 1;
            bullets.swap(last_index, *i);
            bullets.pop();
        }

        for i in destroyed_asteroids.iter() {
            if *i == asteroids.len() {
                println!("bug with destroyed asteroids");
                break;
            }
            let last_index = asteroids.len() - 1;
            asteroids.swap(last_index, *i);
            asteroids.pop();
            score += SCORE_FOR_ASTEROID;
        }

        ui_obj.update(score as u32, player.health);
        ui_obj.render();

        if is_key_down(KeyCode::Escape) {
            break;
        }
        next_frame().await
    }
}
