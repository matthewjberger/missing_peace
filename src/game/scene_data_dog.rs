use super::scenes::{SceneDef, SceneEffect, SceneTrigger};
use super::state::DogPersonality;

pub fn dog_discovery_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "dog_first_walk",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(1, 3),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "This is your first real walk together.",
                "He pulls the leash. Stops at every tree.",
                "Sniffs everything twice.",
                "You're not sure who's walking who.",
                "But it's nice. The fresh air is nice.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(2),
                SceneEffect::UnlockMemory(
                    "first_walk".to_string(),
                    "Your first walk together".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "dog_settled_in",
            trigger: SceneTrigger::Compound(vec![SceneTrigger::Day(3), SceneTrigger::BondAbove(5)]),
            lines: vec![
                "He's found his spot.",
                "That specific patch of sunlight by the window.",
                "He lies there every afternoon now.",
                "Chin on paws. Watching the street.",
                "This is his house now too.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1)],
        },
        SceneDef {
            id: "dog_learns_name",
            trigger: SceneTrigger::BondAbove(10),
            lines: vec![
                "You say his name from the other room.",
                "You hear the scramble of paws.",
                "He appears in the doorway, head tilted.",
                "He knows his name now.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2)],
        },
        SceneDef {
            id: "dog_toy_destroyed",
            trigger: SceneTrigger::DayRange(5, 9),
            lines: vec![
                "You come home to find the remains of a chew toy.",
                "Cotton everywhere. He's sitting in the middle of the carnage.",
                "Looking extremely guilty.",
                "You try to be stern but it's hard.",
                "That face.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "dog_first_trick",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(1, 4),
                SceneTrigger::BondAbove(3),
            ]),
            lines: vec![
                "You hold up a treat. \"Sit.\"",
                "He stares at you.",
                "\"Sit.\"",
                "He lies down instead.",
                "Close enough.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "dog_follows_room",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(2, 5),
                SceneTrigger::BondAbove(6),
            ]),
            lines: vec![
                "You get up to go to the kitchen.",
                "He follows.",
                "You go back to the living room.",
                "He follows.",
                "You stand in the hallway just to see what happens.",
                "He sits at your feet and looks up.",
                "You have a shadow now.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2)],
        },
        SceneDef {
            id: "dog_first_belly",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(3, 7),
                SceneTrigger::BondAbove(8),
            ]),
            lines: vec![
                "He rolls over.",
                "Just like that. Right in the middle of the living room.",
                "Paws in the air. Tail wagging.",
                "He trusts you.",
                "You scratch his belly and he makes a sound",
                "that is somewhere between a groan and a sigh.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(2),
                SceneEffect::Peace(1),
                SceneEffect::UnlockMemory(
                    "first_belly".to_string(),
                    "The first time he rolled over for you".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "dog_shoe_thief",
            trigger: SceneTrigger::DayRange(3, 8),
            lines: vec![
                "Your shoe is missing.",
                "Just one of them. The left one.",
                "You find it under the couch.",
                "He's watching you from across the room,",
                "tail going, clearly proud of his work.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1)],
        },
    ]
}

