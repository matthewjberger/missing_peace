use ember::prelude::*;

use super::day_loop::DayLoopState;
use super::onboarding::OnboardingState;
use super::save;
use super::screen::ScreenBuffer;
use super::state::SimulateMode;
use super::transcript;

enum IntroTransition {
    None,
    NewGame,
    Continue,
}

enum IntroPhase {
    Preamble,
    Title,
}

pub struct IntroState {
    screen: Option<ScreenBuffer>,
    needs_redraw: bool,
    transition: IntroTransition,
    has_save: bool,
    phase: IntroPhase,
    simulate: SimulateMode,
    simulate_timer: f64,
}

impl IntroState {
    pub fn new(simulate: SimulateMode) -> Self {
        if simulate.is_active() {
            transcript::init();
            transcript::log("=== SIMULATION START ===");
        }
        Self {
            screen: None,
            needs_redraw: true,
            transition: IntroTransition::None,
            has_save: save::has_save(),
            phase: IntroPhase::Preamble,
            simulate,
            simulate_timer: 0.0,
        }
    }

    fn simulate_delay(&self) -> f64 {
        match self.simulate {
            SimulateMode::Fast => 0.01,
            SimulateMode::Slow => 2.0,
            SimulateMode::Off => 0.0,
        }
    }

    fn render(&mut self) {
        let screen = self.screen.as_mut().expect("screen not initialized");
        screen.clear();

        match self.phase {
            IntroPhase::Preamble => self.render_preamble(),
            IntroPhase::Title => self.render_title(),
        }
    }

    fn render_preamble(&mut self) {
        let screen = self.screen.as_mut().expect("screen not initialized");
        let center_row = screen.height / 2;

        screen.write_centered(
            center_row.saturating_sub(3),
            "You just moved into a new house.",
            TermColor::Grey,
            TermColor::Black,
        );
        screen.write_centered(
            center_row.saturating_sub(2),
            "New job. New start. Things have been hectic.",
            TermColor::Grey,
            TermColor::Black,
        );
        screen.write_centered(
            center_row,
            "All you want is some peace and quiet.",
            TermColor::Grey,
            TermColor::Black,
        );
        screen.write_centered(
            center_row + 1,
            "Time to get things under control.",
            TermColor::Grey,
            TermColor::Black,
        );
        screen.write_centered(
            center_row + 4,
            "Press any key",
            TermColor::DarkGrey,
            TermColor::Black,
        );
    }

    fn render_title(&mut self) {
        let screen = self.screen.as_mut().expect("screen not initialized");
        let center_row = screen.height / 2;

        screen.write_centered(
            center_row.saturating_sub(4),
            "m i s s i n g _ p e a c e",
            TermColor::White,
            TermColor::Black,
        );

        screen.write_centered(
            center_row.saturating_sub(1),
            "\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
            TermColor::DarkGrey,
            TermColor::Black,
        );

        if self.has_save {
            screen.write_centered(
                center_row + 2,
                "Press Enter to continue",
                TermColor::Grey,
                TermColor::Black,
            );
            screen.write_centered(
                center_row + 4,
                "Press N for a new game",
                TermColor::DarkGrey,
                TermColor::Black,
            );
        } else {
            screen.write_centered(
                center_row + 2,
                "Press any key to begin",
                TermColor::Grey,
                TermColor::Black,
            );
        }
    }
}

impl State for IntroState {
    fn title(&self) -> &str {
        "missing_peace"
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
                    IntroPhase::Preamble => {
                        self.phase = IntroPhase::Title;
                        self.needs_redraw = true;
                    }
                    IntroPhase::Title => {
                        if self.has_save {
                            save::delete_save();
                        }
                        self.transition = IntroTransition::NewGame;
                    }
                }
            }
        }

        if self.needs_redraw {
            self.render();
            self.screen.as_ref().unwrap().apply(world, 0.15);
            self.needs_redraw = false;
        }
    }

    fn on_keyboard_input(&mut self, world: &mut World, key: KeyCode, pressed: bool) {
        if !pressed {
            return;
        }

        if matches!(key, KeyCode::Esc) {
            world.resources.should_exit = true;
            return;
        }

        match self.phase {
            IntroPhase::Preamble => {
                self.phase = IntroPhase::Title;
                self.needs_redraw = true;
            }
            IntroPhase::Title => match key {
                KeyCode::Char('n') | KeyCode::Char('N') if self.has_save => {
                    save::delete_save();
                    self.transition = IntroTransition::NewGame;
                }
                KeyCode::Enter if self.has_save => {
                    self.transition = IntroTransition::Continue;
                }
                _ if !self.has_save => {
                    self.transition = IntroTransition::NewGame;
                }
                _ => {}
            },
        }
    }

    fn next_state(&mut self, world: &mut World) -> Option<Box<dyn State>> {
        match &self.transition {
            IntroTransition::NewGame => {
                if let Some(screen) = &self.screen {
                    screen.despawn(world);
                }
                Some(Box::new(OnboardingState::new_with_simulate(self.simulate)))
            }
            IntroTransition::Continue => {
                if let Some(screen) = &self.screen {
                    screen.despawn(world);
                }
                if let Some(game_state) = save::load_game() {
                    Some(Box::new(DayLoopState::new_with_simulate(
                        game_state,
                        self.simulate,
                    )))
                } else {
                    Some(Box::new(OnboardingState::new_with_simulate(self.simulate)))
                }
            }
            IntroTransition::None => None,
        }
    }
}
