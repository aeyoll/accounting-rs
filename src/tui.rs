use std::cmp::PartialEq;
use crate::models::Account;
use crate::selected_tab::SelectedTab;
use crate::tab_widget::{PeopleViewState, TabWidget};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{ListState, Widget};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::Tabs,
    DefaultTerminal, Frame,
};
use std::io;
use strum::IntoEnumIterator;

fn render_title(area: Rect, buf: &mut Buffer) {
    "Accounting".bold().render(area, buf);
}

const INFO_TEXT: [&str; 2] = ["◄ ► to change tab", "Press q to quit"];
const PEOPLE_INFO_TEXT: [&str; 5] = [
    "(↑) Move up",
    "(↓) Move down",
    "(Enter) Edit",
    "(Esc) Cancel",
    "Press q to quit",
];

fn render_footer(area: Rect, buf: &mut Buffer, selected_tab: SelectedTab) {
    if selected_tab == SelectedTab::People {
        Line::raw(PEOPLE_INFO_TEXT.join(" | "))
            .centered()
            .render(area, buf);
    } else {
        Line::raw(INFO_TEXT.join(" | "))
            .centered()
            .render(area, buf);
    }
}

pub struct AppState {
    pub account: Account,
    pub people_list_state: ListState,
    pub people_view_state: PeopleViewState,
}

impl AppState {
    fn new(account: Account) -> Self {
        let mut people_list_state = ListState::default();
        people_list_state.select(Some(0)); // Select first item by default

        Self {
            account,
            people_list_state,
            people_view_state: PeopleViewState::default(),
        }
    }
}

pub struct App {
    pub state: AppState,
    pub selected_tab: SelectedTab,
    pub exit: bool,
}

impl App {
    pub fn new() -> Result<App, anyhow::Error> {
        Ok(App {
            state: AppState::new(Account::load_from_file()?),
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
            KeyCode::Up if self.selected_tab == SelectedTab::People => {
                if let PeopleViewState::List = self.state.people_view_state {
                    let current = self.state.people_list_state.selected().unwrap_or(0);
                    self.state
                        .people_list_state
                        .select(Some(current.saturating_sub(1)));
                }
            }
            KeyCode::Down if self.selected_tab == SelectedTab::People => {
                if let PeopleViewState::List = self.state.people_view_state {
                    let current = self.state.people_list_state.selected().unwrap_or(0);
                    let max = self.state.account.persons.len().saturating_sub(1);
                    self.state
                        .people_list_state
                        .select(Some(current.saturating_add(1).min(max)));
                }
            }
            KeyCode::Enter if self.selected_tab == SelectedTab::People => {
                if let PeopleViewState::List = self.state.people_view_state {
                    if self.state.people_list_state.selected().is_some() {
                        self.state.people_view_state = PeopleViewState::EditForm;
                    }
                }
            }
            KeyCode::Esc if self.selected_tab == SelectedTab::People => {
                if let PeopleViewState::EditForm = self.state.people_view_state {
                    self.state.people_view_state = PeopleViewState::List;
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
        let tab_widget = TabWidget {
            tab: self.selected_tab,
            state: &self.state,
        }
        .render(inner_area, buf);
        render_footer(footer_area, buf, self.selected_tab);
    }
}
