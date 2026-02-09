use super::scenes::{SceneDef, SceneEffect, SceneTrigger};
use super::state::{Act, GriefPath};

pub fn grief_wife_dialogue_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "grief_wife_morning_1",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(21, 22),
            ]),
            lines: vec![
                "She's in the kitchen. Quiet.",
                "She made coffee for two. Habit.",
                "\"How are you?\" she asks.",
                "You shrug.",
                "She nods. Doesn't push.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(1)],
        },
        SceneDef {
            id: "grief_wife_water_bowl",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(21, 23),
            ]),
            lines: vec![
                "She picked up the water bowl.",
                "Put it in the cupboard where you can't see it.",
                "She didn't ask. Just did it.",
                "You're not sure if you're grateful or angry.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(1)],
        },
        SceneDef {
            id: "grief_wife_tries_talk",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(22, 24),
                SceneTrigger::PartnerInvestmentAbove(15),
            ]),
            lines: vec![
                "\"Do you want to talk about it?\"",
                "\"No.\"",
                "\"Okay.\"",
                "She sits next to you anyway.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(1)],
        },
        SceneDef {
            id: "grief_wife_dinner",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(22, 24),
            ]),
            lines: vec![
                "She made dinner. Your favorite.",
                "You eat it. It tastes like nothing.",
                "\"Thank you,\" you say.",
                "She squeezes your hand.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(1), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "grief_wife_crying",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(23, 25),
                SceneTrigger::PartnerInvestmentAbove(20),
            ]),
            lines: vec![
                "You hear her crying in the bathroom.",
                "She's trying to be quiet about it.",
                "You knock. \"Hey.\"",
                "A pause. Water running.",
                "\"I'm fine.\"",
                "She's not fine. Neither are you.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2)],
        },
        SceneDef {
            id: "grief_wife_leash",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(22, 24),
            ]),
            lines: vec![
                "The leash is still by the door.",
                "She hasn't moved it. You haven't moved it.",
                "Neither of you mentions it.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(-1)],
        },
        SceneDef {
            id: "grief_wife_patience",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(24, 26),
                SceneTrigger::PartnerInvestmentAbove(22),
            ]),
            lines: vec![
                "You snapped at her. Over nothing.",
                "She didn't snap back.",
                "\"It's okay,\" she said.",
                "It's not okay. But she means the snapping. Not the rest.",
                "Later you apologize. She says she knows.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "grief_wife_routine",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(24, 27),
            ]),
            lines: vec![
                "She's keeping the house running.",
                "Groceries. Dishes. Laundry.",
                "You notice because you haven't been doing any of it.",
                "The guilt is quiet. But it's there.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Home(2)],
        },
        SceneDef {
            id: "grief_wife_hand",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(25, 28),
                SceneTrigger::PartnerInvestmentAbove(25),
            ]),
            lines: vec![
                "You're sitting on the porch. Staring at the yard.",
                "She comes out. Sits next to you.",
                "Puts her hand on your knee.",
                "Doesn't say anything.",
                "You put your hand on hers.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "grief_wife_photo",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(25, 28),
                SceneTrigger::PeaceAbove(10),
            ]),
            lines: vec![
                "She printed a photo.",
                "Him in the sunbeam. That one.",
                "She put it on the mantle without asking.",
                "You look at it every morning now.",
                "It hurts. But the good kind.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Peace(2),
                SceneEffect::UnlockMemory(
                    "grief_photo".to_string(),
                    "The photo she put up after he was gone".to_string(),
                    false,
                ),
            ],
        },
    ]
}

