use amethyst::ecs::{Component, VecStorage, DenseVecStorage};
use utils;

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

pub struct DropValue {
    pub value: i32,
    pub drop_type: DropType,
    pub width: f32,
    pub height: f32,
    pub velocity: f32,
}

impl DropValue {
    pub fn new(drop_type: DropType) -> DropValue {
        DropValue {
            value: utils::drop_health_mapping(&drop_type),
            drop_type: drop_type,
            width: 8.0,
            height: 8.0,
            velocity: 10.0,
        }
    }
}

impl Component for DropValue {
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
            width: 16.0,
            height: 4.0,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
