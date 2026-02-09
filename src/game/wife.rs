use super::state::{GameState, Room, WifeMood};

pub fn wife_room_at_time(current_time: f64) -> Room {
    match current_time as u32 {
        6..=8 => Room::Kitchen,
        9..=11 => Room::Living,
        12..=13 => Room::Kitchen,
        14..=17 => Room::Living,
        18..=19 => Room::Kitchen,
        20..=23 => Room::Bedroom,
        _ => Room::Bedroom,
    }
}

pub fn wife_activity_at_time(current_time: f64) -> &'static str {
    match current_time as u32 {
        6..=8 => "making coffee",
        9..=11 => "reading on the couch",
        12..=13 => "making lunch",
        14..=17 => "working on her laptop",
        18..=19 => "cooking dinner",
        20..=21 => "getting ready for bed",
        _ => "asleep",
    }
}

pub fn update_mood(game: &mut GameState) {
    let partner_investment = game.partner_investment;
    game.wife_mood = if partner_investment >= 40 {
        WifeMood::Happy
    } else if partner_investment >= 25 {
        WifeMood::Content
    } else if partner_investment >= 15 {
        WifeMood::Neutral
    } else if partner_investment >= 5 {
        WifeMood::Lonely
    } else {
        WifeMood::Distant
    };

    if partner_investment >= 45 && game.partner_milestone_reached < 3 {
        game.partner_milestone_reached = 3;
    } else if partner_investment >= 30 && game.partner_milestone_reached < 2 {
        game.partner_milestone_reached = 2;
    } else if partner_investment >= 15 && game.partner_milestone_reached < 1 {
        game.partner_milestone_reached = 1;
    }
}

pub fn mood_description(mood: WifeMood) -> &'static str {
    match mood {
        WifeMood::Happy => "She seems happy. Relaxed.",
        WifeMood::Content => "She looks content.",
        WifeMood::Neutral => "She seems fine. Quiet.",
        WifeMood::Lonely => "She looks a little lonely.",
        WifeMood::Distant => "She seems distant. Preoccupied.",
    }
}

pub fn should_offer_walk_invite(game: &GameState) -> bool {
    if game.wife_mood == WifeMood::Distant {
        return false;
    }

    if game.days_since_walk_invite < 2 {
        return false;
    }

    let phase_threshold = match game.dog_age_phase() {
        super::state::DogAgePhase::Settled | super::state::DogAgePhase::Comfortable => 3,
        super::state::DogAgePhase::Prime | super::state::DogAgePhase::Slowing => 2,
        super::state::DogAgePhase::Fading | super::state::DogAgePhase::Final => 2,
    };

    game.days_since_walk_invite >= phase_threshold
}

pub fn check_description(game: &GameState) -> Vec<String> {
    let partner_name = &game.partner.name;
    let wife_room = wife_room_at_time(game.current_time);
    let activity = wife_activity_at_time(game.current_time);
    let mood_desc = mood_description(game.wife_mood);

    let mut lines = Vec::new();

    if wife_room == game.current_room {
        lines.push(format!("{} is here, {}.", partner_name, activity));
        lines.push(mood_desc.to_string());
    } else {
        lines.push(format!(
            "{} is in the {}, probably {}.",
            partner_name,
            wife_room.display_name(),
            activity
        ));
    }

    lines
}

pub fn talk_response(game: &GameState, honest: bool) -> Vec<String> {
    let partner_name = &game.partner.name;
    let mut lines = Vec::new();

    if honest {
        match game.wife_mood {
            WifeMood::Happy | WifeMood::Content => {
                lines.push(format!(
                    "\"I'm glad we can talk like this,\" {} says.",
                    partner_name
                ));
                lines.push("You have an honest conversation. It feels good.".to_string());
            }
            WifeMood::Neutral => {
                lines.push(format!(
                    "\"Thanks for being honest with me,\" {} says quietly.",
                    partner_name
                ));
            }
            WifeMood::Lonely | WifeMood::Distant => {
                lines.push(format!(
                    "\"I appreciate you trying,\" {} says. \"I've been feeling a bit alone lately.\"",
                    partner_name
                ));
                lines.push("The honesty helps, even if the conversation is hard.".to_string());
            }
        }
    } else {
        match game.wife_mood {
            WifeMood::Happy => {
                lines.push("\"How's your day going?\" you ask.".to_string());
                lines.push(format!(
                    "\"Good! Really good,\" {} says with a smile.",
                    partner_name
                ));
            }
            WifeMood::Content => {
                lines.push("\"Everything okay?\" you ask.".to_string());
                lines.push(format!(
                    "\"Yeah, everything's fine,\" {} says.",
                    partner_name
                ));
            }
            WifeMood::Neutral => {
                lines.push(format!(
                    "You make small talk. {} responds, but seems distracted.",
                    partner_name
                ));
            }
            WifeMood::Lonely => {
                lines.push(format!(
                    "\"We should do something together soon,\" {} says.",
                    partner_name
                ));
                lines.push("You nod.".to_string());
            }
            WifeMood::Distant => {
                lines.push(format!(
                    "{} gives a short answer and goes back to what she was doing.",
                    partner_name
                ));
            }
        }
    }

    lines
}
