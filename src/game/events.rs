use std::collections::HashMap;

use petgraph::Direction;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;

use super::state::{Act, DayPhase, GameState, Room};

#[derive(Default)]
pub struct EventCondition {
    pub required_act: Option<Act>,
    pub required_phase: Option<DayPhase>,
    pub min_time: Option<f64>,
    pub max_time: Option<f64>,
    pub min_energy: Option<u32>,
    pub min_money: Option<i32>,
    pub requires_not_fired: bool,
    pub requires_dog_not_fed: bool,
    pub requires_dog_not_walked: bool,
    pub requires_bedtime: bool,
    pub custom_check: Option<fn(&GameState) -> bool>,
}

pub struct EventEffect {
    pub career: i32,
    pub home: i32,
    pub peace: i32,
    pub bond: i32,
    pub partner_investment: i32,
    pub money: i32,
    pub energy: i32,
    pub time_hours: f64,
    pub set_phase: Option<DayPhase>,
    pub set_room: Option<Room>,
    pub set_dog_fed: bool,
    pub set_dog_walked: bool,
    pub custom_effect: Option<fn(&mut GameState) -> Vec<String>>,
}

impl Default for EventEffect {
    fn default() -> Self {
        Self {
            career: 0,
            home: 0,
            peace: 0,
            bond: 0,
            partner_investment: 0,
            money: 0,
            energy: 0,
            time_hours: 0.0,
            set_phase: None,
            set_room: None,
            set_dog_fed: false,
            set_dog_walked: false,
            custom_effect: None,
        }
    }
}

pub struct EventDef {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub message: &'static str,
    pub condition: EventCondition,
    pub effect: EventEffect,
    pub cooldown: u32,
    pub dynamic_description: Option<fn(&GameState) -> String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Unlocks,
    Replaces,
    MutuallyExclusive,
}

pub struct EventTree {
    pub graph: DiGraph<EventDef, EdgeKind>,
    pub event_indices: HashMap<&'static str, NodeIndex>,
}

pub struct ActionEntry {
    pub node_index: NodeIndex,
    pub label: String,
    pub description: String,
}

pub struct ActionResult {
    pub messages: Vec<String>,
}

