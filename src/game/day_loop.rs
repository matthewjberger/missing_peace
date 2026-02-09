use ember::prelude::*;
use rand::RngExt;

use super::event_data;
use super::events::EventTree;
use super::grief;
use super::hud;
use super::memory;
use super::resolution::ResolutionState;
use super::rooms;
use super::save;
use super::scene_data;
use super::scenes::{self, SceneManager};
use super::screen::ScreenBuffer;
use super::state::{Act, GameState, SimulateMode};
use super::transcript;
use super::vibrance;
use super::wife;

pub struct DayLoopState {
    game: GameState,
    screen: Option<ScreenBuffer>,
    needs_redraw: bool,
    input_buffer: String,
    text_log: Vec<String>,
    cursor_visible: bool,
    cursor_timer: f64,
    scene_manager: SceneManager,
    all_scenes: Vec<scenes::SceneDef>,
    event_tree: EventTree,
    simulate: SimulateMode,
    simulate_timer: f64,
    last_logged_day: u32,
}

impl DayLoopState {
    pub fn new_with_simulate(mut game_state: GameState, simulate: SimulateMode) -> Self {
        game_state.current_weather = super::weather::weather_for_day(game_state.day);
        let day = game_state.day;
        Self {
            game: game_state,
            screen: None,
            needs_redraw: true,
            input_buffer: String::new(),
            text_log: Vec::new(),
            cursor_visible: true,
            cursor_timer: 0.0,
            scene_manager: SceneManager::new(),
            all_scenes: scene_data::all_scenes(),
            event_tree: event_data::build_event_tree(),
            simulate,
            simulate_timer: 0.0,
            last_logged_day: day,
        }
    }

    fn simulate_delay(&self) -> f64 {
        match self.simulate {
            SimulateMode::Fast => 0.01,
            SimulateMode::Slow => 0.5,
            SimulateMode::Off => 0.0,
        }
    }

    fn push_log(&mut self, text: &str) {
        self.text_log.push(text.to_string());
        if self.simulate.is_active() {
            if self.game.day != self.last_logged_day {
                self.last_logged_day = self.game.day;
                transcript::log_separator();
                let act_label = if self.game.act == Act::Grief {
                    " [GRIEF]"
                } else {
                    ""
                };
                transcript::log(&format!(
                    "=== Day {} {}{} ===",
                    self.game.day,
                    self.game.time_display(),
                    act_label,
                ));
                transcript::log(&format!(
                    "  Career:{} Home:{} Peace:{} Bond:{} PI:{} Money:${} Energy:{}",
                    self.game.career,
                    self.game.home,
                    self.game.peace,
                    self.game.bond,
                    self.game.partner_investment,
                    self.game.money,
                    self.game.energy,
                ));
            }
            if !text.is_empty() {
                transcript::log(text);
            }
        }
        self.needs_redraw = true;
    }

    fn push_empty_log(&mut self) {
        self.text_log.push(String::new());
    }

    fn check_scenes(&mut self, action_taken: Option<&str>) {
        if self.scene_manager.is_displaying() {
            return;
        }

        if let Some(scene) = scenes::check_for_scenes(&self.game, &self.all_scenes, action_taken) {
            self.scene_manager.queue_scene(scene);
            self.needs_redraw = true;
        }
    }

