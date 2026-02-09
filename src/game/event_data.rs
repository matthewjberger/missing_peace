use super::career;
use super::events::{EventCondition, EventDef, EventEffect, EventTree};
use super::grief;
use super::state::{Act, DayPhase, GameState, Room};
use super::wife;

pub fn build_event_tree() -> EventTree {
    let mut tree = EventTree::new();

    tree.add_event(EventDef {
        id: "wake_up",
        label: "Get out of bed",
        description: "",
        message: "You drag yourself out of bed.",
        condition: EventCondition {
            required_phase: Some(DayPhase::WakeUp),
            ..Default::default()
        },
        effect: EventEffect {
            set_phase: Some(DayPhase::MandatoryChore),
            set_room: Some(Room::Kitchen),
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "mandatory_chore",
        label: "Morning chore",
        description: "",
        message: "",
        condition: EventCondition {
            required_phase: Some(DayPhase::MandatoryChore),
            custom_check: Some(|game| !game.day.is_multiple_of(7)),
            ..Default::default()
        },
        effect: EventEffect {
            home: 2,
            time_hours: 1.0,
            set_phase: Some(DayPhase::FreeTime),
            custom_effect: Some(apply_mandatory_chore),
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: Some(mandatory_chore_description),
    });

    tree.add_event(EventDef {
        id: "weekend_morning",
        label: "Enjoy the morning",
        description: "",
        message: "No chores today. You take it easy.",
        condition: EventCondition {
            required_phase: Some(DayPhase::MandatoryChore),
            custom_check: Some(|game| game.day.is_multiple_of(7)),
            ..Default::default()
        },
        effect: EventEffect {
            peace: 1,
            time_hours: 0.5,
            set_phase: Some(DayPhase::FreeTime),
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "morning_shift",
        label: "Morning Shift",
        description: "",
        message: "You put in a solid shift at work.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            requires_not_fired: true,
            max_time: Some(12.0),
            ..Default::default()
        },
        effect: EventEffect {
            career: 2,
            time_hours: 5.0,
            energy: -1,
            money: 8,
            custom_effect: Some(apply_work_bonus),
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "afternoon_shift",
        label: "Afternoon Shift",
        description: "",
        message: "You put in a solid shift at work.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            requires_not_fired: true,
            min_time: Some(12.0),
            max_time: Some(18.0),
            ..Default::default()
        },
        effect: EventEffect {
            career: 2,
            time_hours: 5.0,
            energy: -1,
            money: 8,
            custom_effect: Some(apply_work_bonus),
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "side_project",
        label: "Side Project",
        description: "",
        message: "You work on your side project. It feels good to build something for yourself.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            requires_not_fired: true,
            min_energy: Some(1),
            max_time: Some(20.0),
            custom_check: Some(|game| game.career_phase != super::state::CareerPhase::Phase1),
            ..Default::default()
        },
        effect: EventEffect {
            career: 1,
            time_hours: 3.0,
            energy: -1,
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "overtime",
        label: "Overtime",
        description: "",
        message: "You stay late. The office is quiet. You get a lot done.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            requires_not_fired: true,
            min_time: Some(17.0),
            min_energy: Some(1),
            custom_check: Some(|game| {
                matches!(
                    game.career_phase,
                    super::state::CareerPhase::Phase3 | super::state::CareerPhase::Phase4
                )
            }),
            ..Default::default()
        },
        effect: EventEffect {
            career: 2,
            peace: -1,
            time_hours: 3.0,
            energy: -1,
            money: 5,
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "networking_call",
        label: "Networking Call",
        description: "",
        message: "You call a colleague. Good conversation.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            requires_not_fired: true,
            ..Default::default()
        },
        effect: EventEffect {
            career: 1,
            time_hours: 1.5,
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "feed_dog",
        label: "[OPTIONAL]: Feed {dog}",
        description: "",
        message: "{dog} eats eagerly.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            required_act: Some(Act::Life),
            requires_dog_not_fed: true,
            ..Default::default()
        },
        effect: EventEffect {
            bond: 1,
            time_hours: 0.5,
            set_dog_fed: true,
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "walk_dog",
        label: "[OPTIONAL]: Walk {dog}",
        description: "",
        message: "You take {dog} for a walk around the neighborhood. {dog} seems happy.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            required_act: Some(Act::Life),
            requires_dog_not_walked: true,
            min_energy: Some(1),
            ..Default::default()
        },
        effect: EventEffect {
            bond: 4,
            time_hours: 1.5,
            energy: -1,
            set_dog_walked: true,
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "play_dog",
        label: "[OPTIONAL]: Play with {dog}",
        description: "",
        message: "You play with {dog} for a while. Tail wagging the whole time.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            required_act: Some(Act::Life),
            min_energy: Some(1),
            custom_check: Some(|game| game.has_dog_toy),
            ..Default::default()
        },
        effect: EventEffect {
            bond: 3,
            time_hours: 1.0,
            energy: -1,
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "sit_with_dog",
        label: "[OPTIONAL]: Sit with {dog}",
        description: "",
        message: "You sit with {dog}. Just the two of you, quiet. It's enough.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            required_act: Some(Act::Life),
            min_time: Some(18.0),
            custom_check: Some(|game| game.bond > 10),
            ..Default::default()
        },
        effect: EventEffect {
            bond: 2,
            time_hours: 0.5,
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "buy_dog_toy",
        label: "Buy a toy for {dog}",
        description: "",
        message: "You pick up a toy on the way home. {dog} is going to love this.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            required_act: Some(Act::Life),
            min_money: Some(5),
            custom_check: Some(|game| !game.has_dog_toy),
            ..Default::default()
        },
        effect: EventEffect {
            money: -5,
            time_hours: 1.0,
            custom_effect: Some(apply_buy_dog_toy),
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "accept_major_project",
        label: "Accept Major Project",
        description: "",
        message: "",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            requires_not_fired: true,
            min_energy: Some(1),
            custom_check: Some(|game| {
                game.major_project_uses < 3
                    && matches!(
                        game.career_phase,
                        super::state::CareerPhase::Phase3 | super::state::CareerPhase::Phase4
                    )
            }),
            ..Default::default()
        },
        effect: EventEffect {
            custom_effect: Some(apply_major_project_skip),
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "work_trip",
        label: "Work Trip",
        description: "",
        message: "",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            required_act: Some(Act::Life),
            requires_not_fired: true,
            min_energy: Some(1),
            custom_check: Some(|game| {
                !game.work_trip_taken
                    && matches!(
                        game.career_phase,
                        super::state::CareerPhase::Phase3 | super::state::CareerPhase::Phase4
                    )
            }),
            ..Default::default()
        },
        effect: EventEffect {
            custom_effect: Some(apply_work_trip_skip),
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "walk_partner",
        label: "[OPTIONAL] Walk with {partner}",
        description: "",
        message: "",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            min_energy: Some(1),
            custom_check: Some(wife::should_offer_walk_invite),
            ..Default::default()
        },
        effect: EventEffect {
            partner_investment: 3,
            bond: 3,
            time_hours: 2.0,
            energy: -1,
            custom_effect: Some(apply_partner_walk),
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "gift_partner",
        label: "Get a gift for {partner}",
        description: "",
        message: "",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            min_money: Some(20),
            ..Default::default()
        },
        effect: EventEffect {
            partner_investment: 3,
            money: -20,
            time_hours: 1.5,
            custom_effect: Some(apply_gift_partner),
            ..Default::default()
        },
        cooldown: 5,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "date_night",
        label: "Date night with {partner}",
        description: "",
        message: "",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            min_money: Some(15),
            min_time: Some(18.0),
            min_energy: Some(1),
            custom_check: Some(|game| {
                game.wife_mood == super::state::WifeMood::Happy
                    || game.wife_mood == super::state::WifeMood::Content
            }),
            ..Default::default()
        },
        effect: EventEffect {
            partner_investment: 4,
            peace: 2,
            money: -15,
            time_hours: 3.0,
            energy: -1,
            custom_effect: Some(apply_date_night),
            ..Default::default()
        },
        cooldown: 5,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "shared_meal",
        label: "Cook together with {partner}",
        description: "",
        message: "",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            min_time: Some(17.0),
            max_time: Some(20.0),
            custom_check: Some(|game| !game.cooked_today),
            ..Default::default()
        },
        effect: EventEffect {
            partner_investment: 2,
            home: 2,
            peace: 1,
            time_hours: 2.0,
            custom_effect: Some(apply_shared_meal),
            ..Default::default()
        },
        cooldown: 3,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "dishes",
        label: "Do the dishes",
        description: "",
        message: "You wash the dishes. The kitchen looks better.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            custom_check: Some(|game| !game.did_dishes_today),
            ..Default::default()
        },
        effect: EventEffect {
            home: 2,
            time_hours: 1.0,
            ..Default::default()
        },
        cooldown: 2,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "laundry",
        label: "Do the laundry",
        description: "",
        message: "You run a load of laundry.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            custom_check: Some(|game| !game.did_laundry_today),
            ..Default::default()
        },
        effect: EventEffect {
            home: 3,
            time_hours: 2.0,
            ..Default::default()
        },
        cooldown: 2,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "vacuum",
        label: "Vacuum",
        description: "",
        message: "You vacuum the house. Everything feels cleaner.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            custom_check: Some(|game| !game.vacuumed_today),
            ..Default::default()
        },
        effect: EventEffect {
            home: 2,
            time_hours: 1.5,
            ..Default::default()
        },
        cooldown: 2,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "cook",
        label: "Cook a meal",
        description: "",
        message: "You cook dinner. {partner} appreciates the effort.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            custom_check: Some(|game| !game.cooked_today),
            ..Default::default()
        },
        effect: EventEffect {
            home: 2,
            partner_investment: 1,
            energy: 1,
            time_hours: 1.5,
            ..Default::default()
        },
        cooldown: 3,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "groceries",
        label: "Get groceries",
        description: "",
        message: "You pick up groceries. The fridge is full again.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            min_money: Some(30),
            custom_check: Some(|game| !game.shopped_today),
            ..Default::default()
        },
        effect: EventEffect {
            home: 3,
            money: -30,
            time_hours: 2.0,
            ..Default::default()
        },
        cooldown: 2,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "video_games",
        label: "Play video games",
        description: "",
        message: "You play video games for a while. It's nice to unwind.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            custom_check: Some(|game| !game.played_games_today),
            ..Default::default()
        },
        effect: EventEffect {
            peace: 2,
            time_hours: 2.0,
            ..Default::default()
        },
        cooldown: 1,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "check_news",
        label: "Check the news",
        description: "",
        message: "You scroll through the news. Nothing too exciting.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            custom_check: Some(|game| !game.checked_news_today),
            ..Default::default()
        },
        effect: EventEffect {
            peace: 1,
            time_hours: 1.0,
            ..Default::default()
        },
        cooldown: 1,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "visit_spot",
        label: "Visit {dog}'s spot",
        description: "",
        message: "You sit by the window. His spot. The sunbeam is warm.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            required_act: Some(Act::Grief),
            ..Default::default()
        },
        effect: EventEffect {
            time_hours: 1.0,
            custom_effect: Some(apply_grief_visit_spot),
            ..Default::default()
        },
        cooldown: 2,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "write_letter",
        label: "Write a letter",
        description: "",
        message: "",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            required_act: Some(Act::Grief),
            min_energy: Some(1),
            ..Default::default()
        },
        effect: EventEffect {
            time_hours: 1.5,
            energy: -1,
            custom_effect: Some(apply_write_letter),
            ..Default::default()
        },
        cooldown: 3,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "call_friend",
        label: "Call a friend",
        description: "",
        message: "You call someone you haven't talked to in a while. They listen.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            required_act: Some(Act::Grief),
            ..Default::default()
        },
        effect: EventEffect {
            time_hours: 1.0,
            custom_effect: Some(apply_grief_call_friend),
            ..Default::default()
        },
        cooldown: 3,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "walk_alone",
        label: "Go for a walk alone",
        description: "",
        message: "You walk the old route. Without the leash. It's different, but it's something.",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            required_act: Some(Act::Grief),
            min_energy: Some(1),
            ..Default::default()
        },
        effect: EventEffect {
            time_hours: 1.5,
            energy: -1,
            custom_effect: Some(apply_grief_walk_alone),
            ..Default::default()
        },
        cooldown: 2,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "look_at_photos",
        label: "Look at old photos",
        description: "",
        message: "",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            required_act: Some(Act::Grief),
            ..Default::default()
        },
        effect: EventEffect {
            time_hours: 1.0,
            custom_effect: Some(apply_look_at_photos),
            ..Default::default()
        },
        cooldown: 3,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "reach_out_partner",
        label: "Reach out to {partner}",
        description: "",
        message: "",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            required_act: Some(Act::Grief),
            custom_check: Some(|game| {
                game.grief_path == Some(super::state::GriefPath::B)
                    && game.path_b_effort_counter >= 10
            }),
            ..Default::default()
        },
        effect: EventEffect {
            partner_investment: 3,
            time_hours: 1.0,
            custom_effect: Some(apply_reach_out_partner),
            ..Default::default()
        },
        cooldown: 3,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "go_to_bed",
        label: "Go to bed",
        description: "",
        message: "",
        condition: EventCondition {
            required_phase: Some(DayPhase::FreeTime),
            requires_bedtime: true,
            ..Default::default()
        },
        effect: EventEffect {
            set_phase: Some(DayPhase::BedRitual),
            set_room: Some(Room::Bedroom),
            custom_effect: Some(apply_bed_transition),
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree.add_event(EventDef {
        id: "go_to_sleep",
        label: "Go to sleep",
        description: "",
        message: "",
        condition: EventCondition {
            required_phase: Some(DayPhase::BedRitual),
            ..Default::default()
        },
        effect: EventEffect {
            custom_effect: Some(apply_sleep),
            ..Default::default()
        },
        cooldown: 0,
        dynamic_description: None,
    });

    tree
}

