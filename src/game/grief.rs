use rand::RngExt;

use super::state::{Act, DayPhase, GameState, GriefPath, Room};

pub fn calculate_grief_path(game: &GameState) -> GriefPath {
    let bond = game.bond;
    let partner_investment = game.partner_investment;

    if bond >= 30 && partner_investment >= 30 {
        GriefPath::A
    } else if bond <= 15 && partner_investment <= 15 {
        GriefPath::B
    } else {
        GriefPath::C
    }
}

pub fn trigger_act_2(game: &mut GameState) -> Vec<String> {
    let path = calculate_grief_path(game);
    game.grief_path = Some(path);
    game.act = Act::Grief;
    game.pre_grief_peace = game.peace;
    game.peace = 0;
    game.grief_day_count = 0;
    game.day += 1;
    game.current_time = 6.0;
    game.day_phase = DayPhase::WakeUp;
    game.current_room = Room::Bedroom;

    match path {
        GriefPath::A => path_a_death_text(game),
        GriefPath::B => path_b_death_text(game),
        GriefPath::C => path_c_death_text(game),
    }
}

fn path_a_death_text(game: &GameState) -> Vec<String> {
    let dog_name = &game.dog.name;
    let partner_name = &game.partner.name;

    vec![
        String::new(),
        String::new(),
        format!(
            "You come downstairs. {} is at the kitchen table.",
            partner_name
        ),
        format!("{}'s bowl is in front of her.", dog_name),
        "She's holding it with both hands.".to_string(),
        String::new(),
        "She looks up at you.".to_string(),
        "She doesn't need to say it.".to_string(),
        String::new(),
        format!("{} is gone.", dog_name),
        String::new(),
        "She stands up and puts her arms around you.".to_string(),
        "Neither of you says anything for a long time.".to_string(),
        String::new(),
        "[Press any key to continue]".to_string(),
    ]
}

fn path_b_death_text(game: &GameState) -> Vec<String> {
    let dog_name = &game.dog.name;
    let partner_name = &game.partner.name;

    vec![
        String::new(),
        String::new(),
        format!("It's late. {} comes into the bedroom.", partner_name),
        format!("\"{}...\" she starts.", game.player.name),
        String::new(),
        "She sits on the edge of the bed.".to_string(),
        format!("\"It's {}. He's... he didn't wake up.\"", dog_name),
        String::new(),
        "You stare at the ceiling.".to_string(),
        "The house is very quiet.".to_string(),
        String::new(),
        "[Press any key to continue]".to_string(),
    ]
}

fn path_c_death_text(game: &GameState) -> Vec<String> {
    let dog_name = &game.dog.name;
    let partner_name = &game.partner.name;

    vec![
        String::new(),
        String::new(),
        format!("You notice {} has been quiet today.", partner_name),
        "Quieter than usual.".to_string(),
        String::new(),
        "She's in the kitchen, standing at the counter.".to_string(),
        "Not doing anything. Just standing there.".to_string(),
        String::new(),
        format!("\"What's wrong?\" you ask."),
        String::new(),
        format!("\"It's {}.\"", dog_name),
        String::new(),
        "You know before she finishes.".to_string(),
        String::new(),
        format!("{} is gone.", dog_name),
        String::new(),
        "[Press any key to continue]".to_string(),
    ]
}

