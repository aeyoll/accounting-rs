use crate::models::Account;
use crate::tab_widget::TabWidget;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Span, Style, Stylize, Text, Widget};
use ratatui::style::palette::tailwind;
use ratatui::symbols;
use ratatui::widgets::{Block, Borders, List, ListItem, Padding, Paragraph};
use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Account")]
    Account,
    #[strum(to_string = "People")]
    People,
    #[strum(to_string = "Expenses")]
    Expenses,
    #[strum(to_string = "Balance")]
    Balance,
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    pub(crate) fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub(crate) fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl<'a> Widget for TabWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.tab {
            SelectedTab::Account => self.tab.render_account(area, buf, &self.state.account),
            SelectedTab::People => self.tab.render_people(area, buf, &self.state.account),
            SelectedTab::Expenses => self.tab.render_expenses(area, buf, &self.state.account),
            SelectedTab::Balance => self.tab.render_balance(area, buf, &self.state.account),
        }
    }
}

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    pub(crate) fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    fn render_people(&self, area: Rect, buf: &mut Buffer, account: &Account) {
        let title = self.title();
        title.render(area, buf);

        let items: Vec<ListItem> = account
            .persons
            .iter()
            .map(|p| {
                ListItem::new(Line::from(vec![
                    Span::raw(&p.name),
                    Span::raw(" - "),
                    Span::raw(p.income.to_string()),
                ]))
            })
            .collect();

        List::new(items).block(self.block()).render(area, buf);
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

        List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .render(area, buf);
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

        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .render(area, buf);
    }

    fn render_account(&self, area: Rect, buf: &mut Buffer, account: &Account) {
        let text = Text::from(vec![Line::from(vec![
            Span::raw("Account Name: "),
            Span::styled(account.name.to_string(), Style::default().fg(Color::Yellow)),
        ])]);

        let block = Block::default().borders(Borders::ALL);

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }

    /// A block surrounding the tab's content
    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    pub(crate) const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Account => tailwind::GRAY,
            Self::People => tailwind::YELLOW,
            Self::Expenses => tailwind::GREEN,
            Self::Balance => tailwind::RED,
        }
    }
}
