use ember::prelude::*;

use super::onboarding::OnboardingState;
use super::save;
use super::screen::ScreenBuffer;
use super::state::{GameState, GriefPath, SimulateMode};
use super::transcript;

enum ResolutionPhase {
    Intro,
    Body,
    TitleCard,
}

enum ResolutionTransition {
    None,
    Exit,
    NewGamePlus,
}

pub struct ResolutionState {
    game: GameState,
    screen: Option<ScreenBuffer>,
    needs_redraw: bool,
    phase: ResolutionPhase,
    lines: Vec<String>,
    line_index: usize,
    transition: ResolutionTransition,
    simulate: SimulateMode,
    simulate_timer: f64,
}

impl ResolutionState {
    pub fn new_with_simulate(game_state: GameState, simulate: SimulateMode) -> Self {
        let lines = generate_ending_lines(&game_state);
        if simulate.is_active() {
            transcript::log_separator();
            transcript::log("=== RESOLUTION ===");
            let path_name = match game_state.grief_path {
                Some(GriefPath::A) => "Path A",
                Some(GriefPath::B) => "Path B",
                Some(GriefPath::C) => "Path C",
                None => "No path",
            };
            transcript::log(&format!("Grief path: {}", path_name));
            transcript::log(&format!(
                "Final stats - Peace:{} Bond:{} PI:{} Career:{} Home:{} Money:${}",
                game_state.peace,
                game_state.bond,
                game_state.partner_investment,
                game_state.career,
                game_state.home,
                game_state.money,
            ));
            transcript::log_separator();
        }
        Self {
            game: game_state,
            screen: None,
            needs_redraw: true,
            phase: ResolutionPhase::Intro,
            lines,
            line_index: 0,
            transition: ResolutionTransition::None,
            simulate,
            simulate_timer: 0.0,
        }
    }

    fn simulate_delay(&self) -> f64 {
        match self.simulate {
            SimulateMode::Fast => 0.01,
            SimulateMode::Slow => 1.0,
            SimulateMode::Off => 0.0,
        }
    }

    fn render(&mut self) {
        let lines_snapshot: Vec<String> = self.lines.clone();
        let line_index = self.line_index;

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

        match self.phase {
            ResolutionPhase::Intro | ResolutionPhase::Body => {
                let visible_end = (line_index + 1).min(lines_snapshot.len());
                let visible_lines = &lines_snapshot[..visible_end];

                let start_row = screen_height / 2 - visible_lines.len() / 2;

                for (index, line) in visible_lines.iter().enumerate() {
                    let target_row = start_row + index;
                    if target_row > 0 && target_row < screen_height - 1 && !line.is_empty() {
                        screen.write_centered(target_row, line, TermColor::White, TermColor::Black);
                    }
                }
            }
            ResolutionPhase::TitleCard => {
                let center_row = screen_height / 2;

                let title_text = if self.game.is_ng_plus {
                    "m i s s i n g _ p i e c e"
                } else {
                    "m i s s i n g _ p e a c e"
                };
                screen.write_centered(
                    center_row.saturating_sub(3),
                    title_text,
                    TermColor::White,
                    TermColor::Black,
                );

                let tagline = match self.game.grief_path {
                    Some(GriefPath::A) => {
                        "There is no peace to find. There is only peace to notice."
                    }
                    Some(GriefPath::B) => {
                        "There is no peace to find. There is only peace to build. Slowly. Together. Starting late."
                    }
                    Some(GriefPath::C) => {
                        "There is no peace to find. There is only peace to choose. Every day. Again."
                    }
                    None => "",
                };

                screen.write_centered(center_row, tagline, TermColor::Grey, TermColor::Black);

                screen.write_centered(
                    center_row + 3,
                    "Thank you for playing.",
                    TermColor::DarkGrey,
                    TermColor::Black,
                );

                screen.write_centered(
                    center_row + 5,
                    "[Press N for New Game+]",
                    TermColor::Grey,
                    TermColor::Black,
                );

                screen.write_centered(
                    center_row + 7,
                    "[Press any other key to exit]",
                    TermColor::DarkGrey,
                    TermColor::Black,
                );
            }
        }
    }
}

impl State for ResolutionState {
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

        if self.simulate.is_active() {
            self.simulate_timer += world.resources.timing.delta_seconds;
            if self.simulate_timer >= self.simulate_delay() {
                self.simulate_timer = 0.0;
                match self.phase {
                    ResolutionPhase::Intro | ResolutionPhase::Body => {
                        if self.line_index < self.lines.len() {
                            let line = &self.lines[self.line_index];
                            if !line.is_empty() {
                                transcript::log(line);
                            }
                        }
                        self.line_index += 1;
                        if self.line_index >= self.lines.len() {
                            self.phase = ResolutionPhase::TitleCard;
                        } else {
                            self.phase = ResolutionPhase::Body;
                        }
                        self.needs_redraw = true;
                    }
                    ResolutionPhase::TitleCard => {
                        transcript::log_separator();
                        transcript::log("=== SIMULATION END ===");
                        save::delete_save();
                        self.transition = ResolutionTransition::Exit;
                    }
                }
            }
        }