pub fn bed_ritual_text(game: &GameState) -> Vec<String> {
    if game.spiral_active {
        return vec!["You get into bed.".to_string()];
    }

    let grief_day = game.grief_day_count;
    let partner_name = &game.partner.name;

    match game.grief_path {
        Some(GriefPath::A) => {
            if grief_day < 3 {
                vec![
                    format!(
                        "{} is already in bed. She reaches for your hand.",
                        partner_name
                    ),
                    "You lie there together in the dark.".to_string(),
                    "Neither of you sleeps well.".to_string(),
                    "+1 Peace.".to_string(),
                ]
            } else if grief_day < 6 {
                vec![
                    format!("{} pulls the covers over both of you.", partner_name),
                    "She doesn't say anything. She doesn't need to.".to_string(),
                    "Your hand finds hers in the dark.".to_string(),
                    "+1 Peace.".to_string(),
                ]
            } else {
                vec![
                    format!("\"Goodnight,\" {} says.", partner_name),
                    "\"Goodnight.\"".to_string(),
                    "It feels almost normal. Almost warm.".to_string(),
                    "+1 Peace.".to_string(),
                ]
            }
        }
        Some(GriefPath::B) => {
            if grief_day < 3 {
                vec![
                    "The bed feels too big tonight.".to_string(),
                    "You lie there staring at the ceiling.".to_string(),
                    "Sleep doesn't come.".to_string(),
                ]
            } else if grief_day < 6 {
                vec![
                    "You lie down. Pull the covers up.".to_string(),
                    "The ceiling is the same as yesterday.".to_string(),
                    "But your eyes close a little faster tonight.".to_string(),
                ]
            } else {
                vec![
                    "You get into bed.".to_string(),
                    "It's still too quiet. But you're tired enough to sleep.".to_string(),
                ]
            }
        }
        Some(GriefPath::C) => {
            if grief_day < 3 {
                vec![
                    format!(
                        "{} is in bed. You can't tell if she's asleep.",
                        partner_name
                    ),
                    "You lie on your side, facing away.".to_string(),
                    "The gap between you feels wider than the bed.".to_string(),
                ]
            } else if grief_day < 6 {
                vec![
                    format!("{} shifts when you get in.", partner_name),
                    "She's awake. You both know it.".to_string(),
                    "Neither of you speaks. But you're both here.".to_string(),
                ]
            } else {
                vec![
                    format!("\"You coming to bed?\" {} asks.", partner_name),
                    "\"Yeah.\"".to_string(),
                    "You lie closer tonight. Not touching. But closer.".to_string(),
                ]
            }
        }
        None => Vec::new(),
    }
}

pub fn chore_break_text(game: &mut GameState, room: Room) -> Option<Vec<String>> {
    if game.act != Act::Grief {
        return None;
    }

    if game.is_ng_plus {
        return None;
    }

    let base_chance = if game.grief_path == Some(GriefPath::B) {
        game.chore_break_chance.max(0.3)
    } else {
        0.4
    };

    let effective_chance = if game.spiral_active {
        (base_chance * 2.0).min(1.0)
    } else {
        base_chance
    };

    let should_trigger = {
        let mut rng = rand::rng();
        let triggered = rng.random_range(0.0..1.0) < effective_chance;
        if triggered && game.grief_path == Some(GriefPath::B) {
            game.chore_break_chance = (game.chore_break_chance + 0.15).min(0.8);
        }
        triggered
    };

    if !should_trigger {
        return None;
    }

    let dog_name = game.dog.name.clone();
    let partner_name = game.partner.name.clone();

    let mut lines = match room {
        Room::Kitchen => vec![
            format!(
                "You reach for {}'s water bowl. It's not there anymore.",
                dog_name
            ),
            "You stare at the empty spot for a moment.".to_string(),
        ],
        Room::Living => vec![
            "You vacuum around the spot by the window.".to_string(),
            format!("The one where {} used to lie in the sun.", dog_name),
            "There's still hair there.".to_string(),
        ],
        Room::Bedroom => vec![
            "You make the bed.".to_string(),
            format!(
                "The indent at the foot where {} slept is still there.",
                dog_name
            ),
        ],
        Room::Backyard => vec![
            format!("You find one of {}'s toys in the garden.", dog_name),
            "A chewed-up tennis ball.".to_string(),
            "You put it in your pocket.".to_string(),
        ],
        _ => return None,
    };

    match game.grief_path {
        Some(GriefPath::A) => {
            lines.push(String::new());
            lines.push(format!("{} finds you standing there.", partner_name));
            lines.push("She doesn't say anything. Just takes over.".to_string());
            lines.push("\"Go sit down. I've got this.\"".to_string());
        }
        Some(GriefPath::B) => {
            lines.push(String::new());
            lines.push("Nobody comes.".to_string());
            lines.push("You stand there alone for a while.".to_string());
            game.home = game.home.saturating_sub(1);
            game.peace = game.peace.saturating_sub(1);
            lines.push("-1 Home. -1 Peace.".to_string());
        }
        Some(GriefPath::C) => {
            lines.push(String::new());
            lines.push(format!("{} is in the other room.", partner_name));
            lines.push("She might have noticed. She doesn't come over.".to_string());
            lines.push("You finish the chore yourself.".to_string());
            game.peace = game.peace.saturating_sub(1);
            lines.push("-1 Peace.".to_string());
        }
        None => {}
    }

    Some(lines)
}

pub fn check_spiral(game: &mut GameState) -> Option<Vec<String>> {
    if game.is_ng_plus {
        return None;
    }

    if game.grief_path != Some(GriefPath::B) {
        return None;
    }

    if game.peace < 10 {
        game.spiral_days += 1;
    } else {
        game.spiral_days = 0;
    }

    if game.spiral_days >= 2 && !game.spiral_active {
        game.spiral_active = true;
        return Some(spiral_text(game));
    }

    None
}

