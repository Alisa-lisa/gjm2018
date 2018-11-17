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
