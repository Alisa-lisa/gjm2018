use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use components::{Position, Side, Health, HealthState, DropType, Velocity, Paddle};
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
