use crate::models::Account;
use crate::tab_widget::{PeopleViewState, TabWidget};
use crate::tables::people_table::PeopleTable;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Span, Style, Stylize, Text, Widget};
use ratatui::style::palette::tailwind;
use ratatui::symbols;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph};
use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, Display, PartialEq, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "People")]
    People,
    #[strum(to_string = "Expenses")]
    Expenses,
    #[strum(to_string = "Balance")]
    Balance,
    #[strum(to_string = "Account")]
    Account,
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    pub fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl<'a> Widget for TabWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.tab {
            SelectedTab::People => self.tab.render_people(
                area,
                buf,
                &self.state.account,
                &self.state.people_list_state,
                &self.state.people_view_state,
            ),
            SelectedTab::Expenses => self.tab.render_expenses(area, buf, &self.state.account),
            SelectedTab::Balance => self.tab.render_balance(area, buf, &self.state.account),
            SelectedTab::Account => self.tab.render_account(area, buf, &self.state.account),
        }
    }
}

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    pub fn title(self) -> Line<'static> {
        format!("  {self}  ").fg(tailwind::SLATE.c50).into()
    }

    /// Render the people tab
    fn render_people(
        &self,
        area: Rect,
        buf: &mut Buffer,
        account: &Account,
        list_state: &ListState,
        view_state: &PeopleViewState,
    ) {
        match view_state {
            PeopleViewState::List => {
                PeopleTable::new(&account.persons, list_state)
                    .get()
                    .block(self.block())
                    .render(area, buf);
            }
            PeopleViewState::EditForm => {
                // Existing edit form code remains unchanged
                if let Some(selected) = list_state.selected() {
                    if let Some(person) = account.persons.get(selected) {
                        let form = vec![
                            Line::from(vec![
                                Span::raw("Name: "),
                                Span::styled(&person.name, Style::default().fg(Color::Yellow)),
                            ]),
                            Line::from(vec![
                                Span::raw("Income: "),
                                Span::styled(
                                    person.income.to_string(),
                                    Style::default().fg(Color::Yellow),
                                ),
                            ]),
                        ];

                        Paragraph::new(form)
                            .block(Block::default().title("Edit Person").borders(Borders::ALL))
                            .render(area, buf);
                    }
                }
            }
        }
    }

    fn render_expenses(&self, area: Rect, buf: &mut Buffer, account: &Account) {
        let items: Vec<ListItem> = account
            .expenses
            .iter()
            .map(|e| {
                ListItem::new(Line::from(vec![
                    Span::raw(&e.description),
                    Span::raw(" - "),
                    Span::raw(e.amount.to_string()),
                    Span::raw(" - "),
                    Span::raw(&e.person.name),
                ]))
            })
            .collect();

        List::new(items).block(self.block()).render(area, buf);
    }

    fn render_balance(&self, area: Rect, buf: &mut Buffer, account: &Account) {
        let total_income: f64 = account.persons.iter().map(|p| p.income).sum();
        let total_expenses: f64 = account.expenses.iter().map(|e| e.amount).sum();

        let text = Text::from(vec![
            Line::from(vec![
                Span::raw("Total Income: "),
                Span::raw(format!("{:.2}€", total_income)),
            ]),
            Line::from(vec![
                Span::raw("Total Expenses: "),
                Span::raw(format!("{:.2}€", total_expenses)),
            ]),
        ]);

        Paragraph::new(text).block(self.block()).render(area, buf);
    }

    fn render_account(&self, area: Rect, buf: &mut Buffer, account: &Account) {
        let text = Text::from(vec![Line::from(vec![
            Span::raw("Account Name: "),
            Span::styled(account.name.to_string(), Style::default().fg(Color::Yellow)),
        ])]);

        Paragraph::new(text)
            .centered()
            .block(self.block())
            .render(area, buf);
    }

    /// A block surrounding the tab's content
    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PLAIN)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
            .title(self.title())
            .title_alignment(ratatui::layout::Alignment::Center)
            .padding(Padding::uniform(1))
    }

    pub const fn palette(self) -> tailwind::Palette {
        match self {
            Self::People => tailwind::YELLOW,
            Self::Expenses => tailwind::GREEN,
            Self::Balance => tailwind::RED,
            Self::Account => tailwind::GRAY,
        }
    }
}
