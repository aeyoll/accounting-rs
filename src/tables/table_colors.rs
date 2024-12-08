use ratatui::prelude::Color;
use ratatui::style::palette::tailwind;
use ratatui::style::palette::tailwind::Palette;
use ratatui::style::Style;

pub struct TableColors {
    pub header_bg: Color,
    pub header_fg: Color,

    pub normal_row_fg: Color,
    pub alt_row_fg: Color,
    pub selected_row_fg: Color,

    pub normal_row_bg: Color,
    pub alt_row_bg: Color,
    pub selected_row_bg: Color,
}

impl TableColors {
    pub fn new(palette: Palette) -> Self {
        Self {
            header_bg: palette.c700,
            header_fg: tailwind::SLATE.c50,

            normal_row_fg: tailwind::SLATE.c50,
            alt_row_fg: tailwind::SLATE.c50,
            selected_row_fg: tailwind::SLATE.c50,

            normal_row_bg: tailwind::SLATE.c950,
            alt_row_bg: tailwind::SLATE.c700,
            selected_row_bg: palette.c700,
        }
    }

    pub fn get_header_style(&self) -> Style {
        Style::default().fg(self.header_fg).bg(self.header_bg)
    }

    pub fn get_row_style(&self, is_alt: bool, is_selected: bool) -> Style {
        let mut color = match is_alt {
            true => self.alt_row_fg,
            false => self.normal_row_fg,
        };

        let mut background = match is_alt {
            true => self.alt_row_bg,
            false => self.normal_row_bg,
        };

        if is_selected {
            color = self.selected_row_fg;
            background = self.selected_row_bg;
        }

        Style::default().fg(color).bg(background)
    }
}
