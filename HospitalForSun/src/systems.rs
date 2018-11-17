use amethyst::core::{Transform, Time};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use components::{DropValue, Side, Health, HealthState, DropType, Paddle};
use entities::{ARENA_WIDTH};


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