fn apply_work_bonus(game: &mut GameState) -> Vec<String> {
    let mut messages = Vec::new();

    if game.spiral_active {
        game.career = game.career.saturating_sub(2);
        return vec![
            "You can't focus. The work blurs together.".to_string(),
            "Career +0.".to_string(),
        ];
    }

    if game.is_ng_plus {
        let tedious_lines = [
            "You answer emails. They're the same as yesterday's.",
            "You work. You're not sure why it matters.",
            "Another meeting. Another set of action items.",
            "You do the work. It's fine. It's always fine.",
        ];
        let index = (game.day as usize) % tedious_lines.len();
        messages.push(tedious_lines[index].to_string());
    }

    if game.on_probation {
        let half = 4;
        game.money -= half;
        messages.push("Probation pay. Half what it should be.".to_string());
    }

    let bonus = career::major_project_work_bonus(game);
    if bonus > 0 {
        game.career += bonus;
        messages.push(format!("+{} bonus from major project.", bonus));
    }

    messages
}

fn apply_buy_dog_toy(game: &mut GameState) -> Vec<String> {
    game.has_dog_toy = true;
    vec![format!(
        "You got a toy for {}. Now you can play together.",
        game.dog.name
    )]
}

fn apply_major_project_skip(game: &mut GameState) -> Vec<String> {
    let mut messages = Vec::new();
    let partner_name = game.partner.name.clone();
    let dog_name = game.dog.name.clone();

    let start_day = game.day;
    for offset in 0..3 {
        messages.push(format!("Day {}: You work.", start_day + offset));
    }

    game.day += 3;
    game.days_skipped += 3;
    game.major_project_uses += 1;
    game.career += 10;
    game.money += 5;
    game.peace += 3;

    game.current_time = 6.0;
    game.day_phase = super::state::DayPhase::WakeUp;
    game.current_room = super::state::Room::Bedroom;
    game.dog_fed_today = false;
    game.dog_walked_today = false;
    game.check_dog_today = 0;
    game.keep_talking_available = false;
    game.recent_actions.clear();

    if game.home > 3 {
        game.home -= 3;
    }

    game.current_weather = super::weather::weather_for_day(game.day);

    messages.push(String::new());
    messages.push(format!(
        "{} handled everything while you were busy.",
        partner_name
    ));
    messages.push(format!("The house is clean. {} is fine.", dog_name));
    messages.push(String::new());
    messages.push("Career +10. Money +$5. Peace +3.".to_string());

    messages
}

