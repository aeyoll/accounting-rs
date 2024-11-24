use crate::selected_tab::SelectedTab;
use crate::tui::AppState;

pub struct TabWidget<'a> {
    pub tab: SelectedTab,
    pub state: &'a AppState,
}
