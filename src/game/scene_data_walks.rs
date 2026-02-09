use super::scenes::{SceneDef, SceneEffect, SceneTrigger};

pub fn walk_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "walk_morning_mist",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(3, 7),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "Morning mist on the street.",
                "Everything is soft. Muted.",
                "He trots ahead, nose to the ground.",
                "Your breath makes small clouds.",
                "The world feels like it's still waking up.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "walk_other_dog",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(4, 9),
                SceneTrigger::BondAbove(8),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "Another dog across the street.",
                "He freezes. Tail up. Alert.",
                "The other dog freezes too.",
                "They stare at each other for an eternity.",
                "Then both owners pull them along.",
                "He looks back three times.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1)],
        },
        SceneDef {
            id: "walk_puddle",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(3, 10),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "He finds a puddle.",
                "Walks through it. Turns around. Walks through it again.",
                "Stands in the middle of it and looks at you.",
                "\"Really?\"",
                "He wags his tail. Splashes.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "walk_sunset",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(7, 13),
                SceneTrigger::BondAbove(15),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "The sunset tonight is ridiculous.",
                "Orange and pink and purple. Like a painting.",
                "You stop walking. He stops too.",
                "You stand there together watching the sky change.",
                "He probably doesn't care about the sunset.",
                "But he cares that you stopped. So he stays.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(2),
                SceneEffect::Peace(2),
                SceneEffect::UnlockMemory(
                    "sunset_walk".to_string(),
                    "Watching the sunset together on a walk".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "walk_neighbor_chat",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(5, 12),
                SceneTrigger::BondAbove(10),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "A neighbor stops you. \"What a handsome boy!\"",
                "He soaks up the attention. Tail going.",
                "\"How old is he?\"",
                "You realize you don't know exactly. It doesn't matter.",
                "\"He's perfect,\" you say.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "walk_autumn_leaves",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(11, 16),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "The leaves are changing.",
                "He tries to catch one as it falls. Misses.",
                "Tries again. Misses again.",
                "Gets one. Spits it out immediately.",
                "Tries to catch another.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "walk_long_route",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(8, 15),
                SceneTrigger::BondAbove(20),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "You take the long route today.",
                "Past the park. Down by the creek.",
                "He drinks from the water. Gets his whole face wet.",
                "You sit on a bench and watch the light on the water.",
                "No rush. Nowhere to be.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "walk_partner_morning",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(5, 11),
                SceneTrigger::PartnerInvestmentAbove(12),
                SceneTrigger::ActionTaken("walk_partner"),
            ]),
            lines: vec![
                "She matches your pace.",
                "Neither of you talks for the first few minutes.",
                "Then she starts. About her week. Her plans.",
                "You listen. The morning air is crisp.",
                "It's nice to just... walk.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "walk_partner_evening",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(9, 16),
                SceneTrigger::PartnerInvestmentAbove(18),
                SceneTrigger::ActionTaken("walk_partner"),
            ]),
            lines: vec![
                "Evening walk. Just the two of you.",
                "Streetlights coming on. Air cooling down.",
                "She takes your arm.",
                "\"We should do this more often,\" she says.",
                "You agree. You mean it.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(3), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "walk_found_stick",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(2, 8),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "He found a stick.",
                "Not just any stick. The stick.",
                "It's three times his body length.",
                "He carries it proudly. Hits a mailbox.",
                "Doesn't care. Keeps going.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1), SceneEffect::Peace(1)],
        },
    ]
}

pub fn combined_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "combined_couch_evening",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(7, 13),
                SceneTrigger::BondAbove(15),
                SceneTrigger::PartnerInvestmentAbove(12),
            ]),
            lines: vec![
                "Evening. The three of you on the couch.",
                "Well \u{2014} two of you on the couch. He's on the floor.",
                "But pressed against both of your legs.",
                "She has her feet tucked under a blanket.",
                "Nobody is talking. The TV is on but nobody's watching.",
                "This is the good part.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(2),
                SceneEffect::PartnerInvestment(2),
                SceneEffect::Peace(3),
                SceneEffect::UnlockMemory(
                    "couch_evening".to_string(),
                    "The evening on the couch, all three of you".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "combined_walk_three",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(5, 12),
                SceneTrigger::BondAbove(12),
                SceneTrigger::PartnerInvestmentAbove(10),
                SceneTrigger::ActionTaken("walk_partner"),
            ]),
            lines: vec![
                "The three of you walk together.",
                "He's ahead, pulling a little. She's beside you.",
                "Your hands brush. She takes yours.",
                "He stops to sniff something. You wait.",
                "Nobody's in a rush.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(2),
                SceneEffect::PartnerInvestment(3),
                SceneEffect::UnlockMemory(
                    "walk_together".to_string(),
                    "Walking together, the three of you".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "combined_morning_chaos",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(4, 9),
                SceneTrigger::BondAbove(8),
                SceneTrigger::PartnerInvestmentAbove(6),
            ]),
            lines: vec![
                "Morning rush. She's looking for her keys.",
                "You're making coffee. He's underfoot.",
                "She trips over him. Doesn't fall.",
                "\"Why is he always right there?\"",
                "\"Because he loves you.\"",
                "\"He loves being in the way.\"",
                "But she scratches his ears on the way out.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(1),
                SceneEffect::PartnerInvestment(1),
                SceneEffect::Peace(1),
            ],
        },
        SceneDef {
            id: "combined_lazy_sunday",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(8, 15),
                SceneTrigger::BondAbove(18),
                SceneTrigger::PartnerInvestmentAbove(16),
                SceneTrigger::PeaceAbove(35),
            ]),
            lines: vec![
                "Nobody has any plans today.",
                "She reads. You sit. He sleeps.",
                "At some point you all end up in the living room.",
                "The afternoon stretches out like honey.",
                "You think: I could live in this moment.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(2),
                SceneEffect::PartnerInvestment(2),
                SceneEffect::Peace(3),
                SceneEffect::UnlockMemory(
                    "lazy_sunday".to_string(),
                    "That lazy afternoon when nobody moved".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "combined_backyard_play",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(5, 12),
                SceneTrigger::BondAbove(12),
                SceneTrigger::PartnerInvestmentAbove(10),
            ]),
            lines: vec![
                "She throws the ball. He brings it back.",
                "She throws it again. He brings it back.",
                "She throws it again. He keeps it.",
                "\"Hey! Give it!\"",
                "He starts running. She chases him.",
                "You watch from the porch, laughing.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(2),
                SceneEffect::PartnerInvestment(2),
                SceneEffect::Peace(2),
            ],
        },
        SceneDef {
            id: "combined_bedtime_pile",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(11, 17),
                SceneTrigger::BondAbove(25),
                SceneTrigger::PartnerInvestmentAbove(20),
            ]),
            lines: vec![
                "Bedtime.",
                "She's on the left. You're on the right.",
                "He's in the middle. Taking up most of the bed.",
                "\"This is ridiculous,\" she whispers.",
                "\"Completely,\" you agree.",
                "Nobody moves him.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(2),
                SceneEffect::PartnerInvestment(2),
                SceneEffect::Peace(2),
                SceneEffect::UnlockMemory(
                    "bedtime_pile".to_string(),
                    "All three of you, piled in bed".to_string(),
                    true,
                ),
            ],
        },
    ]
}