    fn process_command(&mut self, input: &str) {
        let input = input.trim();
        if input.is_empty() {
            return;
        }

        if self.game.keep_talking_available {
            self.game.keep_talking_available = false;
            if input == "1" {
                self.game.partner_investment += 1;
                self.game.advance_time(0.25);
                let partner_name = self.game.partner.name.clone();
                self.push_log(&format!(
                    "You keep talking with {}. About nothing important. It helps.",
                    partner_name
                ));
                self.push_log("+1 PI.");
                return;
            }
        }

        if let Ok(number) = input.parse::<usize>() {
            let action_list = self.event_tree.available_events(&self.game);
            if number >= 1 && number <= action_list.len() {
                let node_index = action_list[number - 1].node_index;
                let event_id = self.event_tree.graph[node_index].id;
                self.execute_event(node_index, event_id);
                return;
            }
            self.push_log("Invalid choice.");
            return;
        }

        let lower = input.to_lowercase();
        if lower.starts_with("check ") {
            let target = input[6..].trim().to_lowercase();
            let dog_lower = self.game.dog.name.to_lowercase();
            let partner_lower = self.game.partner.name.to_lowercase();

            if target == dog_lower || target == "dog" {
                self.game.advance_time(5.0 / 60.0);
                if self.game.act == Act::Grief {
                    self.push_log(&format!("{} is gone.", self.game.dog.name));
                } else {
                    if self.game.check_dog_today < 3 {
                        self.game.bond += 1;
                    }
                    self.game.check_dog_today += 1;
                    let lines = super::dog::check_description(&self.game);
                    for line in &lines {
                        self.push_log(line);
                    }
                }
            } else if target == partner_lower || target == "wife" || target == "partner" {
                self.game.advance_time(5.0 / 60.0);
                let lines = wife::check_description(&self.game);
                for line in &lines {
                    self.push_log(line);
                }
            } else if target == "room" {
                self.game.advance_time(2.0 / 60.0);
                let description = rooms::room_description(&self.game);
                self.push_log(&description);
            } else if target == "watch" || target == "time" {
                self.push_log(&format!("It's {}.", self.game.time_display()));
            } else if target == "news" {
                self.game.advance_time(0.25);
                self.push_log("You scroll through the news.");
                self.push_log(
                    "Economy's uncertain. Weather's strange. Someone famous said something.",
                );
                self.push_log("Fifteen minutes gone. You don't feel any better informed.");
            } else if target == "stocks" {
                self.game.advance_time(10.0 / 60.0);
                self.push_log("You check your investments.");
                self.push_log("Numbers went up, then down, then sideways.");
                self.push_log("You close the app. Ten minutes you won't get back.");
            } else if target == "email" {
                self.game.advance_time(5.0 / 60.0);
                self.push_log("You check your work email.");
                self.push_log("Three new messages. None of them urgent. All of them feel urgent.");
                self.push_log("You close it before you start replying.");
            } else {
                self.push_log(&format!(
                    "You look at {}. Nothing notable.",
                    input[6..].trim()
                ));
            }
        } else if lower.starts_with("talk to ") {
            let rest = input[8..].trim();
            let target = rest.to_lowercase();
            let partner_lower = self.game.partner.name.to_lowercase();
            let honest = target.ends_with("(honest)") || target.ends_with(" honest");

            if target.starts_with(&partner_lower)
                || target.starts_with("wife")
                || target.starts_with("partner")
            {
                self.game.advance_time(0.25);
                let lines = wife::talk_response(&self.game, honest);
                for line in &lines {
                    self.push_log(line);
                }
                if honest {
                    self.game.partner_investment += 2;
                } else {
                    self.game.partner_investment += 1;
                }
                self.push_empty_log();
                self.push_log("[1] Keep talking  [2] Back to what you were doing");
                self.game.keep_talking_available = true;
            } else {
                self.push_log(&format!("You talk to {}.", rest));
            }
        } else if lower == "help" || lower == "commands" {
            self.push_log("Available commands:");
            self.push_log("  <number>          - Choose a numbered action");
            self.push_log("  look              - Look around the room");
            self.push_log(
                "  check <target>    - Check on dog, partner, room, time, news, stocks, email",
            );
            self.push_log("  talk to <target>  - Talk to partner (add 'honest' for deeper talk)");
            self.push_log("  remember          - List your memories");
            self.push_log("  remember <number> - Recall a specific memory");
            self.push_log("  help              - Show this list");
        } else if lower == "look" {
            let description = rooms::room_description(&self.game);
            self.push_log(&description);
        } else if lower == "remember" {
            let lines = memory::remember_list(&self.game);
            for line in &lines {
                self.push_log(line);
            }
        } else if lower.starts_with("remember ") {
            let rest = input[9..].trim();
            if let Ok(number) = rest.parse::<usize>() {
                if number >= 1 {
                    self.game.advance_time(0.25);
                    let lines = memory::recall_memory(&mut self.game, number - 1);
                    for line in &lines {
                        self.push_log(line);
                    }
                } else {
                    self.push_log("No such memory.");
                }
            } else {
                self.push_log("Usage: remember <number>");
            }
        } else if let Some(response) = self.try_freeform_action(&lower) {
            for line in &response {
                self.push_log(line);
            }
        } else {
            let suggestion = suggest_command(&lower);
            if let Some(hint) = suggestion {
                self.push_log(&format!(
                    "Unknown command. Did you mean '{}'? Type 'help' for commands.",
                    hint
                ));
            } else {
                self.push_log("Unknown command. Type 'help' for a list of commands.");
            }
        }
    }