pub fn grief_honest_conversation_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "grief_honest_first",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(23, 26),
                SceneTrigger::GriefPathIs(GriefPath::B),
                SceneTrigger::PeaceAbove(5),
            ]),
            lines: vec![
                "\"I miss him,\" you say.",
                "It's the first time you've said it out loud.",
                "She doesn't say anything for a moment.",
                "\"I miss him too.\"",
                "Something loosens in your chest.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(3), SceneEffect::PartnerInvestment(3)],
        },
        SceneDef {
            id: "grief_honest_angry",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(24, 27),
                SceneTrigger::GriefPathIs(GriefPath::B),
                SceneTrigger::PeaceAbove(8),
            ]),
            lines: vec![
                "\"I'm angry,\" you tell her.",
                "\"At what?\"",
                "\"Everything. Nothing. I don't know.\"",
                "\"That's okay. You're allowed to be angry.\"",
                "\"It doesn't feel okay.\"",
                "\"It will.\"",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(3), SceneEffect::PartnerInvestment(2)],
        },
        SceneDef {
            id: "grief_honest_guilt",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(25, 28),
                SceneTrigger::GriefPathIs(GriefPath::C),
                SceneTrigger::PeaceAbove(8),
            ]),
            lines: vec![
                "\"I feel like I should have done more,\" you say.",
                "\"More what?\"",
                "\"I don't know. More walks. More time. More... everything.\"",
                "\"You gave him a good life.\"",
                "\"How do you know?\"",
                "\"Because he was happy. You could see it.\"",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(3), SceneEffect::PartnerInvestment(3)],
        },
        SceneDef {
            id: "grief_honest_distance",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(24, 27),
                SceneTrigger::GriefPathIs(GriefPath::C),
                SceneTrigger::PeaceAbove(6),
            ]),
            lines: vec![
                "\"I know I've been distant,\" you say.",
                "She looks surprised.",
                "\"I didn't know how to... I didn't know what to do.\"",
                "\"You don't have to know. You just have to be here.\"",
                "\"I'm here.\"",
                "\"I know.\"",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(3), SceneEffect::PartnerInvestment(3)],
        },
        SceneDef {
            id: "grief_honest_memories",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(26, 29),
                SceneTrigger::PeaceAbove(14),
            ]),
            lines: vec![
                "You start telling her about the sock.",
                "The one he left on your pillow.",
                "She laughs. Then she's crying.",
                "Then you're both crying and laughing at the same time.",
                "It feels terrible and wonderful.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Peace(4),
                SceneEffect::PartnerInvestment(3),
                SceneEffect::UnlockMemory(
                    "shared_grief".to_string(),
                    "Crying and laughing together about the sock".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "grief_honest_thank_you",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(27, 30),
                SceneTrigger::PeaceAbove(17),
                SceneTrigger::PartnerInvestmentAbove(28),
            ]),
            lines: vec![
                "\"Thank you,\" you say.",
                "\"For what?\"",
                "\"For being patient with me. Through all of this.\"",
                "She's quiet for a moment.",
                "\"That's what this is,\" she says. \"That's what we are.\"",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(4), SceneEffect::PartnerInvestment(4)],
        },
    ]
}

pub fn grief_chore_break_vignette_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "grief_vignette_collar",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(22, 24),
            ]),
            lines: vec![
                "You find his collar in the laundry basket.",
                "It must have fallen off the hook.",
                "You hold it for a while.",
                "Put it in the drawer. The quiet drawer.",
                "The one with things you'll look at someday.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(-1)],
        },
        SceneDef {
            id: "grief_vignette_food",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(21, 23),
            ]),
            lines: vec![
                "The bag of dog food is still in the pantry.",
                "Half full.",
                "You should throw it away.",
                "You close the pantry.",
                "",
                "[Press any key]",
            ],
            effects: vec![],
        },
        SceneDef {
            id: "grief_vignette_sound",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(22, 25),
            ]),
            lines: vec![
                "You hear a sound in the hallway.",
                "Nails on hardwood.",
                "Your heart jumps.",
                "It's the radiator. Just the radiator.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(-1)],
        },
        SceneDef {
            id: "grief_vignette_bed_indent",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(22, 24),
            ]),
            lines: vec![
                "The indent at the foot of the bed is fading.",
                "You didn't notice until now.",
                "You press your hand into it.",
                "It springs back. Gone.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(-1)],
        },
        SceneDef {
            id: "grief_vignette_neighbor",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(23, 26),
            ]),
            lines: vec![
                "The neighbor asks how the dog is.",
                "You say he passed.",
                "\"Oh. I'm sorry. He was a good dog.\"",
                "\"Yeah. He was.\"",
                "You go inside before they can say anything else.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(-1)],
        },
        SceneDef {
            id: "grief_vignette_habit",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(23, 26),
            ]),
            lines: vec![
                "You almost called his name.",
                "It was in your mouth. Ready.",
                "Then you remembered.",
                "The house is very quiet.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(-1)],
        },
        SceneDef {
            id: "grief_vignette_window",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(24, 27),
            ]),
            lines: vec![
                "The sunbeam by the window is empty.",
                "It's there every afternoon. Right on schedule.",
                "Warming a spot nobody lies in anymore.",
                "",
                "[Press any key]",
            ],
            effects: vec![],
        },
        SceneDef {
            id: "grief_vignette_dream",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(24, 27),
                SceneTrigger::PeaceAbove(5),
            ]),
            lines: vec![
                "You dreamed about him last night.",
                "He was running in a field. Younger. Faster.",
                "He looked back at you, once.",
                "Then kept running.",
                "You woke up and the pillow was wet.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Peace(1),
                SceneEffect::UnlockMemory(
                    "grief_dream".to_string(),
                    "The dream where he was running".to_string(),
                    true,
                ),
            ],
        },
    ]
}