        let resolution_vibrance = match self.game.grief_path {
            Some(GriefPath::A) => 0.7,
            Some(GriefPath::B) => 0.5,
            Some(GriefPath::C) => 0.6,
            None => 0.5,
        };

        if self.needs_redraw {
            self.render();
            self.screen
                .as_ref()
                .unwrap()
                .apply(world, resolution_vibrance);
            self.needs_redraw = false;
        }
    }

    fn on_keyboard_input(&mut self, _world: &mut World, key: KeyCode, pressed: bool) {
        if !pressed {
            return;
        }

        match self.phase {
            ResolutionPhase::Intro | ResolutionPhase::Body => {
                self.line_index += 1;
                if self.line_index >= self.lines.len() {
                    self.phase = ResolutionPhase::TitleCard;
                } else {
                    self.phase = ResolutionPhase::Body;
                }
                self.needs_redraw = true;
            }
            ResolutionPhase::TitleCard => match key {
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    self.transition = ResolutionTransition::NewGamePlus;
                }
                _ => {
                    save::delete_save();
                    self.transition = ResolutionTransition::Exit;
                }
            },
        }
    }

    fn next_state(&mut self, world: &mut World) -> Option<Box<dyn State>> {
        match self.transition {
            ResolutionTransition::Exit => {
                world.resources.should_exit = true;
                None
            }
            ResolutionTransition::NewGamePlus => {
                if let Some(screen) = &self.screen {
                    screen.despawn(world);
                }
                Some(Box::new(OnboardingState::new_ng_plus_with_simulate(
                    &self.game,
                    self.simulate,
                )))
            }
            ResolutionTransition::None => None,
        }
    }
}

pub fn create_ng_plus_state(previous_game: &GameState) -> GameState {
    GameState {
        is_ng_plus: true,
        peace: 60,
        player: previous_game.player.clone(),
        partner: previous_game.partner.clone(),
        dog: super::state::DogInfo {
            name: previous_game.dog.name.clone(),
            personality: previous_game.dog.personality,
        },
        memories: previous_game
            .memories
            .iter()
            .map(|memory| super::state::Memory {
                id: memory.id.clone(),
                description: memory.description.clone(),
                firsthand: memory.firsthand,
                revisited: false,
            })
            .collect(),
        previous_grief_path: previous_game.grief_path,
        echo_scenes_available: true,
        ..Default::default()
    }
}

fn generate_ending_lines(game: &GameState) -> Vec<String> {
    let dog_name = &game.dog.name;
    let partner_name = &game.partner.name;

    match game.grief_path {
        Some(GriefPath::A) => vec![
            String::new(),
            format!("It's been a while since {} passed.", dog_name),
            String::new(),
            "The house doesn't feel empty anymore.".to_string(),
            "It feels... different. Quieter.".to_string(),
            "But not empty.".to_string(),
            String::new(),
            format!("{} planted something in the garden.", partner_name),
            "Where the tennis ball used to be.".to_string(),
            "You helped dig the hole.".to_string(),
            String::new(),
            "Some mornings you still reach for the foot of the bed.".to_string(),
            "But you smile now, instead of flinching.".to_string(),
            String::new(),
            format!(
                "\"Do you remember the sock?\" {} asks one night.",
                partner_name
            ),
            String::new(),
            "You laugh. Actually laugh.".to_string(),
            String::new(),
            format!("\"Yeah,\" you say. \"I remember the sock.\""),
            String::new(),
            format!("{} was here.", dog_name),
            format!("{} was loved.", dog_name),
            "That's enough.".to_string(),
        ],
        Some(GriefPath::B) => vec![
            String::new(),
            format!("It's been a while since {} passed.", dog_name),
            String::new(),
            "The hardest part was learning to come home.".to_string(),
            "To an empty hallway. No scrambling paws.".to_string(),
            String::new(),
            format!("{} tried to help. You know that now.", partner_name),
            "At the time, you couldn't see it.".to_string(),
            String::new(),
            "One morning you woke up and the first thing you thought".to_string(),
            format!("wasn't about {}.", dog_name),
            "And that scared you.".to_string(),
            String::new(),
            "But then you realized — that's what healing is.".to_string(),
            "Not forgetting. Just... making room.".to_string(),
            String::new(),
            format!("You still talk to {} sometimes.", dog_name),
            "In your head. When no one's listening.".to_string(),
            String::new(),
            "You're still here.".to_string(),
            "That counts for something.".to_string(),
        ],
        Some(GriefPath::C) => vec![
            String::new(),
            format!("It's been a while since {} passed.", dog_name),
            String::new(),
            "Some days are okay. Some days aren't.".to_string(),
            "That's the thing they don't tell you about grief —".to_string(),
            "it doesn't move in a straight line.".to_string(),
            String::new(),
            format!(
                "{} is there. You're getting better at letting her be.",
                partner_name
            ),
            "She's getting better at giving you space.".to_string(),
            String::new(),
            "It's uneven. But it's honest.".to_string(),
            String::new(),
            "One day you'll tell her everything.".to_string(),
            "About the sock. The stairs. The way he looked at you.".to_string(),
            String::new(),
            "Not today. But soon.".to_string(),
            String::new(),
            format!("{} was here. And you were changed by it.", dog_name),
            "That's the whole story.".to_string(),
        ],
        None => vec!["Something went wrong.".to_string()],
    }
}
