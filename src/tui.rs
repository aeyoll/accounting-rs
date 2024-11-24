use crate::models::Account;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::style::palette::tailwind;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Padding, Widget};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    symbols,
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    DefaultTerminal, Frame,
};
use std::io;
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

fn render_title(area: Rect, buf: &mut Buffer) {
    "Accounting".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

struct TabWidget<'a> {
    tab: SelectedTab,
    state: &'a AppState,
}

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
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
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
    fn title(self) -> Line<'static> {
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

    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Account => tailwind::GRAY,
            Self::People => tailwind::YELLOW,
            Self::Expenses => tailwind::GREEN,
            Self::Balance => tailwind::RED,
        }
    }
}

pub struct AppState {
    pub account: Account,
}

pub struct App {
    pub state: AppState,
    pub selected_tab: SelectedTab,
    pub exit: bool,
}

impl App {
    pub fn new() -> Result<App, anyhow::Error> {
        Ok(App {
            state: AppState {
                account: Account::load_from_file()?,
            },
            selected_tab: SelectedTab::Account,
            exit: false,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
            KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
            KeyCode::Tab => {
                self.selected_tab = match self.selected_tab {
                    SelectedTab::Account => SelectedTab::People,
                    SelectedTab::People => SelectedTab::Expenses,
                    SelectedTab::Expenses => SelectedTab::Balance,
                    SelectedTab::Balance => SelectedTab::Account,
                }
            }
            _ => {}
        }
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;

        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        TabWidget {
            tab: self.selected_tab,
            state: &self.state,
        }
        .render(inner_area, buf);
        render_footer(footer_area, buf);
    }
}
