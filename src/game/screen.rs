use ember::prelude::*;

use super::vibrance::apply_vibrance;

pub struct BarColors {
    pub filled: TermColor,
    pub empty: TermColor,
    pub background: TermColor,
}

pub struct ScreenBuffer {
    entity: Entity,
    pub width: usize,
    pub height: usize,
    cells: Vec<TilemapCell>,
}

impl ScreenBuffer {
    pub fn new(world: &mut World, width: usize, height: usize) -> Self {
        let tilemap = Tilemap::new(width, height);
        let entity = EntityBuilder::new()
            .position(Position {
                column: 0.0,
                row: 0.0,
            })
            .tilemap(tilemap)
            .z_index(ZIndex(0))
            .spawn(world);

        Self {
            entity,
            width,
            height,
            cells: vec![
                TilemapCell {
                    character: ' ',
                    foreground: TermColor::White,
                    background: TermColor::Black,
                };
                width * height
            ],
        }
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.character = ' ';
            cell.foreground = TermColor::White;
            cell.background = TermColor::Black;
        }
    }

    pub fn set_cell(
        &mut self,
        column: usize,
        row: usize,
        character: char,
        foreground: TermColor,
        background: TermColor,
    ) {
        if column < self.width && row < self.height {
            let index = row * self.width + column;
            self.cells[index] = TilemapCell {
                character,
                foreground,
                background,
            };
        }
    }

    pub fn write_text(
        &mut self,
        column: usize,
        row: usize,
        text: &str,
        foreground: TermColor,
        background: TermColor,
    ) {
        for (char_index, character) in text.chars().enumerate() {
            let target_column = column + char_index;
            if target_column < self.width && row < self.height {
                let index = row * self.width + target_column;
                self.cells[index] = TilemapCell {
                    character,
                    foreground,
                    background,
                };
            }
        }
    }

    pub fn write_centered(
        &mut self,
        row: usize,
        text: &str,
        foreground: TermColor,
        background: TermColor,
    ) {
        let text_len = text.chars().count();
        let column = if text_len < self.width {
            (self.width - text_len) / 2
        } else {
            0
        };
        self.write_text(column, row, text, foreground, background);
    }

    pub fn write_wrapped(
        &mut self,
        column: usize,
        row: usize,
        text: &str,
        max_width: usize,
        foreground: TermColor,
        background: TermColor,
    ) -> usize {
        let lines = word_wrap(text, max_width);
        for (line_index, line) in lines.iter().enumerate() {
            let target_row = row + line_index;
            if target_row < self.height {
                self.write_text(column, target_row, line, foreground, background);
            }
        }
        lines.len()
    }

    pub fn draw_box(
        &mut self,
        column: usize,
        row: usize,
        box_width: usize,
        box_height: usize,
        foreground: TermColor,
        background: TermColor,
    ) {
        if box_width < 2 || box_height < 2 {
            return;
        }

        self.set_cell(column, row, '\u{2554}', foreground, background);
        self.set_cell(
            column + box_width - 1,
            row,
            '\u{2557}',
            foreground,
            background,
        );
        self.set_cell(
            column,
            row + box_height - 1,
            '\u{255A}',
            foreground,
            background,
        );
        self.set_cell(
            column + box_width - 1,
            row + box_height - 1,
            '\u{255D}',
            foreground,
            background,
        );

        for horizontal_index in 1..box_width - 1 {
            self.set_cell(
                column + horizontal_index,
                row,
                '\u{2550}',
                foreground,
                background,
            );
            self.set_cell(
                column + horizontal_index,
                row + box_height - 1,
                '\u{2550}',
                foreground,
                background,
            );
        }

        for vertical_index in 1..box_height - 1 {
            self.set_cell(
                column,
                row + vertical_index,
                '\u{2551}',
                foreground,
                background,
            );
            self.set_cell(
                column + box_width - 1,
                row + vertical_index,
                '\u{2551}',
                foreground,
                background,
            );
        }
    }

    pub fn draw_horizontal_divider(
        &mut self,
        column: usize,
        row: usize,
        divider_width: usize,
        foreground: TermColor,
        background: TermColor,
    ) {
        if divider_width < 2 {
            return;
        }

        self.set_cell(column, row, '\u{2560}', foreground, background);
        for horizontal_index in 1..divider_width - 1 {
            self.set_cell(
                column + horizontal_index,
                row,
                '\u{2550}',
                foreground,
                background,
            );
        }
        self.set_cell(
            column + divider_width - 1,
            row,
            '\u{2563}',
            foreground,
            background,
        );
    }

    pub fn draw_bar(
        &mut self,
        column: usize,
        row: usize,
        bar_width: usize,
        fraction: f64,
        colors: &BarColors,
    ) {
        let clamped = fraction.clamp(0.0, 1.0);
        let filled_count = (clamped * bar_width as f64).round() as usize;

        for bar_index in 0..bar_width {
            let (character, foreground) = if bar_index < filled_count {
                ('\u{2588}', colors.filled)
            } else {
                ('\u{2591}', colors.empty)
            };
            self.set_cell(
                column + bar_index,
                row,
                character,
                foreground,
                colors.background,
            );
        }
    }

    pub fn apply(&self, world: &mut World, vibrance: f64) {
        if let Some(tilemap) = world.get_tilemap_mut(self.entity) {
            if tilemap.width != self.width || tilemap.height != self.height {
                *tilemap = Tilemap::new(self.width, self.height);
            }
            for (index, cell) in self.cells.iter().enumerate() {
                tilemap.cells[index] = TilemapCell {
                    character: cell.character,
                    foreground: apply_vibrance(cell.foreground, vibrance),
                    background: apply_vibrance(cell.background, vibrance),
                };
            }
        }
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        self.width = new_width;
        self.height = new_height;
        self.cells = vec![
            TilemapCell {
                character: ' ',
                foreground: TermColor::White,
                background: TermColor::Black,
            };
            new_width * new_height
        ];
    }

    pub fn despawn(&self, world: &mut World) {
        world.despawn_entities(&[self.entity]);
    }
}