impl EventTree {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            event_indices: HashMap::new(),
        }
    }

    pub fn add_event(&mut self, event: EventDef) -> NodeIndex {
        let id = event.id;
        let index = self.graph.add_node(event);
        self.event_indices.insert(id, index);
        index
    }

    pub fn available_events(&self, game: &GameState) -> Vec<ActionEntry> {
        let mut entries = Vec::new();

        for node_index in self.graph.node_indices() {
            let event = &self.graph[node_index];

            if !self.check_condition(event, game) {
                continue;
            }

            if !self.check_prerequisites(node_index, game) {
                continue;
            }

            if self.is_replaced(node_index, game) {
                continue;
            }

            if self.is_excluded(node_index, game) {
                continue;
            }

            if !self.check_cooldown(event, game) {
                continue;
            }

            let label = resolve_template(event.label, game);
            let description = if let Some(dynamic_desc) = event.dynamic_description {
                dynamic_desc(game)
            } else {
                resolve_template(event.description, game)
            };

            entries.push(ActionEntry {
                node_index,
                label,
                description,
            });
        }

        entries
    }

    pub fn execute_event(&self, node_index: NodeIndex, game: &mut GameState) -> ActionResult {
        let event = &self.graph[node_index];
        let mut messages = Vec::new();

        if event.effect.career != 0 {
            if event.effect.career > 0 {
                game.career += event.effect.career as u32;
            } else {
                game.career = game
                    .career
                    .saturating_sub(event.effect.career.unsigned_abs());
            }
        }
        if event.effect.home != 0 {
            if event.effect.home > 0 {
                game.home += event.effect.home as u32;
            } else {
                game.home = game.home.saturating_sub(event.effect.home.unsigned_abs());
            }
        }
        if event.effect.peace != 0 {
            if event.effect.peace > 0 && game.act != Act::Grief {
                game.peace += event.effect.peace as u32;
            } else if event.effect.peace < 0 {
                game.peace = game.peace.saturating_sub(event.effect.peace.unsigned_abs());
            }
        }
        if event.effect.bond != 0 {
            if event.effect.bond > 0 {
                game.bond += event.effect.bond as u32;
            } else {
                game.bond = game.bond.saturating_sub(event.effect.bond.unsigned_abs());
            }
        }
        if event.effect.partner_investment != 0 {
            if event.effect.partner_investment > 0 {
                game.partner_investment += event.effect.partner_investment as u32;
            } else {
                game.partner_investment = game
                    .partner_investment
                    .saturating_sub(event.effect.partner_investment.unsigned_abs());
            }
        }
        if event.effect.money != 0 {
            game.money += event.effect.money;
        }
        if event.effect.energy != 0 {
            if event.effect.energy > 0 {
                game.energy += event.effect.energy as u32;
            } else {
                game.energy = game
                    .energy
                    .saturating_sub(event.effect.energy.unsigned_abs());
            }
        }
        if event.effect.time_hours != 0.0 {
            game.advance_time(event.effect.time_hours);
        }
        if let Some(phase) = event.effect.set_phase {
            game.day_phase = phase;
        }
        if let Some(room) = event.effect.set_room {
            game.current_room = room;
        }
        if event.effect.set_dog_fed {
            game.dog_fed_today = true;
        }
        if event.effect.set_dog_walked {
            game.dog_walked_today = true;
        }

        let message = resolve_template(event.message, game);
        if !message.is_empty() {
            messages.push(message);
        }

        if let Some(custom_effect) = event.effect.custom_effect {
            let custom_messages = custom_effect(game);
            messages.extend(custom_messages);
        }

        if game.act == Act::Life && event.effect.time_hours > 0.0 {
            let passive_bond = (event.effect.time_hours * 0.5) as u32;
            if passive_bond > 0 {
                game.bond += passive_bond;
            }
        }

        game.recent_actions.push(event.id.to_string());
        if game.recent_actions.len() > 20 {
            game.recent_actions.remove(0);
        }

        ActionResult { messages }
    }

    fn check_condition(&self, event: &EventDef, game: &GameState) -> bool {
        if let Some(required_act) = event.condition.required_act
            && game.act != required_act
        {
            return false;
        }
        if let Some(required_phase) = event.condition.required_phase
            && game.day_phase != required_phase
        {
            return false;
        }
        if let Some(min_time) = event.condition.min_time
            && game.current_time < min_time
        {
            return false;
        }
        if let Some(max_time) = event.condition.max_time
            && game.current_time >= max_time
        {
            return false;
        }
        if let Some(min_energy) = event.condition.min_energy
            && game.energy < min_energy
        {
            return false;
        }
        if let Some(min_money) = event.condition.min_money
            && game.money < min_money
        {
            return false;
        }
        if event.condition.requires_not_fired && game.fired {
            return false;
        }
        if event.condition.requires_dog_not_fed && game.dog_fed_today {
            return false;
        }
        if event.condition.requires_dog_not_walked && game.dog_walked_today {
            return false;
        }
        if event.condition.requires_bedtime && !game.is_bedtime() {
            return false;
        }
        if game.is_bedtime()
            && !event.condition.requires_bedtime
            && event.condition.required_phase == Some(DayPhase::FreeTime)
        {
            return false;
        }
        if let Some(custom_check) = event.condition.custom_check
            && !custom_check(game)
        {
            return false;
        }
        true
    }

    fn check_prerequisites(&self, node_index: NodeIndex, game: &GameState) -> bool {
        for edge in self.graph.edges_directed(node_index, Direction::Incoming) {
            if *edge.weight() == EdgeKind::Unlocks {
                let source_event = &self.graph[edge.source()];
                if !game.recent_actions.contains(&source_event.id.to_string()) {
                    return false;
                }
            }
        }
        true
    }

    fn is_replaced(&self, node_index: NodeIndex, game: &GameState) -> bool {
        for edge in self.graph.edges_directed(node_index, Direction::Outgoing) {
            if *edge.weight() == EdgeKind::Replaces {
                let target_event = &self.graph[edge.target()];
                if self.check_condition(target_event, game) {
                    return true;
                }
            }
        }
        false
    }

    fn is_excluded(&self, node_index: NodeIndex, game: &GameState) -> bool {
        for edge in self.graph.edges_directed(node_index, Direction::Outgoing) {
            if *edge.weight() == EdgeKind::MutuallyExclusive {
                let target_event = &self.graph[edge.target()];
                if game.recent_actions.contains(&target_event.id.to_string()) {
                    return true;
                }
            }
        }
        for edge in self.graph.edges_directed(node_index, Direction::Incoming) {
            if *edge.weight() == EdgeKind::MutuallyExclusive {
                let source_event = &self.graph[edge.source()];
                if game.recent_actions.contains(&source_event.id.to_string()) {
                    return true;
                }
            }
        }
        false
    }

    fn check_cooldown(&self, event: &EventDef, game: &GameState) -> bool {
        if event.cooldown == 0 {
            return true;
        }

        let last_occurrence = game
            .recent_actions
            .iter()
            .rposition(|action| action == event.id);

        match last_occurrence {
            Some(position) => {
                let actions_since = game.recent_actions.len() - 1 - position;
                actions_since >= event.cooldown as usize
            }
            None => true,
        }
    }
}

fn resolve_template(template: &str, game: &GameState) -> String {
    let mut result = template
        .replace("{dog}", &game.dog.name)
        .replace("{partner}", &game.partner.name)
        .replace("{player}", &game.player.name)
        .replace("{occupation}", &game.player.occupation);

    if game.is_ng_plus {
        result = result.replace("[OPTIONAL]: ", "");
        result = result.replace("[OPTIONAL] ", "");
    }

    result
}

pub fn resolve_template_public(template: &str, game: &GameState) -> String {
    resolve_template(template, game)
}
