use ember::prelude::*;

use super::screen::ScreenBuffer;
use super::state::{GameState, SimulateMode};
use super::transcript;

enum OnboardingPage {
    Page1,
    Page2,
}

struct FormField {
    label: &'static str,
    value: String,
}

pub struct OnboardingState {
    screen: Option<ScreenBuffer>,
    needs_redraw: bool,
    page: OnboardingPage,
    fields: Vec<FormField>,
    active_field: usize,
    cursor_visible: bool,
    cursor_timer: f64,
    transition: bool,
    game_state: GameState,
    ng_plus: bool,
    simulate: SimulateMode,
    simulate_timer: f64,
}

impl OnboardingState {
    pub fn new_with_simulate(simulate: SimulateMode) -> Self {
        Self {
            screen: None,
            needs_redraw: true,
            page: OnboardingPage::Page1,
            fields: Vec::new(),
            active_field: 0,
            cursor_visible: true,
            cursor_timer: 0.0,
            transition: false,
            game_state: GameState::default(),
            ng_plus: false,
            simulate,
            simulate_timer: 0.0,
        }
    }

    pub fn new_ng_plus_with_simulate(previous_game: &GameState, simulate: SimulateMode) -> Self {
        let ng_plus_state = super::resolution::create_ng_plus_state(previous_game);
        Self {
            screen: None,
            needs_redraw: true,
            page: OnboardingPage::Page1,
            fields: Vec::new(),
            active_field: 0,
            cursor_visible: true,
            cursor_timer: 0.0,
            transition: false,
            game_state: ng_plus_state,
            ng_plus: true,
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

    fn current_page_fields(&self) -> std::ops::Range<usize> {
        if self.ng_plus {
            0..5
        } else {
            match self.page {
                OnboardingPage::Page1 => 0..2,
                OnboardingPage::Page2 => 2..5,
            }
        }
    }

    fn render(&mut self) {
        let range = self.current_page_fields();
        let active_field = self.active_field;
        let cursor_visible = self.cursor_visible;

        let screen = self.screen.as_ref().expect("screen not initialized");
        let center_column = screen.width / 2;
        let start_row = screen.height / 2 - 6;

        let screen = self.screen.as_mut().unwrap();
        screen.clear();

        let title = if self.ng_plus {
            "MOVE-IN REGISTRATION"
        } else {
            match self.page {
                OnboardingPage::Page1 => "Resident Information",
                OnboardingPage::Page2 => "A few more things...",
            }
        };

        let field_snapshots: Vec<(String, String, bool)> = range
            .map(|field_index| {
                let field = &self.fields[field_index];
                (
                    field.label.to_string(),
                    field.value.clone(),
                    field_index == active_field,
                )
            })
            .collect();

        screen.write_centered(start_row, title, TermColor::White, TermColor::Black);

        screen.write_centered(
            start_row + 2,
            "\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
            TermColor::DarkGrey,
            TermColor::Black,
        );

        for (relative_index, (label, value, is_active)) in field_snapshots.iter().enumerate() {
            let field_row = start_row + 4 + relative_index * 3;

            let label_color = if *is_active {
                TermColor::White
            } else {
                TermColor::Grey
            };

            let label_column = center_column.saturating_sub(15);
            screen.write_text(
                label_column,
                field_row,
                label,
                label_color,
                TermColor::Black,
            );

            let input_row = field_row + 1;
            let input_width = 30;
            let input_column = center_column.saturating_sub(15);

            for input_index in 0..input_width {
                let column = input_column + input_index;
                screen.set_cell(
                    column,
                    input_row,
                    '\u{2500}',
                    TermColor::DarkGrey,
                    TermColor::Black,
                );
            }

            screen.write_text(
                input_column,
                input_row,
                value,
                TermColor::White,
                TermColor::Black,
            );

            if *is_active && cursor_visible {
                let cursor_column = input_column + value.len();
                screen.set_cell(
                    cursor_column,
                    input_row,
                    '\u{2588}',
                    TermColor::White,
                    TermColor::Black,
                );
            }
        }

        let field_count = field_snapshots.len();
        let hint_row = start_row + 4 + field_count * 3 + 1;
        screen.write_centered(
            hint_row,
            "Tab: next field    Enter: continue",
            TermColor::DarkGrey,
            TermColor::Black,
        );
    }

    fn finalize_fields(&mut self) {
        self.game_state.player.name = self.fields[0].value.trim().to_string();
        self.game_state.partner.name = self.fields[1].value.trim().to_string();
        self.game_state.dog.name = self.fields[2].value.trim().to_string();
        self.game_state.player.favorite_color = self.fields[3].value.trim().to_string();
        self.game_state.player.favorite_food = self.fields[4].value.trim().to_string();

        if self.simulate.is_active() {
            transcript::log(&format!(
                "Player: {}, Partner: {}, Dog: {}, Color: {}, Food: {}",
                self.game_state.player.name,
                self.game_state.partner.name,
                self.game_state.dog.name,
                self.game_state.player.favorite_color,
                self.game_state.player.favorite_food,
            ));
            transcript::log_separator();
        }

        self.transition = true;
    }
}

impl State for OnboardingState {
    fn title(&self) -> &str {
        if self.ng_plus {
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

        let prefill_name = self.game_state.player.name.clone();
        let prefill_partner = self.game_state.partner.name.clone();
        let prefill_dog = self.game_state.dog.name.clone();
        let prefill_color = self.game_state.player.favorite_color.clone();
        let prefill_food = self.game_state.player.favorite_food.clone();

        self.fields = vec![
            FormField {
                label: "Your name",
                value: prefill_name,
            },
            FormField {
                label: "Your partner's name",
                value: prefill_partner,
            },
            FormField {
                label: "Pet name",
                value: prefill_dog,
            },
            FormField {
                label: "Favorite color",
                value: prefill_color,
            },
            FormField {
                label: "Favorite food",
                value: prefill_food,
            },
        ];

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

        if self.simulate.is_active() && !self.fields.is_empty() {
            self.simulate_timer += world.resources.timing.delta_seconds;
            if self.simulate_timer >= self.simulate_delay() {
                self.simulate_timer = 0.0;
                let sim_values = ["Alex", "Sarah", "Buddy", "Blue", "Pizza"];
                for (field_index, field) in self.fields.iter_mut().enumerate() {
                    if field.value.is_empty() {
                        field.value = sim_values[field_index].to_string();
                    }
                }
                self.finalize_fields();
                self.needs_redraw = true;
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

        match key {
            KeyCode::Esc => world.resources.should_exit = true,
            KeyCode::Tab | KeyCode::BackTab => {
                let range = self.current_page_fields();
                if key == KeyCode::BackTab {
                    if self.active_field > range.start {
                        self.active_field -= 1;
                    }
                } else if self.active_field + 1 < range.end {
                    self.active_field += 1;
                }
                self.needs_redraw = true;
                self.cursor_visible = true;
                self.cursor_timer = 0.0;
            }
            KeyCode::Enter => {
                if self.ng_plus {
                    let all_filled = self.fields[0..5]
                        .iter()
                        .all(|field| !field.value.trim().is_empty());
                    if all_filled {
                        self.finalize_fields();
                    }
                } else {
                    match self.page {
                        OnboardingPage::Page1 => {
                            let all_filled = self.fields[0..2]
                                .iter()
                                .all(|field| !field.value.trim().is_empty());
                            if all_filled {
                                self.page = OnboardingPage::Page2;
                                self.active_field = 2;
                                self.needs_redraw = true;
                            }
                        }
                        OnboardingPage::Page2 => {
                            let all_filled = self.fields[2..5]
                                .iter()
                                .all(|field| !field.value.trim().is_empty());
                            if all_filled {
                                self.finalize_fields();
                            }
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                self.fields[self.active_field].value.pop();
                self.needs_redraw = true;
                self.cursor_visible = true;
                self.cursor_timer = 0.0;
            }
            KeyCode::Char(character) => {
                if self.fields[self.active_field].value.len() < 28 {
                    self.fields[self.active_field].value.push(character);
                    self.needs_redraw = true;
                    self.cursor_visible = true;
                    self.cursor_timer = 0.0;
                }
            }
            _ => {}
        }
    }

    fn next_state(&mut self, world: &mut World) -> Option<Box<dyn State>> {
        if self.transition {
            if let Some(screen) = &self.screen {
                screen.despawn(world);
            }
            return Some(Box::new(super::day_loop::DayLoopState::new_with_simulate(
                self.game_state.clone(),
                self.simulate,
            )));
        }
        None
    }
}