    fn try_freeform_action(&mut self, input: &str) -> Option<Vec<String>> {
        let dog_lower = self.game.dog.name.to_lowercase();
        let partner_lower = self.game.partner.name.to_lowercase();

        if input.starts_with("pet ") || input.starts_with("pat ") || input.starts_with("scratch ") {
            let target = input
                .split_whitespace()
                .skip(1)
                .collect::<Vec<_>>()
                .join(" ");
            if target == dog_lower || target == "dog" || target == "the dog" {
                if self.game.act == Act::Grief {
                    return Some(vec![format!("{} isn't here anymore.", self.game.dog.name)]);
                }
                self.game.advance_time(0.25);
                if !self.game.petted_dog_today {
                    self.game.petted_dog_today = true;
                    self.game.bond += 1;
                    return Some(vec![
                        format!(
                            "You reach down and {} {}.",
                            input.split_whitespace().next().unwrap(),
                            self.game.dog.name
                        ),
                        format!("{}'s tail starts going.", self.game.dog.name),
                    ]);
                }
                return Some(vec![
                    format!(
                        "You {} {} again.",
                        input.split_whitespace().next().unwrap(),
                        self.game.dog.name
                    ),
                    format!("{} leans into your hand.", self.game.dog.name),
                ]);
            }
            return None;
        }

        if input == "hug"
            || input.starts_with("hug ")
            || input == "kiss"
            || input.starts_with("kiss ")
        {
            let target = input
                .split_whitespace()
                .skip(1)
                .collect::<Vec<_>>()
                .join(" ");
            if target.is_empty()
                || target == partner_lower
                || target == "wife"
                || target == "partner"
                || target == "her"
            {
                let verb = input.split_whitespace().next().unwrap();
                self.game.advance_time(5.0 / 60.0);
                self.game.partner_investment += 1;
                let partner_name = self.game.partner.name.clone();
                return Some(vec![
                    format!("You {} {}.", verb, partner_name),
                    "She looks surprised. Then she doesn't.".to_string(),
                ]);
            }
            if target == dog_lower || target == "dog" || target == "the dog" {
                if self.game.act == Act::Grief {
                    return Some(vec![format!("{} isn't here anymore.", self.game.dog.name)]);
                }
                self.game.advance_time(5.0 / 60.0);
                self.game.bond += 1;
                return Some(vec![
                    format!("You kneel down and hug {}.", self.game.dog.name),
                    format!("{} squirms a little, then leans in.", self.game.dog.name),
                ]);
            }
            return None;
        }

        if input == "nap" || input == "take a nap" || input == "rest" || input == "lie down" {
            if self.game.energy >= 5 {
                return Some(vec!["You're not tired enough to nap.".to_string()]);
            }
            self.game.advance_time(1.0);
            self.game.energy += 1;
            return Some(vec![
                "You close your eyes for a bit.".to_string(),
                "When you open them, you feel slightly better.".to_string(),
            ]);
        }

        if input == "sit outside"
            || input == "go outside"
            || input == "porch"
            || input == "step outside"
        {
            self.game.advance_time(0.5);
            self.game.peace += 1;
            self.game.current_room = super::state::Room::Porch;
            return Some(vec![
                "You step outside and sit on the porch.".to_string(),
                "The air is fresh. You watch the street for a while.".to_string(),
            ]);
        }

        if input == "read" || input == "read a book" || input == "read book" {
            self.game.advance_time(0.5);
            self.game.peace += 1;
            return Some(vec![
                "You pick up a book. Something you've been meaning to finish.".to_string(),
                "You read a few chapters. The world gets quiet.".to_string(),
            ]);
        }

        if input == "stretch" || input == "exercise" || input == "work out" || input == "workout" {
            self.game.advance_time(0.5);
            self.game.peace += 1;
            return Some(vec![
                "You stretch out. Shoulders, back, legs.".to_string(),
                "Something pops. It feels better afterward.".to_string(),
            ]);
        }

        if input == "water plants" || input == "garden" || input == "tend garden" {
            self.game.advance_time(0.5);
            self.game.home += 1;
            self.game.current_room = super::state::Room::Backyard;
            return Some(vec![
                "You water the plants in the backyard.".to_string(),
                "They seem to be doing alright. Better than you expected.".to_string(),
            ]);
        }

        if input == "sing"
            || input == "listen to music"
            || input == "play music"
            || input == "music"
        {
            self.game.advance_time(0.25);
            self.game.peace += 1;
            return Some(vec![
                "You put something on. Nothing specific. Just noise to fill the quiet.".to_string(),
                "It helps, a little.".to_string(),
            ]);
        }

        if input == "open window" || input == "fresh air" || input == "open a window" {
            return Some(vec![
                "You crack a window open.".to_string(),
                "Cool air drifts in. The curtains shift.".to_string(),
            ]);
        }

        if input == "coffee" || input == "make coffee" || input == "tea" || input == "make tea" {
            self.game.advance_time(0.25);
            let drink = if input.contains("tea") {
                "tea"
            } else {
                "coffee"
            };
            return Some(vec![
                format!("You make yourself a cup of {}.", drink),
                "You hold it with both hands. It's warm.".to_string(),
            ]);
        }

        if input == "clean" || input == "tidy" || input == "tidy up" {
            self.game.advance_time(0.5);
            self.game.home += 1;
            return Some(vec![
                "You straighten a few things. Move a pile from here to there.".to_string(),
                "It's not much, but the room looks a little better.".to_string(),
            ]);
        }

        if input == "stare out window" || input == "look out window" || input == "window" {
            self.game.advance_time(0.25);
            return Some(vec![
                "You stand by the window and watch the street.".to_string(),
                "A car passes. A bird lands on the fence. Nothing happens.".to_string(),
                "It's fine.".to_string(),
            ]);
        }

        if input == "cry" || input == "break down" {
            if self.game.act == Act::Grief {
                self.game.advance_time(0.5);
                self.game.peace += 1;
                return Some(vec![
                    "You let it happen.".to_string(),
                    "It doesn't fix anything. But holding it in was worse.".to_string(),
                ]);
            }
            return Some(vec!["You're not sure why you would.".to_string()]);
        }

        if input == "do nothing" || input == "wait" || input == "sit" {
            self.game.advance_time(0.5);
            return Some(vec![
                "You sit there for a while. Not doing anything.".to_string(),
                "Time passes.".to_string(),
            ]);
        }

        None
    }