fn spiral_text(game: &GameState) -> Vec<String> {
    let partner_name = &game.partner.name;

    vec![
        String::new(),
        "You haven't moved from the couch in hours.".to_string(),
        "The TV is on. You're not watching.".to_string(),
        String::new(),
        format!("{} sits down next to you.", partner_name),
        "She doesn't say anything at first.".to_string(),
        String::new(),
        "\"I'm worried about you,\" she says finally.".to_string(),
        String::new(),
        "You don't respond.".to_string(),
        String::new(),
        format!("\"{}. Look at me.\"", game.player.name),
        String::new(),
        "You look at her.".to_string(),
        String::new(),
        "\"You can talk to me. About anything. I mean it.\"".to_string(),
        String::new(),
        "[You can now use 'talk to partner (honest)']".to_string(),
    ]
}

pub fn is_resolution_ready(game: &GameState) -> bool {
    if game.act != Act::Grief {
        return false;
    }
    if game.day >= 30 {
        return true;
    }
    let threshold = match game.grief_path {
        Some(GriefPath::A) => 25,
        Some(GriefPath::B) => 20,
        Some(GriefPath::C) => 22,
        None => 25,
    };
    game.peace >= threshold
}

pub fn apply_grief_bed_ritual(game: &mut GameState) {
    game.grief_day_count += 1;
    game.peace_floor = (game.grief_day_count * 3).min(25);

    match game.grief_path {
        Some(GriefPath::A) => {
            if game.is_ng_plus {
                game.peace += 3;
            } else {
                game.peace += 2;
            }
        }
        Some(GriefPath::B) | Some(GriefPath::C) | None => {
            if game.is_ng_plus {
                game.peace += 2;
            } else {
                game.peace += 1;
            }
        }
    }

    if game.peace < game.peace_floor {
        game.peace = game.peace_floor;
    }
}

pub fn check_good_day(game: &mut GameState) -> Vec<String> {
    if game.act != Act::Grief {
        return Vec::new();
    }

    if !game.grief_day_count.is_multiple_of(4) || game.grief_day_count == 0 {
        return Vec::new();
    }

    let mut rng = rand::rng();
    let roll: f64 = rng.random_range(0.0..1.0);
    if roll > 0.7 {
        return Vec::new();
    }

    let (peace_gain, message) = match game.grief_path {
        Some(GriefPath::A) => (
            5,
            "You and her have a good evening together. It doesn't fix anything, but it helps.",
        ),
        Some(GriefPath::B) => (
            3,
            "Something shifts today. Just a little. The weight lifts, briefly.",
        ),
        Some(GriefPath::C) => (4, "A quiet moment of clarity. Not everything is broken."),
        None => (3, "Today was a little better than yesterday."),
    };

    game.peace += peace_gain;
    vec![message.to_string(), format!("+{} Peace.", peace_gain)]
}

pub fn check_turning_point(game: &mut GameState) -> Vec<String> {
    if game.turning_point_triggered {
        return Vec::new();
    }

    if game.grief_day_count < 5 || game.peace < 15 {
        return Vec::new();
    }

    game.turning_point_triggered = true;

    let partner_name = game.partner.name.clone();

    match game.grief_path {
        Some(GriefPath::A) => vec![
            "Something changes today.".to_string(),
            format!("\"I think we're going to be okay,\" {} says.", partner_name),
            "You're not sure. But you want to believe her.".to_string(),
            "For the first time, that feels possible.".to_string(),
        ],
        Some(GriefPath::B) => vec![
            "You got out of bed today without having to convince yourself.".to_string(),
            "Small. But it's something.".to_string(),
            "The world is still grey. But maybe less so.".to_string(),
        ],
        Some(GriefPath::C) => vec![
            format!("{} looks at you across the kitchen.", partner_name),
            "\"You seem... a little better today.\"".to_string(),
            "\"Maybe,\" you say.".to_string(),
            "It's the most honest thing you've said in weeks.".to_string(),
        ],
        None => Vec::new(),
    }
}

pub fn track_grief_effort(game: &mut GameState, action_id: &str) {
    if game.grief_path != Some(GriefPath::B) {
        return;
    }

    let grief_actions = [
        "visit_spot",
        "write_letter",
        "call_friend",
        "walk_alone",
        "look_at_photos",
    ];
    if grief_actions.contains(&action_id) {
        game.path_b_effort_counter += 1;
    }
}