pub fn dog_companion_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "dog_morning_routine",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(5, 11),
                SceneTrigger::BondAbove(14),
            ]),
            lines: vec![
                "He has a routine now.",
                "Wake up. Stretch. Walk to the door.",
                "Wait for you to notice.",
                "When you get up, he does a little spin.",
                "Every morning. Same spin.",
                "It makes you smile every time.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "dog_rain_refusal",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(7, 12),
                SceneTrigger::BondAbove(16),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "It's raining.",
                "You open the door. He looks outside.",
                "Looks at you.",
                "Looks outside again.",
                "Goes back to his bed.",
                "You can't even be mad.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "dog_couch_claim",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(7, 13),
                SceneTrigger::BondAbove(18),
            ]),
            lines: vec![
                "You come back from the kitchen.",
                "He's on the couch. Your spot. Dead center.",
                "He opens one eye. Doesn't move.",
                "You sit on the floor next to him.",
                "This is your life now.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "dog_porch_watch",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(5, 12),
                SceneTrigger::BondAbove(15),
            ]),
            lines: vec![
                "You sit on the porch. He sits next to you.",
                "A squirrel crosses the yard.",
                "His whole body tenses. Ears forward.",
                "Then he looks at you, like he's asking permission.",
                "\"Not today, buddy.\"",
                "He sighs dramatically.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2)],
        },
        SceneDef {
            id: "dog_bath_resistance",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(8, 13),
                SceneTrigger::BondAbove(20),
            ]),
            lines: vec![
                "Bath day.",
                "He knows. He can tell from the way you looked at him.",
                "He's behind the couch.",
                "You spend fifteen minutes coaxing him out.",
                "Afterward, he does the zoomies for ten minutes straight.",
                "Crashes into a chair. Doesn't care.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(1),
                SceneEffect::Peace(1),
                SceneEffect::UnlockMemory(
                    "bath_day".to_string(),
                    "The bath day zoomies".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "dog_leash_excitement",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(4, 10),
                SceneTrigger::BondAbove(12),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "You pick up the leash.",
                "The effect is instant. Total chaos.",
                "Jumping. Spinning. Barking.",
                "You can't even clip it on because he won't hold still.",
                "\"Sit. SIT.\"",
                "He sits for exactly one second.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2)],
        },
        SceneDef {
            id: "dog_head_on_lap",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(8, 15),
                SceneTrigger::BondAbove(22),
            ]),
            lines: vec![
                "You're sitting on the couch. Reading. Not paying attention.",
                "Then you feel it. The weight.",
                "He's put his head on your lap.",
                "Just resting it there. Eyes half-closed.",
                "Your hand finds the top of his head automatically.",
                "You stop reading.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(3),
                SceneEffect::Peace(2),
                SceneEffect::UnlockMemory(
                    "head_on_lap".to_string(),
                    "His head on your lap, eyes half-closed".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "dog_sleeping_positions",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(7, 13),
                SceneTrigger::BondAbove(18),
            ]),
            lines: vec![
                "He sleeps in the strangest positions.",
                "Tonight it's upside down. All four legs in the air.",
                "Tongue slightly out. Completely unconscious.",
                "You take a mental picture.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "dog_garden_dig",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(9, 15),
                SceneTrigger::BondAbove(20),
            ]),
            lines: vec![
                "He's been in the garden.",
                "You can tell because there's a hole.",
                "And he has dirt on his nose.",
                "He looks at you with an expression that says",
                "he has no idea how that hole got there.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1), SceneEffect::Home(-1)],
        },
        SceneDef {
            id: "dog_tv_buddy",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(5, 12),
                SceneTrigger::BondAbove(16),
            ]),
            lines: vec![
                "A dog appears on TV.",
                "His head snaps up. Ears forward.",
                "He walks to the screen. Sniffs it.",
                "The TV dog barks. He barks back.",
                "You turn the volume up, just for him.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1), SceneEffect::Peace(1)],
        },
    ]
}