fn apply_work_trip_skip(game: &mut GameState) -> Vec<String> {
    let mut messages = Vec::new();
    let partner_name = game.partner.name.clone();
    let dog_name = game.dog.name.clone();

    let start_day = game.day;
    for offset in 0..5 {
        messages.push(format!(
            "Day {}: Hotel room. Meetings. Airport food.",
            start_day + offset
        ));
    }

    game.day += 5;
    game.days_skipped += 5;
    game.work_trip_taken = true;
    game.career += 15;
    game.money += 10;
    game.peace = game.peace.saturating_sub(3);
    game.partner_investment = game.partner_investment.saturating_sub(2);

    game.current_time = 6.0;
    game.day_phase = super::state::DayPhase::WakeUp;
    game.current_room = super::state::Room::Bedroom;
    game.dog_fed_today = false;
    game.dog_walked_today = false;
    game.check_dog_today = 0;
    game.keep_talking_available = false;
    game.recent_actions.clear();

    if game.home > 5 {
        game.home -= 5;
    }

    game.current_weather = super::weather::weather_for_day(game.day);

    messages.push(String::new());
    messages.push("You come home to a quiet house.".to_string());
    messages.push(format!(
        "{} managed everything while you were away.",
        partner_name
    ));
    messages.push(format!("{} barely looks up when you walk in.", dog_name));
    messages.push(String::new());
    messages.push("Career +15. Money +$10. Peace -3. Home -5.".to_string());

    messages
}

