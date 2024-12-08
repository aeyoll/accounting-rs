use crate::models::Person;
use crate::tables::table_colors::TableColors;
use ratatui::{
    layout::{Constraint},
    style::palette::tailwind,
    style::{ Stylize},
    widgets::{Cell, ListState, Row, Table},
};

pub struct PersonTable<'a> {
    pub persons: &'a Vec<Person>,
    pub state: &'a ListState,
    colors: TableColors,
}

impl<'a> PersonTable<'a> {
    pub fn new(persons: &'a Vec<Person>, state: &'a ListState) -> Self {
        Self {
            persons,
            state,
            colors: TableColors::new(tailwind::YELLOW),
        }
    }

    pub fn get(self) -> Table<'static> {
        let header = Row::new(vec!["Name".bold(), "Income".bold()])
            .style(self.colors.get_header_style())
            .height(1);

        let is_alt = |i: usize| i % 2 == 0;
        let is_selected = |i: usize| self.state.selected().unwrap_or(0) == i;

        let rows = self.persons.iter().enumerate().map(|(i, p)| {
            Row::new(vec![
                Cell::from(p.name.clone()),
                Cell::from(format!("{:.2}€", p.income)),
            ])
            .style(self.colors.get_row_style(is_alt(i), is_selected(i)))
            .height(1)
        });

        Table::new(
            rows,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .header(header)
    }
}
