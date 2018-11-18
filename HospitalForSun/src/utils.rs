use components;


pub fn drop_health_mapping(drop_type: &components::DropType) -> i32 {
    let mut res = 0;
    if drop_type == &components::DropType::Icecream {
        res = -5;
    }
    else if drop_type == &components::DropType::Water {
        res = -2;
    }
    else if drop_type == &components::DropType::Fire {
        res = 5;
    }
    else if drop_type == &components::DropType::Soup {
        res = 2;
    }
    res
}

// Collides when: 
// p_left <= x <= p_right 
// and
// p_up <= y
pub fn is_colliding(x: f32, y: f32, left: f32, right: f32, up: f32) -> bool {
    left <= x && x <= right && y <= up
}

// get proper sprite id based on health_state
pub fn sun_sprite_mapping(health_state: &components::HealthState) -> usize {
    // mapping according to textures/pong_sprotesheet.ron
    let mut idx = 2;
    if health_state == &components::HealthState::Hot {
        idx = 3;
    }
    else if health_state == &components::HealthState::Fever {
        idx = 4;
    }
    else if health_state == &components::HealthState::Cold {
        idx = 5;
    }
    else if health_state == &components::HealthState::Freezing {
        idx = 6;
    }
    idx as usize
}

// get health_state based on actual health
pub fn get_health_state(health: i32) -> components::HealthState {
    let mut health_state = components::HealthState::Healthy;
    if 350 <= health && health <= 650 {
        health_state = components::HealthState::Healthy;
    }
    else if 650 < health && health <= 800 {
        health_state = components::HealthState::Hot;
    } 
    else if 800 < health && health <= 1000 {
        health_state = components::HealthState::Fever;
    } 
    else if 200 < health && health < 350  {
        health_state = components::HealthState::Cold;
    } 
    else if 0 <= health && health < 200 {
        health_state = components::HealthState::Freezing;
    } 
    health_state
}
