#[derive(Clone)]
pub struct State {
    pub navigation: Navigation,
}

impl State {
    pub fn new () -> Self {
        State {
            navigation: Navigation::new(),
        }
    }
}

#[derive(Clone)]
pub struct Navigation {
    pub selected_tab: SelectedTab,
}

impl Navigation {
    pub fn new () -> Self {
        Navigation {
            selected_tab: SelectedTab::Home,
        }
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum SelectedTab {
    Home,
    Blog,
}