pub fn dog_bond_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "dog_knows_schedule",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(11, 16),
                SceneTrigger::BondAbove(30),
            ]),
            lines: vec![
                "He knows your schedule now.",
                "Waits by the door five minutes before you usually come home.",
                "If you're early, he's surprised. If you're late, he worries.",
                "You can see it in his face when you walk in.",
                "The relief.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(3), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "dog_separation_whine",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(9, 15),
                SceneTrigger::BondAbove(28),
            ]),
            lines: vec![
                "You grab your keys. He hears the jingle.",
                "The whine starts immediately.",
                "Low. Continuous. Absolutely devastating.",
                "\"I'll be back soon.\"",
                "He doesn't believe you. He never believes you.",
                "But you always come back.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(2),
                SceneEffect::UnlockMemory(
                    "separation_whine".to_string(),
                    "The sound he made when you left".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "dog_storm_comfort",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(11, 17),
                SceneTrigger::BondAbove(32),
            ]),
            lines: vec![
                "Thunder. He's under the bed.",
                "You lie on the floor and reach under.",
                "\"Hey. It's okay.\"",
                "He army-crawls toward your hand.",
                "You stay on the floor for the whole storm.",
                "Your back hurts the next day. Worth it.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(4),
                SceneEffect::Peace(1),
                SceneEffect::UnlockMemory(
                    "storm_floor".to_string(),
                    "Lying on the floor during the storm".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "dog_your_person",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(12, 17),
                SceneTrigger::BondAbove(36),
            ]),
            lines: vec![
                "Friends are over. He's friendly with everyone.",
                "But when it's time to settle down,",
                "he crosses the room and lies at your feet.",
                "Not hers. Not theirs. Yours.",
                "You're his person.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(3), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "dog_morning_face",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(12, 17),
                SceneTrigger::BondAbove(38),
            ]),
            lines: vec![
                "You wake up and his face is six inches from yours.",
                "Staring.",
                "Tail starts the moment your eyes open.",
                "\"Morning.\"",
                "The tail goes faster.",
                "You can't help it. You laugh.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(2),
                SceneEffect::Peace(2),
                SceneEffect::UnlockMemory(
                    "morning_face".to_string(),
                    "Waking up to his face six inches away".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "dog_silent_understanding",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(13, 19),
                SceneTrigger::BondAbove(40),
            ]),
            lines: vec![
                "Bad day.",
                "You don't say anything. Don't have to.",
                "He comes over. Lies against you.",
                "Puts his chin on your thigh.",
                "Doesn't ask for anything. Doesn't need anything.",
                "Just stays.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(4),
                SceneEffect::Peace(3),
                SceneEffect::UnlockMemory(
                    "silent_understanding".to_string(),
                    "The way he knew when you needed him".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "dog_perfect_day",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(13, 19),
                SceneTrigger::BondAbove(42),
                SceneTrigger::PeaceAbove(40),
            ]),
            lines: vec![
                "Walk in the morning. Work. Come home.",
                "He's at the door. Dinner. Couch. Bed.",
                "Nothing special happened today.",
                "But lying in bed, listening to him breathe,",
                "you think: this is what a good life sounds like.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(3),
                SceneEffect::Peace(3),
                SceneEffect::UnlockMemory(
                    "perfect_day".to_string(),
                    "That ordinary, perfect day".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "dog_lean_in",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(11, 16),
                SceneTrigger::BondAbove(34),
            ]),
            lines: vec![
                "You're standing in the kitchen.",
                "He comes up and leans his full weight against your leg.",
                "Not asking for food. Not asking for a walk.",
                "Just leaning.",
                "It's the dog equivalent of a hug.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(3), SceneEffect::Peace(1)],
        },
    ]
}

