use components;


pub fn drop_health_mapping(drop_type: components::DropType) -> i32 {
    let mut res = 0;
    if drop_type == components::DropType::Icecream {
        res = -5;
    }
    else if drop_type == components::DropType::Water {
        res = -2;
    }
    else if drop_type == components::DropType::Fire {
        res = 5;
    }
    else if drop_type == components::DropType::Soup {
        res = 2;
    }
    res
}
