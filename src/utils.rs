use crate::asteroid;
use crate::bullet;
use crate::math;
use crate::player;

pub fn check_collission_with_asteroid(
    bullet: &bullet::Bullet,
    asteroid: &asteroid::Asteroid,
) -> bool {
    math::dist(&bullet.pos, &asteroid.pos) < asteroid.radius
}

pub fn check_collission_asteroid_player(
    player: &player::Player,
    asteroid: &asteroid::Asteroid,
) -> bool {
    math::dist(&player.pos, &asteroid.pos) < asteroid.radius
}
