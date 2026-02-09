use super::scenes::{SceneDef, SceneEffect, SceneTrigger};
use super::state::WifeMood;

pub fn wife_routine_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "wife_morning_coffee",
            trigger: SceneTrigger::Day(2),
            lines: vec![
                "She's already made coffee when you get to the kitchen.",
                "Your mug is on the counter. Right where you like it.",
                "\"Morning,\" she says without looking up from her phone.",
                "It's not much. But it's something.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(1)],
        },
        SceneDef {
            id: "wife_asks_about_work",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(3, 5),
                SceneTrigger::CareerAbove(5),
            ]),
            lines: vec![
                "\"How's work going?\" she asks over dinner.",
                "You tell her about the project. The deadlines.",
                "She listens. Really listens.",
                "\"You'll figure it out,\" she says. \"You always do.\"",
                "You're not sure that's true. But it helps to hear.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "wife_cooking_together",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(4, 10),
                SceneTrigger::PartnerInvestmentAbove(8),
            ]),
            lines: vec![
                "She's in the kitchen when you start cooking.",
                "\"Need a hand?\" she asks.",
                "You end up cooking together.",
                "She chops. You stir. You bump into each other.",
                "It's messy and inefficient and perfect.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::PartnerInvestment(3),
                SceneEffect::Peace(1),
                SceneEffect::UnlockMemory(
                    "cooking_together".to_string(),
                    "Cooking together in the kitchen".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "wife_laundry_fold",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(3, 8),
                SceneTrigger::PartnerInvestmentAbove(4),
            ]),
            lines: vec![
                "You're folding laundry.",
                "She sits down and starts matching socks.",
                "Neither of you says anything for a while.",
                "It's comfortable. The quiet kind of together.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(1), SceneEffect::Home(1)],
        },
        SceneDef {
            id: "wife_tv_argument",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(5, 9),
                SceneTrigger::PartnerInvestmentAbove(6),
            ]),
            lines: vec![
                "\"We are not watching that again.\"",
                "\"It's a good show!\"",
                "\"You've seen it four times.\"",
                "\"And I'll see it a fifth.\"",
                "She throws a pillow at you. You catch it.",
                "You watch her show. It's actually pretty good.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "wife_surprise_lunch",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(7, 12),
                SceneTrigger::PartnerInvestmentAbove(12),
            ]),
            lines: vec![
                "She made you lunch.",
                "It's in the fridge with a sticky note.",
                "Just a smiley face.",
                "The sandwich is better than anything you'd have made.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2)],
        },
        SceneDef {
            id: "wife_weekend_plan",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(8, 13),
                SceneTrigger::PartnerInvestmentAbove(15),
            ]),
            lines: vec![
                "\"We should do something this weekend.\"",
                "\"Like what?\"",
                "\"I don't know. Something.\"",
                "\"That's very specific.\"",
                "She smiles. \"Shut up. I'll think of something.\"",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "wife_garden_idea",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(9, 15),
                SceneTrigger::PartnerInvestmentAbove(18),
            ]),
            lines: vec![
                "\"I was thinking about the garden,\" she says.",
                "\"What about it?\"",
                "\"We should plant something. Something that comes back every year.\"",
                "You nod. \"I'd like that.\"",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2)],
        },
        SceneDef {
            id: "wife_sleeping_in",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(4, 11),
                SceneTrigger::PartnerInvestmentAbove(10),
            ]),
            lines: vec![
                "She's still asleep when you get up.",
                "Hair everywhere. One arm hanging off the bed.",
                "You pull the blanket back over her shoulder.",
                "She murmurs something. Doesn't wake up.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(1)],
        },
        SceneDef {
            id: "wife_remembers_thing",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(11, 16),
                SceneTrigger::PartnerInvestmentAbove(20),
            ]),
            lines: vec![
                "She bought your favorite snack.",
                "\"I was at the store and I remembered you like these.\"",
                "You didn't even know she noticed.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "wife_bad_day_comfort",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(5, 13),
                SceneTrigger::PartnerInvestmentAbove(14),
                SceneTrigger::PeaceBelow(35),
            ]),
            lines: vec![
                "She comes home quiet. Bad day.",
                "You don't ask. You make tea.",
                "Set it in front of her.",
                "She wraps both hands around the mug.",
                "\"Thanks,\" she says. Just that.",
                "Sometimes that's enough.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(3), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "wife_photo_frame",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(12, 17),
                SceneTrigger::PartnerInvestmentAbove(22),
            ]),
            lines: vec![
                "She put up a new photo on the mantle.",
                "The three of you. From that walk last week.",
                "You didn't even know she took it.",
                "You look happy in it. All of you.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::PartnerInvestment(2),
                SceneEffect::Peace(2),
                SceneEffect::UnlockMemory(
                    "photo_frame".to_string(),
                    "The photo she put on the mantle".to_string(),
                    true,
                ),
            ],
        },
    ]
}

