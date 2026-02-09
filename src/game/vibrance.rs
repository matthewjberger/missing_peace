use ember::prelude::TermColor;

use super::state::{Act, GameState};

pub fn calculate_vibrance(game: &GameState) -> f64 {
    match game.act {
        Act::Life => {
            let bond_fraction = (game.bond as f64 / 60.0).min(1.0);
            0.15 + bond_fraction * 0.85
        }
        Act::Grief => {
            let peace_component = (game.peace as f64 / 50.0).min(1.0) * 0.55;
            let days_in_grief = (game.day.saturating_sub(30)) as f64;
            let time_component = (days_in_grief / 16.0).min(1.0) * 0.15;
            0.05 + peace_component + time_component
        }
    }
}

pub fn apply_vibrance(color: TermColor, vibrance: f64) -> TermColor {
    let (red, green, blue) = match color {
        TermColor::Black => return TermColor::Black,
        TermColor::Rgb { r, g, b } => (r, g, b),
        TermColor::White => (255, 255, 255),
        TermColor::Grey => (128, 128, 128),
        TermColor::DarkGrey => (64, 64, 64),
        TermColor::Red => (255, 0, 0),
        TermColor::DarkRed => (128, 0, 0),
        TermColor::Green => (0, 255, 0),
        TermColor::DarkGreen => (0, 128, 0),
        TermColor::Yellow => (255, 255, 0),
        TermColor::DarkYellow => (128, 128, 0),
        TermColor::Blue => (0, 0, 255),
        TermColor::DarkBlue => (0, 0, 128),
        TermColor::Magenta => (255, 0, 255),
        TermColor::DarkMagenta => (128, 0, 128),
        TermColor::Cyan => (0, 255, 255),
        TermColor::DarkCyan => (0, 128, 128),
    };

    let luminance = red as f64 * 0.299 + green as f64 * 0.587 + blue as f64 * 0.114;
    let clamped_vibrance = vibrance.clamp(0.0, 1.0);

    let result_red = (luminance + (red as f64 - luminance) * clamped_vibrance)
        .round()
        .clamp(0.0, 255.0) as u8;
    let result_green = (luminance + (green as f64 - luminance) * clamped_vibrance)
        .round()
        .clamp(0.0, 255.0) as u8;
    let result_blue = (luminance + (blue as f64 - luminance) * clamped_vibrance)
        .round()
        .clamp(0.0, 255.0) as u8;

    TermColor::Rgb {
        r: result_red,
        g: result_green,
        b: result_blue,
    }
}

pub fn atmosphere_text(game: &GameState) -> &'static str {
    match game.peace {
        0..=9 => "Everything feels grey.",
        10..=24 => "The world feels muted.",
        25..=39 => "Things are... okay.",
        40..=59 => "There's a quiet steadiness to things.",
        60..=79 => "The world feels brighter today.",
        _ => "The world feels vivid.",
    }
}