fn apply_partner_walk(game: &mut GameState) -> Vec<String> {
    game.days_since_walk_invite = 0;
    vec![
        format!(
            "You and {} go for a walk together. It's nice.",
            game.partner.name
        ),
        "You talk about nothing in particular. It feels easy.".to_string(),
    ]
}

fn apply_bed_transition(game: &mut GameState) -> Vec<String> {
    if game.act == Act::Life {
        vec![format!(
            "{} curls up at the foot of the bed.",
            game.dog.name
        )]
    } else {
        vec!["You head to bed.".to_string()]
    }
}

fn apply_sleep(game: &mut GameState) -> Vec<String> {
    let mut messages = Vec::new();

    if game.act == Act::Life {
        let partner_name = game.partner.name.clone();
        let dog_name = game.dog.name.clone();
        if !game.dog_fed_today {
            messages.push(format!("{} fed {}.", partner_name, dog_name));
        }
        if !game.dog_walked_today {
            messages.push(format!("{} took {} for a walk.", partner_name, dog_name));
        }
    }

    game.start_new_day();
    game.current_weather = super::weather::weather_for_day(game.day);
    career::update_career_phase(game);
    wife::update_mood(game);

    if let Some(project_lines) = career::advance_major_project(game) {
        messages.extend(project_lines);
    }

    if game.performance_review_pending {
        game.performance_review_pending = false;
        game.advance_time(2.0);
        messages.push("You spend the morning in a performance review.".to_string());
        messages.push("Two hours gone. Not a great start to the day.".to_string());
    }

    let career_messages = career::check_career_consequences(game);
    messages.extend(career_messages);

    if let Some(promotion_lines) = career::check_promotion(game) {
        messages.extend(promotion_lines);
    }

    if game.act == Act::Grief {
        let good_day_lines = grief::check_good_day(game);
        messages.extend(good_day_lines);

        let turning_point_lines = grief::check_turning_point(game);
        messages.extend(turning_point_lines);
    }

    let day_milestone = day_milestone_message(game.day);
    if let Some(milestone) = day_milestone {
        messages.push(milestone.to_string());
    }

    if game.day.is_multiple_of(7) && game.act == Act::Life {
        messages.push("It's the weekend. No mandatory chores today.".to_string());
    }

    messages
}

