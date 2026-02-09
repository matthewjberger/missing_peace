use super::state::{Room, Weather};

pub fn weather_for_day(day: u32) -> Weather {
    let hash = day.wrapping_mul(2654435761);
    match hash % 10 {
        0..=4 => Weather::Sunny,
        5..=7 => Weather::Overcast,
        8 => Weather::Rainy,
        _ => Weather::Stormy,
    }
}

pub fn weather_room_modifier(weather: Weather, room: Room) -> Option<&'static str> {
    match (weather, room) {
        (Weather::Rainy, Room::Porch) => Some("Rain patters on the roof"),
        (Weather::Rainy, Room::Backyard) => Some("The garden is wet and grey"),
        (Weather::Rainy, Room::Kitchen) => Some("Rain streaks down the kitchen window"),
        (Weather::Stormy, Room::Porch) => Some("Wind howls across the porch"),
        (Weather::Stormy, Room::Backyard) => Some("The trees are bending in the wind"),
        (Weather::Stormy, Room::Bedroom) => Some("Thunder rumbles in the distance"),
        (Weather::Sunny, Room::Porch) => Some("Warm sunlight across the porch"),
        (Weather::Sunny, Room::Backyard) => Some("The sun is out. Good day for the garden"),
        (Weather::Overcast, Room::Porch) => Some("Grey sky. Feels like it might rain"),
        _ => None,
    }
}

pub fn weather_wake_modifier(weather: Weather) -> Option<&'static str> {
    match weather {
        Weather::Rainy => Some("Rain against the window."),
        Weather::Stormy => Some("A storm outside. The house creaks."),
        Weather::Overcast => Some("Grey light through the curtains."),
        Weather::Sunny => None,
    }
}
