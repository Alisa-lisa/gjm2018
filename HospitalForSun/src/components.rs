use amethyst::ecs::{Component, VecStorage, DenseVecStorage};

#[derive(Default, Debug)]
pub struct Health {
    pub amount: i32,
}

impl Component for Health {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub enum HealthState {
    Dead,
    Freezing,
    Cold,
    Healthy,
    Warm,
    Hot,
}

impl Component for HealthState {
    type Storage = DenseVecStorage<Self>;
}

#[derive(PartialEq, Debug)]
pub enum DropType {
    Icecream,
    Water,
    Fire,
    Soup,
}

impl Component for DropType {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Debug)]
pub struct Velocity(pub f32);

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Bottom,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        Paddle {
            side: side,
            width: 1000.0,
            height: 1.0,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