fn day_milestone_message(day: u32) -> Option<&'static str> {
    match day {
        7 => Some("One week. Time moves differently when you're paying attention."),
        10 => Some("Ten days in. You've found a rhythm."),
        13 => Some("Almost two weeks. The house feels like a home."),
        17 => Some("Seventeen days. You notice things you didn't before."),
        _ => None,
    }
}

pub fn event_id_to_scene_action(event_id: &str) -> Option<&'static str> {
    match event_id {
        "walk_dog" => Some("walk_dog"),
        "walk_partner" => Some("walk_partner"),
        _ => None,
    }
}

fn mandatory_chore_description(game: &GameState) -> String {
    let chore = mandatory_chore_for_day(game.day);
    chore.0.to_string()
}

fn mandatory_chore_for_day(day: u32) -> (&'static str, &'static str) {
    match day % 5 {
        0 => ("Dishes", "You wash the dishes. The kitchen looks better."),
        1 => (
            "Tidying up",
            "You tidy up the house. Things look less chaotic.",
        ),
        2 => (
            "Take out trash",
            "You take the trash out. One less thing to worry about.",
        ),
        3 => (
            "Sweeping",
            "You sweep the floors. The dust bunnies put up a fight.",
        ),
        _ => (
            "Quick clean",
            "You wipe down the counters and straighten up.",
        ),
    }
}

fn apply_mandatory_chore(game: &mut GameState) -> Vec<String> {
    let (_, message) = mandatory_chore_for_day(game.day);
    vec![message.to_string()]
}

