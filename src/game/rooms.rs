use super::dog;
use super::state::{Act, GameState, Room};
use super::vibrance;
use super::weather;
use super::wife;

pub fn room_description(game: &GameState) -> String {
    let room_name = game.current_room.display_name();
    let partner_name = &game.partner.name;
    let dog_name = &game.dog.name;

    if game.spiral_active {
        return match game.current_room {
            Room::Bedroom => "The room is empty.".to_string(),
            _ => "The house is quiet.".to_string(),
        };
    }

    let wife_room = wife::wife_room_at_time(game.current_time);
    let wife_activity = wife::wife_activity_at_time(game.current_time);

    let wife_text = if wife_room == game.current_room {
        format!("{} is {}.", partner_name, wife_activity)
    } else {
        String::new()
    };

    let dog_text = if game.act == Act::Life {
        let dog_presence = dog::presence_line(game);
        format!("{} {}.", dog_name, dog_presence)
    } else {
        String::new()
    };

    let mut flavor_parts: Vec<String> = if game.act == Act::Grief {
        grief_room_flavor(game.current_room, dog_name, partner_name, game.bond)
    } else {
        room_flavor(game.current_room, game.peace)
    };

    let weather_note = weather::weather_room_modifier(game.current_weather, game.current_room);
    let atmosphere = vibrance::atmosphere_text(game);
    if let Some(note) = weather_note {
        flavor_parts.push(note.to_string());
    }
    flavor_parts.push(atmosphere.to_string());
    let parts: Vec<&str> = [room_name]
        .into_iter()
        .chain(flavor_parts.iter().map(|string| string.as_str()))
        .collect();

    let mut description = parts.join(". ");

    if !wife_text.is_empty() {
        description.push(' ');
        description.push_str(&wife_text);
    }

    if !dog_text.is_empty() {
        description.push(' ');
        description.push_str(&dog_text);
    }

    description
}

fn room_flavor(room: Room, peace: u32) -> Vec<String> {
    match peace {
        0..=24 => match room {
            Room::Bedroom => vec![
                "The sheets are tangled. You can't remember the last time you changed them"
                    .to_string(),
            ],
            Room::Kitchen => vec!["Dishes are piling up. The air smells stale".to_string()],
            Room::Living => {
                vec!["The couch has a permanent indent where you sit. Curtains drawn".to_string()]
            }
            Room::Office => vec!["Papers and empty cups cover every surface".to_string()],
            Room::Porch => vec!["The light feels too bright".to_string()],
            Room::Backyard => {
                vec!["The garden has gone wild. You haven't been out here in a while".to_string()]
            }
        },
        25..=49 => match room {
            Room::Bedroom => vec!["The bed is unmade".to_string()],
            Room::Kitchen => vec!["The counters could use a wipe".to_string()],
            Room::Living => vec!["The couch looks comfortable enough".to_string()],
            Room::Office => vec!["Your desk is cluttered but functional".to_string()],
            Room::Porch => vec!["The air is cool".to_string()],
            Room::Backyard => vec!["The garden needs some attention".to_string()],
        },
        50..=79 => match room {
            Room::Bedroom => vec!["Sunlight filters through the curtains".to_string()],
            Room::Kitchen => vec!["The kitchen is tidy. Coffee is ready".to_string()],
            Room::Living => vec!["The couch looks inviting".to_string()],
            Room::Office => vec!["Your desk is organized. Ready for work".to_string()],
            Room::Porch => vec!["The morning air is fresh and cool".to_string()],
            Room::Backyard => vec!["The garden is coming along nicely".to_string()],
        },
        _ => match room {
            Room::Bedroom => vec!["The room feels warm and lived-in. Comfortable".to_string()],
            Room::Kitchen => {
                vec!["The kitchen smells like fresh coffee. Everything in its place".to_string()]
            }
            Room::Living => vec!["The living room is cozy. A good place to be".to_string()],
            Room::Office => vec!["Your workspace feels productive. You've got this".to_string()],
            Room::Porch => vec!["The morning is beautiful. Birds are singing".to_string()],
            Room::Backyard => vec!["The garden is flourishing. You're proud of it".to_string()],
        },
    }
}

fn grief_room_flavor(room: Room, dog_name: &str, partner_name: &str, bond: u32) -> Vec<String> {
    match room {
        Room::Kitchen => vec![format!(
            "{}'s bowl is by the counter. {} hasn't moved it.",
            dog_name, partner_name
        )],
        Room::Living => vec![format!("{}'s spot by the window is empty.", dog_name)],
        Room::Office => {
            if bond > 20 {
                vec!["Your foot is cold. It's never been cold before.".to_string()]
            } else {
                vec!["The office is quiet.".to_string()]
            }
        }
        Room::Bedroom => vec!["The foot of the bed is empty.".to_string()],
        Room::Porch => vec![format!("{}'s spot on the porch is empty.", dog_name)],
        Room::Backyard => vec!["The backyard is quiet.".to_string()],
    }
}

pub fn wake_up_text(game: &GameState) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("Day {}.", game.day));

    if let Some(weather_text) = weather::weather_wake_modifier(game.current_weather) {
        lines.push(weather_text.to_string());
    }

    lines.push(String::new());

    if game.act == Act::Life {
        let dog_name = &game.dog.name;
        match game.peace {
            0..=24 => {
                lines.push("You drag yourself awake. Everything feels heavy.".to_string());
                lines.push(format!("{} is at the foot of the bed.", dog_name));
            }
            25..=49 => {
                lines.push(format!(
                    "You wake up. {} is at the foot of the bed.",
                    dog_name
                ));
            }
            50..=79 => {
                lines.push(format!(
                    "You wake up feeling okay. {} is at the foot of the bed, tail wagging.",
                    dog_name
                ));
            }
            _ => {
                lines.push(format!(
                    "You wake up rested. {} is at the foot of the bed, already watching you.",
                    dog_name
                ));
            }
        }
    } else {
        match game.peace {
            0..=24 => {
                lines.push("You wake up. Or rather, you stop pretending to sleep.".to_string());
                lines.push(format!(
                    "The foot of the bed is empty. You miss {}.",
                    game.dog.name
                ));
                if game.is_ng_plus {
                    lines.push("You knew you would.".to_string());
                }
            }
            25..=49 => {
                lines.push("You wake up.".to_string());
                lines.push(format!("You miss {}.", game.dog.name));
                if game.is_ng_plus {
                    lines.push("You knew you would.".to_string());
                }
            }
            _ => {
                lines.push("You wake up.".to_string());
                lines.push(format!(
                    "You think of {} for a moment. Then you get up.",
                    game.dog.name
                ));
            }
        }
    }

    lines
}
