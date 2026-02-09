use super::scene_data_ambient;
use super::scene_data_dog;
use super::scene_data_grief;
use super::scene_data_walks;
use super::scene_data_wife;
use super::scenes::SceneDef;

pub fn all_scenes() -> Vec<SceneDef> {
    let mut scenes = Vec::new();
    scenes.extend(scene_data_ambient::staple_scenes());
    scenes.extend(scene_data_dog::dog_discovery_scenes());
    scenes.extend(scene_data_dog::dog_companion_scenes());
    scenes.extend(scene_data_dog::dog_bond_scenes());
    scenes.extend(scene_data_dog::dog_elder_scenes());
    scenes.extend(scene_data_dog::dog_personality_scenes());
    scenes.extend(scene_data_wife::wife_routine_scenes());
    scenes.extend(scene_data_wife::wife_dog_adjacent_scenes());
    scenes.extend(scene_data_wife::wife_late_game_scenes());
    scenes.extend(scene_data_wife::wife_milestone_scenes());
    scenes.extend(scene_data_walks::walk_scenes());
    scenes.extend(scene_data_walks::combined_scenes());
    scenes.extend(scene_data_ambient::interrupt_scenes());
    scenes.extend(scene_data_ambient::ambient_scenes());
    scenes.extend(scene_data_ambient::ng_plus_echo_scenes());
    scenes.extend(scene_data_grief::grief_wife_dialogue_scenes());
    scenes.extend(scene_data_grief::grief_honest_conversation_scenes());
    scenes.extend(scene_data_grief::grief_chore_break_vignette_scenes());
    scenes.extend(scene_data_grief::grief_breakthrough_scenes());
    scenes.extend(scene_data_grief::grief_unique_scenes());
    scenes
}