pub fn wife_dog_adjacent_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "wife_worried",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(15, 19),
                SceneTrigger::PartnerInvestmentAbove(15),
            ]),
            lines: vec![
                "\"Have you noticed he's...?\" she starts.",
                "She doesn't finish.",
                "You both know.",
                "She puts her hand on yours.",
                "\"I'm glad he has us,\" she says.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::PartnerInvestment(2),
                SceneEffect::UnlockMemory(
                    "wife_worried".to_string(),
                    "When she asked if you'd noticed".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "wife_dog_training",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(3, 7),
                SceneTrigger::BondAbove(5),
                SceneTrigger::PartnerInvestmentAbove(5),
            ]),
            lines: vec![
                "She's trying to teach him to shake.",
                "\"Paw. Give me your paw.\"",
                "He lies down instead.",
                "She looks at you. \"Is he broken?\"",
                "\"He's perfect,\" you say.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(1), SceneEffect::Bond(1)],
        },
        SceneDef {
            id: "wife_dog_bed_argument",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(4, 9),
                SceneTrigger::BondAbove(8),
                SceneTrigger::PartnerInvestmentAbove(6),
            ]),
            lines: vec![
                "\"He's on the bed again.\"",
                "\"He's cold.\"",
                "\"He has a bed. A very expensive bed.\"",
                "\"But he likes our bed.\"",
                "She sighs. \"Fine. But he stays on your side.\"",
                "He is already on her side.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::PartnerInvestment(1),
                SceneEffect::Bond(2),
                SceneEffect::Peace(1),
            ],
        },
        SceneDef {
            id: "wife_catches_feeding",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(5, 11),
                SceneTrigger::BondAbove(12),
                SceneTrigger::PartnerInvestmentAbove(10),
            ]),
            lines: vec![
                "You catch her slipping him food under the table.",
                "She freezes. \"I wasn't \u{2014}\"",
                "\"I saw that.\"",
                "\"He looked at me with those eyes. What was I supposed to do?\"",
                "You both know this is a lost cause.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::PartnerInvestment(2),
                SceneEffect::Bond(1),
                SceneEffect::Peace(1),
            ],
        },
        SceneDef {
            id: "wife_walks_him",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(7, 13),
                SceneTrigger::BondAbove(15),
                SceneTrigger::PartnerInvestmentAbove(12),
            ]),
            lines: vec![
                "You come home and she's not there.",
                "Neither is he.",
                "You look out the window. There they are.",
                "Walking down the street. She's talking to him.",
                "He's looking up at her like she's saying something important.",
                "She probably is.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2), SceneEffect::Bond(2)],
        },
        SceneDef {
            id: "wife_dog_nickname",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(5, 12),
                SceneTrigger::BondAbove(10),
                SceneTrigger::PartnerInvestmentAbove(8),
            ]),
            lines: vec![
                "She's started calling him \"the baby.\"",
                "\"The baby needs his dinner.\"",
                "\"The baby is hogging the blanket.\"",
                "You don't correct her.",
                "She's not wrong.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(1), SceneEffect::Bond(1)],
        },
        SceneDef {
            id: "wife_dog_photo",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(8, 15),
                SceneTrigger::BondAbove(18),
                SceneTrigger::PartnerInvestmentAbove(14),
            ]),
            lines: vec![
                "Her phone background has changed.",
                "It used to be a sunset from that trip last year.",
                "Now it's him. Sleeping in the sunbeam.",
                "She catches you noticing. Shrugs.",
                "\"It's a good picture.\"",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2), SceneEffect::Bond(1)],
        },
        SceneDef {
            id: "wife_dog_vet_worry",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(16, 19),
                SceneTrigger::BondAbove(20),
                SceneTrigger::PartnerInvestmentAbove(18),
            ]),
            lines: vec![
                "\"Should we take him to the vet?\" she asks quietly.",
                "\"He's just... slowing down.\"",
                "\"He's old,\" you say.",
                "\"I know. I just...\"",
                "She doesn't finish. You pull her close.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::PartnerInvestment(3),
                SceneEffect::UnlockMemory(
                    "vet_worry".to_string(),
                    "When she asked about the vet".to_string(),
                    true,
                ),
            ],
        },
    ]
}

