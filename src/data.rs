#[derive(Clone)]
pub struct Project {
    pub title: String,
    pub status: String,
    pub description: String,
    pub status_tag: ProjectStatus,
}

#[derive(Clone, PartialEq)]
pub enum ProjectStatus {
    Red,
    Amber,
    Emerald,
    Cyan,
}