pub fn grief_breakthrough_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "grief_breakthrough_10",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::PeaceAbove(8),
            ]),
            lines: vec![
                "You ate a full meal today.",
                "Didn't think about it. Just ate.",
                "Afterward you realized: that's the first time in a while.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "grief_breakthrough_20",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::PeaceAbove(14),
            ]),
            lines: vec![
                "You caught yourself humming.",
                "An old song. You don't even know where it came from.",
                "You stopped when you noticed.",
                "But you didn't feel guilty about it.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "grief_breakthrough_30",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::PeaceAbove(20),
            ]),
            lines: vec![
                "You slept through the night.",
                "No waking at 3 AM. No staring at the ceiling.",
                "When you woke up, the sun was out.",
                "You lay there for a minute, just breathing.",
                "It felt okay. Not good. But okay.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(2)],
        },
    ]
}

pub fn grief_unique_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "grief_garden_start",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(25, 28),
                SceneTrigger::PeaceAbove(14),
                SceneTrigger::PartnerInvestmentAbove(20),
            ]),
            lines: vec![
                "She's in the garden.",
                "On her knees. Digging.",
                "\"What are you doing?\"",
                "\"Planting something.\"",
                "\"What?\"",
                "\"Something that comes back every year.\"",
                "You kneel down next to her. She hands you a trowel.",
                "You dig together.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Peace(3),
                SceneEffect::PartnerInvestment(3),
                SceneEffect::Home(2),
                SceneEffect::UnlockMemory(
                    "garden_planting".to_string(),
                    "Planting something together in the garden".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "grief_first_laugh",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::DayRange(24, 28),
                SceneTrigger::PeaceAbove(10),
            ]),
            lines: vec![
                "She tells a joke. A bad one.",
                "You laugh before you can stop yourself.",
                "It surprises both of you.",
                "She smiles. You can see the relief in her eyes.",
                "\"There you are,\" she says quietly.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(3), SceneEffect::PartnerInvestment(2)],
        },
        SceneDef {
            id: "grief_resolution_walk",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::PeaceAbove(20),
                SceneTrigger::PartnerInvestmentAbove(25),
            ]),
            lines: vec![
                "\"Want to go for a walk?\" she asks.",
                "You haven't walked since he died.",
                "\"Yeah,\" you say. \"I think I do.\"",
                "The street is the same. The route is the same.",
                "But it's different without the pull of the leash.",
                "She takes your hand.",
                "\"We'll walk this way sometimes,\" she says.",
                "\"For him.\"",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Peace(4),
                SceneEffect::PartnerInvestment(3),
                SceneEffect::UnlockMemory(
                    "resolution_walk".to_string(),
                    "The first walk after, with her".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "grief_garden_sprout",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::PeaceAbove(22),
            ]),
            lines: vec![
                "The garden. Something's happening.",
                "A green shoot. Barely visible.",
                "She sees it too. Kneels down.",
                "\"Look at that.\"",
                "You look.",
                "It's small. But it's alive.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(3), SceneEffect::PartnerInvestment(2)],
        },
        SceneDef {
            id: "grief_new_morning",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::ActIs(Act::Grief),
                SceneTrigger::PeaceAbove(24),
            ]),
            lines: vec![
                "You wake up.",
                "The sun is out. The house is quiet.",
                "But it's a different quiet now.",
                "Not empty. Just... still.",
                "You make coffee. Look at the photo on the mantle.",
                "\"Morning, buddy,\" you say.",
                "And then you start your day.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Peace(3),
                SceneEffect::UnlockMemory(
                    "new_morning".to_string(),
                    "The morning you said good morning to his photo".to_string(),
                    true,
                ),
            ],
        },
    ]
}
