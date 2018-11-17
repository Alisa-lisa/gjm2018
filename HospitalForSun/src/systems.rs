use amethyst::core::{Transform, Time};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Entities};
use amethyst::input::InputHandler;
use components::{DropValue, Side, Health, HealthState, DropType, Paddle};
use entities::{ARENA_WIDTH};
use utils;

pub struct PaddleMovement;

impl<'s> System<'s> for PaddleMovement {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (p, mut t) in (&paddles, &mut transforms).join() {
            let movement  = match p.side {
                Side::Bottom => input.axis_value("player_paddle"),
            };
            if let Some(mv) = movement {
                let amount = 1.2 * mv as f32;
                t.translation[0] = (t.translation[0] + amount)
                    .min(ARENA_WIDTH - 8.0)
                    .max(8.0);
            }
        }
    }
}

pub struct DropFall;

impl<'s> System<'s> for DropFall {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, DropValue>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut pos, drops, time): Self::SystemData) {
        for (d, p) in (&drops, &mut pos).join() {
            p.translation[1] -= d.velocity * time.delta_seconds();
        }
        
    }
}

pub struct Collide;

impl<'s> System<'s> for Collide {
    type SystemData = (
        WriteStorage<'s, Paddle>,
        WriteStorage<'s, DropValue>,
        ReadStorage<'s, Transform>,
        Entities<'s>
        );

    fn run(&mut self, (mut player, mut drops, pos, entities): Self::SystemData) {
        for (d, t, e) in (&mut drops, &pos, &*entities).join() {
            let d_x = t.translation[0];
            let d_y = t.translation[1];
            
            for (p, p_coord) in (&mut player, &pos).join() {
                let p_left = p_coord.translation[0] - p.width * 0.5;
                let p_right = p_coord.translation[0] + p.width * 0.5;
                let p_up = p_coord.translation[1] + p.height * 0.5;
                if utils::is_colliding(d_x, d_y, p_left, p_right, p_up) {
                    // update sun's health
                    
                    // destroy drop entity
                    entities.delete(e);
                }
            }

        }
    }
}