    fn execute_event(&mut self, node_index: petgraph::graph::NodeIndex, event_id: &str) {
        let event_label = {
            let event = &self.event_tree.graph[node_index];
            super::events::resolve_template_public(event.label, &self.game)
        };
        self.push_log(&format!("> {}", event_label));
        self.push_empty_log();

        if event_id == "go_to_sleep" && self.game.day >= 20 && self.game.act == Act::Life {
            let death_lines = grief::trigger_act_2(&mut self.game);
            for line in &death_lines {
                self.push_log(line);
            }
            let _ = save::save_game(&self.game);
            self.push_empty_log();
            let wake_lines = rooms::wake_up_text(&self.game);
            for line in &wake_lines {
                self.push_log(line);
            }
            self.needs_redraw = true;
            return;
        }

        if event_id == "go_to_sleep" && self.game.act == Act::Grief {
            let ritual_lines = grief::bed_ritual_text(&self.game);
            for line in &ritual_lines {
                self.push_log(line);
            }
            grief::apply_grief_bed_ritual(&mut self.game);
        }

        let scene_action = event_data::event_id_to_scene_action(event_id);

        let result = self.event_tree.execute_event(node_index, &mut self.game);
        event_data::set_daily_flags(event_id, &mut self.game);

        for message in &result.messages {
            self.push_log(message);
        }

        if self.game.act == super::state::Act::Life
            && let Some(echo) = super::dog::post_action_echo(event_id, &self.game.dog.name)
        {
            self.push_empty_log();
            self.push_log(&echo);
        }

        if event_id == "go_to_sleep" {
            if self.game.act == Act::Grief
                && let Some(spiral_lines) = grief::check_spiral(&mut self.game)
            {
                self.push_empty_log();
                for line in &spiral_lines {
                    self.push_log(line);
                }
            }

            let _ = save::save_game(&self.game);

            self.push_empty_log();
            let wake_lines = rooms::wake_up_text(&self.game);
            for line in &wake_lines {
                self.push_log(line);
            }
            self.check_scenes(None);
        } else if event_id == "wake_up" {
            self.push_empty_log();
            let description = rooms::room_description(&self.game);
            self.push_log(&description);
            self.check_scenes(None);
        } else {
            if self.game.act == Act::Grief
                && let Some(room) = event_data::event_id_to_grief_room(event_id)
                && let Some(break_lines) = grief::chore_break_text(&mut self.game, room)
            {
                self.push_empty_log();
                for line in &break_lines {
                    self.push_log(line);
                }
            }

            self.check_scenes(scene_action);
        }

        self.needs_redraw = true;
    }

