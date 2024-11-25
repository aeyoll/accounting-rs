use crate::selected_tab::SelectedTab;
use crate::tui::AppState;

// Add this new enum to handle different view states
#[derive(Default)]
pub enum PeopleViewState {
    #[default]
    List,
    EditForm,
}

pub struct TabWidget<'a> {
    pub tab: SelectedTab,
    pub state: &'a AppState,
}
