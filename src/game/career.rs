use super::state::{CareerPhase, GameState};

pub fn update_career_phase(game: &mut GameState) {
    if game.fired {
        return;
    }

    game.career_phase = if game.career >= 40 {
        CareerPhase::Phase4
    } else if game.career >= 25 {
        CareerPhase::Phase3
    } else if game.career >= 10 {
        CareerPhase::Phase2
    } else {
        CareerPhase::Phase1
    };
}

pub fn check_career_consequences(game: &mut GameState) -> Vec<String> {
    if game.fired || game.promoted {
        return Vec::new();
    }

    let mut messages = Vec::new();

    if game.day == 7 && game.career < 10 && !game.career_warning_given {
        game.career_warning_given = true;
        messages.push(String::new());
        messages.push("You have a new email from your manager.".to_string());
        messages.push(
            "\"Just wanted to check in on your progress. Let's make sure we're on track.\""
                .to_string(),
        );
        messages.push("It reads friendly. It doesn't feel friendly.".to_string());
    }

    if game.day == 10 && game.career < 20 && !game.performance_review_pending {
        game.performance_review_pending = true;
        messages.push(String::new());
        messages.push(
            "Your manager has scheduled a performance review for tomorrow morning.".to_string(),
        );
        messages.push("You'll lose 2 hours at the start of the day.".to_string());
    }

    if game.day == 14 && game.career < 30 && !game.on_probation {
        game.on_probation = true;
        messages.push(String::new());
        messages.push("You've been placed on probation.".to_string());
        messages.push(
            "\"We need to see improvement. Effective immediately, your compensation is adjusted.\""
                .to_string(),
        );
        messages.push("Money from work is halved while on probation.".to_string());
    }

    if game.day >= 17 && game.career < 40 {
        game.fired = true;
        messages.push(String::new());
        messages.push("Your manager calls you into their office.".to_string());
        messages.push("\"We've given you every opportunity. I'm sorry.\"".to_string());
        messages.push(String::new());
        messages.push("You've been let go. No more work actions available.".to_string());
    }

    messages
}

pub fn check_promotion(game: &mut GameState) -> Option<Vec<String>> {
    if game.fired || game.promoted {
        return None;
    }

    if game.career >= 50 && game.day >= 19 {
        game.promoted = true;
        game.money += 100;
        return Some(vec![
            String::new(),
            "Your manager calls you into their office.".to_string(),
            format!(
                "\"{}. We've been impressed with your work. We'd like to offer you a promotion.\"",
                game.player.name
            ),
            String::new(),
            "You got promoted! +$100 bonus.".to_string(),
        ]);
    }

    None
}

pub fn major_project_work_bonus(game: &GameState) -> u32 {
    if game.major_project_days_remaining > 0 {
        2
    } else {
        0
    }
}

pub fn advance_major_project(game: &mut GameState) -> Option<Vec<String>> {
    if game.major_project_days_remaining == 0 {
        return None;
    }

    game.major_project_days_remaining -= 1;

    if game.major_project_days_remaining == 0 {
        game.career += 5;
        return Some(vec![
            String::new(),
            "You finished the major project. Great work.".to_string(),
            "+5 Career.".to_string(),
        ]);
    }

    Some(vec![format!(
        "Major project: {} days remaining.",
        game.major_project_days_remaining
    )])
}