    fn simulate_step(&mut self) {
        if self.scene_manager.is_displaying() {
            let finished = self.scene_manager.advance();
            if finished {
                let scene_messages = self.scene_manager.finish_scene(&mut self.game);
                for message in &scene_messages {
                    self.push_log(message);
                }
                self.check_scenes(None);
            }
            self.needs_redraw = true;
            return;
        }

        if self.game.keep_talking_available {
            self.game.keep_talking_available = false;
            let mut rng = rand::rng();
            if rng.random_range(0..2) == 0 {
                self.game.partner_investment += 1;
                self.game.advance_time(0.25);
                self.push_log("(sim) Keep talking.");
            }
            self.needs_redraw = true;
            return;
        }

        let action_list = self.event_tree.available_events(&self.game);
        if action_list.is_empty() {
            return;
        }

        let mut rng = rand::rng();
        let choice = rng.random_range(0..action_list.len());
        let node_index = action_list[choice].node_index;
        let event_id = self.event_tree.graph[node_index].id;
        self.execute_event(node_index, event_id);
    }

    fn render(&mut self) {
        if self.scene_manager.is_displaying() {
            self.render_scene();
            return;
        }
        self.render_gameplay();
    }

    fn render_scene(&mut self) {
        let scene_lines: Vec<String> = self
            .scene_manager
            .current_scene_lines()
            .map(|lines| lines.iter().map(|line| line.to_string()).collect())
            .unwrap_or_default();

        let screen = self.screen.as_mut().expect("screen not initialized");
        screen.clear();

        let screen_width = screen.width;
        let screen_height = screen.height;

        screen.draw_box(
            0,
            0,
            screen_width,
            screen_height,
            TermColor::DarkGrey,
            TermColor::Black,
        );

        let start_row = screen_height / 2 - scene_lines.len() / 2;
        let inner_width = screen_width.saturating_sub(8);

        for (line_index, line) in scene_lines.iter().enumerate() {
            let target_row = start_row + line_index;
            if target_row > 0 && target_row < screen_height - 1 {
                if line.is_empty() {
                    continue;
                }
                let text_len = line.chars().count();
                let column = if text_len < inner_width {
                    (screen_width - text_len) / 2
                } else {
                    4
                };
                screen.write_text(column, target_row, line, TermColor::White, TermColor::Black);
            }
        }
    }