pub fn wife_late_game_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "wife_deep_talk",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(13, 17),
                SceneTrigger::PartnerInvestmentAbove(30),
            ]),
            lines: vec![
                "It's late. Neither of you can sleep.",
                "\"Do you ever think about the future?\" she asks.",
                "\"Sometimes.\"",
                "\"What do you see?\"",
                "You think about it.",
                "\"This. Just... more of this.\"",
                "She's quiet for a moment. Then: \"Good answer.\"",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::PartnerInvestment(3),
                SceneEffect::Peace(3),
                SceneEffect::UnlockMemory(
                    "deep_talk".to_string(),
                    "The late-night talk about the future".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "wife_anniversary_memory",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(15, 19),
                SceneTrigger::PartnerInvestmentAbove(32),
            ]),
            lines: vec![
                "\"Remember our first apartment?\" she says out of nowhere.",
                "\"The one with the leak.\"",
                "\"And the neighbor who played accordion at midnight.\"",
                "You both laugh. It's easy to laugh about it now.",
                "\"We've come a long way,\" she says.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(3), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "wife_gratitude",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(16, 20),
                SceneTrigger::PartnerInvestmentAbove(35),
            ]),
            lines: vec![
                "She catches your hand as you walk past.",
                "\"Hey.\"",
                "\"What?\"",
                "\"I just wanted to say... I'm glad it's you.\"",
                "\"That I'm stuck with, I mean.\"",
                "She smiles. The real one. The one you fell for.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::PartnerInvestment(4),
                SceneEffect::Peace(3),
                SceneEffect::UnlockMemory(
                    "glad_its_you".to_string(),
                    "\"I'm glad it's you\"".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "wife_dance_kitchen",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(13, 19),
                SceneTrigger::PartnerInvestmentAbove(28),
            ]),
            lines: vec![
                "Music from the other room. She's humming along.",
                "You walk in and she grabs your hand.",
                "\"Dance with me.\"",
                "\"There's no music in here.\"",
                "\"I'm humming. That counts.\"",
                "You dance badly in the kitchen for two minutes.",
                "He watches from his bed. Tail wagging.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::PartnerInvestment(3),
                SceneEffect::Peace(2),
                SceneEffect::Bond(1),
                SceneEffect::UnlockMemory(
                    "kitchen_dance".to_string(),
                    "Dancing badly in the kitchen".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "wife_team",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(17, 20),
                SceneTrigger::PartnerInvestmentAbove(38),
                SceneTrigger::HomeAbove(40),
            ]),
            lines: vec![
                "You look around the house.",
                "It's clean. Not perfect, but clean.",
                "There's dinner on the stove. Laundry folded.",
                "She's reading on the couch. He's at her feet.",
                "You built this. Together.",
                "It works.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(3), SceneEffect::Peace(4)],
        },
    ]
}

pub fn wife_milestone_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "partner_milestone_1",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::PartnerMilestone(1),
                SceneTrigger::WifeMoodIs(WifeMood::Neutral),
            ]),
            lines: vec![
                "She pauses in the doorway.",
                "\"Hey. I just wanted to say...\"",
                "She trails off. Tries again.",
                "\"I've noticed. The effort. I notice.\"",
                "She squeezes your arm and goes back to what she was doing.",
                "It's small. But it lands.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "partner_milestone_2",
            trigger: SceneTrigger::PartnerMilestone(2),
            lines: vec![
                "She's in the kitchen. You walk in.",
                "She turns around and just... looks at you.",
                "\"What?\" you ask.",
                "\"Nothing. I just like having you around.\"",
                "She turns back to the counter.",
                "You stand there for a moment, surprised.",
                "Then you smile.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::PartnerInvestment(3),
                SceneEffect::Peace(3),
                SceneEffect::UnlockMemory(
                    "partner_milestone_2".to_string(),
                    "\"I just like having you around\"".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "partner_milestone_3",
            trigger: SceneTrigger::PartnerMilestone(3),
            lines: vec![
                "Late evening. You're both on the couch.",
                "She leans against you. Quiet.",
                "\"I don't say this enough,\" she says.",
                "\"We're a good team. You know that, right?\"",
                "You put your arm around her.",
                "\"Yeah. I know.\"",
                "The house is quiet. The good kind of quiet.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::PartnerInvestment(4),
                SceneEffect::Peace(4),
                SceneEffect::UnlockMemory(
                    "partner_milestone_3".to_string(),
                    "\"We're a good team\"".to_string(),
                    true,
                ),
            ],
        },
    ]
}
