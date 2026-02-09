use super::scenes::{SceneDef, SceneEffect, SceneTrigger};
use super::state::Act;

pub fn staple_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "staple_the_door",
            trigger: SceneTrigger::Day(2),
            lines: vec![
                "You come home from work.",
                "Before you even get through the door, you hear it.",
                "Nails on hardwood. Scrambling.",
                "The door opens and there he is.",
                "Tail going so fast his whole body shakes.",
                "He doesn't even wait for you to put your bag down.",
                "Just pushes his head into your hand.",
                "You didn't know you needed this today.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(3),
                SceneEffect::UnlockMemory(
                    "the_door".to_string(),
                    "The way he greeted you at the door".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "staple_the_noise",
            trigger: SceneTrigger::Day(5),
            lines: vec![
                "It's late. The house is quiet.",
                "Then you hear it. A small whimper from the foot of the bed.",
                "You look down. He's dreaming.",
                "His paws twitch. A soft woof escapes.",
                "You reach down and put your hand on his side.",
                "He settles immediately. Sighs.",
                "You stay like that for a while.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(2),
                SceneEffect::UnlockMemory(
                    "the_noise".to_string(),
                    "His dreaming sounds at night".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "staple_the_warmth",
            trigger: SceneTrigger::Day(8),
            lines: vec![
                "It's cold outside. Rain on the windows.",
                "You're on the couch. He's next to you.",
                "Not on the couch \u{2014} he knows better \u{2014} but pressed against it.",
                "His warmth seeps through the cushion into your leg.",
                "You haven't moved in an hour.",
                "Neither has he.",
                "This is enough.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(3),
                SceneEffect::Peace(2),
                SceneEffect::UnlockMemory(
                    "the_warmth".to_string(),
                    "The warmth of him against the couch".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "staple_the_gift",
            trigger: SceneTrigger::Day(11),
            lines: vec![
                "You find it on your pillow.",
                "A sock. Just a sock.",
                "He's sitting at the bedroom door, watching you.",
                "Tail wagging slowly. Proud.",
                "He brought you something.",
                "It's the most disgusting sock you've ever seen.",
                "You tell him he's a good boy.",
                "He already knew.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(2),
                SceneEffect::Peace(1),
                SceneEffect::UnlockMemory(
                    "the_gift".to_string(),
                    "The sock he left on your pillow".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "staple_the_look",
            trigger: SceneTrigger::Day(15),
            lines: vec![
                "You're in the kitchen making coffee.",
                "You turn around and he's just... looking at you.",
                "Not begging. Not asking for anything.",
                "Just looking.",
                "His eyes are soft. His ears are relaxed.",
                "It's the simplest thing in the world.",
                "And it undoes you a little.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(4),
                SceneEffect::UnlockMemory(
                    "the_look".to_string(),
                    "The way he looked at you in the kitchen".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "staple_the_slow",
            trigger: SceneTrigger::Day(19),
            lines: vec![
                "He doesn't get up when you come home today.",
                "He lifts his head. Tail moves once. Twice.",
                "But he doesn't get up.",
                "You go to him instead.",
                "Sit on the floor next to him.",
                "Put your hand on his side. He's warm.",
                "His breathing is slow. Steady.",
                "You stay there until the light changes.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(5),
                SceneEffect::UnlockMemory(
                    "the_slow".to_string(),
                    "The day he stopped getting up to greet you".to_string(),
                    true,
                ),
            ],
        },
    ]
}

pub fn interrupt_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "career_first_deadline",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(3, 7),
                SceneTrigger::CareerAbove(10),
            ]),
            lines: vec![
                "Your first real deadline.",
                "You stayed up too late working on it.",
                "But you got it done.",
                "Your manager nods. \"Good work.\"",
                "Two words. They matter more than they should.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Career(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "interrupt_coworker",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(5, 11),
                SceneTrigger::CareerAbove(15),
            ]),
            lines: vec![
                "A coworker asks about your dog.",
                "You show them a photo. Then another.",
                "Then another.",
                "\"You really love that dog, huh?\"",
                "\"Yeah,\" you say. \"I really do.\"",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(1), SceneEffect::Bond(1)],
        },
        SceneDef {
            id: "interrupt_power_outage",
            trigger: SceneTrigger::DayRange(7, 11),
            lines: vec![
                "The power goes out.",
                "The whole street is dark.",
                "She lights candles. You make sandwiches.",
                "He's nervous at first. Then he settles.",
                "You eat by candlelight. It's weirdly nice.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::PartnerInvestment(1),
                SceneEffect::Peace(1),
                SceneEffect::Bond(1),
            ],
        },
        SceneDef {
            id: "interrupt_phone_call",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(4, 9),
                SceneTrigger::PartnerInvestmentAbove(6),
            ]),
            lines: vec![
                "She calls you at work.",
                "\"Nothing important. Just wanted to hear your voice.\"",
                "You smile at your desk like an idiot.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "interrupt_mail_surprise",
            trigger: SceneTrigger::DayRange(9, 13),
            lines: vec![
                "A package arrives. You didn't order anything.",
                "It's a dog toy. She ordered it.",
                "\"He deserves something new,\" she says.",
                "He sniffs the toy. Carries it to his bed.",
                "Doesn't chew it. Just holds it.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(1), SceneEffect::Bond(1)],
        },
        SceneDef {
            id: "interrupt_flat_tire",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::DayRange(8, 15),
                SceneTrigger::CareerAbove(20),
            ]),
            lines: vec![
                "Flat tire on the way to work.",
                "You're going to be late.",
                "A stranger stops. Helps you change it.",
                "Doesn't ask for anything. Just helps.",
                "\"Pay it forward,\" they say.",
                "You think about that all day.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "interrupt_old_photo",
            trigger: SceneTrigger::DayRange(12, 17),
            lines: vec![
                "You find an old photo while cleaning.",
                "You and her. Years ago.",
                "You both look so young.",
                "She sees you looking at it.",
                "\"We were babies,\" she says.",
                "\"We still are,\" you say.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::PartnerInvestment(2), SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "interrupt_good_weather",
            trigger: SceneTrigger::DayRange(10, 15),
            lines: vec![
                "Perfect weather today.",
                "The kind of day where everything feels possible.",
                "You open all the windows.",
                "He lies in the breeze. She hums in the kitchen.",
                "Days like this don't last. But they matter.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(2)],
        },
    ]
}

pub fn ambient_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "ambient_rain",
            trigger: SceneTrigger::DayRange(6, 10),
            lines: vec![
                "It rains all day.",
                "The sound on the roof is steady. Constant.",
                "You find yourself staring out the window more than usual.",
                "There's something comforting about a day like this.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(1)],
        },
        SceneDef {
            id: "ambient_sunday",
            trigger: SceneTrigger::Day(9),
            lines: vec![
                "Two weeks already.",
                "It doesn't feel like it.",
                "You look around the house. It looks lived in now.",
                "Your stuff mixed with her stuff mixed with his.",
                "This is a life. Your life.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Peace(2)],
        },
    ]
}

pub fn ng_plus_echo_scenes() -> Vec<SceneDef> {
    vec![
        SceneDef {
            id: "echo_deja_vu_door",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::IsNgPlus,
                SceneTrigger::Day(2),
                SceneTrigger::ActIs(Act::Life),
            ]),
            lines: vec![
                "You come home.",
                "Before you even get through the door \u{2014}",
                "Wait. You've... been here before.",
                "The scrambling of paws. The push of a head into your hand.",
                "It's the same. Exactly the same.",
                "But it's different too. You know that now.",
                "You hold onto the moment a little longer this time.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(3), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "echo_deja_vu_warmth",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::IsNgPlus,
                SceneTrigger::Day(8),
                SceneTrigger::ActIs(Act::Life),
            ]),
            lines: vec![
                "You're on the couch. He's pressed against it.",
                "His warmth seeps through.",
                "You've felt this before. You're sure of it.",
                "Like a memory that hasn't happened yet.",
                "This time, you pay attention to every second.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(3), SceneEffect::Peace(3)],
        },
        SceneDef {
            id: "echo_deja_vu_gift",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::IsNgPlus,
                SceneTrigger::Day(13),
                SceneTrigger::ActIs(Act::Life),
            ]),
            lines: vec![
                "You find it on your pillow. A sock.",
                "Your hands are shaking and you don't know why.",
                "He's at the door. Watching. Tail going.",
                "You've done this before. You're certain.",
                "You pick him up. Hold him close.",
                "\"I know,\" you whisper. \"I know.\"",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(4),
                SceneEffect::Peace(3),
                SceneEffect::UnlockMemory(
                    "echo_gift".to_string(),
                    "The sock again, and the feeling you couldn't explain".to_string(),
                    true,
                ),
            ],
        },
        SceneDef {
            id: "echo_deja_vu_look",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::IsNgPlus,
                SceneTrigger::Day(17),
                SceneTrigger::ActIs(Act::Life),
            ]),
            lines: vec![
                "He's looking at you from across the kitchen.",
                "That look. You've seen it before.",
                "Not from him. From the him that came before.",
                "Soft eyes. Relaxed ears. Perfectly still.",
                "You know what this means now.",
                "You knew then too. You just didn't have the words.",
                "",
                "[Press any key]",
            ],
            effects: vec![SceneEffect::Bond(4), SceneEffect::Peace(2)],
        },
        SceneDef {
            id: "echo_deja_vu_slow",
            trigger: SceneTrigger::Compound(vec![
                SceneTrigger::IsNgPlus,
                SceneTrigger::Day(19),
                SceneTrigger::ActIs(Act::Life),
            ]),
            lines: vec![
                "He's slowing down.",
                "You knew this was coming. You always know.",
                "But knowing doesn't make it easier.",
                "You sit on the floor next to him.",
                "Put your hand on his side. He's warm.",
                "This time, you're ready.",
                "This time, you won't waste a single day.",
                "",
                "[Press any key]",
            ],
            effects: vec![
                SceneEffect::Bond(5),
                SceneEffect::Peace(2),
                SceneEffect::UnlockMemory(
                    "echo_slow".to_string(),
                    "Knowing what was coming, and staying anyway".to_string(),
                    true,
                ),
            ],
        },
    ]
}