pub fn dog_elder_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "dog_slowing_down",
            trigger: SceneTrigger::DayRange(11, 13),
            lines: vec![
                "He's slower on the stairs now.",
                "Takes them one at a time. Pauses at the top.",
                "He used to bound up them.",
                "You pretend not to notice.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(2),
                SceneEffect::UnlockMemory(
                    "slowing_down".to_string(),
                    "The way he slowed down on the stairs".to_string(),
                    false,
                ),
            ],
        },
        SceneDef {
            id: "dog_food_slower",
            trigger: SceneTrigger::DayRange(11, 13),
            lines: vec![
                "He eats slower now.",
                "Used to inhale his food in thirty seconds flat.",
                "Now he chews. Pauses. Chews again.",
                "You sit on the floor next to him while he eats.",
                "He looks up at you between bites.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2)],
        },
        SceneDef {
            id: "dog_grey_muzzle",
            trigger: SceneTrigger::DayRange(13, 16),
            lines: vec![
                "You notice it for the first time today.",
                "Grey around his muzzle. When did that happen?",
                "He yawns. His teeth aren't what they were either.",
                "You scratch behind his ears the way he likes.",
                "He leans into your hand.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(3),
                SceneEffect::UnlockMemory(
                    "grey_muzzle".to_string(),
                    "The grey around his muzzle".to_string(),
                    false,
                ),
            ],
        },
        SceneDef {
            id: "dog_stairs_help",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(13, 15),
                SceneTrigger::BondAbove(15),
            ]),
            lines: vec![
                "He stops at the bottom of the stairs.",
                "Looks up at them. Then at you.",
                "You pick him up and carry him.",
                "He's heavier than you expected.",
                "Lighter than he used to be.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(3),
                SceneEffect::UnlockMemory(
                    "stairs_help".to_string(),
                    "Carrying him up the stairs".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "dog_short_walk",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(15, 18),
                SceneTrigger::BondAbove(20),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "The walks are shorter now.",
                "He stops more often. Looks around.",
                "Taking it all in.",
                "You used to rush these. Not anymore.",
                "You match his pace. There's no hurry.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(3),
                SceneEffect::Peace(1),
                SceneEffect::UnlockMemory(
                    "short_walks".to_string(),
                    "When the walks got shorter".to_string(),
                    false,
                ),
            ],
        },
        SceneDef {
            id: "dog_favorite_toy_ignored",
            trigger: SceneTrigger::DayRange(15, 17),
            lines: vec![
                "You toss his favorite toy across the room.",
                "He watches it land.",
                "Doesn't get up.",
                "Just rests his chin on his paws and sighs.",
                "You pick the toy up and put it next to him.",
                "He rests his chin on it instead.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2), SceneEffect::Peace(-1)],
        },
        SceneDef {
            id: "dog_sunbeam_sleep",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(15, 18),
                SceneTrigger::BondAbove(25),
            ]),
            lines: vec![
                "He found the sunbeam again.",
                "The one by the window. His favorite.",
                "He lies there for hours now.",
                "Sometimes you sit next to him.",
                "Just to be close.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(3), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "dog_deep_breath",
            trigger: SceneTrigger::DayRange(17, 18),
            lines: vec![
                "He takes a deep breath. Lets it out slowly.",
                "The kind of sigh that comes from somewhere deep.",
                "You put your hand on his chest.",
                "Feel it rise. Fall. Rise. Fall.",
                "You match your breathing to his.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(3),
                SceneEffect::Peace(-1),
                SceneEffect::UnlockMemory(
                    "deep_breath".to_string(),
                    "Matching your breathing to his".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "dog_last_walk",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(17, 19),
                SceneTrigger::BondAbove(20),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "He can barely make it to the corner.",
                "But he wanted to go. You could tell.",
                "He stands on the sidewalk and lifts his nose to the wind.",
                "Eyes half-closed. Tail moves once.",
                "You stand there with him for a long time.",
                "The neighborhood has never been so quiet.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(5),
                SceneEffect::Peace(1),
                SceneEffect::UnlockMemory(
                    "last_walk".to_string(),
                    "His last walk, nose to the wind".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "dog_last_night",
            trigger: SceneTrigger::Day(19),
            lines: vec![
                "He's sleeping at the foot of the bed.",
                "Breathing slow. Steady.",
                "You reach down and rest your hand on his side.",
                "His tail moves once. Just once.",
                "He knows you're there.",
                "You keep your hand there for a long time.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(5),
                SceneEffect::UnlockMemory(
                    "last_night".to_string(),
                    "The last night, your hand on his side".to_string(),
                    true,
                ),
            ],
        },
    ]
}

pub fn dog_personality_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "personality_playful_zoomies",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DogPersonalityIs(DogPersonality::Playful),
                SceneTrigger::DayRange(3, 8),
                SceneTrigger::BondAbove(8),
            ]),
            lines: vec![
                "Something snaps in him. Some invisible trigger.",
                "He takes off. Full sprint. Living room to kitchen and back.",
                "Slides on the hardwood. Crashes into a chair leg.",
                "Gets up. Does it again.",
                "You watch, helpless, laughing.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "personality_playful_fetch_obsession",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DogPersonalityIs(DogPersonality::Playful),
                SceneTrigger::DayRange(5, 12),
                SceneTrigger::BondAbove(15),
            ]),
            lines: vec![
                "He drops the ball at your feet.",
                "You throw it. He brings it back.",
                "You throw it again. He brings it back.",
                "This has been going on for twenty minutes.",
                "You stop throwing. He stares at you.",
                "The guilt is unbearable. You throw it again.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "personality_playful_wake_up_bounce",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DogPersonalityIs(DogPersonality::Playful),
                SceneTrigger::DayRange(8, 15),
                SceneTrigger::BondAbove(22),
            ]),
            lines: vec![
                "Your alarm goes off.",
                "Before you can even reach for it, he's on the bed.",
                "Bouncing. Licking your face. Tail destroying the pillows.",
                "\"I'm UP. I'm up.\"",
                "He does a victory lap around the bedroom.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "personality_gentle_quiet_comfort",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DogPersonalityIs(DogPersonality::Gentle),
                SceneTrigger::DayRange(3, 8),
                SceneTrigger::BondAbove(8),
            ]),
            lines: vec![
                "You're sitting in the living room. Thinking about nothing.",
                "He walks over slowly. Sits next to you.",
                "Leans his weight against your leg.",
                "Doesn't ask for anything. Just sits.",
                "The weight of him is comforting.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "personality_gentle_paw_on_hand",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DogPersonalityIs(DogPersonality::Gentle),
                SceneTrigger::DayRange(5, 12),
                SceneTrigger::BondAbove(15),
            ]),
            lines: vec![
                "You're at the table, staring at your hands.",
                "He puts his paw on your forearm.",
                "Just rests it there. Heavy and warm.",
                "Looks up at you with those soft eyes.",
                "You don't know how he always knows.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(3), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "personality_gentle_slow_walk",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DogPersonalityIs(DogPersonality::Gentle),
                SceneTrigger::DayRange(8, 15),
                SceneTrigger::BondAbove(22),
                SceneTrigger::ActionTaken("walk_dog"),
            ]),
            lines: vec![
                "He doesn't pull the leash. Never has.",
                "Walks right beside you. Matches your pace exactly.",
                "When you stop, he stops. When you breathe deep, he does too.",
                "It's less like walking a dog",
                "and more like walking with a friend.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "personality_mischievous_sock_bandit",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DogPersonalityIs(DogPersonality::Mischievous),
                SceneTrigger::DayRange(3, 8),
                SceneTrigger::BondAbove(8),
            ]),
            lines: vec![
                "You're missing a sock. Again.",
                "He's under the table. Tail wagging.",
                "He has the sock. Of course he has the sock.",
                "You reach for it. He darts away.",
                "This is his favorite game and you didn't agree to play.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "personality_mischievous_food_heist",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DogPersonalityIs(DogPersonality::Mischievous),
                SceneTrigger::DayRange(5, 12),
                SceneTrigger::BondAbove(15),
            ]),
            lines: vec![
                "You left a sandwich on the counter.",
                "You come back. The sandwich is gone.",
                "He's lying in his bed, looking innocent.",
                "There's a piece of lettuce on his nose.",
                "He doesn't even like lettuce.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(1), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "personality_mischievous_door_escape",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DogPersonalityIs(DogPersonality::Mischievous),
                SceneTrigger::DayRange(8, 15),
                SceneTrigger::BondAbove(22),
            ]),
            lines: vec![
                "The door opens and he bolts.",
                "Full speed into the yard. Rolls in something.",
                "You call his name. He looks at you.",
                "Rolls in it again. Deliberately.",
                "He comes back inside looking extremely pleased with himself.",
                "The bath takes forty minutes.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "personality_loyal_door_guard",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DogPersonalityIs(DogPersonality::Loyal),
                SceneTrigger::DayRange(3, 8),
                SceneTrigger::BondAbove(8),
            ]),
            lines: vec![
                "Someone knocks on the door.",
                "He's there before you are. Standing between you and the door.",
                "Not barking. Just standing.",
                "You open it. It's the mail carrier.",
                "He doesn't move until you do.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "personality_loyal_wait_by_door",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DogPersonalityIs(DogPersonality::Loyal),
                SceneTrigger::DayRange(5, 12),
                SceneTrigger::BondAbove(15),
            ]),
            lines: vec![
                "You come home and he's exactly where he was when you left.",
                "By the door. Facing it.",
                "His tail starts before the door is fully open.",
                "You wonder how long he sits there.",
                "You suspect it's the whole time.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(3),
                SceneEffect::Peace(1),
                SceneEffect::UnlockMemory(
                    "loyal_wait".to_string(),
                    "He waited by the door the whole time".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "personality_loyal_follows_everywhere",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DogPersonalityIs(DogPersonality::Loyal),
                SceneTrigger::DayRange(8, 15),
                SceneTrigger::BondAbove(22),
            ]),
            lines: vec![
                "Kitchen. He follows.",
                "Bathroom. He follows.",
                "You stop in the hallway just to test it.",
                "He sits down next to you. Looks up.",
                "\"You don't have to follow me everywhere.\"",
                "He doesn't move.",
                "You wouldn't have it any other way.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(3), SceneEffect::Peace(2)],
        },
    ]
}
