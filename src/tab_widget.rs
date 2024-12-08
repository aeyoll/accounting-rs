use crate::selected_tab::SelectedTab;
use crate::tui::AppState;

#[derive(Default)]
pub enum PersonViewState {
    #[default]
    List,
    EditForm,
}

pub struct TabWidget<'a> {
    pub tab: SelectedTab,
    pub state: &'a AppState,
}