fn apply_gift_partner(game: &mut GameState) -> Vec<String> {
    let partner_name = game.partner.name.clone();
    game.partner_actions_taken.push("gift".to_string());
    vec![
        "You pick up something small on the way home.".to_string(),
        format!("{} looks surprised. \"What's this for?\"", partner_name),
        "\"No reason.\"".to_string(),
        "She smiles. The real one.".to_string(),
    ]
}

fn apply_date_night(game: &mut GameState) -> Vec<String> {
    let partner_name = game.partner.name.clone();
    game.date_night_count += 1;
    game.partner_actions_taken.push("date_night".to_string());
    vec![
        format!("You and {} go out for the evening.", partner_name),
        "Somewhere quiet. Good food. No phones.".to_string(),
        "You talk about everything and nothing.".to_string(),
        "It feels like it used to feel.".to_string(),
    ]
}

fn apply_shared_meal(game: &mut GameState) -> Vec<String> {
    let partner_name = game.partner.name.clone();
    game.partner_actions_taken.push("shared_meal".to_string());
    vec![
        format!("You cook together. {} chops, you stir.", partner_name),
        "The kitchen is a mess. Neither of you cares.".to_string(),
        "The food turns out better than expected.".to_string(),
    ]
}

fn apply_reach_out_partner(game: &mut GameState) -> Vec<String> {
    let partner_name = game.partner.name.clone();
    game.partner_actions_taken.push("reach_out".to_string());
    game.peace += 2;
    vec![
        format!("You find {} in the kitchen.", partner_name),
        "\"Can we talk?\" you say.".to_string(),
        "She looks surprised. Then relieved.".to_string(),
        String::new(),
        "You sit together. You tell her you've been trying.".to_string(),
        "She says she knows. She's been watching.".to_string(),
        String::new(),
        "It's not fixed. But something shifted.".to_string(),
    ]
}

fn apply_grief_visit_spot(game: &mut GameState) -> Vec<String> {
    grief::track_grief_effort(game, "visit_spot");
    game.peace += 2;
    Vec::new()
}

fn apply_grief_call_friend(game: &mut GameState) -> Vec<String> {
    grief::track_grief_effort(game, "call_friend");
    game.peace += 2;
    Vec::new()
}

fn apply_grief_walk_alone(game: &mut GameState) -> Vec<String> {
    grief::track_grief_effort(game, "walk_alone");
    game.peace += 2;
    Vec::new()
}

fn apply_write_letter(game: &mut GameState) -> Vec<String> {
    let dog_name = game.dog.name.clone();
    grief::track_grief_effort(game, "write_letter");
    game.peace += 3;
    vec![
        format!("You write to {}. Everything you wish you'd said.", dog_name),
        "You don't send it anywhere. That's not the point.".to_string(),
    ]
}

fn apply_look_at_photos(game: &mut GameState) -> Vec<String> {
    grief::track_grief_effort(game, "look_at_photos");
    let dog_name = game.dog.name.clone();

    match game.grief_path {
        Some(super::state::GriefPath::A) => {
            game.peace += 2;
            vec![
                format!("You scroll through photos of {}.", dog_name),
                "It hurts. But they make you smile too.".to_string(),
                "+2 Peace.".to_string(),
            ]
        }
        Some(super::state::GriefPath::B) => {
            game.peace = game.peace.saturating_sub(2);
            vec![
                format!("You look at photos of {}.", dog_name),
                "Each one is a punch to the chest.".to_string(),
                "You close the phone.".to_string(),
                "-2 Peace.".to_string(),
            ]
        }
        _ => {
            game.peace += 1;
            vec![
                format!("You find an old photo of {}.", dog_name),
                "Younger. Happier. You remember that day.".to_string(),
                "+1 Peace.".to_string(),
            ]
        }
    }
}

pub fn set_daily_flags(event_id: &str, game: &mut GameState) {
    match event_id {
        "dishes" => game.did_dishes_today = true,
        "laundry" => game.did_laundry_today = true,
        "vacuum" => game.vacuumed_today = true,
        "cook" => game.cooked_today = true,
        "shared_meal" => game.cooked_today = true,
        "groceries" => game.shopped_today = true,
        "video_games" => game.played_games_today = true,
        "check_news" => game.checked_news_today = true,
        _ => {}
    }
}

pub fn event_id_to_grief_room(event_id: &str) -> Option<Room> {
    match event_id {
        "dishes" | "mandatory_chore" | "cook" | "shared_meal" => Some(Room::Kitchen),
        "vacuum" => Some(Room::Living),
        "laundry" => Some(Room::Bedroom),
        _ => None,
    }
}
