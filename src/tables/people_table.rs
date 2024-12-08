use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::palette::tailwind,
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Cell, HighlightSpacing, ListState, Padding, Row, Table, Widget},
};

use crate::models::Person;

struct TableColors {
    header_bg: Color,
    header_fg: Color,

    normal_row_fg: Color,
    alt_row_fg: Color,
    selected_row_fg: Color,

    normal_row_bg: Color,
    alt_row_bg: Color,
    selected_row_bg: Color,
}

impl TableColors {
    const fn new() -> Self {
        Self {
            header_bg: tailwind::YELLOW.c700,
            header_fg: tailwind::SLATE.c50,

            normal_row_fg: tailwind::SLATE.c50,
            alt_row_fg: tailwind::SLATE.c50,
            selected_row_fg: tailwind::SLATE.c50,

            normal_row_bg: tailwind::SLATE.c950,
            alt_row_bg: tailwind::SLATE.c700,
            selected_row_bg: tailwind::YELLOW.c700,
        }
    }
}

pub struct PeopleTable<'a> {
    pub persons: &'a Vec<Person>,
    pub state: &'a ListState,
    colors: TableColors,
}

impl<'a> PeopleTable<'a> {
    pub fn new(persons: &'a Vec<Person>, state: &'a ListState) -> Self {
        Self {
            persons,
            state,
            colors: TableColors::new(),
        }
    }

    pub fn get(self) -> Table<'static> {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);

        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_fg);

        let header = Row::new(vec!["Name".bold(), "Income".bold()])
            .style(header_style)
            .height(1);

        let rows = self.persons.iter().enumerate().map(|(i, p)| {
            let mut color = match i % 2 {
                0 => self.colors.normal_row_fg,
                _ => self.colors.alt_row_fg,
            };

            let mut background = match i % 2 {
                0 => self.colors.normal_row_bg,
                _ => self.colors.alt_row_bg,
            };

            if self.state.selected().unwrap_or(0) == i {
                color = self.colors.selected_row_fg;
                background = self.colors.selected_row_bg;
            }

            let style = Style::default().fg(color).bg(background);

            Row::new(vec![
                Cell::from(p.name.clone()),
                Cell::from(format!("{:.2}€", p.income)),
            ])
            .style(style)
            .height(1)
        });

        Table::new(
            rows,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .header(header)
        .row_highlight_style(selected_style)
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol("▶ ")
    }
}