    fn render_gameplay(&mut self) {
        let action_list = self.event_tree.available_events(&self.game);
        let room_desc = rooms::room_description(&self.game);
        let input_snapshot = self.input_buffer.clone();
        let cursor_visible = self.cursor_visible;

        let screen = self.screen.as_mut().expect("screen not initialized");
        screen.clear();

        let screen_width = screen.width;
        let screen_height = screen.height;

        screen.draw_box(
            0,
            0,
            screen_width,
            screen_height,
            TermColor::DarkGrey,
            TermColor::Black,
        );

        hud::render_hud(screen, &self.game);

        if self.game.is_ng_plus {
            let witnessed = self.game.scenes_witnessed.len();
            let total = self.all_scenes.len();
            let remembered_text = format!("Remembered: {}/{}", witnessed, total);
            let remembered_column = screen_width.saturating_sub(remembered_text.len() + 2);
            screen.write_text(
                remembered_column,
                3,
                &remembered_text,
                TermColor::DarkGrey,
                TermColor::Black,
            );
        }

        let hud_bottom = 4;
        screen.draw_horizontal_divider(
            0,
            hud_bottom,
            screen_width,
            TermColor::DarkGrey,
            TermColor::Black,
        );

        let input_area_top = screen_height.saturating_sub(3);

        screen.draw_horizontal_divider(
            0,
            input_area_top,
            screen_width,
            TermColor::DarkGrey,
            TermColor::Black,
        );

        let prompt_row = input_area_top + 1;
        let hint = "type 'help' for commands";
        let hint_column = screen_width.saturating_sub(hint.len() + 2);
        screen.write_text(
            hint_column,
            prompt_row,
            hint,
            TermColor::DarkGrey,
            TermColor::Black,
        );
        screen.write_text(2, prompt_row, "> ", TermColor::White, TermColor::Black);
        screen.write_text(
            4,
            prompt_row,
            &input_snapshot,
            TermColor::White,
            TermColor::Black,
        );
        if cursor_visible {
            let cursor_column = 4 + input_snapshot.len();
            if cursor_column < screen_width - 1 {
                screen.set_cell(
                    cursor_column,
                    prompt_row,
                    '\u{2588}',
                    TermColor::White,
                    TermColor::Black,
                );
            }
        }

        let actions_needed_rows = action_list.len() + 1;
        let action_area_top = input_area_top.saturating_sub(1);
        let action_start_row = action_area_top.saturating_sub(actions_needed_rows);

        screen.draw_horizontal_divider(
            0,
            action_start_row,
            screen_width,
            TermColor::DarkGrey,
            TermColor::Black,
        );

        for (action_index, action) in action_list.iter().enumerate() {
            let action_row = action_start_row + 1 + action_index;
            let action_text = if action.description.is_empty() {
                format!("  {}. {}", action_index + 1, action.label)
            } else {
                format!(
                    "  {}. {}  {}",
                    action_index + 1,
                    action.label,
                    action.description
                )
            };
            screen.write_text(
                1,
                action_row,
                &action_text,
                TermColor::White,
                TermColor::Black,
            );
        }

        let room_row = action_start_row.saturating_sub(3);
        screen.draw_horizontal_divider(
            0,
            room_row,
            screen_width,
            TermColor::DarkGrey,
            TermColor::Black,
        );
        let inner_width = screen_width.saturating_sub(4);
        screen.write_wrapped(
            2,
            room_row + 1,
            &room_desc,
            inner_width,
            TermColor::Grey,
            TermColor::Black,
        );

        let text_area_top = hud_bottom + 1;
        let text_area_bottom = room_row;
        let text_area_height = text_area_bottom.saturating_sub(text_area_top);

        if !self.text_log.is_empty() && text_area_height > 0 {
            let visible_start = if self.text_log.len() > text_area_height {
                self.text_log.len() - text_area_height
            } else {
                0
            };

            let visible_lines = &self.text_log[visible_start..];
            for (line_index, line) in visible_lines.iter().enumerate() {
                let target_row = text_area_top + line_index;
                if target_row < text_area_bottom {
                    screen.write_text(2, target_row, line, TermColor::White, TermColor::Black);
                }
            }
        }
    }
}

