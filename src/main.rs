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
    //let space: Texture2D = load_texture("res/images/space.png").await.unwrap();
    //let space2: Texture2D = load_texture("res/images/space2.png").await.unwrap();

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

    let mut asteroids: Vec<asteroid::Asteroid> = Vec::new();

    let mut ui_obj = ui::UiObject {
        score: 0,
        health: player.health,
        heart,
    };

    let mut frames_btw_waves = 60.0;
    let mut cur_frame = 0.0;

    loop {
        clear_background(BLACK);
        /*
        draw_texture_ex(
            &space,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        */
        draw_text(
            "ASTEROIDS",
            crate::WINDOW_WIDTH as f32 / 2.0 - 120.0,
            60.0,
            80.0,
            WHITE,
        );
        draw_text(
            "PLAY (click enter)",
            crate::WINDOW_WIDTH as f32 / 2.0 - 120.0,
            300.0,
            40.0,
            WHITE,
        );
        draw_text(
            "QUIT (click escape)",
            crate::WINDOW_WIDTH as f32 / 2.0 - 120.0,
            400.0,
            40.0,
            WHITE,
        );

        if is_key_down(KeyCode::Enter){
            break;
        }
        if is_key_down(KeyCode::Escape){
            break;
        }

        next_frame().await
    }

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));
        /*
        draw_texture_ex(
            &space2,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        */

        let mut score = 0;

        cur_frame += 1.0;
        if cur_frame >= frames_btw_waves && asteroids.len() < MAX_ASTEROIDS_COUNT {
            asteroids.push(asteroid::Asteroid::new(
                &asteroid_damage_sound,
                &asteroid_destroy_sound,
            ));
            frames_btw_waves = frames_btw_waves * 0.99;
            cur_frame = 0.0;
        }
        if let Some(bullet) = player.update() {
            if bullets.len() < 10 {
                bullets.push(bullet);
            }
        }

        let mut bullet_oob_index = Vec::new(); //bullet out of bounds index
        let mut destroyed_asteroids = Vec::new();

        for (i, asteroid) in asteroids.iter_mut().enumerate() {
            asteroid.reset_color();
            asteroid.update();
            if asteroid.is_outside_window() {
                destroyed_asteroids.push(i);
            }
            asteroid.render();
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
            //println!("laste indexx {}", last_index);
            bullets.swap(last_index, *i);
            bullets.pop();
        }

        for i in destroyed_asteroids.iter() {
            if *i == asteroids.len() {
                println!("bug with destroyed asteroids");
                break;
            }
            let last_index = asteroids.len() - 1;
            //println!("lasjdkdlk indedx {}", last_index);
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
