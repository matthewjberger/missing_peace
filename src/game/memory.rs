use super::state::{GameState, GriefPath};

pub fn remember_list(game: &GameState) -> Vec<String> {
    if game.memories.is_empty() {
        return vec!["You have no memories to recall.".to_string()];
    }

    let mut lines = Vec::new();
    lines.push("Memories:".to_string());
    lines.push(String::new());

    for (index, memory) in game.memories.iter().enumerate() {
        let indicator = if memory.firsthand { "+" } else { "-" };
        let revisited_mark = if memory.revisited { " (revisited)" } else { "" };
        lines.push(format!(
            "  {}. [{}] {}{}",
            index + 1,
            indicator,
            memory.description,
            revisited_mark
        ));
    }

    lines.push(String::new());
    lines.push("Type 'remember <number>' to recall a memory.".to_string());
    lines
}

pub fn recall_memory(game: &mut GameState, index: usize) -> Vec<String> {
    if index >= game.memories.len() {
        return vec!["No such memory.".to_string()];
    }

    let memory = &game.memories[index];
    let description = memory.description.clone();
    let firsthand = memory.firsthand;
    let already_revisited = memory.revisited;

    let mut lines = Vec::new();
    lines.push(String::new());
    lines.push(format!("You remember: {}", description));
    lines.push(String::new());

    if firsthand {
        if already_revisited {
            lines
                .push("The memory is warm, but familiar now. It doesn't hit the same.".to_string());
        } else {
            lines.push("The memory is vivid. You were there.".to_string());
            lines.push("+1 Peace.".to_string());
            game.peace += 1;
        }
    } else if already_revisited {
        lines.push("You've been here before. The sting has faded.".to_string());
    } else {
        lines.push("You heard about this. You weren't there.".to_string());
        match game.grief_path {
            Some(GriefPath::A) => {
                lines.push("It stings, but less than you expected.".to_string());
            }
            Some(GriefPath::B) | Some(GriefPath::C) => {
                lines.push("It hurts in a different way.".to_string());
                lines.push("-1 Peace.".to_string());
                game.peace = game.peace.saturating_sub(1);
            }
            None => {
                lines.push("It hurts in a different way.".to_string());
                lines.push("-1 Peace.".to_string());
                game.peace = game.peace.saturating_sub(1);
            }
        }
    }

    game.memories[index].revisited = true;

    lines
}
