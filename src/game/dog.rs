use super::state::{DogAgePhase, DogVisualStage, GameState};

pub fn presence_line(game: &GameState) -> &'static str {
    let bond = game.bond;
    let phase = game.dog_age_phase();

    if matches!(phase, DogAgePhase::Fading | DogAgePhase::Final) {
        return elder_presence_line(bond, phase);
    }

    match bond {
        0..=5 => "is somewhere in the house",
        6..=12 => "is watching you",
        13..=22 => "looks happy to see you",
        23..=35 => "is in its usual spot",
        36..=50 => "settles in next to you",
        _ => "is here",
    }
}

fn elder_presence_line(bond: u32, phase: DogAgePhase) -> &'static str {
    match (phase, bond) {
        (DogAgePhase::Fading, 30..) => "is close, breathing slowly, leaning into your warmth",
        (DogAgePhase::Fading, _) => "is resting near you, moving less each day",
        (DogAgePhase::Final, 30..) => "is very still, pressed against you, breathing softly",
        (DogAgePhase::Final, _) => "is very still. Sleeps more. Stays close",
        _ => "is nearby",
    }
}

pub fn visual_ascii(stage: DogVisualStage) -> &'static str {
    match stage {
        DogVisualStage::Stage1 => "o",
        DogVisualStage::Stage2 => "(o)",
        DogVisualStage::Stage3 => " ^ ^\n(o o)\n \\__/",
        DogVisualStage::Stage4 => "  /\\_/\\\n ( o.o )\n  > ^ <~",
        DogVisualStage::Stage5 => "  /\\_/\\\n ( ^.^ )\n  > ^ <~\n  || ||",
    }
}

pub fn age_description(phase: DogAgePhase, dog_name: &str) -> String {
    match phase {
        DogAgePhase::Settled => format!(
            "{} is still getting used to the place. A bit skittish, but warming up.",
            dog_name
        ),
        DogAgePhase::Comfortable => format!(
            "{} knows where everything is now. Comfortable. At ease.",
            dog_name
        ),
        DogAgePhase::Prime => format!("{} is in full stride. Energetic and happy.", dog_name),
        DogAgePhase::Slowing => format!(
            "{} is slowing down a little. Takes longer to get up. Still happy to see you.",
            dog_name
        ),
        DogAgePhase::Fading => format!(
            "{} moves carefully now. Rests more. But still looks for you when you enter a room.",
            dog_name
        ),
        DogAgePhase::Final => format!(
            "{} is very still lately. Sleeps more. Stays close.",
            dog_name
        ),
    }
}

pub fn check_description(game: &GameState) -> Vec<String> {
    let dog_name = &game.dog.name;
    let bond = game.bond;
    let stage = game.dog_visual_stage();

    let mut lines = Vec::new();

    let description = match bond {
        0..=5 => "A small shape by the door. Still.".to_string(),
        6..=10 => format!("{} is watching you from across the room.", dog_name),
        11..=15 => "It's looking at you. Two small eyes.".to_string(),
        16..=20 => format!("{} perks up when you enter.", dog_name),
        21..=25 => "It's a dog. It's been a dog this whole time.".to_string(),
        26..=35 => format!("{} comes over to greet you.", dog_name),
        36..=50 => format!("{} is right there, tail going.", dog_name),
        _ => format!("{} is here.", dog_name),
    };
    lines.push(description);

    lines.push(String::new());
    lines.push(age_description(game.dog_age_phase(), dog_name));

    let ascii = visual_ascii(stage);
    lines.push(String::new());
    for ascii_line in ascii.lines() {
        lines.push(ascii_line.to_string());
    }

    if !game.dog_fed_today {
        lines.push(String::new());
        lines.push(format!("{} looks hungry.", dog_name));
    }
    if !game.dog_walked_today {
        lines.push(format!(
            "{} is looking at you expectantly. Maybe a walk?",
            dog_name
        ));
    }

    lines
}

pub fn post_action_echo(event_id: &str, dog_name: &str) -> Option<String> {
    match event_id {
        "walk_dog" => Some(format!("{} is tired from the walk. Happy tired.", dog_name)),
        "feed_dog" => Some(format!(
            "{} licks the bowl clean. Looks at you like you're a hero.",
            dog_name
        )),
        "play_dog" => Some(format!(
            "{} flops down, panting. Tail still going.",
            dog_name
        )),
        _ => None,
    }
}
