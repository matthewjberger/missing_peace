mod game;

use game::intro::IntroState;
use game::state::SimulateMode;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let simulate = if args.iter().any(|arg| arg == "--simulate-slow") {
        SimulateMode::Slow
    } else if args.iter().any(|arg| arg == "--simulate") {
        SimulateMode::Fast
    } else {
        SimulateMode::Off
    };
    ember::prelude::launch(Box::new(IntroState::new(simulate)))
}
