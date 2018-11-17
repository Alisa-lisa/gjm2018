use amethyst::prelude::{ReadStorage, WriteStorage, Entity, System};

use components::{Position, Health, HealthState, DropType, Velocity, Paddle};


pub struct DebugPrint;

impl<'a> System<'a> for DebugPrint {
    type SystemData = (ReadStorage<'a, Position>, 
                       ReadStorage<'a, DropType>);

    fn run(&mut self, data: Self::Systemdata) {
        let (pos, drop) = data;
        println!("Drop {:?} at position {:?}", &pos, &drop);
    }
}

pub struct MovePlayer {
    player: Entity,
}

impl<'a> System<'a> for MovePlayer {
    type SystemData = WriteStorage<'a, Position>;

    fn run(&mut self, )
}