impl State for DayLoopState {
    fn title(&self) -> &str {
        if self.game.is_ng_plus {
            "missing_piece"
        } else {
            "missing_peace"
        }
    }

    fn initialize(&mut self, world: &mut World) {
        world.resources.timing.target_fps = 30;
        let columns = world.resources.terminal_size.columns as usize;
        let rows = world.resources.terminal_size.rows as usize;
        self.screen = Some(ScreenBuffer::new(world, columns, rows));

        let wake_lines = rooms::wake_up_text(&self.game);
        for line in wake_lines {
            self.text_log.push(line);
        }

        self.needs_redraw = true;
    }

    fn run_systems(&mut self, world: &mut World) {
        let columns = world.resources.terminal_size.columns as usize;
        let rows = world.resources.terminal_size.rows as usize;

        if let Some(screen) = &self.screen
            && (screen.width != columns || screen.height != rows)
        {
            self.screen.as_mut().unwrap().resize(columns, rows);
            self.needs_redraw = true;
        }

        self.cursor_timer += world.resources.timing.delta_seconds;
        if self.cursor_timer >= 0.5 {
            self.cursor_timer = 0.0;
            self.cursor_visible = !self.cursor_visible;
            self.needs_redraw = true;
        }

        if self.simulate.is_active() {
            self.simulate_timer += world.resources.timing.delta_seconds;
            if self.simulate_timer >= self.simulate_delay() {
                self.simulate_timer = 0.0;
                self.simulate_step();
            }
        }

        if self.needs_redraw {
            let current_vibrance = vibrance::calculate_vibrance(&self.game);
            self.render();
            self.screen.as_ref().unwrap().apply(world, current_vibrance);
            self.needs_redraw = false;
        }
    }

    fn on_keyboard_input(&mut self, world: &mut World, key: KeyCode, pressed: bool) {
        if !pressed {
            return;
        }

        if self.scene_manager.is_displaying() {
            let finished = self.scene_manager.advance();
            if finished {
                let scene_messages = self.scene_manager.finish_scene(&mut self.game);
                for message in &scene_messages {
                    self.push_log(message);
                }
                self.check_scenes(None);
            }
            self.needs_redraw = true;
            return;
        }

        match key {
            KeyCode::Esc => world.resources.should_exit = true,
            KeyCode::Enter => {
                let input = self.input_buffer.clone();
                self.input_buffer.clear();
                self.process_command(&input);
                self.needs_redraw = true;
                self.cursor_visible = true;
                self.cursor_timer = 0.0;
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
                self.needs_redraw = true;
                self.cursor_visible = true;
                self.cursor_timer = 0.0;
            }
            KeyCode::Char(character) => {
                if self.input_buffer.len() < 60 {
                    self.input_buffer.push(character);
                    self.needs_redraw = true;
                    self.cursor_visible = true;
                    self.cursor_timer = 0.0;
                }
            }
            _ => {}
        }
    }

    fn next_state(&mut self, _world: &mut World) -> Option<Box<dyn State>> {
        if !self.scene_manager.is_displaying() && grief::is_resolution_ready(&self.game) {
            return Some(Box::new(ResolutionState::new_with_simulate(
                self.game.clone(),
                self.simulate,
            )));
        }
        None
    }
}

fn suggest_command(input: &str) -> Option<&'static str> {
    let commands = ["look", "check", "talk to", "remember", "help"];
    for command in commands {
        if command.starts_with(input) || input.starts_with(command) {
            return Some(command);
        }
    }
    if input.contains("loo") || input.contains("lok") {
        return Some("look");
    }
    if input.contains("chek") || input.contains("chck") {
        return Some("check");
    }
    if input.contains("tal") || input.contains("spea") {
        return Some("talk to");
    }
    if input.contains("rem") || input.contains("mem") {
        return Some("remember");
    }
    None
}
