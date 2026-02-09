use super::state::{Act, DogPersonality, GameState, GriefPath, WifeMood};

#[derive(Debug, Clone)]
pub enum SceneTrigger {
    Day(u32),
    DayRange(u32, u32),
    BondAbove(u32),
    PartnerInvestmentAbove(u32),
    ActionTaken(&'static str),
    CareerAbove(u32),
    ActIs(Act),
    GriefPathIs(GriefPath),
    PeaceAbove(u32),
    PeaceBelow(u32),
    HomeAbove(u32),
    DogPersonalityIs(DogPersonality),
    WifeMoodIs(WifeMood),
    PartnerMilestone(u32),
    IsNgPlus,
    Compound(Vec<SceneTrigger>),
}

#[derive(Debug, Clone)]
pub enum SceneEffect {
    Bond(i32),
    PartnerInvestment(i32),
    Peace(i32),
    Career(i32),
    Home(i32),
    UnlockMemory(String, String, bool),
}

#[derive(Debug, Clone)]
pub struct SceneDef {
    pub id: &'static str,
    pub trigger: SceneTrigger,
    pub lines: Vec<&'static str>,
    pub effects: Vec<SceneEffect>,
}

pub struct SceneManager {
    pending_scene: Option<SceneDef>,
    scene_line_index: usize,
    displaying_scene: bool,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            pending_scene: None,
            scene_line_index: 0,
            displaying_scene: false,
        }
    }

    pub fn is_displaying(&self) -> bool {
        self.displaying_scene
    }

    pub fn queue_scene(&mut self, scene: SceneDef) {
        self.pending_scene = Some(scene);
        self.scene_line_index = 0;
        self.displaying_scene = true;
    }

    pub fn current_scene_lines(&self) -> Option<&[&'static str]> {
        self.pending_scene
            .as_ref()
            .map(|scene| scene.lines.as_slice())
    }

    pub fn advance(&mut self) -> bool {
        if let Some(scene) = &self.pending_scene {
            self.scene_line_index += 1;
            if self.scene_line_index >= scene.lines.len() {
                return true;
            }
        }
        false
    }

    pub fn finish_scene(&mut self, game: &mut GameState) -> Vec<String> {
        let mut result_messages = Vec::new();

        if let Some(scene) = self.pending_scene.take() {
            game.scenes_witnessed.insert(scene.id.to_string());

            for effect in &scene.effects {
                match effect {
                    SceneEffect::Bond(amount) => {
                        if *amount >= 0 {
                            game.bond += *amount as u32;
                        } else {
                            game.bond = game.bond.saturating_sub(amount.unsigned_abs());
                        }
                    }
                    SceneEffect::PartnerInvestment(amount) => {
                        if *amount >= 0 {
                            game.partner_investment += *amount as u32;
                        } else {
                            game.partner_investment = game
                                .partner_investment
                                .saturating_sub(amount.unsigned_abs());
                        }
                    }
                    SceneEffect::Peace(amount) => {
                        if *amount >= 0 {
                            game.peace += *amount as u32;
                        } else {
                            game.peace = game.peace.saturating_sub(amount.unsigned_abs());
                        }
                    }
                    SceneEffect::Career(amount) => {
                        if *amount >= 0 {
                            game.career += *amount as u32;
                        } else {
                            game.career = game.career.saturating_sub(amount.unsigned_abs());
                        }
                    }
                    SceneEffect::Home(amount) => {
                        if *amount >= 0 {
                            game.home += *amount as u32;
                        } else {
                            game.home = game.home.saturating_sub(amount.unsigned_abs());
                        }
                    }
                    SceneEffect::UnlockMemory(id, description, firsthand) => {
                        let already_has = game.memories.iter().any(|memory| memory.id == *id);
                        if !already_has {
                            game.memories.push(super::state::Memory {
                                id: id.clone(),
                                description: description.clone(),
                                firsthand: *firsthand,
                                revisited: false,
                            });
                            result_messages.push(format!("[Memory unlocked: {}]", description));
                        }
                    }
                }
            }
        }

        self.displaying_scene = false;
        self.scene_line_index = 0;
        result_messages
    }
}

pub fn evaluate_trigger(
    trigger: &SceneTrigger,
    game: &GameState,
    action_taken: Option<&str>,
) -> bool {
    match trigger {
        SceneTrigger::Day(day) => game.day == *day,
        SceneTrigger::DayRange(start, end) => game.day >= *start && game.day <= *end,
        SceneTrigger::BondAbove(threshold) => game.bond > *threshold,
        SceneTrigger::PartnerInvestmentAbove(threshold) => game.partner_investment > *threshold,
        SceneTrigger::ActionTaken(action) => action_taken.is_some_and(|taken| taken == *action),
        SceneTrigger::CareerAbove(threshold) => game.career > *threshold,
        SceneTrigger::ActIs(act) => game.act == *act,
        SceneTrigger::GriefPathIs(path) => game.grief_path == Some(*path),
        SceneTrigger::PeaceAbove(threshold) => game.peace > *threshold,
        SceneTrigger::PeaceBelow(threshold) => game.peace < *threshold,
        SceneTrigger::HomeAbove(threshold) => game.home > *threshold,
        SceneTrigger::DogPersonalityIs(personality) => game.dog.personality == *personality,
        SceneTrigger::WifeMoodIs(mood) => game.wife_mood == *mood,
        SceneTrigger::PartnerMilestone(threshold) => game.partner_milestone_reached >= *threshold,
        SceneTrigger::IsNgPlus => game.is_ng_plus && game.echo_scenes_available,
        SceneTrigger::Compound(triggers) => triggers
            .iter()
            .all(|sub_trigger| evaluate_trigger(sub_trigger, game, action_taken)),
    }
}

pub fn check_for_scenes(
    game: &GameState,
    all_scenes: &[SceneDef],
    action_taken: Option<&str>,
) -> Option<SceneDef> {
    for scene in all_scenes {
        if game.scenes_witnessed.contains(scene.id) {
            continue;
        }
        if evaluate_trigger(&scene.trigger, game, action_taken) {
            return Some(scene.clone());
        }
    }
    None
}
