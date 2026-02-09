use ember::prelude::TermColor;

use super::screen::{BarColors, ScreenBuffer};
use super::state::{Act, GameState};

pub fn render_hud(screen: &mut ScreenBuffer, game: &GameState) {
    let inner_width = screen.width.saturating_sub(4);
    let bar_width = 10;

    let day_text = match game.act {
        Act::Life => format!("Day {} / 30", game.day),
        Act::Grief => format!("Day {}", game.day),
    };
    screen.write_text(2, 1, &day_text, TermColor::White, TermColor::Black);

    let money_text = format!("${}", game.money);
    let energy_text = game.energy_display();
    let time_text = game.time_display();
    let money_energy = format!("{}  {}  {}", money_text, energy_text, time_text);
    screen.write_text(2, 2, &money_energy, TermColor::Yellow, TermColor::Black);

    let bar_column = inner_width.saturating_sub(bar_width + 12);

    screen.write_text(bar_column, 1, "Career ", TermColor::Cyan, TermColor::Black);
    let career_colors = BarColors {
        filled: TermColor::Cyan,
        empty: TermColor::DarkGrey,
        background: TermColor::Black,
    };
    screen.draw_bar(
        bar_column + 7,
        1,
        bar_width,
        game.career as f64 / 100.0,
        &career_colors,
    );
    let career_pct = format!(" {}%", game.career.min(100));
    screen.write_text(
        bar_column + 7 + bar_width,
        1,
        &career_pct,
        TermColor::Cyan,
        TermColor::Black,
    );

    screen.write_text(bar_column, 2, "Home   ", TermColor::Green, TermColor::Black);
    let home_colors = BarColors {
        filled: TermColor::Green,
        empty: TermColor::DarkGrey,
        background: TermColor::Black,
    };
    screen.draw_bar(
        bar_column + 7,
        2,
        bar_width,
        game.home as f64 / 100.0,
        &home_colors,
    );
    let home_pct = format!(" {}%", game.home.min(100));
    screen.write_text(
        bar_column + 7 + bar_width,
        2,
        &home_pct,
        TermColor::Green,
        TermColor::Black,
    );

    let peace_color = peace_bar_color(game.peace);
    screen.write_text(bar_column, 3, "Peace  ", peace_color, TermColor::Black);
    let peace_colors = BarColors {
        filled: peace_color,
        empty: TermColor::DarkGrey,
        background: TermColor::Black,
    };
    screen.draw_bar(
        bar_column + 7,
        3,
        bar_width,
        game.peace as f64 / 100.0,
        &peace_colors,
    );
    let peace_pct = format!(" {}%", game.peace.min(100));
    screen.write_text(
        bar_column + 7 + bar_width,
        3,
        &peace_pct,
        peace_color,
        TermColor::Black,
    );
}

fn peace_bar_color(peace: u32) -> TermColor {
    match peace {
        80..=u32::MAX => TermColor::Green,
        50..=79 => TermColor::Yellow,
        25..=49 => TermColor::Red,
        _ => TermColor::Grey,
    }
}
