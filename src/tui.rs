use crate::models::Account;
use crate::selected_tab::SelectedTab;
use crate::tab_widget::TabWidget;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Widget;
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

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
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
